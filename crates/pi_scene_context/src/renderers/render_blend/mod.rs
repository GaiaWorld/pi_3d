

use pi_scene_shell::prelude::*;
use super::*;

#[derive(Clone, Copy, )]
pub struct ModelBlend {
    pub enable: bool,
    pub src_color: BlendFactor,
    pub dst_color: BlendFactor,
    pub src_alpha: BlendFactor,
    pub dst_alpha: BlendFactor,
    pub opt_color: BlendOperation,
    pub opt_alpha: BlendOperation,
}
impl Default for ModelBlend {
    fn default() -> Self {
        Self {
            enable: false,
            src_color: BlendFactor::SrcAlpha,
            dst_color: BlendFactor::OneMinusSrcAlpha,
            src_alpha: BlendFactor::One,
            dst_alpha: BlendFactor::OneMinusSrcAlpha,
            opt_color: BlendOperation::Add,
            opt_alpha: BlendOperation::Add,
        }
    }
}
impl ModelBlend {
    pub fn combine(&mut self) {
        self.enable = true;
    }
    pub fn one_one() -> Self {
        Self {
            enable: true,
            src_color: BlendFactor::One,
            dst_color: BlendFactor::One,
            src_alpha: BlendFactor::One,
            dst_alpha: BlendFactor::One,
            opt_color: BlendOperation::Add,
            opt_alpha: BlendOperation::Add,
        }
    }
}

#[derive(Clone, Copy)]
pub enum OpsRenderBlend {
    Disable(Entity),
    Blend(Entity, PassTag, ModelBlend),
}
impl OpsRenderBlend {
    pub fn ops(mesh: Entity, pass: PassTag, mode: ModelBlend) -> Self {
        Self::Blend(mesh, pass, mode)
    }
}

pub type ActionListBlend = ActionList<OpsRenderBlend>;
pub fn sys_act_model_blend(
    mut cmds: ResMut<ActionListBlend>,
    models: Query<&PassIDs>,
    mut items: Query<&mut ModelBlend>,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            OpsRenderBlend::Disable(_) => todo!(),
            OpsRenderBlend::Blend(entity, tag, value) => {
                if let Ok(passids) = models.get(entity) {
                    let passid = passids.0[tag.index()];
        
                    if let Ok(mut item) = items.get_mut(passid) {
                        *item = value;
                    }
                }
            },
        }
    });
}