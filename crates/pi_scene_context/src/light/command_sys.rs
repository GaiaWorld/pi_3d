
use pi_render::components;
use pi_scene_shell::{add_component, prelude::{pi_world::editor::EntityEditor, *}};

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
    mut editor: EntityEditor,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
) {
    cmds.drain().drain(..).for_each(|OpsLightCreate(scene, entity, ltype)| {
        if !editor.contains_entity(entity) {
            disposereadylist.push(OpsDisposeReadyForRef::ops(entity));
            return;
        };

        ActionTransformNode::init(entity, &mut editor, scene);
        match ltype {
            ELightType::Direct => ActionLight::as_direct_light(entity, &mut editor, ),
            ELightType::Spot => ActionLight::as_spot_light(entity, &mut editor,),
            ELightType::Point => ActionLight::as_point_light(entity, &mut editor,),
            ELightType::Hemispheric => ActionLight::as_hemi_light(entity, &mut editor,),
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
                    log::warn!("sys_act_light_param 1");
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
        editor: &mut EntityEditor,
    ) {
        log::warn!("CreateLight {:?}", entity);
        let components = [editor.init_component::<LightParam>(), editor.init_component::<LightLinkedShadowID>()];
        editor.add_components(entity, &components).unwrap();
        *editor.get_component_unchecked_mut_by_id(entity, components[0]) = LightParam::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[1]) = LightLinkedShadowID(None);

    }
    pub(crate) fn as_direct_light(
        entity: Entity,  
        editor: &mut EntityEditor,
    ) {
        Self::as_light(entity, editor);

        let components = [editor.init_component::<DirectLight>(), editor.init_component::<LayerMask>(), editor.init_component::<ViewerDistanceCompute>(), editor.init_component::<LightDirection>()];
        editor.add_components(entity, &components).unwrap();
        log::warn!("as_direct_light 1 {:?}", entity);
        *editor.get_component_unchecked_mut_by_id(entity, components[0]) = DirectLight;
        *editor.get_component_unchecked_mut_by_id(entity, components[1]) = LayerMask::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[2]) = ViewerDistanceCompute::Direction;
        *editor.get_component_unchecked_mut_by_id(entity, components[3]) = LightDirection::default();
    }
    pub(crate) fn as_spot_light(
        entity: Entity,  
        editor: &mut EntityEditor,
    ) {
        Self::as_light(entity, editor);
        // Self::as_shadow_light(commands);
        let components = [editor.init_component::<SpotLight>(), editor.init_component::<LayerMask>(), editor.init_component::<LightDirection>(), editor.init_component::<SpotLightAngle>(), editor.init_component::<ViewerDistanceCompute>()];
        editor.add_components(entity, &components);
        log::warn!("as_spot_light 1 {:?}", entity);
        *editor.get_component_unchecked_mut_by_id(entity, components[0]) = SpotLight;
        *editor.get_component_unchecked_mut_by_id(entity, components[1]) = LayerMask::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[2]) = LightDirection::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[3]) = SpotLightAngle{ in_value: 0.2, out_value: 0.3 };
        *editor.get_component_unchecked_mut_by_id(entity, components[4]) = ViewerDistanceCompute::Base;
    }
    pub(crate) fn as_point_light(
        entity: Entity,  
        editor: &mut EntityEditor,
    ) {
        Self::as_light(entity, editor);
        // Self::as_shadow_light(commands);
        let components = [editor.init_component::<PointLight>(), editor.init_component::<LayerMask>(), editor.init_component::<ViewerDistanceCompute>()];
        editor.add_components(entity, &components);

        *editor.get_component_unchecked_mut_by_id(entity, components[0]) = PointLight;
        *editor.get_component_unchecked_mut_by_id(entity, components[1]) = LayerMask::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[2]) = ViewerDistanceCompute::Base;

        // editor.alter(entity, (PointLight, LayerMask::default(), ViewerDistanceCompute::Base));
    }
    pub(crate) fn as_hemi_light(
        entity: Entity,  
        editor: &mut EntityEditor,
    ) {
        Self::as_light(entity, editor);
        let components = [editor.init_component::<HemisphericLight>(), editor.init_component::<LayerMask>(), editor.init_component::<HemiGrounds>(), editor.init_component::<ViewerDistanceCompute>()];
        editor.add_components(entity, &components);

        *editor.get_component_unchecked_mut_by_id(entity, components[0]) = PointLight;
        *editor.get_component_unchecked_mut_by_id(entity, components[1]) = LayerMask::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[2]) = HemiGrounds::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[3]) = ViewerDistanceCompute::Base;

        // editor.alter(entity, (HemisphericLight, LayerMask::default(), HemiGrounds::default(), ViewerDistanceCompute::Base));
    }
}

