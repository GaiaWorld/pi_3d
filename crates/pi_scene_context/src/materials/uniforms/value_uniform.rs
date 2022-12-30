use lazy_static::__Deref;
use pi_atom::Atom;
use pi_render::rhi::{dyn_uniform_buffer::{BindOffset, AsBind}, device::RenderDevice, bind_group::BindGroup};
use render_data_container::UniformValueBindKey;

use crate::{resources::RenderDynUniformBuffer, materials::bind_group::{RenderBindGroupPool, RenderBindGroupKey, RenderBindGroup}, meshes::model::BuildinModelBind, shaders::FragmentUniformBind};

struct TempMaterialValueBind(pub u32, pub usize);
impl AsBind for TempMaterialValueBind {
    fn index(&self) -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(self.0 as usize)
    }
    fn min_size(&self) -> usize {
        self.1 as usize
    }
}

#[derive(Debug)]
pub struct MaterialValueBind {
    pub bind: u32,
    pub bind_offset: Option<BindOffset>,
    pub mat4_count: u8,
    pub mat2_count: u8,
    pub vec4_count: u8,
    pub vec2_count: u8,
    pub float_count: u8,
    pub int_count: u8,
    pub uint_count: u8,

    pub fill_vec2_count: u8,
    pub fill_int_count: u8,
    
    pub mat4_begin: u32,
    pub mat2_begin: u32,
    pub vec4_begin: u32,
    pub vec2_begin: u32,
    pub float_begin: u32,
    pub int_begin: u32,
    pub uint_begin: u32,
    pub total_size: u32,
    pub bind_group: Atom,
}
impl AsBind for MaterialValueBind {
    fn index(&self) -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(self.bind as usize)
    }
    fn min_size(&self) -> usize {
        self.total_size as usize
    }
}
impl MaterialValueBind {
    pub const LABEL_MASK: &'static str = "#";
    pub const MAT4_BYTES: u32 = 16 * 4;
    pub const MAT2_BYTES: u32 = 4 * 4;
    pub const VEC4_BYTES: u32 = 4 * 4;
    pub const VEC2_BYTES: u32 = 2 * 4;
    pub const FLOAT_BYTES: u32 = 1 * 4;
    pub const INT_BYTES: u32 = 1 * 4;
    pub const UINT_BYTES: u32 = 1 * 4;

    const ALIGN_SIZE: u32 = 16;

    const BIND_INDEX: usize = 1;

    pub const BIND_GROUP_SET: u32 = RenderBindGroupPool::MODEL_BIND_GROUP_SET;

    pub fn new(
        device: &RenderDevice,
        dynbuffer: &mut RenderDynUniformBuffer,
        bind: u32,
        mat4_count: u8,
        mat2_count: u8,
        vec4_count: u8,
        vec2_count: u8,
        float_count: u8,
        int_count: u8,
        uint_count: u8,
        align_bytes: u32,
    ) -> Self {
        let fill_vec2_count    = vec2_count % 2;
        let fill_int_count     = (float_count + int_count + uint_count) % 4;

        let mut total_size = 0;

        let mat4_begin: u32  = total_size;
        total_size += mat4_count as u32 * Self::MAT4_BYTES;

        let mat2_begin: u32  = total_size;
        total_size += mat2_count as u32 * Self::MAT2_BYTES;

        let vec4_begin: u32  = total_size;
        total_size += vec4_count as u32 * Self::VEC4_BYTES;

        let vec2_begin: u32  = total_size;
        total_size += (vec2_count as u32 + fill_vec2_count as u32) * Self::VEC2_BYTES;

        let float_begin: u32 = total_size;
        total_size += float_count as u32 * Self::FLOAT_BYTES;

        let int_begin: u32   = total_size;
        total_size += int_count as u32 * Self::INT_BYTES;

        let uint_begin: u32  = total_size;
        total_size += uint_count as u32 * Self::UINT_BYTES;

        total_size += fill_int_count as u32 * Self::INT_BYTES;

        let label = Atom::from(MaterialValueBind::label(
            mat4_count,
            mat2_count,
            vec4_count,
            vec2_count,
            float_count,
            int_count,
            uint_count,
        ));

        let key = RenderBindGroupKey::from(label);

        let temp = TempMaterialValueBind(bind, total_size as usize);
        let result = Self {
            bind, bind_offset: if total_size > 0 { Some(dynbuffer.alloc_binding_with_asbind(&temp)) } else { None },
            mat4_count, mat2_count, vec4_count, vec2_count, float_count, int_count, uint_count, fill_vec2_count, fill_int_count,
            mat4_begin, mat2_begin, vec4_begin, vec2_begin, float_begin, int_begin, uint_begin, total_size, bind_group: key,
        };

        // println!("{:?}", result);

        result
    }

