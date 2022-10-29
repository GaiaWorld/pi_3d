use pi_ecs::{prelude::{ResMut, Query, Res}};
use pi_ecs_macros::setup;
use pi_render::{rhi::{bind_group_layout::BindGroupLayout, device::RenderDevice, bind_group::BindGroup}};


use crate::{object::{ObjectID, GameObject}, cameras::camera::CameraRenderData, scene::scene_time::SceneTime, environment::{fog::SceneFog, ambient_light::AmbientLight}, materials::{bind_group::RenderBindGroup, SingleDynUnifromBufferReBindFlag}, shaders::FragmentUniformBind, flags::SceneID, resources::RenderDynUniformBuffer};


pub struct IDMainCameraRenderBindGroup(pub ObjectID);
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
        mut bindgroups: Query<GameObject, &mut RenderBindGroup>,
        id: ResMut<IDMainCameraRenderBindGroup>,
    ) {
        println!("Sys MainCameraRender BindGroup Update");
        if dynbuffer_flag.0 {
            match bindgroups.get_mut(id.0) {
                Some(mut group) => {
                    println!("Sys MainCameraRender BindGroup Update bind_group");
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
        query_scenes: Query<GameObject, (ObjectID, &SceneTime, ObjectID)>,
        query_cameras: Query<GameObject, (&SceneID, &CameraRenderData)>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        println!("Sys MainCameraRender Uniform Update");
        query_scenes.iter().for_each(|scene| {
            query_cameras.iter().for_each(|camera| {
                if scene.0 == camera.0.0 {
                    println!("MainCameraRender Uniform Update set_uniform");
                    dynbuffer.as_mut().set_uniform(&scene.1.bind_offset, scene.1);
                    // dynbuffer.set_uniform(&scene.2.bind_offset, scene.2);
                    // dynbuffer.set_uniform(&scene.3.bind_offset, scene.3);
                    dynbuffer.as_mut().set_uniform(&camera.1.bind_offset, camera.1);
                }
            });
        });
    }
}