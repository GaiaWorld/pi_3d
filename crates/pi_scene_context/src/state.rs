

use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;

use crate::{
    pass::*,
    renderers::prelude::*, 
    meshes::prelude::*,
    geometry::prelude::*,
    viewer::prelude::*,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EMeshState {
    Null,
    Init,
    GeomtryReady,
    Set0Ready(PassTag),
    Set1Ready(PassTag),
    Set2Ready(PassTag),
    BindGroupsReady(PassTag),
    ShaderReady(PassTag),
    PipelineReady(PassTag),
    DrawReady(PassTag),
}
impl EMeshState {
    fn num(&self) -> u8 {
        match self {
            EMeshState::Init                => 001,
            EMeshState::GeomtryReady        => 002,
            EMeshState::Set0Ready(_)           => 003,
            EMeshState::Set1Ready(_)           => 004,
            EMeshState::Set2Ready(_)           => 005,
            EMeshState::BindGroupsReady(_)     => 006,
            EMeshState::ShaderReady(_)         => 007,
            EMeshState::PipelineReady(_)       => 008,
            EMeshState::DrawReady(_)           => 009,
            EMeshState::Null                => 000,
        }
    }
    fn val(v: u8, pass: PassTag) -> EMeshState {
        match v {
            001 => EMeshState::Init                ,
            002 => EMeshState::GeomtryReady        ,
            003 => EMeshState::Set0Ready(pass)           ,
            004 => EMeshState::Set1Ready(pass)           ,
            005 => EMeshState::Set2Ready(pass)           ,
            006 => EMeshState::BindGroupsReady(pass)     ,
            007 => EMeshState::ShaderReady(pass)         ,
            008 => EMeshState::PipelineReady(pass)       ,
            009 => EMeshState::DrawReady(pass)           ,
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
impl TMeshState for RenderGeometryComp                  { const MESH_STATE: u8 = 002; }
impl TMeshStatePass for PassBindGroupScene              { const MESH_STATE: u8 = 003; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassBindGroupModel              { const MESH_STATE: u8 = 004; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassBindGroupTextureSamplers    { const MESH_STATE: u8 = 005; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassBindGroups                  { const MESH_STATE: u8 = 006; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassShader                      { const MESH_STATE: u8 = 007; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassPipeline                    { const MESH_STATE: u8 = 008; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassDraw                        { const MESH_STATE: u8 = 009; fn is_some(&self) -> bool { self.val().is_some() } }

#[derive(Debug, Clone, Default, Component)]
pub struct MeshStates(pub Vec<EMeshState>);

#[derive(Component)]
pub struct DirtyMeshStates;

#[derive(Resource)]
pub struct StateRecordCfg {
    pub write_state: bool,
}

// pub struct SysMeshStatePass<T: TMeshStatePass + Component, P: TPass + Component>(PhantomData<(T, P)>);
// impl<T: TMeshStatePass + Component, P: TPass + Component> TSystemStageInfo for SysMeshStatePass<T, P> {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysRendererDraws::key()
//         ]
//     }
// }
// #[setup]
// impl<T: TMeshStatePass + Component, P: TPass + Component> SysMeshStatePass<T, P> {
//     #[system]
    fn sys_mesh_state_by_pass<T: TMeshStatePass + Component, P: TPass + Component>(
        mut models: Query<&mut MeshStates>,
        passes: Query<(&ModelPass, &T, &P), Changed<T>>,
        mut commands: Commands,
    ) {
        passes.iter().for_each(|(id_model, val, _)| {
            if val.is_some() {
                if let Ok(mut states) = models.get_mut(id_model.0) {
                    let state = EMeshState::val(T::MESH_STATE, P::TAG);
                    if !states.0.contains(&state) {
                        states.0.push(state);
                    }
                    if let Some(mut cmd) = commands.get_entity(id_model.0) {
                        cmd.insert(DirtyMeshStates);
                    }
                }
            }
        });
    }
// }

// pub struct SysMeshState<T: TMeshState + Component>(PhantomData<T>);
// impl<T: TMeshState + Component> TSystemStageInfo for SysMeshState<T> {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysPassDraw::<Pass01, PassID01>::key()
//         ]
//     }
// }
// #[setup]
// impl<T: TMeshState + Component> SysMeshState<T> {
//     #[system]
    fn sys_mesh_state_by_model<T: TMeshState + Component>(
        mut models: Query<(ObjectID, &mut MeshStates), Changed<T>>,
        mut commands: Commands,
    ) {
        models.iter_mut().for_each(|(id_model, mut states)| {
            let state = EMeshState::val(T::MESH_STATE, 0);
            if !states.0.contains(&state) {
                states.0.push(state);
            }
            if let Some(mut cmd) = commands.get_entity(id_model) {
                cmd.insert(DirtyMeshStates);
            }
        });
    }
// }

// pub struct SysGeometryState;
// impl TSystemStageInfo for SysGeometryState {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysPassDraw::<Pass01, PassID01>::key()
//         ]
//     }
// }
// #[setup]
// impl SysGeometryState {
//     #[system]
    fn sys_mesh_state_by_geometry(
        mut models: Query<(ObjectID, &mut MeshStates), Changed<RenderGeometryEable>>,
        mut commands: Commands,
    ) {
        models.iter_mut().for_each(|(id_model, mut states)| {
            let state = EMeshState::val(RenderGeometryComp::MESH_STATE, 0);
            if !states.0.contains(&state) {
                states.0.push(state);
            }
            if let Some(mut cmd) = commands.get_entity(id_model) {
                cmd.insert(DirtyMeshStates);
            }
        });
    }
// }

// pub struct SysMeshStateToFile;
// impl TSystemStageInfo for SysMeshStateToFile {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysMeshStatePass::<PassPipeline, Pass01>::key()
//         ]
//     }
// }
// #[setup]
// impl SysMeshStateToFile {
    const FILE_NAME: &'static str = "meshstate.md";
    // #[system]
    fn sys_mesh_state_to_file(
        items: Query<(ObjectID, &MeshStates), Changed<DirtyMeshStates>>,
        cfg: Res<StateRecordCfg>,
    ) {
        if cfg.write_state {
            let mut result = String::from("\r\nFrame:");
            let mut flag = false;
            let mut ids = XHashMap::default();
            items.iter().for_each(|(id, item)| {
                flag = true;
                if !ids.contains_key(&id) {
                    ids.insert(id, id);
                    let mut item = item.0.clone();
                    item.sort();
                    result += format!("\r\n{:?}", item ).as_str();
                }
            });
    
            if flag {
                let root_dir = std::env::current_dir().unwrap();
                let file_name = FILE_NAME;
                let path = root_dir.join(file_name);
                if let Ok(old) = std::fs::read_to_string(path) {
                    result = old + result.as_str();
                    std::fs::write(root_dir.join(file_name), result.as_str());
                }
            }
        }
    }
// }


// pub struct SysCheck;
// impl TSystemStageInfo for SysCheck {}
// #[setup]
// impl SysCheck {
//     #[system]
    fn sys_check(
        items: Query<(&Renderer, &RenderSize, &RenderColorFormat, &RenderColorClear, &RenderDepthFormat, &RenderDepthClear), Changed<RenderSize>>,
        items2: Query<(&ViewerActive, &SceneID, &BindViewer, &ModelList, &ViewerRenderersInfo), Changed<ViewerActive>>,
    ) {
        // items.iter().for_each(|v| {
        //     // log::warn!("####################");
        // });
        // items2.iter().for_each(|v| {
        //     // log::warn!("#################### 2");
        // });
    }
// }

pub struct PluginStateToFile;
impl Plugin for PluginStateToFile {
    // fn init(
    //     &mut self,
    //     engine: &mut crate::engine::Engine,
    //     stages: &mut crate::run_stage::RunStage,
    // ) -> Result<(), crate::plugin::ErrorPlugin> {
        
    //     let root_dir = std::env::current_dir().unwrap();
    //     let file_name = SysMeshStateToFile::FILE_NAME;
    //     std::fs::write(root_dir.join(file_name), "");

    //     let world = engine.world_mut();

    //     SysCheck::setup(world, stages.query_stage::<SysCheck>(ERunStageChap::Draw));

    //     SysMeshState::<AbstructMesh>::setup(world, stages.query_stage::<SysMeshState::<AbstructMesh>>(ERunStageChap::Draw));
    //     SysGeometryState::setup(world, stages.query_stage::<SysGeometryState>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupScene, Pass01>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupScene, Pass01>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupScene, Pass02>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupScene, Pass02>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupScene, Pass03>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupScene, Pass03>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupScene, Pass04>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupScene, Pass04>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupScene, Pass05>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupScene, Pass05>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupScene, Pass06>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupScene, Pass06>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupScene, Pass07>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupScene, Pass07>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupScene, Pass08>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupScene, Pass08>>(ERunStageChap::Draw));

    //     SysMeshStatePass::<PassBindGroupModel, Pass01>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupModel, Pass01>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupModel, Pass02>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupModel, Pass02>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupModel, Pass03>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupModel, Pass03>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupModel, Pass04>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupModel, Pass04>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupModel, Pass05>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupModel, Pass05>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupModel, Pass06>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupModel, Pass06>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupModel, Pass07>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupModel, Pass07>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupModel, Pass08>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupModel, Pass08>>(ERunStageChap::Draw));

    //     SysMeshStatePass::<PassBindGroupTextureSamplers, Pass01>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupTextureSamplers, Pass01>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupTextureSamplers, Pass02>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupTextureSamplers, Pass02>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupTextureSamplers, Pass03>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupTextureSamplers, Pass03>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupTextureSamplers, Pass04>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupTextureSamplers, Pass04>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupTextureSamplers, Pass05>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupTextureSamplers, Pass05>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupTextureSamplers, Pass06>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupTextureSamplers, Pass06>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupTextureSamplers, Pass07>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupTextureSamplers, Pass07>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroupTextureSamplers, Pass08>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroupTextureSamplers, Pass08>>(ERunStageChap::Draw));

    //     SysMeshStatePass::<PassBindGroups, Pass01>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroups, Pass01>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroups, Pass02>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroups, Pass02>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroups, Pass03>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroups, Pass03>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroups, Pass04>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroups, Pass04>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroups, Pass05>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroups, Pass05>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroups, Pass06>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroups, Pass06>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroups, Pass07>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroups, Pass07>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassBindGroups, Pass08>::setup(world, stages.query_stage::<SysMeshStatePass::<PassBindGroups, Pass08>>(ERunStageChap::Draw));

    //     SysMeshStatePass::<PassShader, Pass01>::setup(world, stages.query_stage::<SysMeshStatePass::<PassShader, Pass01>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassShader, Pass02>::setup(world, stages.query_stage::<SysMeshStatePass::<PassShader, Pass02>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassShader, Pass03>::setup(world, stages.query_stage::<SysMeshStatePass::<PassShader, Pass03>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassShader, Pass04>::setup(world, stages.query_stage::<SysMeshStatePass::<PassShader, Pass04>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassShader, Pass05>::setup(world, stages.query_stage::<SysMeshStatePass::<PassShader, Pass05>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassShader, Pass06>::setup(world, stages.query_stage::<SysMeshStatePass::<PassShader, Pass06>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassShader, Pass07>::setup(world, stages.query_stage::<SysMeshStatePass::<PassShader, Pass07>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassShader, Pass08>::setup(world, stages.query_stage::<SysMeshStatePass::<PassShader, Pass08>>(ERunStageChap::Draw));

    //     SysMeshStatePass::<PassPipeline, Pass01>::setup(world, stages.query_stage::<SysMeshStatePass::<PassPipeline, Pass01>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassPipeline, Pass02>::setup(world, stages.query_stage::<SysMeshStatePass::<PassPipeline, Pass02>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassPipeline, Pass03>::setup(world, stages.query_stage::<SysMeshStatePass::<PassPipeline, Pass03>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassPipeline, Pass04>::setup(world, stages.query_stage::<SysMeshStatePass::<PassPipeline, Pass04>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassPipeline, Pass05>::setup(world, stages.query_stage::<SysMeshStatePass::<PassPipeline, Pass05>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassPipeline, Pass06>::setup(world, stages.query_stage::<SysMeshStatePass::<PassPipeline, Pass06>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassPipeline, Pass07>::setup(world, stages.query_stage::<SysMeshStatePass::<PassPipeline, Pass07>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassPipeline, Pass08>::setup(world, stages.query_stage::<SysMeshStatePass::<PassPipeline, Pass08>>(ERunStageChap::Draw));

    //     SysMeshStatePass::<PassDraw, Pass01>::setup(world, stages.query_stage::<SysMeshStatePass::<PassDraw, Pass01>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassDraw, Pass02>::setup(world, stages.query_stage::<SysMeshStatePass::<PassDraw, Pass02>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassDraw, Pass03>::setup(world, stages.query_stage::<SysMeshStatePass::<PassDraw, Pass03>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassDraw, Pass04>::setup(world, stages.query_stage::<SysMeshStatePass::<PassDraw, Pass04>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassDraw, Pass05>::setup(world, stages.query_stage::<SysMeshStatePass::<PassDraw, Pass05>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassDraw, Pass06>::setup(world, stages.query_stage::<SysMeshStatePass::<PassDraw, Pass06>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassDraw, Pass07>::setup(world, stages.query_stage::<SysMeshStatePass::<PassDraw, Pass07>>(ERunStageChap::Draw));
    //     SysMeshStatePass::<PassDraw, Pass08>::setup(world, stages.query_stage::<SysMeshStatePass::<PassDraw, Pass08>>(ERunStageChap::Draw));

    //     SysMeshStateToFile::setup(world, stages.query_stage::<SysMeshStateToFile>(ERunStageChap::Draw));

    //     Ok(())
    // }

    fn build(&self, app: &mut App) {
        
        let root_dir = std::env::current_dir().unwrap();
        let file_name = FILE_NAME;
        std::fs::write(root_dir.join(file_name), "");

        app.insert_resource(StateRecordCfg { write_state: true });
        
        app.add_systems(
            (
                sys_mesh_state_by_pass::<PassBindGroupScene, Pass01>,
                sys_mesh_state_by_pass::<PassBindGroupScene, Pass02>,
                sys_mesh_state_by_pass::<PassBindGroupScene, Pass03>,
                sys_mesh_state_by_pass::<PassBindGroupScene, Pass04>,
                sys_mesh_state_by_pass::<PassBindGroupScene, Pass05>,
                sys_mesh_state_by_pass::<PassBindGroupScene, Pass06>,
                sys_mesh_state_by_pass::<PassBindGroupScene, Pass07>,
                sys_mesh_state_by_pass::<PassBindGroupScene, Pass08>,
            ).in_set(ERunStageChap::StateCheck)
        );
        
        app.add_systems(
            (
                sys_mesh_state_by_pass::<PassBindGroupModel, Pass01>,
                sys_mesh_state_by_pass::<PassBindGroupModel, Pass02>,
                sys_mesh_state_by_pass::<PassBindGroupModel, Pass03>,
                sys_mesh_state_by_pass::<PassBindGroupModel, Pass04>,
                sys_mesh_state_by_pass::<PassBindGroupModel, Pass05>,
                sys_mesh_state_by_pass::<PassBindGroupModel, Pass06>,
                sys_mesh_state_by_pass::<PassBindGroupModel, Pass07>,
                sys_mesh_state_by_pass::<PassBindGroupModel, Pass08>,
            ).in_set(ERunStageChap::StateCheck)
        );
        app.add_systems(
            (
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass01>,
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass02>,
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass03>,
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass04>,
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass05>,
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass06>,
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass07>,
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass08>,
            ).in_set(ERunStageChap::StateCheck)
        );
        app.add_systems(
            (
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass01>,
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass02>,
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass03>,
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass04>,
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass05>,
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass06>,
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass07>,
                sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, Pass08>,
            ).in_set(ERunStageChap::StateCheck)
        );
        app.add_systems(
            (
                sys_mesh_state_by_pass::<PassBindGroups, Pass01>,
                sys_mesh_state_by_pass::<PassBindGroups, Pass02>,
                sys_mesh_state_by_pass::<PassBindGroups, Pass03>,
                sys_mesh_state_by_pass::<PassBindGroups, Pass04>,
                sys_mesh_state_by_pass::<PassBindGroups, Pass05>,
                sys_mesh_state_by_pass::<PassBindGroups, Pass06>,
                sys_mesh_state_by_pass::<PassBindGroups, Pass07>,
                sys_mesh_state_by_pass::<PassBindGroups, Pass08>,
            ).in_set(ERunStageChap::StateCheck)
        );
        app.add_systems(
            (
                sys_mesh_state_by_pass::<PassShader, Pass01>,
                sys_mesh_state_by_pass::<PassShader, Pass02>,
                sys_mesh_state_by_pass::<PassShader, Pass03>,
                sys_mesh_state_by_pass::<PassShader, Pass04>,
                sys_mesh_state_by_pass::<PassShader, Pass05>,
                sys_mesh_state_by_pass::<PassShader, Pass06>,
                sys_mesh_state_by_pass::<PassShader, Pass07>,
                sys_mesh_state_by_pass::<PassShader, Pass08>,
            ).in_set(ERunStageChap::StateCheck)
        );
        app.add_systems(
            (
                sys_mesh_state_by_pass::<PassPipeline, Pass01>,
                sys_mesh_state_by_pass::<PassPipeline, Pass02>,
                sys_mesh_state_by_pass::<PassPipeline, Pass03>,
                sys_mesh_state_by_pass::<PassPipeline, Pass04>,
                sys_mesh_state_by_pass::<PassPipeline, Pass05>,
                sys_mesh_state_by_pass::<PassPipeline, Pass06>,
                sys_mesh_state_by_pass::<PassPipeline, Pass07>,
                sys_mesh_state_by_pass::<PassPipeline, Pass08>,
            ).in_set(ERunStageChap::StateCheck)
        );
        app.add_systems(
            (
                sys_mesh_state_by_pass::<PassDraw, Pass01>,
                sys_mesh_state_by_pass::<PassDraw, Pass02>,
                sys_mesh_state_by_pass::<PassDraw, Pass03>,
                sys_mesh_state_by_pass::<PassDraw, Pass04>,
                sys_mesh_state_by_pass::<PassDraw, Pass05>,
                sys_mesh_state_by_pass::<PassDraw, Pass06>,
                sys_mesh_state_by_pass::<PassDraw, Pass07>,
                sys_mesh_state_by_pass::<PassDraw, Pass08>,
            ).in_set(ERunStageChap::StateCheck)
        );
        app.add_systems(
            (
                sys_mesh_state_by_model::<AbstructMesh>,
                sys_mesh_state_by_geometry,
                sys_check,
                sys_mesh_state_to_file
            ).chain().in_set(ERunStageChap::StateCheck)
        );
    }
}