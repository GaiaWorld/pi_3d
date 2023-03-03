use std::sync::Arc;

use pi_assets::asset::Handle;
use pi_render::{renderer::{bind_group::{BindGroupUsage, BindGroupLayout, KeyBindGroupLayout}, pipeline::KeyRenderPipeline, draw_obj::{DrawObj, DrawBindGroup, DrawBindGroups}, draw_obj_list::DrawList, shader::KeyShaderSetBlocks}, render_3d::{bind_groups::{texture_sampler::BindGroupTextureSamplers, model::BindGroupModel, scene::BindGroupScene}, shader::shader::EKeyShader3DSetBlock}, rhi::{pipeline::RenderPipeline, asset::RenderRes}};


#[derive(Debug, Clone)]
pub struct BindGroups3D {
    pub scene: Arc<BindGroupScene>,
    pub model: Arc<BindGroupModel>, 
    pub textures: Option<Arc<BindGroupTextureSamplers>>,
}
impl BindGroups3D {
    pub fn create(
        scene: Arc<BindGroupScene>,
        model: Arc<BindGroupModel>, 
        textures: Option<Arc<BindGroupTextureSamplers>>,
    ) -> Self {
        Self { scene, model, textures }
    }
    pub fn key_set_blocks(&self) -> KeyShaderSetBlocks<4, EKeyShader3DSetBlock> {
        let mut key_set_blocks = [None, None, None, None];

        key_set_blocks[0] = Some(EKeyShader3DSetBlock::Scene(self.scene.key.clone()));

        key_set_blocks[1] = Some(EKeyShader3DSetBlock::Model(self.model.key.clone()));

        if let Some(set_2) = &self.textures {
            key_set_blocks[2] = Some(EKeyShader3DSetBlock::TextureSampler(set_2.key.clone()));
        }

        KeyShaderSetBlocks(key_set_blocks)
    }
    pub fn bind_group_layouts(&self) -> [Option<Handle<BindGroupLayout>>; 4] {
        let mut bind_group_layouts = [None, None, None, None];
        
        bind_group_layouts[0] = Some(self.scene.bind_group.layout());

        bind_group_layouts[1] = Some(self.model.bind_group.layout());

        if let Some(set_2) = &self.textures {
            bind_group_layouts[2] = Some(set_2.bind_group.layout());
        }

        bind_group_layouts
    }
    pub fn key_bindgroup_layouts(&self) -> [Option<KeyBindGroupLayout>; 4] {
        let mut key_bindgroup_layouts = [None, None, None, None];
        
        key_bindgroup_layouts[0] = Some(self.scene.bind_group.key_layout());

        key_bindgroup_layouts[1] = Some(self.model.bind_group.key_layout());

        if let Some(set_2) = &self.textures {
            key_bindgroup_layouts[2] = Some(set_2.bind_group.key_layout());
        }

        key_bindgroup_layouts
    }
    pub fn groups(&self) -> DrawBindGroups {
        let mut groups = DrawBindGroups::default();
        
        groups.insert_group(0, DrawBindGroup::GroupUsage(self.scene.bind_group.clone()));
        groups.insert_group(1, DrawBindGroup::GroupUsage(self.model.bind_group.clone()));

        if let Some(set_2) = &self.textures {
            groups.insert_group(2, DrawBindGroup::GroupUsage(set_2.bind_group.clone()));
        }

        groups
    }
}
pub type KeyPipeline3D = KeyRenderPipeline<4, EKeyShader3DSetBlock>;
pub type Pipeline3D = RenderRes<RenderPipeline>;
pub type Pipeline3DUsage = Handle<Pipeline3D>;

pub type DrawObj3D = DrawObj;
pub type DrawList3D = DrawList;