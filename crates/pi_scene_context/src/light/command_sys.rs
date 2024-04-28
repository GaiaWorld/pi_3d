
use pi_scene_shell::prelude::*;

use crate::{
    layer_mask::prelude::*, shadow::prelude::LightLinkedShadowID, transforms::command_sys::*, viewer::prelude::ViewerDistanceCompute
};

use super::{
    base::*,
    // shadow_generator::*,
    command::*, spot::SpotLightAngle, hemisphere::HemiGrounds,
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

pub fn sys_act_light_param(
    mut cmds: ResMut<ActionListLightParam>,
    mut directlights: Query<&mut LightDirection>,
    mut spot_lights: Query<&mut SpotLightAngle>,
    mut param_lights: Query<&mut LightParam>,
) {
    cmds.drain().drain(..).for_each(|OpsLightParam(entity, cmd)| {
        match cmd {
            ELightModify::Color(r, g, b) => if let Ok(mut lightcolor) = param_lights.get_mut(entity) {
                lightcolor.color = Vector3::new(r, g, b);
            },
            ELightModify::LightingType(val) => if let Ok(mut item) = param_lights.get_mut(entity) {
                item.mode = val;
            },
            ELightModify::Directional(val) => if let Ok(mut item) = directlights.get_mut(entity) {
                *item = LightDirection(val);
            },
            ELightModify::SpotAngle(inangle, outangle) => if let Ok(mut out_val) = spot_lights.get_mut(entity) {
                out_val.out_value = outangle;
                out_val.in_value = inangle;
            },
            ELightModify::Strength(val) => if let Ok(mut light) = param_lights.get_mut(entity) {
                light.strength = val;
            },
            ELightModify::Radius(val) => if let Ok(mut light) = param_lights.get_mut(entity) {
                light.radius = val;
            },
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
            .insert(LightParam::default())
            .insert(LightLinkedShadowID(None))
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
        commands.insert(SpotLightAngle{ in_value: 0.2, out_value: 0.3 });
        commands.insert(ViewerDistanceCompute::Base);
    }
    pub(crate) fn as_point_light(
        commands: &mut EntityCommands,
    ) {
        Self::as_light(commands);
        // Self::as_shadow_light(commands);
        commands.insert(PointLight);
        commands.insert(LayerMask::default());
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

