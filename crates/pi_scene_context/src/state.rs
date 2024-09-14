

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
// impl TMeshStatePass for PassBindGroupScene              { const MESH_STATE: u8 = 003; fn is_some(&self) -> bool { self.val().is_some() } }
// impl TMeshStatePass for PassBindGroupModel              { const MESH_STATE: u8 = 004; fn is_some(&self) -> bool { self.val().is_some() } }
// impl TMeshStatePass for PassBindGroupTextureSamplers    { const MESH_STATE: u8 = 005; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassBindGroups                  { const MESH_STATE: u8 = 006; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassShader                      { const MESH_STATE: u8 = 007; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassPipeline                    { const MESH_STATE: u8 = 008; fn is_some(&self) -> bool { self.val().is_some() } }
impl TMeshStatePass for PassDraw                        { const MESH_STATE: u8 = 009; fn is_some(&self) -> bool { self.val() } }

#[derive(Resource)]
pub struct StateRecordCfg {
    pub write_state: bool,
}

pub struct PluginStateToFile;
impl Plugin for PluginStateToFile {
    fn build(&self, app: &mut App) {
        app.insert_resource(StateRecordCfg { write_state: true });
    }
}