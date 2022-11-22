use pi_ecs::{prelude::Query, query::{Changed, Or, Write, WithOut}, sys::system};
use pi_ecs_macros::setup;
use pi_engine_shell::object::{GameObject, ObjectID};

use material_textures::main_texture::{MainTextureKey, MainTextureRes, MainTextureSampler};


pub type UnlitMaterialMode = u16;

#[derive(Debug, Default)]
pub struct UnlitMaterialDefines {
    pub emissive: bool,
}
impl UnlitMaterialDefines {
    pub const DEFFINE_EMISSIVE: UnlitMaterialMode = 0b0000_0000_0000_0001;
    pub fn mode(
        &self
    ) -> UnlitMaterialMode {
        let mut mode = UnlitMaterialMode::MIN;
        if self.emissive {
            mode |= Self::DEFFINE_EMISSIVE;
        }

        mode
    }
}

pub struct SysUnlitMaterialDefinesUpdate;
#[setup]
impl SysUnlitMaterialDefinesUpdate {
    #[system]
    pub fn modify(
        diff1: Query<GameObject, (ObjectID, &MainTextureRes), Changed<MainTextureRes>>,
        diff2: Query<GameObject, (ObjectID), (Changed<MainTextureRes>, WithOut<MainTextureRes>)>,
        mut defines: Query<GameObject, &mut UnlitMaterialDefines>,
    ) {
        println!("SysUnlitMaterialDefinesUpdate:");
        diff1.iter().for_each(|(entity, _) | {
            if let Some(mut define) = defines.get_mut(entity) {
                define.emissive = true;
            }
        });
        diff2.iter().for_each(|(entity) | {
            if let Some(mut define) = defines.get_mut(entity) {
                define.emissive = false;
            }
        });
    }
}