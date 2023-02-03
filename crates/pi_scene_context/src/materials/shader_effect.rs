
use std::mem::replace;

use derive_deref::{Deref, DerefMut};
use pi_assets::{mgr::AssetMgr, asset::{Handle, Asset}};
use pi_ecs::{prelude::{ResMut, Setup, Commands}};
use pi_ecs_macros::setup;
use pi_engine_shell::{assets::sync_load::{PluginAssetSyncLoad, InterfaceAssetSyncCreate, AssetSyncWait, PluginAssetSyncNotNeedLoad, AssetSyncLoad}, engine_shell::EnginShell, run_stage::{TSystemStageInfo, ERunStageChap}};

use pi_scene_math::{Matrix, Matrix2, Vector4, Vector2, Number};
use pi_share::Share;
use render_shader::{shader::{KeyShaderEffect, ResShader, KeyShader, ShaderEffectMeta as ShaderEffectDesc}, unifrom_code::{UniformPropertyName, ErrorUniformSlot, TUnifromShaderProperty, MaterialValueBindDesc}};

use crate::{object::{ObjectID, GameObject}};

use super::material::MaterialUsedList;

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
pub struct AssetKeyShaderEffect(pub KeyShaderEffect);

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResShaderEffectMeta(pub Handle<ResShaderEffectMeta>);
impl From<Handle<ResShaderEffectMeta>> for AssetResShaderEffectMeta {
    fn from(value: Handle<ResShaderEffectMeta>) -> Self {
        Self(value)
    }
}
impl AssetResShaderEffectMeta {
    pub fn query_tex_slot(&self, name: &UniformPropertyName) -> Result<usize, ErrorUniformSlot> {
        if let Some(textures) = &self.0.textures {
            textures.query_slot(name)
        } else {
            Err(ErrorUniformSlot::NotFoundProperty)
        }
    }
}

#[derive(Debug)]
pub enum ECommand {
    Use(ObjectID, KeyShaderEffect),
}
#[derive(Debug, Default)]
pub struct SingleShaderEffectCommands(pub Vec<ECommand>);

pub struct SysShaderEffectCommands;
impl TSystemStageInfo for SysShaderEffectCommands {
    
}
#[setup]
impl SysShaderEffectCommands {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleShaderEffectCommands>,
        mut items: Commands<GameObject, AssetKeyShaderEffect>,
        mut usedlist: Commands<GameObject, MaterialUsedList>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Use(entity, key) => {
                    items.insert(entity, AssetKeyShaderEffect(key));
                    usedlist.insert(entity, MaterialUsedList::default())
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
            let wait = world.get_resource_mut::<AssetSyncWait<KeyShaderEffect, AssetKeyShaderEffect, ResShaderEffectMeta, AssetResShaderEffectMeta>>().unwrap();
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
        let commands = world.get_resource_mut::<SingleShaderEffectCommands>().unwrap();
        commands.0.push(ECommand::Use(entity, shader));
        
        self
    }
}


pub type SysAssetShaderEffectLoad = AssetSyncLoad::<KeyShaderEffect, AssetKeyShaderEffect, ResShaderEffectMeta, AssetResShaderEffectMeta, SysShaderEffectCommands>;
type PluginAssetShaderEffectLoad = PluginAssetSyncLoad::<KeyShaderEffect, AssetKeyShaderEffect, ResShaderEffectMeta, AssetResShaderEffectMeta, SysShaderEffectCommands>;

pub struct PluginShaderEffect;
impl pi_engine_shell::plugin::Plugin for PluginShaderEffect {
    fn init(
        &mut self,
        engine: &mut EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {

        let world = engine.world_mut();
        world.insert_resource(SingleShaderEffectCommands::default());

        SysShaderEffectCommands::setup(world, stages.query_stage::<SysShaderEffectCommands>(ERunStageChap::Initial));

        PluginAssetSyncNotNeedLoad::<KeyShader, ResShader>::new(false, 10 * 1024 * 1024, 60 * 1000).init(engine, stages);
        PluginAssetShaderEffectLoad::new(false, 10 * 1024 * 1024, 60 * 1000).init(engine, stages);

        Ok(())
    }
}