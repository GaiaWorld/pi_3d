
use std::mem::replace;

use derive_deref::{Deref, DerefMut};
use pi_assets::{mgr::AssetMgr, asset::{Handle, GarbageEmpty, Asset}};
use pi_ecs::{prelude::{Id, ResMut, Query, Res, Setup}, query::{Write, Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{assets::sync_load::{PluginAssetSyncLoad, InterfaceAssetSyncCreate, AssetSyncWait, PluginAssetSyncNotNeedLoad}, engine_shell::EnginShell};
use pi_render::rhi::device::RenderDevice;
use pi_scene_math::{Matrix, Matrix2, Vector4, Vector2, Number};
use pi_share::Share;
use render_shader::{shader::{KeyShaderEffect, ResShader, KeyShader, ShaderEffectMeta as ShaderEffectDesc}, skin_code::ESkinCode, scene_about_code::ERenderTag, unifrom_code::{UniformPropertyName, ErrorUniformSlot, TUnifromShaderProperty, MaterialValueBindDesc}};

use crate::{object::{ObjectID, GameObject}, renderers::render_mode::ERenderMode};

#[derive(Clone, Debug)]
pub struct UniformPropertyMat4(pub UniformPropertyName, pub Matrix);
impl TUnifromShaderProperty for UniformPropertyMat4 {
    fn tag(&self) -> &UniformPropertyName {
        &self.0
    }
}
#[derive(Clone, Debug)]
pub struct UniformPropertyMat2(pub UniformPropertyName, pub Matrix2);
impl TUnifromShaderProperty for UniformPropertyMat2 {
    fn tag(&self) -> &UniformPropertyName {
        &self.0
    }
}
#[derive(Clone, Debug)]
pub struct UniformPropertyVec4(pub UniformPropertyName, pub Vector4);
impl TUnifromShaderProperty for UniformPropertyVec4 {
    fn tag(&self) -> &UniformPropertyName {
        &self.0
    }
}
#[derive(Clone, Debug)]
pub struct UniformPropertyVec2(pub UniformPropertyName, pub Vector2);
impl TUnifromShaderProperty for UniformPropertyVec2 {
    fn tag(&self) -> &UniformPropertyName {
        &self.0
    }
}
#[derive(Clone, Debug)]
pub struct UniformPropertyFloat(pub UniformPropertyName, pub Number);
impl TUnifromShaderProperty for UniformPropertyFloat {
    fn tag(&self) -> &UniformPropertyName {
        &self.0
    }
}
#[derive(Clone, Debug)]
pub struct UniformPropertyInt(pub UniformPropertyName, pub i32);
impl TUnifromShaderProperty for UniformPropertyInt {
    fn tag(&self) -> &UniformPropertyName {
        &self.0
    }
}
#[derive(Clone, Debug)]
pub struct UniformPropertyUint(pub UniformPropertyName, pub u32);
impl TUnifromShaderProperty for UniformPropertyUint {
    fn tag(&self) -> &UniformPropertyName {
        &self.0
    }
}

pub type ShaderEffectMeta = ShaderEffectDesc<UniformPropertyMat4, UniformPropertyMat2, UniformPropertyVec4, UniformPropertyVec2, UniformPropertyFloat, UniformPropertyInt, UniformPropertyUint>;
pub type ShaderEffectValueUniformDesc = MaterialValueBindDesc<UniformPropertyMat4, UniformPropertyMat2, UniformPropertyVec4, UniformPropertyVec2, UniformPropertyFloat, UniformPropertyInt, UniformPropertyUint>;

#[derive(Clone, Debug)]
pub struct ResShaderEffectMeta(pub Share<ShaderEffectMeta>);
impl From<ShaderEffectMeta> for ResShaderEffectMeta {
    fn from(value: ShaderEffectMeta) -> Self {
        ResShaderEffectMeta(Share::new(value))
    }
}
impl std::ops::Deref for ResShaderEffectMeta {
    type Target = ShaderEffectMeta;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Asset for ResShaderEffectMeta {
    type Key = KeyShaderEffect;

    fn size(&self) -> usize {
        self.0.size
    }
}

#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyMaterialMeta(pub KeyShaderEffect);

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResMaterailMeta(pub Handle<ResShaderEffectMeta>);
impl From<Handle<ResShaderEffectMeta>> for AssetResMaterailMeta {
    fn from(value: Handle<ResShaderEffectMeta>) -> Self {
        Self(value)
    }
}
impl AssetResMaterailMeta {
    pub fn query_tex_slot(&self, name: &UniformPropertyName) -> Result<usize, ErrorUniformSlot> {
        if let Some(textures) = &self.0.textures {
            textures.query_slot(name)
        } else {
            Err(ErrorUniformSlot::NotFoundProperty)
        }
    }
}

#[derive(Debug)]
enum ECommand {
    Use(ObjectID, KeyShaderEffect),
}
#[derive(Debug, Default)]
struct SingleMaterialMetaCommands(pub Vec<ECommand>);
struct SysMaterialMetaCommands;
#[setup]
impl SysMaterialMetaCommands {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleMaterialMetaCommands>,
        mut items: Query<GameObject, Write<AssetKeyMaterialMeta>>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Use(entity, key) => {
                    if let Some(mut item) = items.get(entity) {
                        item.write(AssetKeyMaterialMeta(key));
                    }
                },
            }
        });
    }
}

pub trait InterfaceMaterialMeta {
    fn regist_material_meta(
        &self,
        key: KeyShaderEffect,
        meta: ShaderEffectMeta,
    ) -> &Self;
    fn as_material(
        &self,
        entity: ObjectID,
        shader: KeyShaderEffect,
    ) -> &Self;
}

impl InterfaceMaterialMeta for EnginShell {
    fn regist_material_meta(
        &self,
        key: KeyShaderEffect,
        meta: ShaderEffectMeta,
    ) -> &Self {
        let world = self.world();

        let meta = ResShaderEffectMeta::from(meta);
        let asset_mgr = world.get_resource::<Share<AssetMgr<ResShaderEffectMeta>>>().unwrap();
        if !asset_mgr.check_asset(&key) {
            let meta = asset_mgr.create_asset(key.clone(), meta);
            let wait = world.get_resource_mut::<AssetSyncWait<KeyShaderEffect, AssetKeyMaterialMeta, ResShaderEffectMeta, AssetResMaterailMeta>>().unwrap();
            wait.1.push((key.clone(), meta));
        }

        self
    }

    fn as_material(
        &self,
        entity: ObjectID,
        shader: KeyShaderEffect,
    ) -> &Self {
        let world = self.world();
        let commands = world.get_resource_mut::<SingleMaterialMetaCommands>().unwrap();
        commands.0.push(ECommand::Use(entity, shader));
        
        self
    }
}


type PluginAssetMaterialMetaLoad = PluginAssetSyncLoad::<KeyShaderEffect, AssetKeyMaterialMeta, ResShaderEffectMeta, AssetResMaterailMeta>;

pub struct PluginMaterialMeta;
impl pi_engine_shell::plugin::Plugin for PluginMaterialMeta {
    fn init(
        &mut self,
        engine: &mut EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        PluginAssetSyncNotNeedLoad::<KeyShader, ResShader>::new(false, 10 * 1024 * 1024, 60 * 1000).init(engine, stages);

        PluginAssetMaterialMetaLoad::new(false, 10 * 1024 * 1024, 60 * 1000).init(engine, stages);

        let world = engine.world_mut();
        world.insert_resource(SingleMaterialMetaCommands::default());

        SysMaterialMetaCommands::setup(world, stages.command_stage());

        Ok(())
    }
}