
use std::mem::replace;

use derive_deref::{Deref, DerefMut};
use pi_assets::{mgr::AssetMgr, asset::{Handle, GarbageEmpty}};
use pi_ecs::{prelude::{Id, ResMut, Query, Res, Setup}, query::{Write, Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{assets::sync_load::{PluginAssetSyncLoad, InterfaceAssetSyncCreate, AssetSyncWait, PluginAssetSyncNotNeedLoad}, engine_shell::EnginShell};
use pi_render::rhi::device::RenderDevice;
use pi_share::Share;
use render_shader::{shader::{KeyPreShader, ResPreShaderMeta, ResShader, KeyShader, PreShaderMeta}, skin_code::ESkinCode, scene_about_code::ERenderTag, unifrom_code::{UniformPropertyName, ErrorUniformSlot}};

use crate::{object::{ObjectID, GameObject}, renderers::render_mode::ERenderMode};

#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyMaterialMeta(pub KeyPreShader);

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResMaterailMeta(pub Handle<ResPreShaderMeta>);
impl From<Handle<ResPreShaderMeta>> for AssetResMaterailMeta {
    fn from(value: Handle<ResPreShaderMeta>) -> Self {
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
    Use(ObjectID, KeyPreShader),
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
        key: KeyPreShader,
        meta: PreShaderMeta,
    ) -> &Self;
    fn as_material(
        &self,
        entity: ObjectID,
        shader: KeyPreShader,
    ) -> &Self;
}

impl InterfaceMaterialMeta for EnginShell {
    fn regist_material_meta(
        &self,
        key: KeyPreShader,
        meta: PreShaderMeta,
    ) -> &Self {
        let world = self.world();

        let meta = ResPreShaderMeta::from(meta);
        let asset_mgr = world.get_resource::<Share<AssetMgr<ResPreShaderMeta>>>().unwrap();
        if !asset_mgr.check_asset(&key) {
            let meta = asset_mgr.create_asset(key.clone(), meta);
            let wait = world.get_resource_mut::<AssetSyncWait<KeyPreShader, AssetKeyMaterialMeta, ResPreShaderMeta, AssetResMaterailMeta>>().unwrap();
            wait.1.push((key.clone(), meta));
        }

        self
    }

    fn as_material(
        &self,
        entity: ObjectID,
        shader: KeyPreShader,
    ) -> &Self {
        let world = self.world();
        let commands = world.get_resource_mut::<SingleMaterialMetaCommands>().unwrap();
        commands.0.push(ECommand::Use(entity, shader));
        
        self
    }
}


type PluginAssetMaterialMetaLoad = PluginAssetSyncLoad::<KeyPreShader, AssetKeyMaterialMeta, ResPreShaderMeta, AssetResMaterailMeta>;

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