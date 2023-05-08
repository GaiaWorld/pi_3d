
use pi_engine_shell::prelude::*;

use crate::light::{shadow_generator::base::{ShadowMinZ, ShadowMaxZ, ShadowFrustumSize}, base::Light};

use super::DirectionalShadowProjection;


// pub struct SysDirectionalShadowModify;
// impl TSystemStageInfo for SysDirectionalShadowModify {
    
// }
// #[setup]
// impl SysDirectionalShadowModify {
//     #[system]
    pub fn sys_directional_light_shadow_modify(
        lights: Query<(ObjectID, &Light, &ShadowMinZ, &ShadowMaxZ, &ShadowFrustumSize), Or<(Changed<Light>, Changed<ShadowMinZ>, Changed<ShadowMaxZ>, Changed<ShadowFrustumSize>)>>,
        mut commands: Commands,
    ) {
        lights.iter().for_each(|(id_light, light, minz, maxz, size)| {
            match light {
                Light::Directional => {
                    commands.entity(id_light).insert(DirectionalShadowProjection { minz: minz.0, maxz: maxz.0, frustum_size: size.0 });
                },
                Light::Point => {},
                Light::Spot => {},
                Light::Hemispheric => {
                    
                },
            }
        });
    }
// }
