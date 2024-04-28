// use bevy_ecs::{system::Resource, entity::Entity};
use pi_bevy_render_plugin::GraphError;
use pi_world::world::Entity;
use pi_slotmap::Key;

pub type EErorr = u32;

// #[derive(Resource)]
pub struct ErrorRecord(pub Vec<u32>, pub bool);
impl ErrorRecord {
    pub fn record(&mut self, entity: Entity, error: EErorr) {
        if self.1 {
            self.0.push(entity.index() as u32);
            self.0.push(error);
        }
    }
    pub fn drain(&mut self, mut count: usize) -> std::vec::Drain<'_, u32> {
        count = count.min(self.0.len());
        self.0.drain(0..count)
    }

    pub fn graphic(&mut self, node: Entity, err: GraphError) {
        let error = match err {
            GraphError::NoneNGraph(_) => Self::ERROR_GRAPHIC_NONE_NGRAPHIC,
            GraphError::NoneNode(_) => Self::ERROR_GRAPHIC_NONE_NODE,
            GraphError::ExitNode(_) => Self::ERROR_GRAPHIC_EXTI_NODE,
            GraphError::RunNGraphError(_) => Self::ERROR_GRAPHIC_RUN_ERR,
            GraphError::BuildError(_) => Self::ERROR_GRAPHIC_BUILD_ERR,
            GraphError::SubGraphInputError => Self::ERROR_GRAPHIC_INPUT_ERR,
            GraphError::SubGraphOutputError => Self::ERROR_GRAPHIC_OUTPUT_ERR,
            GraphError::CustomBuildError(_) => Self::ERROR_GRAPHIC_CUSTOM_BUILD_ERR,
            GraphError::CustomRunError(_) => Self::ERROR_GRAPHIC_CUSTOM_RUN_ERR,
            GraphError::WrongNodeType => Self::ERROR_GRAPHIC_WRONG_NODE_TYPE,
            GraphError::MismatchedParam => Self::ERROR_GRAPHIC_MISMATCH_PARAM,
            GraphError::CrossGraphDepend(_, _) => Self::ERROR_GRAPHIC_BUILD_ERR,
        };

        self.record(node, error);
    }
        
    pub const ERROR_VERTEX_BUFFER_CREATE_FAIL: EErorr       = 00001;
    pub const ERROR_BIND_BUFFER_CREATE_FAIL: EErorr         = 00002;
    pub const ERROR_BIND_GROUP_CREATE_FAIL: EErorr          = 00003;
    pub const ERROR_SHADER_CREATE_FAIL: EErorr              = 00004;
    pub const ERROR_PIPELINE_CREATE_FAIL: EErorr            = 00005;
    pub const ERROR_TEXTURE_CREATE_FAIL: EErorr             = 00006;
    pub const ERROR_TEXTURE_VIEW_CREATE_FAIL: EErorr        = 00007;
    pub const ERROR_SAMPER_CREATE_FAIL: EErorr              = 00008;
    pub const ERROR_BIND_VIEWER_CREATE_FAIL: EErorr         = 00009;

    pub const ERROR_BIND_EFFECT_CREATE_FAIL: EErorr         = 00010;
    pub const ERROR_MODIFY_ERROR_MATERIAL_TEXTURE: EErorr   = 00011;
    pub const ERROR_MATERIAL_SHADER_NOTFOUND: EErorr        = 00012;
    pub const ERROR_USE_MATERIAL_NULL_MAT: EErorr           = 00013;
    pub const ERROR_USE_MATERIAL_NULL_TARGET: EErorr        = 00013;
    
    pub const ERROR_ANIMATION_START_FAIL: EErorr            = 00100;
    pub const ERROR_ANIMATION_PAUSE_FAIL: EErorr            = 00101;
    pub const ERROR_ANIMATION_STOP_FAIL: EErorr             = 00102;
    pub const ERROR_ADD_TARGET_ANIMATION_FAIL: EErorr       = 00103;
    
    pub const ERROR_GRAPHIC_NONE_NGRAPHIC: EErorr           = 00100;
    pub const ERROR_GRAPHIC_NONE_NODE: EErorr               = 00101;
    pub const ERROR_GRAPHIC_EXTI_NODE: EErorr               = 00102;
    pub const ERROR_GRAPHIC_RUN_ERR: EErorr                 = 00103;
    pub const ERROR_GRAPHIC_BUILD_ERR: EErorr               = 00104;
    pub const ERROR_GRAPHIC_INPUT_ERR: EErorr               = 00105;
    pub const ERROR_GRAPHIC_OUTPUT_ERR: EErorr              = 00106;
    pub const ERROR_GRAPHIC_CUSTOM_BUILD_ERR: EErorr        = 00107;
    pub const ERROR_GRAPHIC_CUSTOM_RUN_ERR: EErorr          = 00108;
    pub const ERROR_GRAPHIC_WRONG_NODE_TYPE: EErorr         = 00109;
    pub const ERROR_GRAPHIC_MISMATCH_PARAM: EErorr          = 00110;

    pub const ERROR_ENTITY_NONE: EErorr                     = 10001;
    pub const ERROR_ENTITY_DISPOSED: EErorr                 = 10002;

    pub const ERROR_SCENE_NONE: EErorr                      = 20001;
    pub const ERROR_SCENE_BIND_FAIL: EErorr                 = 20002;

    pub const ERROR_PASS_BIND_SCENE_NONE: EErorr            = 50000;
    pub const ERROR_PASS_BIND_VIEWER_NONE: EErorr           = 50001;
    pub const ERROR_PASS_SET0_FAIL: EErorr                  = 50002;
    pub const ERROR_PASS_BIND_MODEL_NONE: EErorr            = 50003;
    pub const ERROR_PASS_BIND_EFFECT_VALUE_NONE: EErorr     = 50004;
    pub const ERROR_PASS_BIND_LIGHTING_NONE: EErorr         = 50005;
    pub const ERROR_PASS_BIND_SKIN_NONE: EErorr             = 50006;
    pub const ERROR_PASS_SET1_FAIL: EErorr                  = 50007;
    pub const ERROR_PASS_SET2_FAIL: EErorr                  = 50008;
    pub const ERROR_PASS_BIND_SHADOW_NONE: EErorr           = 50010;
    pub const ERROR_PASS_BIND_BRDF_NONE: EErorr             = 50011;
    pub const ERROR_PASS_BIND_CAMERA_OPAQUE_NONE: EErorr    = 50012;
    pub const ERROR_PASS_BIND_CAMERA_DEPTH_NONE: EErorr     = 50013;
    pub const ERROR_PASS_BIND_ENV_NONE: EErorr              = 50014;
    pub const ERROR_PASS_SET3_FAIL: EErorr                  = 50015;
    pub const ERROR_PASS_BIND_GROUPS_FAIL: EErorr           = 50016;
    pub const ERROR_PASS_SHADER_FAIL: EErorr                = 50017;
    pub const ERROR_PASS_PIPELINE_FAIL: EErorr              = 50018;
    pub const ERROR_PASS_DRAW_FAIL: EErorr                  = 50019;

}

