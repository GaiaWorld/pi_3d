use pi_scene_shell::prelude::*;
use pi_slotmap::Key;

pub mod instanced_buffer;
pub mod types;
pub mod instance_world_matrix;
pub mod instance_color;
pub mod instance_tilloff;
pub mod instance_boneoffset;
pub mod instance_float;
pub mod instance_vec4;
pub mod instance_vec3;
pub mod sys_instance;

/// 标识节点为 实例Mesh
#[derive(Debug, Clone, Component, Default)]
pub struct InstanceMesh(pub Entity);
impl InstanceMesh {
    pub fn id(&self) -> String {
        self.0.index().to_string()
    }
}
impl TEntityRef for InstanceMesh {
    fn id(&self) -> Entity {
        self.0
    }
}

/// 标识同Mesh的实例同层级时如何排序
#[derive(Debug, Clone, Component, Default, PartialEq, Eq)]
pub enum EInstanceSortMode {
    LocalPositionX = 0,
    LocalPositionY = 1,
    LocalPositionZ = 2,
    NagativeLocalPositionX = 3,
    NagativeLocalPositionY = 4,
    NagativeLocalPositionZ = 5,
    GlobalPositionX = 6,
    GlobalPositionY = 7,
    #[default]
    GlobalPositionZ = 8,
    NagativeGlobalPositionX = 9,
    NagativeGlobalPositionY = 10,
    NagativeGlobalPositionZ = 11,
}
impl EInstanceSortMode {
    pub fn from_u8(val: u8) -> Self {
        match val {
            0 => EInstanceSortMode::LocalPositionX,
            1 => EInstanceSortMode::LocalPositionY,
            2 => EInstanceSortMode::LocalPositionZ,
            3 => EInstanceSortMode::NagativeLocalPositionX,
            4 => EInstanceSortMode::NagativeLocalPositionY,
            5 => EInstanceSortMode::NagativeLocalPositionZ,
            6 => EInstanceSortMode::GlobalPositionX,
            7 => EInstanceSortMode::GlobalPositionY,

            
            9 =>  EInstanceSortMode::NagativeGlobalPositionX,
            10 => EInstanceSortMode::NagativeGlobalPositionY,
            11 => EInstanceSortMode::NagativeGlobalPositionZ,
            _ => EInstanceSortMode::GlobalPositionZ,
        }
    }
    pub fn arg_for_sortparam(&self) -> (bool, usize, f32) {
        match self {
            EInstanceSortMode::LocalPositionX => (false, 0, 1.),
            EInstanceSortMode::LocalPositionY => (false, 1, 1.),
            EInstanceSortMode::LocalPositionZ => (false, 2, 1.),
            EInstanceSortMode::NagativeLocalPositionX => (false, 0, -1.),
            EInstanceSortMode::NagativeLocalPositionY => (false, 1, -1.),
            EInstanceSortMode::NagativeLocalPositionZ => (false, 2, -1.),
            EInstanceSortMode::GlobalPositionX => (true, 0, 1.),
            EInstanceSortMode::GlobalPositionY => (true, 1, 1.),
            EInstanceSortMode::GlobalPositionZ => (true, 2, 1.),
            EInstanceSortMode::NagativeGlobalPositionX => (true, 0, -1.),
            EInstanceSortMode::NagativeGlobalPositionY => (true, 1, -1.),
            EInstanceSortMode::NagativeGlobalPositionZ => (true, 2, -1.),
        }
    }
}

/// 标识实例对应的源Mesh是否脏
#[derive(Debug, Clone, Component, Default)]
pub struct DirtyInstanceSourceRefs;

/// 记录实例的源Mesh派生的所有实例
pub type InstanceSourceRefs = EntityRefInfo<DirtyInstanceSourceRefs>;

/// 标识实例对应的 源Mesh 的顶点Buffer是否脏 - 针对大量实例的Mesh会使用单独的Buffer,一般情况使用共用的Buffer
#[derive(Debug, Clone, Component, Default)]
pub struct DirtyInstanceSourceForSingleBuffer;
