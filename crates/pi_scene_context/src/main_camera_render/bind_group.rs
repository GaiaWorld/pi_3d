use pi_ecs::{prelude::{ResMut, Query, Res}, query::{With, Changed, Or}};
use pi_ecs_macros::setup;
use pi_render::{rhi::{bind_group_layout::BindGroupLayout, device::RenderDevice, bind_group::BindGroup}};


use crate::{
    object::{ObjectID, GameObject},
    cameras::{camera::{CameraRenderData, CameraViewMatrix, CameraProjectionMatrix, CameraTransformMatrix, CameraGlobalPosition, CameraDirection}},
    scene::scene_time::{SceneTime},
    environment::{fog::SceneFog, ambient_light::AmbientLight},
    materials::{bind_group::{RenderBindGroup, RenderBindGroupKey, RenderBindGroupPool}, uniform_buffer::SingleDynUnifromBufferReBindFlag},
    shaders::FragmentUniformBind,
    flags::SceneID,
    resources::RenderDynUniformBuffer
};


pub struct IDMainCameraRenderBindGroup(pub RenderBindGroupKey);
impl IDMainCameraRenderBindGroup {
    const LABEL: &'static str = "MainCameraRenderBindGroup";
    pub const SET: u32 = 0;

    pub fn layout(device: &RenderDevice) -> BindGroupLayout {
        BindGroupLayout::from(
            device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some(Self::LABEL),
                    entries: &[
                        CameraRenderData::ENTRY,
                        SceneTime::ENTRY,
                        SceneFog::ENTRY,
                        AmbientLight::ENTRY,
                    ],
                }
            )
        )
    }

    pub fn bind_group(
        device: &RenderDevice,
        group: &mut RenderBindGroup,
        dynbuffer: &RenderDynUniformBuffer,
    ) {
        group.bind_group = Some(
            BindGroup::from(
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: Some(Self::LABEL),
                        layout: &group.layout,
                        entries: &[
                            CameraRenderData::dyn_entry(dynbuffer),
                            SceneTime::dyn_entry(dynbuffer),
                            SceneFog::dyn_entry(dynbuffer),
                            AmbientLight::dyn_entry(dynbuffer),
                        ],
                    }
                )
            )
        ); 
    }
}

pub struct SysMainCameraRenderBindGroupUpdate;
#[setup]
impl SysMainCameraRenderBindGroupUpdate {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        dynbuffer: Res<RenderDynUniformBuffer>,
        dynbuffer_flag: Res<SingleDynUnifromBufferReBindFlag>,
        mut bindgroups: ResMut<RenderBindGroupPool>,
        id: ResMut<IDMainCameraRenderBindGroup>,
    ) {
        // println!("Sys MainCameraRender BindGroup Update");
        if dynbuffer_flag.0 {
            match bindgroups.get_mut(id.0) {
                Some(mut group) => {
                    // println!("Sys MainCameraRender BindGroup Update bind_group");
                    IDMainCameraRenderBindGroup::bind_group(&device, &mut group, &dynbuffer);
                },
                None => todo!(),
            }
        }
    }
}

pub struct SysMainCameraRenderUniformUpdate;
#[setup]
impl SysMainCameraRenderUniformUpdate {
    #[system]
    pub fn tick(
        // query_scenes: Query<GameObject, (ObjectID, &SceneTime, &SceneFog, &AmbientLight)>,
        mut query_scenes: Query<GameObject, (ObjectID, &SceneTime)>,
        mut query_cameras: Query<GameObject, (&SceneID, &CameraRenderData, &CameraViewMatrix, &CameraProjectionMatrix, &CameraTransformMatrix, &CameraGlobalPosition, &CameraDirection), Or<(Changed<CameraViewMatrix>, Changed<CameraProjectionMatrix>, Changed<CameraTransformMatrix>, Changed<CameraGlobalPosition>)>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        //  println!("Sys MainCameraRender Uniform Update");
        query_scenes.iter_mut().for_each(|(sceneid, scene_time)| {
            query_cameras.iter_mut().for_each(|(camera_scene, camera_data, view_matrix, project_matrix, transform_matrix, position, direction)| {
                if sceneid == camera_scene.0 {
                    println!("MainCameraRender Uniform Update set_uniform");
                    dynbuffer.as_mut().set_uniform(&scene_time.bind_offset, scene_time);
                    dynbuffer.as_mut().set_uniform(&camera_data.bind_offset, view_matrix);
                    dynbuffer.as_mut().set_uniform(&camera_data.bind_offset, project_matrix);
                    dynbuffer.as_mut().set_uniform(&camera_data.bind_offset, transform_matrix);
                    dynbuffer.as_mut().set_uniform(&camera_data.bind_offset, position);
                    dynbuffer.as_mut().set_uniform(&camera_data.bind_offset, direction);
                }
            });
        });
    }
}