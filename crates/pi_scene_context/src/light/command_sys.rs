
use pi_scene_shell::prelude::*;

use crate::{
    flags::{CullingFlag, Enable, GlobalEnable, RecordEnable}, layer_mask::prelude::*, shadow::prelude::LightLinkedShadowID, transforms::command_sys::*, viewer::prelude::ViewerDistanceCompute
};

use super::{
    base::*, command::*, hemisphere::HemiGrounds, spot::SpotLightAngle, TransformNodeDirty, LocalPosition,
    LocalScaling,
    LocalRotationQuaternion,
    LocalEulerAngles,
    RecordLocalPosition,
    RecordLocalScaling,
    RecordLocalRotationQuaternion,
    RecordLocalEulerAngles,
    LocalRotation,
    LocalMatrix,
    GlobalMatrix,
    AbsoluteTransform,
};

pub fn sys_create_light(
    mut cmds: ResMut<ActionListLightCreate>,
    // mut commands: Commands,
    mut alter1:  Alter<(), (), (DisposeReady, DisposeCan), ()>,
    mut alter2:  Alter<(),(),(SceneID, ), ()>,
    mut alter3:  Alter<(), (), (Down, Up, Layer, Enable, RecordEnable, GlobalEnable), ()>,
    mut alter4:  Alter<
    (),
    (),
    (
        TransformNodeDirty,
        LocalPosition,
        LocalScaling,
        LocalRotationQuaternion,
        LocalEulerAngles,
        RecordLocalPosition,
        RecordLocalScaling,
        RecordLocalRotationQuaternion,
        RecordLocalEulerAngles,
        LocalRotation,
        LocalMatrix,
        GlobalMatrix,
        AbsoluteTransform,
        FlagAnimationStartResetComp,
        CullingFlag
    ), 
    ()>,
    mut alter5: Alter<(), (), (LightParam, LightLinkedShadowID), ()>,
    mut alter6: Alter<(), (), (DirectLight, LayerMask, ViewerDistanceCompute, LightDirection), ()>,
    mut alter7: Alter<(), (), (LightParam, LightLinkedShadowID), ()>, 
    mut alter8: Alter<(), (), (SpotLight, LayerMask, LightDirection, SpotLightAngle, ViewerDistanceCompute), ()>,
    mut alter9:  Alter<(), (), (LightParam, LightLinkedShadowID), ()>, 
    mut alter10: Alter<(), (), (PointLight, LayerMask, ViewerDistanceCompute), ()>,
    mut alter11: Alter<(), (), (LightParam, LightLinkedShadowID), ()>, 
    mut alter12: Alter<(), (), (HemisphericLight, LayerMask, HemiGrounds, ViewerDistanceCompute), ()>,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
) {
    cmds.drain().drain(..).for_each(|OpsLightCreate(scene, entity, ltype)| {
        if !alter1.get(entity).is_ok() {
            disposereadylist.push(OpsDisposeReadyForRef::ops(entity));
            return;
        };

        ActionTransformNode::init(entity, &mut alter1, &mut alter2, &mut alter3, &mut alter4, scene);
        match ltype {
            ELightType::Direct => ActionLight::as_direct_light(entity, &mut alter5, &mut alter6),
            ELightType::Spot => ActionLight::as_spot_light(entity, &mut alter7, &mut alter8),
            ELightType::Point => ActionLight::as_point_light(entity, &mut alter9, &mut alter10),
            ELightType::Hemispheric => ActionLight::as_hemi_light(entity, &mut alter11, &mut alter12),
        }

    });
}

