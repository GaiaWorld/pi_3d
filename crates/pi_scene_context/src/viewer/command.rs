use std::{mem::replace};

use pi_engine_shell::prelude::*;
use pi_scene_math::Number;

use crate::{
    renderers::{
        render_object::RendererID, graphic::RendererGraphicDesc, ViewerRenderersInfo,
        renderer::{Renderer, RenderSize, RenderColorFormat, RenderColorClear, RenderDepthClear, RenderDepthFormat}
    },
    flags::SceneID,
    postprocess::Postprocess,
    pass::PassTagOrders
};

use super::{
    ViewerID
};



#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub x: Number,
    pub y: Number,
    pub w: Number,
    pub h: Number,
}
impl Default for Viewport {
    fn default() -> Self {
        Self { x: 0., y: 0., w: 1., h: 1. }
    }
}


pub enum ERendererCommand {
    Active(RendererID, RendererGraphicDesc),
    RenderSize(RenderSize),
    RenderColorFormat(RenderColorFormat),
    RenderColorClear(RenderColorClear),
    RenderDepthFormat(RenderDepthFormat),
    RenderDepthClear(RenderDepthClear),
}

pub struct ActionViewer;
impl ActionViewer {
    pub fn create_viewer_renderer(
        commands: &mut EntityCommands,
        render_graphic: &mut PiRenderGraph,
        id_viewer: Entity,
        id_renderer: RendererID,
        graphic_desc: RendererGraphicDesc,
    ) {
        let entity = id_renderer.0;
        let renderer = Renderer::new(&graphic_desc.curr, id_renderer.0.clone(), render_graphic);
        if let Some(key_pre) = &graphic_desc.pre {
            render_graphic.add_depend(key_pre.to_string(), graphic_desc.curr.to_string());
        }
        if let Some(key_next) = &graphic_desc.next {
            render_graphic.add_depend(graphic_desc.curr.to_string(), key_next.to_string());
        } else {
            render_graphic.set_finish(graphic_desc.curr.to_string(), true);
        }
        commands
            .insert(graphic_desc.passorders.clone())
            .insert(renderer)
            .insert(ViewerID(id_viewer))
            .insert(Postprocess::default());
    }
    
    pub fn modify_viewer_renderer(
        commands: &mut EntityCommands,
        cmd: ERendererCommand,
    ) {
        match cmd {
            ERendererCommand::Active(_, _) => todo!(),
            ERendererCommand::RenderSize(val) => {
                commands.insert(val);
            },
            ERendererCommand::RenderColorFormat(val) => {
                commands.insert(val)
            },
            ERendererCommand::RenderColorClear(val) => {
                commands.insert(val)
            },
            ERendererCommand::RenderDepthFormat(val) => {
                commands.insert(val)
            },
            ERendererCommand::RenderDepthClear(val) => {
                commands.insert(val)
            },
        }
    }
}

#[derive(Default)]
pub struct SingleRendererCommandList {
    pub list: Vec<ERendererCommand>,
}


// #[derive(Debug, Default)]
// pub struct SysViewerRendererCommandTick;
// impl TSystemStageInfo for SysViewerRendererCommandTick {
// }
// #[setup]
// impl SysViewerRendererCommandTick {

    // /// 视口被销毁则需要销毁对应渲染器
    // #[listen(entity=(GameObject, Delete))]
    // fn listen(
    //     e: Event,
    //     viewers: Query<GameObject, &RendererID>,
    //     mut delete: EntityDelete<GameObject>,
    // ) {
    //     if let Some(id_render) = viewers.get_by_entity(e.id) {
    //         delete.despawn(id_render.0);
    //     }
    // }


    // #[system]
    // pub fn cmd(
    //     mut cmds: ResMut<SingleRendererCommandList>,
    //     mut viewers: Query<GameObject, (&SceneID, &ViewerRenderersInfo)>,
    //     mut viewer_info_cmd: Commands<GameObject, ViewerRenderersInfo>,
    //     mut postprocess_cmd: Commands<GameObject, Postprocess>,
    //     mut renderer_orders_cmd: Commands<GameObject, PassTagOrders>,
    //     mut renderer_cmd: Commands<GameObject, Renderer>,
    //     mut renderer_viewport_cmd: Commands<GameObject, Viewport>,
    //     mut renderer_viewer_cmd: Commands<GameObject, ViewerID>,
        
    //     mut renderersize_cmd: Commands<GameObject, RenderSize>,
    //     mut renderercolor_cmd: Commands<GameObject, RenderColorFormat>,
    //     mut renderercolor_clear_cmd: Commands<GameObject, RenderColorClear>,
    //     mut rendererdepth_cmd: Commands<GameObject, RenderDepthFormat>,
    //     mut rendererdepth_clear_cmd: Commands<GameObject, RenderDepthClear>,

    //     mut render_graphic: ResMut<RenderGraph>,
    //     mut delete: EntityDelete<GameObject>,
    // ) {
    //     let render_graphic = &mut render_graphic;

    //     let mut list = replace(&mut cmds.list, vec![]);

    //     list.drain(..).for_each(|cmd| {
    //         log::debug!("SysRendererCommandTick:");
    //         match cmd {
    //             ERendererCommand::Active(id_viewer, id_renderer, graphic_desc) => {
    //                 if let Some((id_scene, viewer_renderers)) = viewers.get(id_viewer.clone()) {
                        
    //                     let renderer = Renderer::new(&graphic_desc.curr, id_renderer.0.clone(), render_graphic);
    //                     if let Some(key_pre) = &graphic_desc.pre {
    //                         render_graphic.add_depend(key_pre.to_string(), graphic_desc.curr.to_string());
    //                     }
    //                     if let Some(key_next) = &graphic_desc.next {
    //                         render_graphic.add_depend(graphic_desc.curr.to_string(), key_next.to_string());
    //                     } else {
    //                         render_graphic.set_finish(graphic_desc.curr.to_string(), true);
    //                     }

    //                     // Renderer Modify
    //                     renderer_orders_cmd.insert(id_renderer.0.clone(), graphic_desc.passorders.clone());
    //                     renderer_cmd.insert(id_renderer.0.clone(), renderer);
    //                     renderer_viewer_cmd.insert(id_renderer.0.clone(), ViewerID(id_viewer));
    //                     postprocess_cmd.insert(id_renderer.0.clone(), Postprocess::default());
    //                 }
    //             },
    //             ERendererCommand::RenderSize(id_renderer, val) => {
    //                 renderersize_cmd.insert(id_renderer, val)
    //             },
    //             ERendererCommand::RenderColorFormat(id_renderer, val) => {
    //                 renderercolor_cmd.insert(id_renderer, val)
    //             },
    //             ERendererCommand::RenderColorClear(id_renderer, val) => {
    //                 renderercolor_clear_cmd.insert(id_renderer, val)
    //             },
    //             ERendererCommand::RenderDepthFormat(id_renderer, val) => {
    //                 rendererdepth_cmd.insert(id_renderer, val)
    //             },
    //             ERendererCommand::RenderDepthClear(id_renderer, val) => {
    //                 rendererdepth_clear_cmd.insert(id_renderer, val)
    //             },
    //         }
    //     });
    // }
// }