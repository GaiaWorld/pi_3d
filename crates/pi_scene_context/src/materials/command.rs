use pi_ecs::{prelude::{ResMut, Query, Res}, query::Write};
use pi_ecs_macros::setup;
use pi_render::rhi::{bind_group_layout::BindGroupLayout, device::RenderDevice};

use crate::object::{ObjectID, GameObject};

use super::bind_group::RenderBindGroup;
