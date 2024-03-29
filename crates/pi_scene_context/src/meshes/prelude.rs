
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
    pub shadow: ResMut<'w, ActionListMeshShadow>,
    pub blend: ResMut<'w, ActionListBlend>,
    pub cullmode: ResMut<'w, ActionListCullMode>,
    pub polygonmode: ResMut<'w, ActionListPolyginMode>,
    pub frontface: ResMut<'w, ActionListFrontFace>,
    pub topology: ResMut<'w, ActionListTopology>,
    pub unclip_depth: ResMut<'w, ActionListUnClipDepth>,
    pub depth_write: ResMut<'w, ActionListDepthWrite>,
    pub depth_compare: ResMut<'w, ActionListDepthCompare>,
    pub depth_bias: ResMut<'w, ActionListDepthBias>,
    pub stencil_front: ResMut<'w, ActionListStencilFront>,
    pub stencil_back: ResMut<'w, ActionListStencilBack>,
    pub stencil_read: ResMut<'w, ActionListStencilRead>,
    pub stencil_write: ResMut<'w, ActionListStencilWrite>,
    pub render_queue: ResMut<'w, ActionListRenderQueue>,
    pub render_alignment: ResMut<'w, ActionListMeshRenderAlignment>,
    pub indexrange: ResMut<'w, ActionListMeshRenderIndiceRange>,
    pub vertexrange: ResMut<'w, ActionListMeshRenderVertexRange>,
    pub bounding: ResMut<'w, ActionListMeshBounding>,
    pub boundingculling: ResMut<'w, ActionListMeshBoundingCullingMode>,
    pub layermask: ResMut<'w, ActionListLayerMask>,
    pub boneoffset: ResMut<'w, ActionListBoneOffset>,
}

#[derive(SystemParam)]
pub struct ActionSetInstanceMesh<'w> {
    pub create: ResMut<'w, ActionListInstanceMeshCreate>,
    pub floats: ResMut<'w, ActionListInstanceFloat>,
    pub vec4s: ResMut<'w, ActionListInstanceVec4>,
    pub vec3s: ResMut<'w, ActionListInstanceVec3>,
    pub vec2s: ResMut<'w, ActionListInstanceVec2>,
    pub uints: ResMut<'w, ActionListInstanceUint>,
    pub sints: ResMut<'w, ActionListInstanceSint>,
}

#[derive(SystemParam)]
pub struct ActionSetAbstructMesh<'w> {
    pub scaling_mode: ResMut<'w, ActionListAbstructMeshScalingMode>,
    pub velocity: ResMut<'w, ActionListAbstructMeshVelocity>,
    pub force_point_light: ResMut<'w, ActionListMeshForcePointLighting>,
    pub force_spot_light: ResMut<'w, ActionListMeshForceSpotLighting>,
    pub force_hemi_light: ResMut<'w, ActionListMeshForceHemiLighting>,
    // pub targetanime: ResMut<'w, ActionListTargetAnimationAttribute>,
}

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