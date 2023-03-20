use pi_ecs::{prelude::{Query, Commands}, query::{Or, Changed}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::TSystemStageInfo};

use crate::light::{shadow_generator::base::{ShadowMinZ, ShadowMaxZ, ShadowFrustumSize}, base::Light};

use super::DirectionalShadowProjection;


pub struct SysDirectionalShadowModify;
impl TSystemStageInfo for SysDirectionalShadowModify {
    
}
#[setup]
impl SysDirectionalShadowModify {
    #[system]
    fn sys(
        lights: Query<GameObject, (ObjectID, &Light, &ShadowMinZ, &ShadowMaxZ, &ShadowFrustumSize), Or<(Changed<Light>, Changed<ShadowMinZ>, Changed<ShadowMaxZ>, Changed<ShadowFrustumSize>)>>,
        mut param_cmd: Commands<GameObject, DirectionalShadowProjection>,
    ) {
        lights.iter().for_each(|(id_light, light, minz, maxz, size)| {
            match light {
                Light::Directional => {
                    param_cmd.insert(id_light, DirectionalShadowProjection { minz: minz.0, maxz: maxz.0, frustum_size: size.0 });
                },
                Light::Point => {},
                Light::Spot => {},
            }
        });
    }
}
