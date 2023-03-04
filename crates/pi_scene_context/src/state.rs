use std::marker::PhantomData;

use futures::stream::Chain;
use pi_ecs::{prelude::{Query, Setup, Component, Commands}, query::Changed};
use pi_ecs_macros::setup;
use pi_engine_shell::{run_stage::{TSystemStageInfo, ERunStageChap}, object::{GameObject, ObjectID}, plugin::Plugin};
use pi_hash::XHashMap;

use crate::{renderers::{sys_renderer::{SysRendererDraws, SysPassDraw}, base::{Pipeline3D, Pipeline3DUsage}, pass::{PassPipeline, PassBindGroups, PassShader, PassDraw}}, pass::{PassSource, TPassData, PassBindGroupScene, PassBindGroupModel, PassBindGroupTextureSamplers, Pass01, PassID01}, meshes::abstract_mesh::AbstructMesh, geometry::geometry::RenderGeometry};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EMeshState {
    Null,
    Init,
    GeomtryReady,
    Set0Ready,
    Set1Ready,
    Set2Ready,
    BindGroupsReady,
    ShaderReady,
    PipelineReady,
    DrawReady,
}
impl EMeshState {
    fn num(&self) -> u8 {
        match self {
            EMeshState::Init                => 001,
            EMeshState::GeomtryReady        => 002,
            EMeshState::Set0Ready           => 003,
            EMeshState::Set1Ready           => 004,
            EMeshState::Set2Ready           => 005,
            EMeshState::BindGroupsReady     => 006,
            EMeshState::ShaderReady         => 007,
            EMeshState::PipelineReady       => 008,
            EMeshState::DrawReady           => 009,
            EMeshState::Null                => 000,
        }
    }
    fn val(v: u8) -> EMeshState {
        match v {
            001 => EMeshState::Init                ,
            002 => EMeshState::GeomtryReady        ,
            003 => EMeshState::Set0Ready           ,
            004 => EMeshState::Set1Ready           ,
            005 => EMeshState::Set2Ready           ,
            006 => EMeshState::BindGroupsReady     ,
            007 => EMeshState::ShaderReady         ,
            008 => EMeshState::PipelineReady       ,
            009 => EMeshState::DrawReady           ,
            _   => EMeshState::Null                ,
        }
    }
}

pub trait TMeshState {
    const MESH_STATE: u8;
}
pub trait TMeshStatePass {
    const MESH_STATE: u8;
    fn is_some(&self) -> bool;
}

impl TMeshState for AbstructMesh                    { const MESH_STATE: u8 = 001; }
impl TMeshState for RenderGeometry                  { const MESH_STATE: u8 = 002; }
impl TMeshStatePass for PassBindGroupScene              { const MESH_STATE: u8 = 003; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassBindGroupModel              { const MESH_STATE: u8 = 004; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassBindGroupTextureSamplers    { const MESH_STATE: u8 = 005; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassBindGroups                  { const MESH_STATE: u8 = 006; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassShader                      { const MESH_STATE: u8 = 007; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassPipeline                    { const MESH_STATE: u8 = 008; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassDraw                        { const MESH_STATE: u8 = 009; fn is_some(&self) -> bool { self.val().is_some() } }

#[derive(Debug, Clone, Default)]
pub struct MeshStates(pub XHashMap<EMeshState, EMeshState>);

pub struct DirtyMeshStates;

pub struct SysMeshStatePass<T: TMeshStatePass + Component>(PhantomData<T>);
impl<T: TMeshStatePass + Component> TSystemStageInfo for SysMeshStatePass<T> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysRendererDraws::key()
        ]
    }
}
#[setup]
impl<T: TMeshStatePass + Component> SysMeshStatePass<T> {
    #[system]
    fn sys(
        mut models: Query<GameObject, &mut MeshStates>,
        passes: Query<GameObject, (&PassSource, &T), Changed<T>>,
        mut meshstateflag_cmd: Commands<GameObject, DirtyMeshStates>,
    ) {
        passes.iter().for_each(|(id_model, val)| {
            if val.is_some() {
                if let Some(mut states) = models.get_mut(id_model.0) {
                    let state = EMeshState::val(T::MESH_STATE);
                    states.0.insert(state, state);
                    meshstateflag_cmd.insert(id_model.0, DirtyMeshStates);
                    log::info!("MeshState: {:?}", state);
                }
            }
        });
    }
}

pub struct SysMeshState<T: TMeshState + Component>(PhantomData<T>);
impl<T: TMeshState + Component> TSystemStageInfo for SysMeshState<T> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysPassDraw::<Pass01, PassID01>::key()
        ]
    }
}
#[setup]
impl<T: TMeshState + Component> SysMeshState<T> {
    #[system]
    fn sys(
        mut models: Query<GameObject, (ObjectID, &mut MeshStates), Changed<T>>,
        mut meshstateflag_cmd: Commands<GameObject, DirtyMeshStates>,
    ) {
        models.iter_mut().for_each(|(id_model, mut states)| {
            let state = EMeshState::val(T::MESH_STATE);
            states.0.insert(state, state);
            meshstateflag_cmd.insert(id_model, DirtyMeshStates);
            log::info!("MeshState: {:?}", state);
        });
    }
}

