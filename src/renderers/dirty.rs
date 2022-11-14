
use pi_ecs::{prelude::Query, query::{Write, With}};
use pi_ecs_macros::{listen, setup};

use crate::{object::GameObject, materials::material::MaterialID};

use super::{render_primitive::PrimitiveState, render_depth_and_stencil::RenderDepthAndStencil, render_blend::RenderBlend};

// pub type DirtyPipelineKey = Or<Changed<MaterialID>, Changed<RenderBlend>, Changed<RenderDepthAndStencil>, Changed<PrimitiveState>>;