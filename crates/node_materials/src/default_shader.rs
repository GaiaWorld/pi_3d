use pi_atom::Atom;
use pi_scene_shell::prelude::*;
use pi_scene_context::materials::command_sys::ActionMaterial;
use crate::{base::*, prelude::BlockMainTexture};

pub struct DefaultShader;
impl DefaultShader {
    pub const KEY: &'static str = "Default";
    pub fn res() -> ShaderEffectMeta {
        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.values.uint_list.push(UniformPropertyUint(Atom::from("debug_normal"), 0, false));
        nodemat.fs_define = String::from("
layout(location = 0) out vec4 gl_FragColor;

const vec3 light = vec3(1., -3., 2.);
");

        nodemat.vs = String::from("
        vec3 position = A_POSITION;
        vec3 normal = A_NORMAL;
        mat4 finalWorld = PI_ObjectToWorld;
        
        vec4 positionUpdate =  vec4(position, 1.);
        vec4 worldPos =  finalWorld * positionUpdate;
        // vec4 worldPos =  positionUpdate;
        
        gl_Position = PI_MATRIX_VP * worldPos;
        // gl_Position = positionUpdate;
        
        v_pos = worldPos.xyz;
        
        mat3 normalWorld = mat3(finalWorld);
        v_normal = normalize(normalWorld * normal);
        
        v_color = A_COLOR4;        
        ");
        nodemat.fs = String::from("
        vec4 baseColor = v_color;
        baseColor.rgb *= uMainInfo.rgb;
        
        float alpha = 1.0;
        
        vec3 normal = normalize(v_normal);
        // baseColor.rgb *= max(0., dot(normal, normalize(-light)));
        
        // // float level = dot(v_normal, vec3(0., 0., -1.));
        if (debug_normal > 0) {
            baseColor.rgb = mix(baseColor.rgb, v_normal, 0.5);
        }
        // // baseColor.rgb = (v_pos + vec3(1., 1., 1.)) / 2.;
        
        // baseColor.rgb = v_normal;
        
        baseColor.rgb = max(vec3(0.02, 0.02, 0.02), baseColor.rgb);
        
        gl_FragColor = vec4(baseColor.rgb, alpha);
        ");
        nodemat.binddefines = BindDefines::MODEL_BIND | BindDefines::VIEWER | BindDefines::EFFECT_VALUE_BIND;

        nodemat.values.vec3_list.push(UniformPropertyVec3(Atom::from(BlockMainTexture::KEY_COLOR), [1., 1., 1.], true));
        nodemat.varyings = Varyings(
            vec![
                Varying { 
                    format: Atom::from("vec3"),
                    name: Atom::from("v_normal"),
                },
                Varying { 
                    format: Atom::from("vec3"),
                    name: Atom::from("v_pos"),
                },
                Varying { 
                    format: Atom::from("vec4"),
                    name: Atom::from("v_color"),
                },
            ]
        );

        nodemat.meta()
    }
}

pub struct PluginDefaultMaterial;
impl Plugin for PluginDefaultMaterial {
    fn build(&self, app: &mut App) {
        
        let asset_mgr = app.world.get_resource::<ShareAssetMgr<ShaderEffectMeta>>().unwrap().clone();
        ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(DefaultShader::KEY), DefaultShader::res());

    }
}

