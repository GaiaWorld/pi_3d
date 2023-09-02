use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::{BindEffect, BindEffectValueDirty};
use pi_scene_math::Vector3;

use crate::{command::*, animation::*};

pub fn sys_node_material_uniform_update(
    mut materials: Query<(&MaterialAnimeSlots, &mut BindEffect, &mut BindEffectValueDirty)>,
    items: (
        (
            Query<(Entity, &MainTexUScale),      Changed<MainTexUScale>>,
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
    mut performance: ResMut<Performance>,
) {
    let time0 = pi_time::Instant::now();
    items.0.0.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.0.1.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.0.2.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.0.3.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.1.0.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.1.1.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.1.2.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.1.3.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    
    items.2.0.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.2.1.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.2.2.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.2.3.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.3.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.4.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.5.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            // log::debug!("{:?}", item);
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.6.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });
    items.7.iter().for_each(|(entity, item)| {
        if let Ok((slots, mut bind, mut flag)) = materials.get_mut(entity) {
            item.apply(slots, &mut bind);
            *flag = BindEffectValueDirty;
        }
    });

    performance.uniformupdate = (pi_time::Instant::now() - time0).as_micros() as u32;
}

pub fn sys_act_maintex_tilloff(
    mut cmds: ResMut<ActionListMainTexTilloff>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsMainTexTilloff(idmat, uscale, vscale, uoffset, voffset)| {
        if let Some(mut cmd) = commands.get_entity(idmat) {
            cmd
            .insert(MainTexUScale(uscale)).insert(RecordMainTexUScale(MainTexUScale(uscale)))
            .insert(MainTexVScale(vscale)).insert(RecordMainTexVScale(MainTexVScale(vscale)))
            .insert(MainTexUOffset(uoffset)).insert(RecordMainTexUOffset(MainTexUOffset(uoffset)))
            .insert(MainTexVOffset(voffset)).insert(RecordMainTexVOffset(MainTexVOffset(voffset)))
            ;
        }
    });
}

pub fn sys_act_masktex_tilloff(
    mut cmds: ResMut<ActionListMaskTexTilloff>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsMaskTexTilloff(idmat, uscale, vscale, uoffset, voffset)| {
        if let Some(mut cmd) = commands.get_entity(idmat) {
            cmd
            .insert(MaskTexUScale(uscale)).insert(RecordMaskTexUScale(MaskTexUScale(uscale)))
            .insert(MaskTexVScale(vscale)).insert(RecordMaskTexVScale(MaskTexVScale(vscale)))
            .insert(MaskTexUOffset(uoffset)).insert(RecordMaskTexUOffset(MaskTexUOffset(uoffset)))
            .insert(MaskTexVOffset(voffset)).insert(RecordMaskTexVOffset(MaskTexVOffset(voffset)))
            ;
        }
    });
}

pub fn sys_act_opacitytex_tilloff(
    mut cmds: ResMut<ActionListOpacityTexTilloff>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsOpacityTexTilloff(idmat, uscale, vscale, uoffset, voffset)| {
        if let Some(mut cmd) = commands.get_entity(idmat) {
            cmd
            .insert(OpacityTexUScale(uscale)).insert(RecordOpacityTexUScale(OpacityTexUScale(uscale)))
            .insert(OpacityTexVScale(vscale)).insert(RecordOpacityTexVScale(OpacityTexVScale(vscale)))
            .insert(OpacityTexUOffset(uoffset)).insert(RecordOpacityTexUOffset(OpacityTexUOffset(uoffset)))
            .insert(OpacityTexVOffset(voffset)).insert(RecordOpacityTexVOffset(OpacityTexVOffset(voffset)))
            ;
        }
    });
}

pub fn sys_act_maincolor(
    mut cmds: ResMut<ActionListMainColor>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsMainColor(idmat, r, g, b)| {
        if let Some(mut cmd) = commands.get_entity(idmat) {
            cmd
            .insert(MainColor(Vector3::new(r, g, b))).insert(RecordMainColor(MainColor(Vector3::new(r, g, b))))
            ;
        }
    });
}

pub fn sys_act_lightdiffuse(
    mut cmds: ResMut<ActionListLightDiffuse>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsLightDiffuse(idmat, r, g, b)| {
        if let Some(mut cmd) = commands.get_entity(idmat) {
            cmd
            .insert(LightDiffuse(Vector3::new(r, g, b))).insert(RecordLightDiffuse(LightDiffuse(Vector3::new(r, g, b))))
            ;
        }
    });
}

pub fn sys_act_alpha(
    mut cmds: ResMut<ActionListAlpha>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsAlpha(idmat, val)| {
        if let Some(mut cmd) = commands.get_entity(idmat) {
            cmd
            .insert(Alpha(val)).insert(RecordAlpha(Alpha(val)))
            ;
        }
    });
}

pub fn sys_act_alphacutoff(
    mut cmds: ResMut<ActionListAlphaCutoff>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsAlphaCutoff(idmat, val)| {
        if let Some(mut cmd) = commands.get_entity(idmat) {
            cmd
            .insert(Cutoff(val)).insert(RecordCutoff(Cutoff(val)))
            ;
        }
    });
}

pub fn sys_act_maskcutoff(
    mut cmds: ResMut<ActionListMaskCutoff>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsMaskCutoff(idmat, val)| {
        if let Some(mut cmd) = commands.get_entity(idmat) {
            cmd
            .insert(MaskCutoff(val)).insert(RecordMaskCutoff(MaskCutoff(val)))
            ;
        }
    });
}