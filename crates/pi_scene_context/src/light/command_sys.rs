
use bevy_ecs::system::ResMut;
use pi_scene_shell::prelude::*;

use crate::{
    layer_mask::prelude::*, shadow::prelude::LightLinkedShadowID, transforms::command_sys::*, viewer::prelude::ViewerDistanceCompute
};

use super::{
    base::*,
    // shadow_generator::*,
    command::*, spot::{SpotLightInAngle, SpotLightOutAngle}, hemisphere::HemiGrounds,
};

pub fn sys_create_light(
    mut cmds: ResMut<ActionListLightCreate>,
    mut commands: Commands,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
) {
    cmds.drain().drain(..).for_each(|OpsLightCreate(scene, entity, ltype)| {
        let mut lightcmd = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else {
            disposereadylist.push(OpsDisposeReadyForRef::ops(entity));
            return;
        };

        ActionTransformNode::init(&mut lightcmd, scene);
        match ltype {
            ELightType::Direct => ActionLight::as_direct_light(&mut lightcmd),
            ELightType::Spot => ActionLight::as_spot_light(&mut lightcmd),
            ELightType::Point => ActionLight::as_point_light(&mut lightcmd),
            ELightType::Hemispheric => ActionLight::as_hemi_light(&mut lightcmd),
        }

    });
}

pub fn sys_act_light_color(
    mut cmds: ResMut<ActionListLightColor>,
    mut lights: Query<&mut LightColor>,
) {
    cmds.drain().drain(..).for_each(|OpsLightColor(entity, color)| {
        if let Ok(mut lightcolor) = lights.get_mut(entity) {
            lightcolor.0 = color;
        }
    });
}

pub fn sys_act_light_param(
    mut cmds: ResMut<ActionListLightParam>,
    mut directlights: Query<&mut LightDirection>,
    mut lights: Query<&mut LightingMode>,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            ELightModifyCommand::LightingType(entity, val) => {
                if let Ok(mut item) = lights.get_mut(entity) {
                    *item = val;
                }
            },
            ELightModifyCommand::Directional(entity, val) => {
                if let Ok(mut item) = directlights.get_mut(entity) {
                    *item = LightDirection(val);
                }
            },
            // ELightModifyCommand::ShadowMinz(entity, val) => {
            //     if let Ok(mut item) = shadowlights.get_mut(entity) {
            //         *item.0 = ShadowMinZ(val);
            //     }
            // },
            // ELightModifyCommand::ShadowMaxz(entity, val) => {
            //     if let Ok(mut item) = shadowlights.get_mut(entity) {
            //         *item.1 = ShadowMaxZ(val);
            //     }
            // },
            // ELightModifyCommand::ShadowFrustumSize(entity, val) => {
            //     if let Ok(mut item) = shadowlights.get_mut(entity) {
            //         *item.2 = ShadowFrustumSize(val);
            //     }
            // },
            // ELightModifyCommand::Bias(entity, val) => {
            //     if let Ok(mut item) = shadowlights.get_mut(entity) {
            //         *item.3 = ShadowBias(val);
            //     }
            // },
            // ELightModifyCommand::NormalBias(entity, val) => {
            //     if let Ok(mut item) = shadowlights.get_mut(entity) {
            //         *item.4 = ShadowNormalBias(val);
            //     }
            // },
            // ELightModifyCommand::DepthScale(entity, val) => {
            //     if let Ok(mut item) = shadowlights.get_mut(entity) {
            //         *item.5 = ShadowDepthScale(val);
            //     }
            // },
            // ELightModifyCommand::AtlasSize(entity, val) => {
            //     if let Ok(mut item) = shadowlights.get_mut(entity) {
            //         *item.6 = ShadowAtlasSize(val);
            //         *item.8 = ViewerSize(val, val);
            //     }
            // },
            // ELightModifyCommand::ShadowEnable(entity, val) => {
            //     if let Ok(mut item) = shadowlights.get_mut(entity) {
            //         *item.7 = ShadowEnable(val);
            //     }
            // },
        }
    });
}

pub fn sys_act_spot_light_angle(
    mut cmds: ResMut<ActionListSpotLightAngle>,
    mut lights: Query<(&mut SpotLightOutAngle, &mut SpotLightInAngle)>,
) {
    cmds.drain().drain(..).for_each(|OpsSpotLightAngle(entity, outangle, inangle)| {
        if let Ok((mut out_val, mut in_val)) = lights.get_mut(entity) {
            out_val.0 = outangle;
            in_val.0 = inangle;
        }
    });
}

pub fn sys_act_light_strength(
    mut cmds: ResMut<ActionListLightStrength>,
    mut lights: Query<&mut LightStrength>,
) {
    cmds.drain().drain(..).for_each(|OpsLightStrength(entity, val)| {
        if let Ok(mut light) = lights.get_mut(entity) {
            *light = LightStrength(val);
        }
    });
}


pub fn sys_act_light_radius(
    mut cmds: ResMut<ActionListLightRadius>,
    mut lights: Query<&mut LightRadius>,
) {
    cmds.drain().drain(..).for_each(|OpsLightRadius(entity, val)| {
        if let Ok(mut light) = lights.get_mut(entity) {
            *light = LightRadius(val);
        }
    });
}


pub struct ActionLight;
impl ActionLight {
    pub(crate) fn as_light(
        commands: &mut EntityCommands,
    ) {
        // log::warn!("CreateLight {:?}", commands.id());
        commands
            .insert(LightingMode::Lambert)
            .insert(LightColor::default())
            .insert(LightLinkedShadowID(None))
            .insert(LightStrength::default())
            ;
    }
    pub(crate) fn as_direct_light(
        commands: &mut EntityCommands,
    ) {
        Self::as_light(commands);
        commands.insert(DirectLight);
        commands.insert(LayerMask::default());
        commands.insert(ViewerDistanceCompute::Direction);
        commands.insert(LightDirection::default());
    }
    pub(crate) fn as_spot_light(
        commands: &mut EntityCommands,
    ) {
        Self::as_light(commands);
        // Self::as_shadow_light(commands);
        commands.insert(SpotLight);
        commands.insert(LayerMask::default());
        commands.insert(LightDirection::default());
        commands.insert(SpotLightInAngle(0.2));
        commands.insert(SpotLightOutAngle(0.5));
        commands.insert(LightRadius::default());
        commands.insert(ViewerDistanceCompute::Base);
    }
    pub(crate) fn as_point_light(
        commands: &mut EntityCommands,
    ) {
        Self::as_light(commands);
        // Self::as_shadow_light(commands);
        commands.insert(PointLight);
        commands.insert(LayerMask::default());
        commands.insert(LightRadius::default());
        commands.insert(ViewerDistanceCompute::Base);
    }
    pub(crate) fn as_hemi_light(
        commands: &mut EntityCommands,
    ) {
        Self::as_light(commands);
        // Self::as_shadow_light(commands);
        commands.insert(HemisphericLight);
        commands.insert(LayerMask::default());
        commands.insert(HemiGrounds::default());
        commands.insert(ViewerDistanceCompute::Base);
    }
}