    pub fn bind_offset(&self) -> Option<u32> {
        if let Some(offset) = &self.bind_offset {
            Some(*offset.deref())
        } else {
            None
        }
    }

    pub fn calc_fill(
        &mut self,
    ) {
        self.fill_vec2_count    = self.vec2_count % 2;
        self.fill_int_count     = (self.float_count + self.int_count + self.uint_count) % 4;
    }

    pub fn label(
        mat4_count: u8,
        mat2_count: u8,
        vec4_count: u8,
        vec2_count: u8,
        float_count: u8,
        int_count: u8,
        uint_count: u8,
    ) -> String {
        mat4_count.to_string() 
        + Self::LABEL_MASK + &mat2_count.to_string() 
        + Self::LABEL_MASK + &vec4_count.to_string() 
        + Self::LABEL_MASK + &vec2_count.to_string() 
        + Self::LABEL_MASK + &float_count.to_string() 
        + Self::LABEL_MASK + &int_count.to_string()
        + Self::LABEL_MASK + &uint_count.to_string()
    }

    pub fn layout_entries(
        total_size: usize
    ) -> Vec<wgpu::BindGroupLayoutEntry> {
        if total_size > 0 {
            vec![
                BuildinModelBind::ENTRY,
                Self::layout_entry(total_size),
            ]
        } else {
            vec![
                BuildinModelBind::ENTRY,
            ]
        }
    }

    pub fn calc_key(
        &self,
    ) -> UniformValueBindKey {
        Self::_calc_key(
            self.mat4_count    as usize
            , self.mat2_count as usize
            , self.vec4_count as usize
            , self.vec2_count as usize
            , self.float_count as usize
            , self.int_count  as usize
            , self.uint_count as usize
        )
    }
    fn _calc_key(
        mat4_count : usize,
        mat2_count : usize,
        vec4_count : usize,
        vec2_count : usize,
        float_count: usize,
        int_count  : usize,
        uint_count : usize,
    ) -> UniformValueBindKey {
        mat4_count   as UniformValueBindKey * (1 << (8 * 0))
        + mat2_count   as UniformValueBindKey * (1 << (8 * 1))
        + vec4_count   as UniformValueBindKey * (1 << (8 * 2))
        + vec2_count   as UniformValueBindKey * (1 << (8 * 3))
        + float_count  as UniformValueBindKey * (1 << (8 * 4))
        + int_count    as UniformValueBindKey * (1 << (8 * 5))
        + uint_count   as UniformValueBindKey * (1 << (8 * 6))
    }
    
