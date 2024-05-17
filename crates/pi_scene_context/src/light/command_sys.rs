
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

        match ltype {
            ELightType::Direct =>       lightcmd.insert((ActionTransformNode::init(scene), ActionLight::as_direct_light())),
            ELightType::Spot =>         lightcmd.insert((ActionTransformNode::init(scene), ActionLight::as_spot_light())),
            ELightType::Point =>        lightcmd.insert((ActionTransformNode::init(scene), ActionLight::as_point_light())),
            ELightType::Hemispheric =>  lightcmd.insert((ActionTransformNode::init(scene), ActionLight::as_hemi_light())),
        };
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

pub type LightBaseBundle = (
    LightParam,
    LightLinkedShadowID,
);

pub type DirectLightBundle = (
    LightBaseBundle,
    DirectLight,
    LayerMask,
    ViewerDistanceCompute,
    LightDirection,
);

pub type PointLightBundle = (
    LightBaseBundle,
    PointLight,
    LayerMask,
    ViewerDistanceCompute,
);

pub type SpotLightBundle = (
    LightBaseBundle,
    SpotLight,
    LayerMask,
    LightDirection,
    SpotLightAngle,
    ViewerDistanceCompute,
);

pub type HemiLightBundle = (
    LightBaseBundle,
    HemisphericLight,
    LayerMask,
    HemiGrounds,
    ViewerDistanceCompute,
);

pub struct ActionLight;
impl ActionLight {
    pub(crate) fn as_light() -> LightBaseBundle {
        // log::warn!("CreateLight {:?}", commands.id());
        (
            LightParam::default(),
            LightLinkedShadowID(None),
        )
    }
    pub(crate) fn as_direct_light() -> DirectLightBundle {
        (
            Self::as_light(),
            DirectLight,
            LayerMask::default(),
            ViewerDistanceCompute::Direction,
            LightDirection::default(),
        )
    }
    pub(crate) fn as_spot_light() -> SpotLightBundle {
        (
            Self::as_light(),
        // Self::as_shadow_light(commands);
            SpotLight,
            LayerMask::default(),
            LightDirection::default(),
            SpotLightAngle{ in_value: 0.2, out_value: 0.3 },
            ViewerDistanceCompute::Base,
        )
    }
    pub(crate) fn as_point_light() -> PointLightBundle {
        (
            Self::as_light(),
        // Self::as_shadow_light(commands);
            PointLight,
            LayerMask::default(),
            ViewerDistanceCompute::Base,
        )
    }
    pub(crate) fn as_hemi_light() -> HemiLightBundle {
        (
            Self::as_light(),
        // Self::as_shadow_light(commands);
            HemisphericLight,
            LayerMask::default(),
            HemiGrounds::default(),
            ViewerDistanceCompute::Base,
        )
    }
}

