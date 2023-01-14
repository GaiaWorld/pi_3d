use std::{mem::replace, hash::{Hash, Hasher}};

use pi_ecs::prelude::*;
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::TSystemStageInfo;
use pi_render::{graph::graph::RenderGraph, rhi::device::RenderDevice};
use render_resource::uniform_buffer::RenderDynUniformBuffer;
use render_shader::{set_bind::ShaderSetBind, shader_set::{ShaderSetSceneAbout, ShaderSetSceneAboutBindOffset}};

use crate::{
    renderers::{render_object::RendererID, renderer::{Renderer, RendererHasher}, ModelList, ModelListAfterCulling},
    object::{ObjectID, GameObject},
    postprocess::Postprocess, flags::SceneID,
    viewer::{command::Viewport, ViewerID, ViewerViewMatrix, ViewerProjectionMatrix, ViewerTransformMatrix, ViewerGlobalPosition, ViewerDirection},
    scene::{
        command::SysSceneCommand, 
        environment::fog::SceneFog,
        scene_time::SceneTime
    },
    bindgroup::{RenderBindGroupKey, RenderBindGroupPool}
};


#[derive(Debug)]
pub enum MainCameraRenderCommand {
    Active(ObjectID, RendererID, Option<Viewport>),
}

#[derive(Debug, Default)]
pub struct SingleMainCameraRenderCommandList {
    pub list: Vec<MainCameraRenderCommand>,
}

pub struct SysMainCameraRenderCommand;
impl TSystemStageInfo for SysMainCameraRenderCommand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysSceneCommand::key()
        ]
    }
}
#[setup]
impl SysMainCameraRenderCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleMainCameraRenderCommandList>,
        cameras: Query<GameObject, &SceneID>,
        scenes: Query<GameObject, (&SceneTime, &SceneFog)>,
        mut camera_modellist_cmd: Commands<GameObject, ModelList>,
        mut camera_filter_cmd: Commands<GameObject, ModelListAfterCulling>,
        mut viewport_cmd: Commands<GameObject, Viewport>,
        mut renderid_cmd: Commands<GameObject, RendererID>,
        mut sceneid_cmd: Commands<GameObject, SceneID>,
        mut renderer_cmd: Commands<GameObject, Renderer>,
        mut postprocess_cmd: Commands<GameObject, Postprocess>,
        mut renderer_scene_cmd: Commands<GameObject, SceneID>,
        mut renderer_viewer_cmd: Commands<GameObject, ViewerID>,
        mut renderer_set_cmd: Commands<GameObject, ShaderSetSceneAbout>,
        mut renderer_bindoff_cmd: Commands<GameObject, ShaderSetSceneAboutBindOffset>,
        mut render_graphic: ResMut<RenderGraph>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut bindgrouppool: ResMut<RenderBindGroupPool>,
        device: Res<RenderDevice>,
        mut renderer_hasher: ResMut<RendererHasher>,
    ) {
        let render_graphic = &mut render_graphic;

        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            log::debug!("SysMainCameraRenderCommand:");
            match cmd {
                MainCameraRenderCommand::Active(entity, render_id, viewport) => {
                    if let Some(id_scene) = cameras.get(entity.clone()) {
                        if let Some((scene_time, scene_fog)) = scenes.get(id_scene.0) {
                            render_id.0.hash(&mut renderer_hasher.0);
                            let hash = renderer_hasher.0.finish();
                            let name = hash.to_string();

                            let mut renderer = Renderer::new("MainCameraOpaque", render_id.0.clone(), render_graphic);
                            render_graphic.add_depend("Clear", "MainCameraOpaque");
                            render_graphic.set_finish("MainCameraOpaque", true);
        

                            camera_modellist_cmd.insert(entity, ModelList(vec![]));
                            camera_filter_cmd.insert(entity, ModelListAfterCulling(vec![]));
                            renderid_cmd.insert(entity.clone(), render_id);
                            match viewport {
                                Some(viewport) => {
                                    viewport_cmd.insert(entity.clone(), viewport);
                                },
                                None => {
                                    viewport_cmd.insert(entity.clone(), Viewport::default());
                                },
                            };

                            let renderer_set = ShaderSetSceneAbout::new(ShaderSetBind::SET_SCENE_ABOUT, true, true, false, false);
                            let renderer_bindoff = renderer_set.bind_offset(&mut dynbuffer);

                            renderer_scene_cmd.insert(render_id.0.clone(), id_scene.clone());
                            renderer_cmd.insert(render_id.0.clone(), renderer);
                            postprocess_cmd.insert(render_id.0.clone(), Postprocess::default());
                            sceneid_cmd.insert(render_id.0.clone(), id_scene.clone());
                            
                            dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), &ViewerViewMatrix::default());
                            dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), &ViewerProjectionMatrix::default());
                            dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), &ViewerTransformMatrix::default());
                            dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), &ViewerGlobalPosition::default());
                            dynbuffer.as_mut().set_uniform(renderer_bindoff.camera().bind_offset(), &ViewerDirection::default());
                            if let Some(bind) = renderer_bindoff.time() {
                                dynbuffer.as_mut().set_uniform(bind.bind_offset(), scene_time);
                            }
                            if let Some(bind) = renderer_bindoff.fog() {
                                dynbuffer.as_mut().set_uniform(bind.bind_offset(), scene_fog);
                            }
                            
                            let layout_entries = renderer_set.layout_entries();
                            bindgrouppool.creat(&device, RenderBindGroupKey::SceneAbout(render_id.0.clone()), layout_entries.as_slice(), ShaderSetBind::SET_SCENE_ABOUT);
                            

                            renderer_viewer_cmd.insert(render_id.0.clone(), ViewerID(entity));
                            renderer_set_cmd.insert(render_id.0.clone(), renderer_set);
                            renderer_bindoff_cmd.insert(render_id.0.clone(), renderer_bindoff);
                        } else {
                            cmds.list.push(cmd);
                        }
                    }
                },
            }
        });
    }
}
