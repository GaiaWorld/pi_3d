
use pi_engine_shell::prelude::*;

use crate::{renderers::prelude::*, layer_mask::prelude::*, geometry::command::*};

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
}

#[derive(SystemParam)]
pub struct ActionSetInstanceMesh<'w> {
    pub create: ResMut<'w, ActionListInstanceMeshCreate>,
    pub color: ResMut<'w, ActionListInstanceColor>,
    pub alpha: ResMut<'w, ActionListInstanceAlpha>,
    pub tilloff: ResMut<'w, ActionListInstanceTillOff>,
    pub ins_world_matrixs: ResMut<'w, ActionListInstanceWorldMatrixs>,
    pub ins_colors: ResMut<'w, ActionListInstanceColors>,
    pub ins_tilloffs: ResMut<'w, ActionListInstanceTilloffs>,
}

#[derive(SystemParam)]
pub struct ActionSetAbstructMesh<'w> {
    pub scaling_mode: ResMut<'w, ActionListAbstructMeshScalingMode>,
    pub velocity: ResMut<'w, ActionListAbstructMeshVelocity>,
    pub boneoffset: ResMut<'w, ActionListBoneOffset>,
}