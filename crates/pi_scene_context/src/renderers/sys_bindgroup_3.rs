use std::sync::Arc;
use pi_scene_shell::prelude::*;
use crate::{
    pass::*,
    scene::{prelude::*, environment::{brdf::{BRDFTexture, BRDFSampler}, environment_texture::{EnvTexture, EnvIrradiance, EnvSampler}}}
};

use super::base::create_bind_group;

pub fn sys_set3_modify(
    
) {
    
}


// fn bind_lighting(scene_lighting: &SceneLightingInfos, modellighting: &ModelLightingIndexs) -> Option<(Arc<ShaderBindSceneLightInfos>, Arc<BindModelLightIndexs>)> {
//     Some((scene_lighting.0.clone(), modellighting.bind.as_ref().unwrap().clone()))
// }