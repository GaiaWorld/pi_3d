use super::ShaderSetBind;


fn sampler_ty_code(ty: wgpu::SamplerBindingType) -> String {
    match ty {
        wgpu::SamplerBindingType::Filtering     => String::from(crate::prelude::S_SPACE) + crate::prelude::S_SAMPLER + crate::prelude::S_SPACE,
        wgpu::SamplerBindingType::NonFiltering  => String::from(crate::prelude::S_SPACE) + crate::prelude::S_SAMPLER + crate::prelude::S_SPACE,
        wgpu::SamplerBindingType::Comparison    => String::from(crate::prelude::S_SPACE) + "sampler_comparison" + crate::prelude::S_SPACE,
    }
}
pub fn sampler_bind_code(slotname: &str, ty: wgpu::SamplerBindingType, set: u32, bind: u32) -> String {

    // layout(set = 2, binding = 0) uniform texture2D _MainTex;
    let mut result = ShaderSetBind::code_set_bind_head(set, bind);
    result += sampler_ty_code(ty).as_str();
    result += crate::prelude::S_SAMPLER;
    result += slotname;
    result += ";"; result += crate::prelude::S_BREAK;

    result
}