pub fn sys_act_light_param(
    mut cmds: ResMut<ActionListLightColor>,
    // mut lights: Query<&mut LightColor>,
    mut directcmds: ResMut<ActionListLightParam>,
    mut directlights: Query<&mut LightDirection>,
    // mut directlightsmode: Query<&mut LightingMode>,
    mut spot_cmds: ResMut<ActionListSpotLightAngle>,
    mut spot_lights: Query<&mut SpotLightAngle>,
    mut strength_cmds: ResMut<ActionListLightStrength>,
    // mut strength_lights: Query<&mut LightStrength>,
    mut radius_cmds: ResMut<ActionListLightRadius>,
    // mut radius_lights: Query<&mut LightRadius>,
    mut param_lights: Query<&mut LightParam>,
) {
    cmds.drain().drain(..).for_each(|OpsLightColor(entity, color)| {
        if let Ok(mut lightcolor) = param_lights.get_mut(entity) {
            lightcolor.color = color;
        }
    });
    directcmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            ELightModifyCommand::LightingType(entity, val) => {
                if let Ok(mut item) = param_lights.get_mut(entity) {
                    item.mode = val;
                }
            },
            ELightModifyCommand::Directional(entity, val) => {
                if let Ok(mut item) = directlights.get_mut(entity) {
                    *item = LightDirection(val);
                }
            },
        }
    });
    spot_cmds.drain().drain(..).for_each(|OpsSpotLightAngle(entity, outangle, inangle)| {
        if let Ok(mut out_val) = spot_lights.get_mut(entity) {
            out_val.out_value = outangle;
            out_val.in_value = inangle;
        }
    });
    strength_cmds.drain().drain(..).for_each(|OpsLightStrength(entity, val)| {
        if let Ok(mut light) = param_lights.get_mut(entity) {
            light.strength = val;
        }
    });
    radius_cmds.drain().drain(..).for_each(|OpsLightRadius(entity, val)| {
        if let Ok(mut light) = param_lights.get_mut(entity) {
            light.radius = val;
        }
    });
}

pub struct ActionLight;
impl ActionLight {
    pub(crate) fn as_light(
        entity: Entity,  
        alter1: &mut Alter<(), (), (LightParam, LightLinkedShadowID), ()>,
    ) {
        // log::warn!("CreateLight {:?}", commands.id());
        alter1.alter(entity, (LightParam::default(), LightLinkedShadowID(None)));
    }
    pub(crate) fn as_direct_light(
        entity: Entity,  
        alter0: &mut Alter<(), (), (LightParam, LightLinkedShadowID), ()>,
        alter1: &mut Alter<(), (), (DirectLight, LayerMask, ViewerDistanceCompute, LightDirection), ()>,
    ) {
        Self::as_light(entity, alter0);

        alter1.alter(entity, (DirectLight, LayerMask::default(), ViewerDistanceCompute::Direction, LightDirection::default()));
    }
    pub(crate) fn as_spot_light(
        entity: Entity,  
        alter0: &mut Alter<(), (), (LightParam, LightLinkedShadowID), ()>,
        alter1: &mut Alter<(), (), (SpotLight, LayerMask, LightDirection, SpotLightAngle, ViewerDistanceCompute), ()>,
    ) {
        Self::as_light(entity, alter0);
        // Self::as_shadow_light(commands);
        alter1.alter(entity, (SpotLight, LayerMask::default(), LightDirection::default(), SpotLightAngle{ in_value: 0.2, out_value: 0.3 }, ViewerDistanceCompute::Base));
    }
    pub(crate) fn as_point_light(
        entity: Entity,  
        alter0: &mut Alter<(), (), (LightParam, LightLinkedShadowID), ()>,
        alter1: &mut Alter<(), (), (PointLight, LayerMask, ViewerDistanceCompute), ()>,
    ) {
        Self::as_light(entity, alter0);
        // Self::as_shadow_light(commands);
        alter1.alter(entity, (PointLight, LayerMask::default(), ViewerDistanceCompute::Base));
    }
    pub(crate) fn as_hemi_light(
        entity: Entity,  
        alter0: &mut Alter<(), (), (LightParam, LightLinkedShadowID), ()>,
        alter1: &mut Alter<(), (), (HemisphericLight, LayerMask, HemiGrounds, ViewerDistanceCompute), ()>,
    ) {
        Self::as_light(entity, alter0);
        // Self::as_shadow_light(commands);
        alter1.alter(entity, (HemisphericLight, LayerMask::default(), HemiGrounds::default(), ViewerDistanceCompute::Base));
    }
}

