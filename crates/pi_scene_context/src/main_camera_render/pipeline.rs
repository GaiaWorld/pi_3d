
use pi_assets::{asset::Handle};
use render_shader::{shader::{ResShader, KeyShader}};

use crate::{
    renderers::{
        render_object::{RenderObjectBindGroup},
    },
    meshes::model::BuildinModelBind,
    bindgroup::{RenderBindGroupKey, RenderBindGroupPool}
};


pub struct AssetResShaderMainCamera{
    pub shader: Handle<ResShader>,
    pub shaderkey: KeyShader,
    pub material_bind_group: RenderBindGroupKey,
    pub material_bind_offet: Option<u32>,
    pub tex_bind_group: Option<RenderBindGroupKey>,
}
impl AssetResShaderMainCamera {
    pub fn renderobj_bind_group(&self, model: &BuildinModelBind, bind_groups: &mut Vec<RenderObjectBindGroup>, pool: &RenderBindGroupPool) -> bool {
        let mut resut = true;


        if let Some(bindgroup) = &self.tex_bind_group {
            if pool.get(bindgroup).unwrap().bind_group.is_some() {
                bind_groups.push(RenderObjectBindGroup {
                    bind_group: bindgroup.clone(),
                    offsets: vec![],
                });
            } else {
                resut = false;
            }
        }

        if resut {
            if let Some(offset) = self.material_bind_offet {
                bind_groups.push(RenderObjectBindGroup {
                    bind_group: self.material_bind_group.clone(),
                    offsets: vec![*model.bind_offset, offset],
                });

            } else {
                bind_groups.push(RenderObjectBindGroup {
                    bind_group: self.material_bind_group.clone(),
                    offsets: vec![*model.bind_offset],
                });
            }
        }

        resut
    }
}