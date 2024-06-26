
use pi_scene_shell::prelude::*;

use crate::{
    renderers::prelude::*,
    cullings::prelude::*,
    flags::*,
    geometry::prelude::*,
    layer_mask::prelude::*,
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
    pub blend: ResMut<'w, ActionListBlend>,
    
    pub primitive_state: ResMut<'w, ActionListPrimitiveState>,
    // pub cullmode: ResMut<'w, ActionListCullMode>,
    // pub polygonmode: ResMut<'w, ActionListPolyginMode>,
    // pub frontface: ResMut<'w, ActionListFrontFace>,
    // pub topology: ResMut<'w, ActionListTopology>,
    // pub unclip_depth: ResMut<'w, ActionListUnClipDepth>,

    pub depth_state: ResMut<'w, ActionListDepthState>,
    pub stencil_state: ResMut<'w, ActionListStencilState>,
    // pub depth_write: ResMut<'w, ActionListDepthWrite>,
    // pub depth_compare: ResMut<'w, ActionListDepthCompare>,
    // pub depth_bias: ResMut<'w, ActionListDepthBias>,
    // pub stencil_front: ResMut<'w, ActionListStencilFront>,
    // pub stencil_back: ResMut<'w, ActionListStencilBack>,
    // pub stencil_read: ResMut<'w, ActionListStencilRead>,
    // pub stencil_write: ResMut<'w, ActionListStencilWrite>,

    pub render_queue: ResMut<'w, ActionListRenderQueue>,
    pub value_state: ResMut<'w, ActionListAbstructMeshValueStateModify>,
    // pub indexrange: ResMut<'w, ActionListMeshRenderIndiceRange>,
    // pub vertexrange: ResMut<'w, ActionListMeshRenderVertexRange>,
    pub bounding: ResMut<'w, ActionListMeshBounding>,
    pub layermask: ResMut<'w, ActionListLayerMask>,
    pub forcelighting: ResMut<'w, ActionListMeshForceLighting>,
    // pub boneoffset: ResMut<'w, ActionListBoneOffset>,
}

#[derive(SystemParam)]
pub struct ActionSetInstanceMesh<'w> {
    pub create: ResMut<'w, ActionListInstanceMeshCreate>,
    pub attr: ResMut<'w, ActionListInstanceAttr>,
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
    mut state: ResMut<StateMesh>,
    meshes: Query<(&SceneID, &GlobalEnable, Option<&RenderGeometryEable>, Option<&InstanceMesh>), With<AbstructMesh>>,
) {
    state.abstructmesh = 0;
    state.meshes = 0;
    state.instances = 0;
    state.abstructenable_count = 0;
    state.geometry_enable = 0;
    if let Some(scene) = state.scene {
        meshes.iter().for_each(|(idscene, enable, geoenable, instance)| {
            if scene == idscene.0 {
                state.abstructmesh += 1;
                if enable.0 { state.abstructenable_count += 1; }
                if instance.is_some() {
                    state.instances += 1;
                } else if let Some(geoenable) = geoenable {
                    state.meshes += 1;
                    if geoenable.0 {
                        state.geometry_enable += 1;
                    }
                }
            }
        });
    }
}