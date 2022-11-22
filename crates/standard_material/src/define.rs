use pi_ecs::{prelude::Query, query::{Changed, Or, Write}, sys::system};
use pi_ecs_macros::setup;
use pi_engine_shell::object::{GameObject, ObjectID};

use crate::emissive::EmissiveTexture;


pub type StandardMaterialMode = u16;

#[derive(Debug, Default)]
pub struct StandardMaterialDefines {
    pub emissive: bool,
}
impl StandardMaterialDefines {
    pub const DEFFINE_EMISSIVE: StandardMaterialMode = 0b0000_0000_0000_0001;
    pub fn mode(
        &self
    ) -> StandardMaterialMode {
        let mut mode = StandardMaterialMode::MIN;
        if self.emissive {
            mode |= Self::DEFFINE_EMISSIVE;
        }

        mode
    }
}

pub struct SysStandardMaterialDefinesUpdate;
#[setup]
impl SysStandardMaterialDefinesUpdate {
    #[system]
    pub fn modify(
        diff1: Query<GameObject, (ObjectID, &EmissiveTexture), Changed<EmissiveTexture>>,
        diff2: Query<GameObject, (ObjectID), Changed<EmissiveTexture>>,
        mut define: Query<GameObject, Write<StandardMaterialDefines>>,
    ) {
        println!("SysStandardMaterialDefinesUpdate:");
        diff1.iter().for_each(|(entity, _) | {
            if let Some(diffine) = define.get_mut(entity) {
                if let Some(diffine) = diffine.get_mut() {
                    diffine.emissive = true;
                }
                diffine.notify_modify();
            }
        });
        diff1.iter().for_each(|(entity, _) | {
            if let Some(diffine) = define.get_mut(entity) {
                if let Some(diffine) = diffine.get_mut() {
                    diffine.emissive = false;
                }
                diffine.notify_modify();
            }
        });
    }
}