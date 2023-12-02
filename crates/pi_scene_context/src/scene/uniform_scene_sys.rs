use pi_engine_shell::prelude::*;

use crate::{object::GameObject, environment::{fog::SceneFog, ambient_light::AmbientLight}, scene::scene_time::SceneTime};

use crate::{light::prelude::*, prelude::{SceneLightingInfos, SceneLightingInfosDirty, GlobalTransform, SceneLightsQueue, LightIndex}};

// pub struct SceneUniformTickUpdate;
// #[setup]
// impl SceneUniformTickUpdate {
//     #[system]
    pub fn sys_uniform_scene(
        query_scenes: Query<(&SceneTime, &SceneFog, &AmbientLight)>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        //  log::debug!("Scene Uniform Tick Update");
        query_scenes.iter().for_each(|(time, fog, ambl)| {
            dynbuffer.set_uniform::<SceneTime>(&time.bind_offset, time);
            dynbuffer.set_uniform::<SceneFog>(&fog.bind_offset, fog);
            dynbuffer.set_uniform::<AmbientLight>(&ambl.bind_offset, ambl);
        });
    }
// }


/// 灯光 的 光照信息更新
pub fn sys_scene_light_lighting_update_uniform(
    items: Query<(&SceneLightingInfos, &SceneLightsQueue), Changed<SceneLightingInfosDirty>>,
    direct_lights: Query<Entity, With<DirectLight>>,
    spot_lights: Query<Entity, With<SpotLight>>,
    point_lights: Query<Entity, With<PointLight>>,
    hemi_lights: Query<Entity, With<HemisphericLight>>,
    color_position_lightingmodes: Query<(&LightIndex, &LightColor, &GlobalTransform, &LightingMode)>,
    directions: Query<&LightDirection>,
    // falloff: Query<>,
) {
    items.iter().for_each(|(infos, lights)| {
        lights.lights().for_each(|entity| {
            let entity = *entity;
            if let Ok((idx, color, transform, mode)) = color_position_lightingmodes.get(entity) {
                let mut data = [0.;16];

                let position = transform.position();
                data[0 * 4 + 0] = position.x; data[0 * 4 + 1] = position.y; data[0 * 4 + 2] = position.z;

                data[1 * 4 + 0] = color.0.x; data[1 * 4 + 1] = color.0.y; data[1 * 4 + 2] = color.0.z;

                data[2 * 4 + 3] = mode.val();

                infos.0.data().write_data(idx.val() as usize * 16, bytemuck::cast_slice(&data));
                
                if direct_lights.contains(entity) {
                    if let Ok(direction) = directions.get(entity) {
                        data[3 * 4 + 0] = direction.0.x; data[3 * 4 + 1] = direction.0.y; data[3 * 4 + 2] = direction.0.z;
                    }
                } else if spot_lights.contains(entity) {

                } else if point_lights.contains(entity) {
                    
                } else if hemi_lights.contains(entity) {
                    
                }
            }
        });
    });
}
