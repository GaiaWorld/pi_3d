
use pi_scene_shell::prelude::*;

use crate::{
    layer_mask::prelude::*, prelude::*, shadow::prelude::LightLinkedShadowID, transforms::command_sys::*, viewer::prelude::ViewerDistanceCompute
};

use super::{
    base::*,
    // shadow_generator::*,
    command::*, spot::SpotLightAngle, hemisphere::HemiGrounds,
};

pub fn sys_create_light(
    mut cmds: ResMut<ActionListLightCreate>,
    // mut commands: Commands,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
    mut scenes: Query<(&mut SceneDirectLightsQueue, &mut SceneOtherLightsQueue, &mut SceneLightingInfosDirty)>,
    mut alterdirect: Alter<(), (), (SceneItemIndex, TransformNodeBundle, BundleDirectLight), ()>,
    mut alterpoint: Alter<(), (), (SceneItemIndex, TransformNodeBundle, BundlePointLight), ()>,
    mut alterspot: Alter<(), (), (SceneItemIndex, TransformNodeBundle, BundleSpotLight), ()>,
    mut alterhemi: Alter<(), (), (SceneItemIndex, TransformNodeBundle, BundleHemiLight), ()>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_create_light"));
    cmds.drain().for_each(|OpsLightCreate(scene, entity, ltype)| {
        let itemidx = if let Ok((mut queuedirect, mut queuepoint, mut dirty)) = scenes.get_mut(scene) {
            let itemidx = match ltype {
                ELightType::Direct => {
                    *dirty = SceneLightingInfosDirty;
                    queuedirect.0.add(entity)
                },
                ELightType::Spot => {
                    *dirty = SceneLightingInfosDirty;
                    queuepoint.spot.add(entity)
                },
                ELightType::Point => {
                    *dirty = SceneLightingInfosDirty;
                    queuepoint.point.add(entity)
                },
                ELightType::Hemispheric => {
                    *dirty = SceneLightingInfosDirty;
                    queuepoint.hemi.add(entity)
                }
            };
            itemidx
        } else {
            disposereadylist.push(OpsDisposeReadyForRef::ops(entity));
            return;
        };

        match ltype {
            ELightType::Direct =>       {
                let bundle = (itemidx, ActionTransformNode::init(scene), ActionLight::as_direct_light());
                // lightcmd.insert(bundle);
                alterdirect.alter(entity, bundle);
            },
            ELightType::Spot =>         {
                let bundle = (itemidx, ActionTransformNode::init(scene), ActionLight::as_spot_light());
                // lightcmd.insert(bundle);
                alterspot.alter(entity, bundle);
            },
            ELightType::Point =>        {
                let bundle = (itemidx, ActionTransformNode::init(scene), ActionLight::as_point_light());
                // lightcmd.insert(bundle);
                alterpoint.alter(entity, bundle);
            },
            ELightType::Hemispheric =>  {
                let bundle = (itemidx, ActionTransformNode::init(scene), ActionLight::as_hemi_light());
                // lightcmd.insert(bundle);
                alterhemi.alter(entity, bundle);
            },
        };
    });
}

pub fn sys_act_light_param(
    mut cmds: ResMut<ActionListLightParam>,
    mut directlights: Query<&mut LightDirection>,
    mut spot_lights: Query<&mut SpotLightAngle>,
    mut param_lights: Query<&mut LightParam>,
    mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_act_light_param"));
    cmds.drain().for_each(|OpsLightParam(entity, cmd)| {
        match cmd {
            ELightModify::Color(r, g, b) => if let Ok(mut lightcolor) = param_lights.get_mut(entity) {
                lightcolor.color = Vector3::new(r, g, b);
            },
            ELightModify::LightingType(val) => if let Ok(mut item) = param_lights.get_mut(entity) {
                item.mode = val;
            },
            ELightModify::Directional(x, y, z) => if let Ok(mut item) = directlights.get_mut(entity) {
                *item = LightDirection(Vector3::new(x, y, z));
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

pub type BundleDirectLight = (
    LightParam,
    LightLinkedShadowID,
    DirectLight,
    LayerMask,
    ViewerDistanceCompute,
    LightDirection,
);

pub type BundlePointLight = (
    LightParam,
    LightLinkedShadowID,
    PointLight,
    LayerMask,
    ViewerDistanceCompute,
);

pub type BundleSpotLight = (
    LightParam,
    LightLinkedShadowID,
    SpotLight,
    LayerMask,
    LightDirection,
    SpotLightAngle,
    ViewerDistanceCompute,
);

pub type BundleHemiLight = (
    LightParam,
    LightLinkedShadowID,
    HemisphericLight,
    LayerMask,
    HemiGrounds,
    ViewerDistanceCompute,
);

pub struct ActionLight;
impl ActionLight {
    pub(crate) fn as_direct_light() -> BundleDirectLight {
        (
            LightParam::default(),
            LightLinkedShadowID(None),
            DirectLight,
            LayerMask::default(),
            ViewerDistanceCompute::new(EViewerDistanceCompute::Direction),
            LightDirection::default(),
        )
    }
    pub(crate) fn as_spot_light() -> BundleSpotLight {
        (
            LightParam::default(),
            LightLinkedShadowID(None),
            SpotLight,
            LayerMask::default(),
            LightDirection::default(),
            SpotLightAngle{ in_value: 0.2, out_value: 0.3 },
            ViewerDistanceCompute::default(),
        )
    }
    pub(crate) fn as_point_light() -> BundlePointLight {
        (
            LightParam::default(),
            LightLinkedShadowID(None),
            PointLight,
            LayerMask::default(),
            ViewerDistanceCompute::default(),
        )
    }
    pub(crate) fn as_hemi_light() -> BundleHemiLight {
        (
            LightParam::default(),
            LightLinkedShadowID(None),
            HemisphericLight,
            LayerMask::default(),
            HemiGrounds::default(),
            ViewerDistanceCompute::default(),
        )
    }
}

