use std::mem::replace;

use pi_atom::Atom;
use pi_ecs::{prelude::{ResMut, Query, Res}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{ObjectID, GameObject}, engine_shell::EnginShell};
use pi_render::rhi::device::RenderDevice;
use pi_scene_context::{resources::RenderDynUniformBuffer, materials::bind_group::{RenderBindGroupPool, RenderBindGroupKey}};


use super::{value_uniform::{ValueBindDesc, ValueUniformStatistics, ValueUniformDynBindOffset}, texture_uniform::{TextureBindDesc, UniformTextureStatistics}};


#[derive(Debug)]
enum ECommand {
    Create(ObjectID, ValueBindDesc, TextureBindDesc)
}

#[derive(Debug, Default)]
struct SingleCommands {
    pub list: Vec<ECommand>,
}

struct SysCommand;
#[setup]
impl SysCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleCommands>,
        mut materials: Query<GameObject, (Write<ValueBindDesc>, Write<ValueUniformStatistics>, Write<TextureBindDesc>, Write<ValueUniformDynBindOffset>)>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut bindgrouppool: ResMut<RenderBindGroupPool>,
        device: Res<RenderDevice>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);
        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Create(mat, values, textures) => {
                    if let Some((mut value_binddesc, mut value_statistics, mut tex_binddesc, mut bindoffset)) = materials.get_mut(mat) {
                        let mat4_count      = values.mat4_name_list.len() as u8;
                        let mat2_count      = values.mat2_name_list.len() as u8;
                        let vec4_count      = values.vec4_name_list.len() as u8;
                        let vec2_count      = values.vec2_name_list.len() as u8;
                        let float_count     = values.float_name_list.len() as u8;
                        let int_count       = values.int_name_list.len() as u8;
                        let uint_count      = values.uint_name_list.len() as u8;
                        let align_bytes     = 16;

                        let statistics = ValueUniformStatistics::new(mat4_count, mat2_count, vec4_count, vec2_count, float_count, int_count, uint_count, align_bytes);
                        bindoffset.write(ValueUniformDynBindOffset { bind_offset: dynbuffer.alloc_binding_with_asbind(&statistics) });
                        bindgrouppool.creat(&device, statistics.label.clone(), statistics.layout_entries().as_slice(), ValueUniformStatistics::BIND_GROUP_SET);
                        value_statistics.write(statistics);
                        value_binddesc.write(values);
                        tex_binddesc.write(textures);

                    }
                },
            }
        });
    }
}

pub trait InterfaceMaterialUniformDesc {
    fn material_uniforms(
        &self,
        material: ObjectID,
        value_uniforms: ValueBindDesc,
        texture_uniforms: TextureBindDesc,
    ) -> &Self;
}
impl InterfaceMaterialUniformDesc for EnginShell {
    fn material_uniforms(
        &self,
        material: ObjectID,
        value_uniforms: ValueBindDesc,
        texture_uniforms: TextureBindDesc,
    ) -> &Self {
        let world = self.world();

        world.get_resource_mut::<SingleCommands>().unwrap().list.push(ECommand::Create(material, value_uniforms, texture_uniforms));

        self
    }
}