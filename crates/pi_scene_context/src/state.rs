

use pi_scene_shell::prelude::*;

use crate::{
    pass::*, 
    meshes::prelude::*,
    geometry::prelude::*,
};


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Clone, Component, Default)]
pub struct MeshStates(pub Vec<EMeshState>);

#[derive(Component, Default)]
pub struct DirtyMeshStates;

#[derive(Resource)]
pub struct StateRecordCfg {
    pub write_state: bool,
}

    fn sys_mesh_state_by_pass<T: TMeshStatePass + Component, const P: u16>(
        mut models: Query<&mut MeshStates>,
        passes: Query<(&PassModelID, &T), Changed<T>>,
        mut commands: Commands,
    ) {
        passes.iter().for_each(|(id_model, val)| {
            if val.is_some() {
                if let Ok(mut states) = models.get_mut(id_model.0) {
                    let state = EMeshState::val(T::MESH_STATE, PassTag::new(P));
                    if !states.0.contains(&state) {
                        states.0.push(state);
                    }
                    if let Some(mut cmd) = commands.get_entity(id_model.0) {
                        cmd.insert((DirtyMeshStates,));
                    }
                }
            }
        });
    }
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
                cmd.insert((DirtyMeshStates,));
            }
        });
    }
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
                cmd.insert((DirtyMeshStates,));
            }
        });
    }
    const FILE_NAME: &'static str = "temp/meshstate.md";
    // #[system]
    fn sys_mesh_state_to_file(
        items: Query<(ObjectID, &MeshStates), Changed<DirtyMeshStates>>,
        cfg: Res<StateRecordCfg>,
    ) {
        // if cfg.write_state {
        //     let mut result = String::from("\r\nFrame:");
        //     let mut flag = false;
        //     let mut ids = XHashMap::default();
        //     items.iter().for_each(|(id, item)| {
        //         flag = true;
        //         if !ids.contains_key(&id) {
        //             ids.insert(id, id);
        //             let mut item = item.0.clone();
        //             item.sort();
        //             result += format!("\r\n{:?}", item ).as_str();
        //         }
        //     });
        //     if flag {
        //         let root_dir = std::env::current_dir().unwrap();
        //         let file_name = FILE_NAME;
        //         let path = root_dir.join(file_name);
        //         if let Ok(old) = std::fs::read_to_string(path) {
        //             result = old + result.as_str();
        //             if let Ok(_) = std::fs::write(root_dir.join(file_name), result.as_str()) {

        //             }
        //         }
        //     }
        // }
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
#[cfg(feature = "use_bevy")]
        app.add_systems(
			Update,
            (
                (
                    sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_01 }>,
                    sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_02 }>,
                    sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_03 }>,
                    sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_04 }>,
                    sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_05 }>,
                    sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_06 }>,
                    sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_07 }>,
                    sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_08 }>,
                ).in_set(ERunStageChap::StateCheck),
                (
                    sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_01 }>,
                    sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_02 }>,
                    sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_03 }>,
                    sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_04 }>,
                    sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_05 }>,
                    sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_06 }>,
                    sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_07 }>,
                    sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_08 }>,
                ).in_set(ERunStageChap::StateCheck),
                (
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_01 }>,
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_02 }>,
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_03 }>,
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_04 }>,
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_05 }>,
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_06 }>,
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_07 }>,
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_08 }>,
                ).in_set(ERunStageChap::StateCheck),
                (
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_01 }>,
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_02 }>,
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_03 }>,
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_04 }>,
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_05 }>,
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_06 }>,
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_07 }>,
                    sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_08 }>,
                ).in_set(ERunStageChap::StateCheck),
                (
                    sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_01 }>,
                    sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_02 }>,
                    sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_03 }>,
                    sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_04 }>,
                    sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_05 }>,
                    sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_06 }>,
                    sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_07 }>,
                    sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_08 }>,
                ).in_set(ERunStageChap::StateCheck),
                (
                    sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_01 }>,
                    sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_02 }>,
                    sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_03 }>,
                    sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_04 }>,
                    sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_05 }>,
                    sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_06 }>,
                    sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_07 }>,
                    sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_08 }>,
                ).in_set(ERunStageChap::StateCheck),
                (
                    sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_01 }>,
                    sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_02 }>,
                    sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_03 }>,
                    sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_04 }>,
                    sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_05 }>,
                    sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_06 }>,
                    sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_07 }>,
                    sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_08 }>,
                ).in_set(ERunStageChap::StateCheck),
                (
                    sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_01 }>,
                    sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_02 }>,
                    sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_03 }>,
                    sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_04 }>,
                    sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_05 }>,
                    sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_06 }>,
                    sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_07 }>,
                    sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_08 }>,
                ).in_set(ERunStageChap::StateCheck),
                (
                    sys_mesh_state_by_model::<AbstructMesh>,
                    sys_mesh_state_by_geometry,
                    // sys_check,
                    sys_mesh_state_to_file
                ).chain().in_set(ERunStageChap::StateCheck)
            )
        );

#[cfg(not(feature = "use_bevy"))]
        app
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_01 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_02 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_03 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_04 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_05 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_06 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_07 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupScene, { PassTag::PASS_08 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_01 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_02 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_03 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_04 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_05 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_06 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_07 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupModel, { PassTag::PASS_08 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_01 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_02 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_03 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_04 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_05 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_06 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_07 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_08 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_01 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_02 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_03 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_04 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_05 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_06 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_07 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroupTextureSamplers, { PassTag::PASS_08 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_01 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_02 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_03 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_04 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_05 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_06 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_07 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassBindGroups, { PassTag::PASS_08 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_01 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_02 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_03 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_04 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_05 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_06 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_07 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassShader, { PassTag::PASS_08 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_01 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_02 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_03 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_04 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_05 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_06 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_07 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassPipeline, { PassTag::PASS_08 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_01 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_02 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_03 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_04 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_05 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_06 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_07 }>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_pass::<PassDraw, { PassTag::PASS_08 }>.in_set(ERunStageChap::StateCheck))

            .add_systems(Update, sys_mesh_state_by_model::<AbstructMesh>.in_set(ERunStageChap::StateCheck))
            .add_systems(Update, sys_mesh_state_by_geometry.after(sys_mesh_state_by_model::<AbstructMesh>).in_set(ERunStageChap::StateCheck))
            // sys_check,
            .add_systems(Update, sys_mesh_state_to_file.after(sys_mesh_state_by_geometry).in_set(ERunStageChap::StateCheck))
            ;
    }
}