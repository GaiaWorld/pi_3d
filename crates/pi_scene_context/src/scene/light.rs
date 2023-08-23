use pi_engine_shell::prelude::*;

use crate::light::base::*;

pub enum EMaxDirectLight {
    N004,
    N016,
    N064,
    N256,
    N512,
}

pub enum EMaxSpotLight {
    N004,
    N016,
    N064,
    N256,
    N512,
}

pub enum EMaxHemisphericLight {
    N004,
    N016,
    N064,
    N256,
    N512,
}

#[derive(Component, Deref, DerefMut)]
pub struct DirectLights(pub Vec<Entity>);

#[derive(Component, Deref, DerefMut)]
pub struct SpotLights(pub Vec<Entity>);

#[derive(Component, Deref, DerefMut)]
pub struct HemisphericLights(pub Vec<Entity>);



pub fn sys_scene_lights(
    lights: Query<(Entity, &SceneID, &Light), Changed<Light>>,
    mut scenes: Query<(&mut DirectLights, &mut SpotLights, &mut HemisphericLights)>,
) {
    lights.iter().for_each(|(idlight, idscene, light)| {
        if let Ok((mut directlights, mut spotlights, mut hemisphericlights)) = scenes.get_mut(idscene.0) {
            match light {
                Light::Directional => {
                    directlights.push(idlight);
                },
                Light::Point => {
                    directlights.push(idlight);
                },
                Light::Spot => {
                    spotlights.push(idlight);
                },
                Light::Hemispheric => {
                    hemisphericlights.push(idlight);
                },
            }
        }
    });
}