    fn layout_entry(
        total_size: usize,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: Self::BIND_INDEX as u32,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: true,
                min_binding_size: wgpu::BufferSize::new(total_size as wgpu::BufferAddress),
            },
            count: None,
        }
    }

    pub fn dyn_entry<'a>(
        &self,
        dynbuffer: &'a RenderDynUniformBuffer,
    ) -> wgpu::BindGroupEntry<'a> {
        wgpu::BindGroupEntry {
            binding: self.bind,
            resource: wgpu::BindingResource::Buffer(
                wgpu::BufferBinding {
                    buffer: dynbuffer.buffer().unwrap(),
                    offset:  0 as wgpu::BufferAddress,
                    size: wgpu::BufferSize::new(self.total_size as wgpu::BufferAddress),
                }
            ),
        }
    }
    
    pub fn bind_group(
        &self,
        device: &RenderDevice,
        group: &mut RenderBindGroup,
        dynbuffer: &RenderDynUniformBuffer,
    ) {
        if self.bind_offset.is_some() {
            group.bind_group = Some(
                BindGroup::from(
                    device.create_bind_group(
                        &wgpu::BindGroupDescriptor {
                            label: Some(self.bind_group.as_str()),
                            layout: &group.layout,
                            entries: &[
                                BuildinModelBind::dyn_entry(dynbuffer),
                                self.dyn_entry(dynbuffer),
                            ],
                        }
                    )
                )
            ); 
        } else {
            group.bind_group = Some(
                BindGroup::from(
                    device.create_bind_group(
                        &wgpu::BindGroupDescriptor {
                            label: Some(self.bind_group.as_str()),
                            layout: &group.layout,
                            entries: &[
                                BuildinModelBind::dyn_entry(dynbuffer),
                            ],
                        }
                    )
                )
            ); 
        }
    }

    pub fn to_code(
        binddesc: &ValueBindDesc,
        statistics: &MaterialValueBind,
    ) -> String {
        let mut code = String::from("layout(set = 1") + ", bind = " + &binddesc.binding.to_string() + ") uniform MatParam {\r\n";

        binddesc.mat4_name_list.iter().for_each(|name| {
            code += "mat4 ";
            code += &name;
            code += ";\r\n";
        });
        
        binddesc.mat2_name_list.iter().for_each(|name| {
            code += "mat2 ";
            code += &name;
            code += ";\r\n";
        });
        
        binddesc.vec4_name_list.iter().for_each(|name| {
            code += "vec4 ";
            code += &name;
            code += ";\r\n";
        });
        
        binddesc.vec2_name_list.iter().for_each(|name| {
            code += "vec2 ";
            code += &name;
            code += ";\r\n";
        });
        for i in 0..statistics.fill_vec2_count {
            code += "vec2 _placeholder_vec2_";
            code += &i.to_string();
            code += ";\r\n";
        }
        
        binddesc.float_name_list.iter().for_each(|name| {
            code += "float ";
            code += &name;
            code += ";\r\n";
        });
        
        binddesc.int_name_list.iter().for_each(|name| {
            code += "int ";
            code += &name;
            code += ";\r\n";
        });
        
        binddesc.uint_name_list.iter().for_each(|name| {
            code += "uint ";
            code += &name;
            code += ";\r\n";
        });
        for i in 0..statistics.fill_int_count {
            code += "uint _placeholder_int_";
            code += &i.to_string();
            code += ";\r\n";
        }

        code + "}\r\n"
    }
}


#[derive(Debug)]
pub struct ValueBindDesc {
    pub binding: u32,
    pub mat4_name_list: Vec<Atom>,
    pub mat2_name_list: Vec<Atom>,
    pub vec4_name_list: Vec<Atom>,
    pub vec2_name_list: Vec<Atom>,
    pub float_name_list: Vec<Atom>,
    pub int_name_list: Vec<Atom>,
    pub uint_name_list: Vec<Atom>,
}
impl ValueBindDesc {
    pub fn label(&self) -> String {
        let mut result = String::from("");

        self.mat4_name_list.iter().for_each(|name| {
            result += "#";
            result += name.as_str();
        });
        
        self.mat2_name_list.iter().for_each(|name| {
            result += "#";
            result += name.as_str();
        });

        self.vec4_name_list.iter().for_each(|name| {
            result += "#";
            result += name.as_str();
        });

        self.vec2_name_list.iter().for_each(|name| {
            result += "#";
            result += name.as_str();
        });

        self.float_name_list.iter().for_each(|name| {
            result += "#";
            result += name.as_str();
        });

        self.uint_name_list.iter().for_each(|name| {
            result += "#";
            result += name.as_str();
        });

        result
    }
}