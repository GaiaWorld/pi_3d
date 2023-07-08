use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::{BindEffect, BindEffectValueDirty};
use pi_scene_math::Vector3;

use crate::{command::*, animation::*};

pub fn sys_node_material_uniform_update(
    mut materials: Query<(&MaterialAnimeSlots, &BindEffect, &mut BindEffectValueDirty)>,
    items: (
        (Query<(Entity, &MainTexUScale),      Changed<MainTexUScale>>,
            Query<(Entity, &MainTexVScale),      Changed<MainTexVScale>>,
            Query<(Entity, &MainTexUOffset),     Changed<MainTexUOffset>>,
            Query<(Entity, &MainTexVOffset),     Changed<MainTexVOffset>>,
        ),
        (
            Query<(Entity, &MaskTexUScale),      Changed<MaskTexUScale>>,
            Query<(Entity, &MaskTexVScale),      Changed<MaskTexVScale>>,
            Query<(Entity, &MaskTexUOffset),     Changed<MaskTexUOffset>>,
            Query<(Entity, &MaskTexVOffset),     Changed<MaskTexVOffset>>,
        ),
        (
            Query<(Entity, &OpacityTexUScale),   Changed<OpacityTexUScale>>,
            Query<(Entity, &OpacityTexVScale),   Changed<OpacityTexVScale>>,
            Query<(Entity, &OpacityTexUOffset),  Changed<OpacityTexUOffset>>,
            Query<(Entity, &OpacityTexVOffset),  Changed<OpacityTexVOffset>>
        ),
        Query<(Entity, &Alpha),              Changed<Alpha>>,
        Query<(Entity, &Cutoff),             Changed<Cutoff>>,
        Query<(Entity, &MainColor),          Changed<MainColor>>,
        Query<(Entity, &LightDiffuse),       Changed<LightDiffuse>>,
        Query<(Entity, &MaskCutoff),         Changed<MaskCutoff>>,
    ),
) {
    items.0.0.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.0.1.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.0.2.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.0.3.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.1.0.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.1.1.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.1.2.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.1.3.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    
    items.2.0.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.2.1.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.2.2.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.2.3.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.3.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.4.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.5.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.6.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
    items.7.iter().for_each(|(entity, item)| {
        if let Ok((slots, bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, bind);
            *flag = BindEffectValueDirty(true);
        }
    });
}

pub fn sys_act_maintex_tilloff(
    mut cmds: ResMut<ActionListMainTexTilloff>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsMainTexTilloff(idmat, uscale, vscale, uoffset, voffset)| {
        commands.entity(idmat)
            .insert(MainTexUScale(uscale)).insert(RecordMainTexUScale(MainTexUScale(uscale)))
            .insert(MainTexVScale(vscale)).insert(RecordMainTexVScale(MainTexVScale(vscale)))
            .insert(MainTexUOffset(uoffset)).insert(RecordMainTexUOffset(MainTexUOffset(uoffset)))
            .insert(MainTexVOffset(voffset)).insert(RecordMainTexVOffset(MainTexVOffset(voffset)))
            ;
    });
}

pub fn sys_act_masktex_tilloff(
    mut cmds: ResMut<ActionListMaskTexTilloff>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsMaskTexTilloff(idmat, uscale, vscale, uoffset, voffset)| {
        commands.entity(idmat)
            .insert(MaskTexUScale(uscale)).insert(RecordMaskTexUScale(MaskTexUScale(uscale)))
            .insert(MaskTexVScale(vscale)).insert(RecordMaskTexVScale(MaskTexVScale(vscale)))
            .insert(MaskTexUOffset(uoffset)).insert(RecordMaskTexUOffset(MaskTexUOffset(uoffset)))
            .insert(MaskTexVOffset(voffset)).insert(RecordMaskTexVOffset(MaskTexVOffset(voffset)))
            ;
    });
}

pub fn sys_act_opacitytex_tilloff(
    mut cmds: ResMut<ActionListOpacityTexTilloff>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsOpacityTexTilloff(idmat, uscale, vscale, uoffset, voffset)| {
        commands.entity(idmat)
            .insert(OpacityTexUScale(uscale)).insert(RecordOpacityTexUScale(OpacityTexUScale(uscale)))
            .insert(OpacityTexVScale(vscale)).insert(RecordOpacityTexVScale(OpacityTexVScale(vscale)))
            .insert(OpacityTexUOffset(uoffset)).insert(RecordOpacityTexUOffset(OpacityTexUOffset(uoffset)))
            .insert(OpacityTexVOffset(voffset)).insert(RecordOpacityTexVOffset(OpacityTexVOffset(voffset)))
            ;
    });
}

pub fn sys_act_maincolor(
    mut cmds: ResMut<ActionListMainColor>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsMainColor(idmat, r, g, b)| {
        commands.entity(idmat)
            .insert(MainColor(Vector3::new(r, g, b))).insert(RecordMainColor(MainColor(Vector3::new(r, g, b))))
            ;
    });
}

pub fn sys_act_lightdiffuse(
    mut cmds: ResMut<ActionListLightDiffuse>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsLightDiffuse(idmat, r, g, b)| {
        commands.entity(idmat)
            .insert(LightDiffuse(Vector3::new(r, g, b))).insert(RecordLightDiffuse(LightDiffuse(Vector3::new(r, g, b))))
            ;
    });
}

pub fn sys_act_alpha(
    mut cmds: ResMut<ActionListAlpha>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsAlpha(idmat, val)| {
        commands.entity(idmat)
            .insert(Alpha(val)).insert(RecordAlpha(Alpha(val)))
            ;
    });
}

pub fn sys_act_alphacutoff(
    mut cmds: ResMut<ActionListAlphaCutoff>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsAlphaCutoff(idmat, val)| {
        commands.entity(idmat)
            .insert(Cutoff(val)).insert(RecordCutoff(Cutoff(val)))
            ;
    });
}

pub fn sys_act_maskcutoff(
    mut cmds: ResMut<ActionListMaskCutoff>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsMaskCutoff(idmat, val)| {
        commands.entity(idmat)
            .insert(MaskCutoff(val)).insert(RecordMaskCutoff(MaskCutoff(val)))
            ;
    });
}