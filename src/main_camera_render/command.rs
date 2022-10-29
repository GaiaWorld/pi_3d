use pi_ecs::prelude::*;
use pi_ecs_macros::setup;
use pi_render::{graph::graph::RenderGraph};

use crate::{renderers::render_object::RenderObjectID, object::{ObjectID, GameObject}, cameras::camera::CameraRenderData, postprocess::Postprocess, resources::RenderDynUniformBuffer};

use super::MainCameraRenderer;


#[derive(Debug)]
pub enum MainCameraRenderCommand {
    Active(ObjectID, RenderObjectID),
}

#[derive(Debug, Default)]
pub struct SingleMainCameraRenderCommandList {
    pub list: Vec<MainCameraRenderCommand>,
}

pub struct SysMainCameraRenderCommand;
#[setup]
impl SysMainCameraRenderCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleMainCameraRenderCommandList>,
        mut cameras: Query<GameObject, (Write<CameraRenderData>, Write<RenderObjectID>)>,
        mut renderers: Query<GameObject, (Write<MainCameraRenderer>, Write<Postprocess>)>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut render_graphic: ResMut<RenderGraph>,
    ) {
        let render_graphic = &mut render_graphic;

        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                MainCameraRenderCommand::Active(entity, render_id) => {
                    match cameras.get_mut(entity) {
                        Some(mut item) => {
                            item.0.insert_no_notify(CameraRenderData::new(&mut dynbuffer));
                            item.1.insert_no_notify(render_id);
                        },
                        None => todo!(),
                    }
                    match renderers.get_mut(render_id.0) {
                        Some(mut item) => {
                            let main_renderer = MainCameraRenderer::new(render_id.0, render_graphic);
                            render_graphic.add_depend("Clear", "MainCameraOpaque");
                            render_graphic.set_finish("MainCameraOpaque", true);
                            item.0.insert_no_notify(main_renderer);
                            item.1.insert_no_notify(Postprocess::default());
                        },
                        None => todo!(),
                    }
                },
            }
        });
    }
}
