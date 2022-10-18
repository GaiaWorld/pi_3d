use pi_ecs::{prelude::{Query, Id}, query::With, sys::system};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::EntityTree;

use crate::{object::GameObject, meshes::Mesh};

pub struct PipelineCreate;
#[setup]
impl PipelineCreate {
    #[system]
    pub fn create(
        query_meshes: Query<GameObject, (&Mesh)>,
        // asset_pipelines: Res<'static, Share<AssetMgr<RenderRes<Buffer>>>>,
    ) {

    }
}