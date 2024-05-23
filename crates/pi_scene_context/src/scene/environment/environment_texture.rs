use std::sync::Arc;

use pi_scene_shell::prelude::*;

#[derive(Debug, Default, Clone, Hash, Component)]
pub struct EnvTextureSlot(pub Option<Atom>, pub bool);

#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EnvIrradiance(pub Option<Arc<BindEnvIrradiance>>);

#[derive(Default, Clone, Deref, Hash, PartialEq, Eq, Component)]
pub struct EnvTexture(pub Option<Arc<ShaderBindEnvTexture>>);
impl From<ETextureViewUsage> for EnvTexture {
    fn from(value: ETextureViewUsage) -> Self { Self( Some(Arc::new(ShaderBindEnvTexture(BindDataTexture2D(value)))) ) }
}
impl From<Handle<ImageTextureView>> for EnvTexture {
    fn from(value: Handle<ImageTextureView>) -> Self { Self( Some(Arc::new(ShaderBindEnvTexture(BindDataTexture2D(ETextureViewUsage::Image(value))))) ) }
}
impl EnvTexture {
    pub fn irradiance(&self, allocator: &mut BindBufferAllocator) -> Option<Arc<BindEnvIrradiance>> {
        if let Some(tex) = &self.0 {
            match &tex.0.0 {
                ETextureViewUsage::Image(texture) => {
                    if let Some(result) = BindEnvIrradiance::new(allocator, texture.texture()) {
                        Some(Arc::new(result))                        
                    } else {
                        None
                    }
                },
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Component)]
pub struct EnvSampler(pub Option<Arc<ShaderBindEnvSampler>>);
impl EnvSampler {
    pub fn new(device: &RenderDevice, asset: &Share<AssetMgr<SamplerRes>>) -> Self {
        let desc = SamplerDesc::linear_clamp();
        if let Some(sampler) = BindDataSampler::create(desc, device, asset) {
            Self(Some(Arc::new(ShaderBindEnvSampler(sampler))))
        } else {
            Self(None)
        }
    }
}

pub fn sys_env_texture_load_launch(
    mut items: Query<(Entity, &EnvTextureSlot, &mut EnvTexture, &mut EnvIrradiance), Changed<EnvTextureSlot>>,
    loader: Res<ImageTextureViewLoader<EnvTextureSlot>>,
    // image_assets_mgr: Res<ShareAssetMgr<ImageTexture>>,
    imgtex_assets_mgr: Res<ShareAssetMgr<ImageTextureView>>,
    mut image_loader: ResMut<ImageTextureLoader>,
    mut state: ResMut<StateTextureLoader>,
    mut allocator: ResMut<ResBindBufferAllocator>,
) {
    items.iter_mut().for_each(|(entity, param, mut item, mut irradiance)| {
        let url = if let Some(v) = &param.0 { v } else { return; };

        state.texview_count += 1;
        let key = KeyImageTextureView::new(
            KeyImageTexture { url: url.clone(), file: param.1, depth_or_array_layers: 6, ..Default::default() },
            TextureViewDesc { base_mip_level: 0, array_layer_count: Some(6), ..Default::default() },
        );
        // let ekey = EKeyTexture::Image(key.clone());
        let key_u64 = key.asset_u64();

        match imgtex_assets_mgr.get(&key_u64) {
            Some(view) => {
                *item = EnvTexture::from(ETextureViewUsage::Image(view));
                irradiance.0 = item.irradiance(&mut allocator);
                state.texview_success += 1;
            },
            _ => {
                // let imgkey = key.url();
                let id = image_loader.create_load_env(key.url().clone());
                loader.wait.push((entity, key.clone(), id, 0));
            },
        }
    });
}

pub fn sys_env_texture_loaded_check(
    // entities: Query<Entity>,
    mut items: Query<(&EnvTextureSlot, &mut EnvTexture, &mut EnvIrradiance)>,
    // mut commands: Commands,
    loader: Res<ImageTextureViewLoader<EnvTextureSlot>>,
    imgtex_assets_mgr: Res<ShareAssetMgr<ImageTextureView>>,
    mut image_loader: ResMut<ImageTextureLoader>,
    mut state: ResMut<StateTextureLoader>,
    mut allocator: ResMut<ResBindBufferAllocator>,
) {
    let mut item = loader.wait.pop();
    let mut waitagain = vec![];
    while let Some((entity, key, id, _)) = item {
        item = loader.wait.pop();

        let key_u64 = key.asset_u64();
        // let imgkey = key.url();
        
        let viewkey = key.clone();
        if let Some(image) = image_loader.query_success(id) {
            let result = AssetMgr::load(&imgtex_assets_mgr, &key_u64);
            let (success, fail) = (loader.success.clone(), loader.fail.clone());
            let texkey = EKeyTexture::Image(key);
            RENDER_RUNTIME.spawn(async move {
                match ImageTextureView::async_load(image, viewkey, result).await {
                    Ok(res) => { success.push((entity, texkey, ETextureViewUsage::Image(res), 0)); }
                    Err(_e) => { fail.push((entity, texkey, 0)); }
                };
            })
            .unwrap();
        } else if let Some(_fail) = image_loader.query_failed_reason(id) {
            loader.fail.push((entity, EKeyTexture::Image(key), 0));
            state.texview_fail += 1;
        } else {
            waitagain.push((entity, key, id, 0));
        }
    }
    waitagain.drain(..).for_each(|item| { loader.wait.push(item) });
    state.texview_waiting = loader.wait.len() as u32;

    let mut item = loader.success.pop();
    while let Some((entity, _key, view, _)) = item {
        item = loader.success.pop();
        if let Ok((_, mut item, mut irradiance)) = items.get_mut(entity) {
            *item = EnvTexture::from(view);
            irradiance.0 = item.irradiance(&mut allocator);
            state.texview_success += 1;
        }
    }

    // let whitekey = KeyTexture::from(DefaultTexture::WHITE_2D);
    // // let white = EKeyTexture::Tex(whitekey.clone());
    // let key_u64 = whitekey.asset_u64();
    // let view = texres_assets_mgr.get(&key_u64).unwrap();
    // let mut item = loader.fail.pop();
    // while let Some((entity, _key)) = item {
    //     item = loader.fail.pop();
    //     if let Ok((_, mut item)) = items.get_mut(entity) {
    //         *item = EnvTexture::from(ETextureViewUsage::Tex(view.clone()));
    //         state.texview_success += 1;
    //     }
    //     // if let Some(mut cmd) = commands.get_entity(entity) {
    //     //     cmd.insert( D::from(ETextureViewUsage::Tex(view.clone())) );
    //     // }
    // }
}
