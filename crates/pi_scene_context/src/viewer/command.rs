use std::{mem::replace};

use pi_ecs::prelude::{ResMut, Commands, Event, Query, EntityDelete};
use pi_ecs_macros::{setup, listen};
use pi_engine_shell::{object::{ObjectID, GameObject}, run_stage::TSystemStageInfo};
use pi_render::{
    graph::graph::RenderGraph,
};
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
    Active(ObjectID, RendererID, RendererGraphicDesc),
    RenderSize(ObjectID, RenderSize),
    RenderColorFormat(ObjectID, RenderColorFormat),
    RenderColorClear(ObjectID, RenderColorClear),
    RenderDepthFormat(ObjectID, RenderDepthFormat),
    RenderDepthClear(ObjectID, RenderDepthClear),
}

#[derive(Default)]
pub struct SingleRendererCommandList {
    pub list: Vec<ERendererCommand>,
}

#[derive(Debug, Default)]
pub struct SysViewerRendererCommandTick;
impl TSystemStageInfo for SysViewerRendererCommandTick {
}
#[setup]
impl SysViewerRendererCommandTick {
    /// 视口被销毁则需要销毁对应渲染器
    #[listen(entity=(GameObject, Delete))]
    fn listen(
        e: Event,
        viewers: Query<GameObject, &RendererID>,
        mut delete: EntityDelete<GameObject>,
    ) {
        if let Some(id_render) = viewers.get_by_entity(e.id) {
            delete.despawn(id_render.0);
        }
    }
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleRendererCommandList>,
        mut viewers: Query<GameObject, (&SceneID, &ViewerRenderersInfo)>,
        mut viewer_info_cmd: Commands<GameObject, ViewerRenderersInfo>,
        mut postprocess_cmd: Commands<GameObject, Postprocess>,
        mut renderer_orders_cmd: Commands<GameObject, PassTagOrders>,
        mut renderer_cmd: Commands<GameObject, Renderer>,
        mut renderer_viewport_cmd: Commands<GameObject, Viewport>,
        mut renderer_viewer_cmd: Commands<GameObject, ViewerID>,
        
        mut renderersize_cmd: Commands<GameObject, RenderSize>,
        mut renderercolor_cmd: Commands<GameObject, RenderColorFormat>,
        mut renderercolor_clear_cmd: Commands<GameObject, RenderColorClear>,
        mut rendererdepth_cmd: Commands<GameObject, RenderDepthFormat>,
        mut rendererdepth_clear_cmd: Commands<GameObject, RenderDepthClear>,

        mut render_graphic: ResMut<RenderGraph>,
        mut delete: EntityDelete<GameObject>,
    ) {
        let render_graphic = &mut render_graphic;

        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            log::info!("SysRendererCommandTick:");
            match cmd {
                ERendererCommand::Active(id_viewer, id_renderer, graphic_desc) => {
                    if let Some((id_scene, viewer_renderers)) = viewers.get(id_viewer.clone()) {
                        let mut viewer_renderers = viewer_renderers.clone();
                        if let Some((render, id_render)) = viewer_renderers.map.get(&graphic_desc.curr) {
                            delete.despawn(id_render.0);
                            render_graphic.remove_node(render.curr.to_string());
                        }
                        viewer_renderers.map.insert(graphic_desc.curr.clone(), (graphic_desc.clone(), id_renderer.clone()));
                        viewer_info_cmd.insert(id_viewer, viewer_renderers);

                        
                        let renderer = Renderer::new(&graphic_desc.curr, id_renderer.0.clone(), render_graphic);
                        if let Some(key_pre) = &graphic_desc.pre {
                            render_graphic.add_depend(key_pre.to_string(), graphic_desc.curr.to_string());
                        }
                        if let Some(key_next) = &graphic_desc.next {
                            render_graphic.add_depend(graphic_desc.curr.to_string(), key_next.to_string());
                        } else {
                            render_graphic.set_finish(graphic_desc.curr.to_string(), true);
                        }

                        // Renderer Modify
                        renderer_orders_cmd.insert(id_renderer.0.clone(), graphic_desc.passorders.clone());
                        renderer_cmd.insert(id_renderer.0.clone(), renderer);
                        renderer_viewer_cmd.insert(id_renderer.0.clone(), ViewerID(id_viewer));
                        postprocess_cmd.insert(id_renderer.0.clone(), Postprocess::default());
                    }
                },
                ERendererCommand::RenderSize(id_renderer, val) => {
                    log::warn!(">>> RenderSize");
                    renderersize_cmd.insert(id_renderer, val)
                },
                ERendererCommand::RenderColorFormat(id_renderer, val) => {
                    log::warn!(">>> RenderColorFormat");
                    renderercolor_cmd.insert(id_renderer, val)
                },
                ERendererCommand::RenderColorClear(id_renderer, val) => {
                    log::warn!(">>> RenderColorClear");
                    renderercolor_clear_cmd.insert(id_renderer, val)
                },
                ERendererCommand::RenderDepthFormat(id_renderer, val) => {
                    log::warn!(">>> RenderDepthFormat");
                    rendererdepth_cmd.insert(id_renderer, val)
                },
                ERendererCommand::RenderDepthClear(id_renderer, val) => {
                    log::warn!(">>> RenderDepthClear");
                    rendererdepth_clear_cmd.insert(id_renderer, val)
                },
            }
        });
    }
}