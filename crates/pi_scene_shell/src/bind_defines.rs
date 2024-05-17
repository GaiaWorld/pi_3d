pub struct BindDefines;
impl BindDefines {
    pub const SCENE_EFFECT: u32             = 1 << 0;
    pub const LIGHTING: u32                 = 1 << 1;
    pub const SHADOWMAP: u32                = 1 << 2;
    pub const ENVIRONMENT_BRDF_TEXTURE: u32 = 1 << 3;
    pub const ENVIRONMENT_LIGHTING: u32     = 1 << 4;
    pub const SCREEN_OPAQUE_TARGET: u32     = 1 << 5;
    pub const SCREEN_DEPTH_TARGET: u32      = 1 << 6;
    pub const EFFECT_TEXTURE_BIND: u32      = 1 << 7;
    pub const EFFECT_VALUE_BIND: u32        = 1 << 8;
    pub const VIEWER: u32                   = 1 << 9;
    pub const MODEL_BIND: u32               = 1 << 10;

    // pub const SCREEN_DEPTH_TARGET: u32      = 0b0000_0000_0000_0000_0000_0010_0000_0000;
    pub fn need_viewer(val: u32) -> bool {
        val & Self::VIEWER == Self::VIEWER
    }
    pub fn need_scene_effect(val: u32) -> bool {
        val & Self::SCENE_EFFECT == Self::SCENE_EFFECT
    }
    pub fn need_lighting(val: u32) -> bool {
        val & Self::LIGHTING == Self::LIGHTING
    }
    pub fn need_shadowmap(val: u32) -> bool {
        val & Self::SHADOWMAP == Self::SHADOWMAP
    }
    pub fn need_screen_opaque(val: u32) -> bool {
        val & Self::SCREEN_OPAQUE_TARGET == Self::SCREEN_OPAQUE_TARGET
    }
    pub fn need_screen_depth(val: u32) -> bool {
        val & Self::SCREEN_DEPTH_TARGET == Self::SCREEN_DEPTH_TARGET
    }
    pub fn need_brdf(val: u32) -> bool {
        val & Self::ENVIRONMENT_BRDF_TEXTURE == Self::ENVIRONMENT_BRDF_TEXTURE
    }
    pub fn need_env(val: u32) -> bool {
        val & Self::ENVIRONMENT_LIGHTING == Self::ENVIRONMENT_LIGHTING
    }
    pub fn need_model(val: u32) -> bool {
        val & Self::MODEL_BIND == Self::MODEL_BIND
    }
    pub fn need_effect_value(val: u32) -> bool {
        val & Self::EFFECT_VALUE_BIND == Self::EFFECT_VALUE_BIND
    }
    pub fn need_effect_textures(val: u32) -> bool {
        val & Self::EFFECT_TEXTURE_BIND == Self::EFFECT_TEXTURE_BIND
    }
    
    pub fn need_bind_group_set0(val: u32) -> bool {
        Self::need_viewer(val) || Self::need_scene_effect(val)
    }

    pub fn need_bind_group_set1(val: u32) -> bool {
        Self::need_model(val) || Self::need_effect_value(val)
    }

    pub fn need_bind_group_set2(val: u32) -> bool {
        Self::need_effect_textures(val)
    }

    pub fn need_bind_group_set3(val: u32) -> bool {
        false
        // Self::need_lighting(val) || Self::need_shadowmap(val) || Self::need_screen_opaque(val) || Self::need_screen_depth(val) || Self::need_brdf(val) || Self::need_env(val)
    }
}