pub struct SysMeshStateToFile;
impl TSystemStageInfo for SysMeshStateToFile {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysMeshStatePass::<PassPipeline>::key()
        ]
    }
}
#[setup]
impl SysMeshStateToFile {
    const FILE_NAME: &'static str = "meshstate.md";
    #[system]
    fn sys(
        items: Query<GameObject, (ObjectID, &MeshStates), Changed<DirtyMeshStates>>,
    ) {
        let mut result = String::from("\r\nFrame:");
        let mut flag = false;
        let mut ids = XHashMap::default();
        items.iter().for_each(|(id, item)| {
            flag = true;
            if !ids.contains_key(&id) {
                ids.insert(id, id);
                result += format!("\r\n{:?}", item.0.values()).as_str();
            }
        });

        if flag {
            let root_dir = std::env::current_dir().unwrap();
            let file_name = Self::FILE_NAME;
            let path = root_dir.join(file_name);
            if let Ok(old) = std::fs::read_to_string(path) {
                result = old + result.as_str();
                std::fs::write(root_dir.join(file_name), result.as_str());
            }
        }
    }
}

pub struct PluginStateToFile;
impl Plugin for PluginStateToFile {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        
        let root_dir = std::env::current_dir().unwrap();
        let file_name = SysMeshStateToFile::FILE_NAME;
        std::fs::write(root_dir.join(file_name), "");

        let world = engine.world_mut();

        SysMeshState::<AbstructMesh>::setup(world, stages.query_stage::<SysMeshState::<AbstructMesh>>(ERunStageChap::Uniform));
        SysMeshState::<RenderGeometry>::setup(world, stages.query_stage::<SysMeshState::<RenderGeometry>>(ERunStageChap::Uniform));
        SysMeshStatePass::<PassBindGroupScene>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupScene>>(ERunStageChap::Uniform));
        SysMeshStatePass::<PassBindGroupModel>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupModel>>(ERunStageChap::Uniform));
        SysMeshStatePass::<PassBindGroupTextureSamplers>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupTextureSamplers>>(ERunStageChap::Uniform));
        SysMeshStatePass::<PassBindGroups>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroups>>(ERunStageChap::Uniform));
        SysMeshStatePass::<PassShader>::setup(world, stages.query_stage::<SysMeshStatePass::<PassShader>>(ERunStageChap::Uniform));
        SysMeshStatePass::<PassPipeline>::setup(world, stages.query_stage::<SysMeshStatePass::<PassPipeline>>(ERunStageChap::Uniform));
        SysMeshStatePass::<PassDraw>::setup(world, stages.query_stage::<SysMeshStatePass::<PassDraw>>(ERunStageChap::Uniform));

        SysMeshStateToFile::setup(world, stages.query_stage::<SysMeshStateToFile>(ERunStageChap::Uniform));

        Ok(())
    }
}