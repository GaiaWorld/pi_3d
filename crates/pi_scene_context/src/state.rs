

use pi_scene_shell::prelude::*;

use crate::{
    pass::*, 
    meshes::prelude::*,
    geometry::prelude::*,
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
    pub fn num(&self) -> u8 {
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
//     fn depends() -> Vec<pi_scene_shell::run_stage::KeySystem> {
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
        passes: Query<(&PassModelID, &T, &P), Changed<T>>,
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
//     fn depends() -> Vec<pi_scene_shell::run_stage::KeySystem> {
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
            let state = EMeshState::val(T::MESH_STATE, PassTag::PASS_TAG_08);
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
//     fn depends() -> Vec<pi_scene_shell::run_stage::KeySystem> {
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
            let state = EMeshState::val(RenderGeometryComp::MESH_STATE, PassTag::PASS_TAG_08);
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
//     fn depends() -> Vec<pi_scene_shell::run_stage::KeySystem> {
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
                    if let Ok(_) = std::fs::write(root_dir.join(file_name), result.as_str()) {

                    }
                }
            }
        }
    }

pub struct PluginStateToFile;
impl Plugin for PluginStateToFile {
    fn build(&self, app: &mut App) {

        app.insert_resource(StateRecordCfg { write_state: true });
        
        let root_dir = std::env::current_dir().unwrap();
        let file_name = FILE_NAME;
        if let Ok(_) = std::fs::write(root_dir.join(file_name), "") {

        } else {
            return;
        }
        
        app.add_systems(
			Update,
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
			Update,
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
			Update,
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
			Update,
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
			Update,
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
			Update,
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
			Update,
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
			Update,
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
			Update,
            (
                sys_mesh_state_by_model::<AbstructMesh>,
                sys_mesh_state_by_geometry,
                // sys_check,
                sys_mesh_state_to_file
            ).chain().in_set(ERunStageChap::StateCheck)
        );
    }
}