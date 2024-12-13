
use pi_scene_shell::prelude::*;

pub use super::material::*;
pub use super::shader_effect::*;
pub use super::command::*;
pub use super::uniforms::{
    uniform::*,
    float::*,
    // int::*,
    uint::*,
    vec2::*,
    vec4::*,
    // mat2::*,
    mat4::*,
    texture::*,
    // sys_uniform::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageMaterial {
    MatCreate,
    _MatCreate,
    MatUse,
    MatCommand,
    MatReady,
    MatDispose,
}

#[derive(Resource, Default)]
pub struct StateMaterial {
    pub count: u32,
    pub count_ready: u32,
    pub count_tex0: u32,
    pub count_tex0_ready: u32,
    pub count_tex1: u32,
    pub count_tex1_ready: u32,
    pub count_tex2: u32,
    pub count_tex2_ready: u32,
    pub count_tex3: u32,
    pub count_tex3_ready: u32,
    pub count_tex4: u32,
    pub count_tex4_ready: u32,
    pub count_tex5: u32,
    pub count_tex5_ready: u32,
    pub count_tex6: u32,
    pub count_tex6_ready: u32,
}

#[derive(SystemParam)]
pub struct ActionSetMaterial<'w> {
    pub usemat: ResMut<'w, ActionListMaterialUse>,
    pub create: ResMut<'w, ActionListMaterialCreate>,
    pub val: ResMut<'w, ActionListUniformVal>,
    pub valb: ResMut<'w, ActionListUniformValB>,
}
impl<'w> MemSize for ActionSetMaterial<'w> {
    fn memsize(&self) -> usize {
        self.usemat.memsize()
        + self.create.memsize()
        + self.val.memsize()
        + self.valb.memsize()
    }
}

#[cfg(feature = "use_bevy")]
pub type StateMaterialQuery = QueryState<(&'static AssetResShaderEffectMeta, &'static EffectTextureSamplersComp)>;
#[cfg(not(feature = "use_bevy"))]
pub type StateMaterialQuery = QueryState<(&'static AssetResShaderEffectMeta, &'static EffectTextureSamplersComp), ()>;

pub fn sys_state_material(
    mut state: ResMut<StateMaterial>,
    materials: Query<(&AssetResShaderEffectMeta, &EffectTextureSamplersComp)>,
    mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_state_material"));
    state.count = 0;
    state.count_ready = 0;

    materials.iter().for_each(|(meta, texs)| {
        let meta = meta.0.as_ref().unwrap();
        state.count += 1;
        if let Some(texs) = &texs.0 {
            if texs.textures.len() == meta.textures.len() {
                state.count_ready += 1;
            }
        } else if meta.textures.len() == 0 {
            state.count_ready += 1;
        }
    });
}