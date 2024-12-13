
use pi_scene_shell::prelude::*;

use crate::{
    cullings::prelude::*, flags::*, geometry::prelude::*, layer_mask::prelude::*, pass::ActionListRenderState, renderers::prelude::*
};

pub use super::{
    model::*,
    command::*,
    system::*,
    render_group::*,
    shader_about::*,
    abstract_mesh::*,
    skeleton::*,
    lighting::*,
    animation::*,
};


#[derive(SystemParam)]
pub struct ActionSetMesh<'w> {
    pub create: ResMut<'w, ActionListMeshCreate>,
    pub state: ResMut<'w, ActionListMeshStateModify>,
    pub pose: ResMut<'w, ActionListAbstractMeshPose>,

    pub render_state: ResMut<'w, ActionListRenderState>,
    pub value_state: ResMut<'w, ActionListAbstructMeshValueStateModify>,

    pub bounding: ResMut<'w, ActionListMeshBounding>,
    pub layermask: ResMut<'w, ActionListLayerMask>,
    pub forcelighting: ResMut<'w, ActionListMeshForceLighting>,
    // pub boneoffset: ResMut<'w, ActionListBoneOffset>,
}
impl<'w> MemSize for ActionSetMesh<'w> {
    fn memsize(&self) -> usize {
        self.create.memsize()
        + self.state.memsize()
        + self.pose.memsize()
        + self.render_state.memsize()
        + self.value_state.memsize()
        + self.bounding.memsize()
        + self.layermask.memsize()
        + self.forcelighting.memsize()
    }
}

#[derive(SystemParam)]
pub struct ActionSetInstanceMesh<'w> {
    pub create: ResMut<'w, ActionListInstanceMeshCreate>,
    pub attr: ResMut<'w, ActionListInstanceAttr>,
}
impl<'w> MemSize for ActionSetInstanceMesh<'w> {
    fn memsize(&self) -> usize {
        self.create.memsize()
        + self.attr.memsize()
    }
}

// #[derive(SystemParam)]
// pub struct ActionSetAbstructMesh<'w> {
//     pub force_point_light: ResMut<'w, ActionListMeshForcePointLighting>,
//     pub force_spot_light: ResMut<'w, ActionListMeshForceSpotLighting>,
//     pub force_hemi_light: ResMut<'w, ActionListMeshForceHemiLighting>,
//     // pub targetanime: ResMut<'w, ActionListTargetAnimationAttribute>,
// }

#[derive(Resource, Default)]
pub struct StateMesh {
    pub scene: Option<Entity>,
    pub abstructmesh: u32,
    pub meshes: u32,
    pub instances: u32,
    pub abstructenable_count: u32,
    pub geometry_enable: u32,
}


pub type StateMeshQuery = QueryState<(&'static SceneID, &'static GlobalEnable, Option<&'static RenderGeometryEable>, Option<&'static InstanceMesh>), With<AbstructMesh>>;

pub fn sys_state_mesh(
    // mut state: ResMut<StateMesh>,
    // meshes: Query<(&SceneID, &GlobalEnable, &RenderGeometryEable), With<AbstructMesh>>,
    // instances: Query<&InstanceMesh>,
) {
    // state.abstructmesh = 0;
    // state.meshes = 0;
    // state.instances = 0;
    // state.abstructenable_count = 0;
    // state.geometry_enable = 0;
    // if let Some(scene) = state.scene {
    //     meshes.iter().for_each(|(idscene, enable, geoenable, instance)| {
    //         if scene == idscene.0 {
    //             state.abstructmesh += 1;
    //             if enable.0 { state.abstructenable_count += 1; }
    //             if instance.is_some() {
    //                 state.instances += 1;
    //             } else if let Some(geoenable) = geoenable {
    //                 state.meshes += 1;
    //                 if geoenable.0 {
    //                     state.geometry_enable += 1;
    //                 }
    //             }
    //         }
    //     });
    // }
}