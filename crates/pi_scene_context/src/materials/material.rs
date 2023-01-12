
use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, Commands}, query::{With}};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::TSystemStageInfo;
use pi_scene_math::{Number, Matrix, Vector4, Vector2, Matrix2};

use crate::{object::{ObjectID, GameObject}, renderers::render_mode::ERenderMode};

use super::uniforms::{mat4::Mat4Uniform, mat2::Mat2Uniform, vec4::Vec4Uniform, vec2::Vec2Uniform, float::FloatUniform, int::IntUniform, uint::UintUniform};

pub trait TMaterial {
    fn render_mode(&self) -> ERenderMode;
}
#[derive(Debug)]
pub enum ECommand {
    Mat4(ObjectID, usize, Matrix, bool),
    Mat2(ObjectID, usize, Matrix2, bool),
    Vec4(ObjectID, usize, Vector4, bool),
    Vec2(ObjectID, usize, Vector2, bool),
    Float(ObjectID, usize, Number, bool),
    Int(ObjectID, usize, i32, bool),
    Uint(ObjectID, usize, u32, bool),
}

#[derive(Debug, Default)]
pub struct SingleValueUniformCommands(pub Vec<ECommand>);

pub struct SysEffectValueUniformComand;
impl TSystemStageInfo for SysEffectValueUniformComand {

}
#[setup]
impl SysEffectValueUniformComand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleValueUniformCommands>,
        mut mat4:   Query<GameObject, &mut Mat4Uniform, With<Mat4Uniform>>,
        mut mat2:   Query<GameObject, &mut Mat2Uniform, With<Mat2Uniform>>,
        mut vec4:   Query<GameObject, &mut Vec4Uniform, With<Vec4Uniform>>,
        mut vec2:   Query<GameObject, &mut Vec2Uniform, With<Vec2Uniform>>,
        mut float:  Query<GameObject, &mut FloatUniform, With<FloatUniform>>,
        mut int:    Query<GameObject, &mut IntUniform, With<IntUniform>>,
        mut uint:   Query<GameObject, &mut UintUniform, With<UintUniform>>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Mat4  (entity, slot, value, ismust) => {
                    if let Some(mut item) = mat4.get_mut(entity) {
                        item.set(slot, value.as_slice());
                    } else {
                        cmds.0.push(cmd);
                    }
                },
                ECommand::Mat2  (entity, slot, value, ismust) => {
                    if let Some(mut item) = mat2.get_mut(entity) {
                        item.set(slot, value.as_slice());
                    } else {
                        cmds.0.push(cmd);
                    }
                },
                ECommand::Vec4  (entity, slot, value, ismust) => {
                    if let Some(mut item) = vec4.get_mut(entity) {
                        item.set(slot, value.as_slice());
                    } else {
                        cmds.0.push(cmd);
                    }
                },
                ECommand::Vec2  (entity, slot, value, ismust) => {
                    if let Some(mut item) = vec2.get_mut(entity) {
                        item.set(slot, value.as_slice());
                    } else {
                        cmds.0.push(cmd);
                    }
                },
                ECommand::Float (entity, slot, value, ismust) => {
                    if let Some(mut item) = float.get_mut(entity) {
                        item.set(slot, value);
                    } else {
                        cmds.0.push(cmd);
                    }
                },
                ECommand::Int   (entity, slot, value, ismust) => {
                    if let Some(mut item) = int.get_mut(entity) {
                        item.set(slot, value);
                    } else {
                        cmds.0.push(cmd);
                    }
                },
                ECommand::Uint  (entity, slot, value, ismust) => {
                    if let Some(mut item) = uint.get_mut(entity) {
                        item.set(slot, value);
                    } else {
                        cmds.0.push(cmd);
                    }
                },
            }
        });
    }
}

///
/// 材质单独与 GameObject 关联
/// Mesh 使用

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaterialID (pub ObjectID);

#[derive(Debug)]
pub enum MaterialIDCommand {
    Use(ObjectID, MaterialID),
    UnUse(ObjectID, MaterialID),
}

#[derive(Debug, Default)]
pub struct SingleMaterialIDCommandList {
    pub list: Vec<MaterialIDCommand>,
}

pub struct SysMaterialIDCommand;
impl TSystemStageInfo for SysMaterialIDCommand {

}
#[setup]
impl SysMaterialIDCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleMaterialIDCommandList>,
        mut material_cmd: Commands<GameObject, MaterialID>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                MaterialIDCommand::Use(obj, id) => {
                    material_cmd.insert(obj, id);
                },
                MaterialIDCommand::UnUse(mat, id) => {
                    
                },
            }
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UniformModifier {
    Mat4(usize, Matrix),
    Mat2(usize, Matrix2),
    Vec4(usize, Vector4),
    Vec2(usize, Vector2),
    Float(usize, Number),
    Int32(usize, i32),
    Uint32(usize, u32),
}

pub trait InterfaceMaterial {
    fn use_material(
        & self,
        object: ObjectID,
        material: ObjectID,
    );
    fn set_uniform_mat4(
        &self,
        entity: ObjectID,
        slot: usize,
        value: Matrix,
        force_apply: bool,
    ) -> &Self;
    fn set_uniform_mat2(
        &self,
        entity: ObjectID,
        slot: usize,
        value: Matrix2,
        force_apply: bool,
    ) -> &Self;
    fn set_uniform_vec4(
        &self,
        entity: ObjectID,
        slot: usize,
        value: Vector4,
        force_apply: bool,
    ) -> &Self;
    fn set_uniform_vec2(
        &self,
        entity: ObjectID,
        slot: usize,
        value: Vector2,
        force_apply: bool,
    ) -> &Self;
    fn set_uniform_float(
        &self,
        entity: ObjectID,
        slot: usize,
        value: Number,
        force_apply: bool,
    ) -> &Self;
    fn set_uniform_int(
        &self,
        entity: ObjectID,
        slot: usize,
        value: i32,
        force_apply: bool,
    ) -> &Self;
    fn set_uniform_uint(
        &self,
        entity: ObjectID,
        slot: usize,
        value: u32,
        force_apply: bool,
    ) -> &Self;
}

impl InterfaceMaterial for crate::engine::Engine {
    fn use_material(
        & self,
        object: ObjectID,
        material: ObjectID,
    ) {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleMaterialIDCommandList>().unwrap();
        commands.list.push(MaterialIDCommand::Use(object, MaterialID(material)));
    }

    fn set_uniform_mat4(
        &self,
        entity: ObjectID,
        slot: usize,
        value: Matrix,
        force_apply: bool,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleValueUniformCommands>().unwrap();
        commands.0.push(ECommand::Mat4(entity, slot, value, force_apply));

        self
    }

    fn set_uniform_mat2(
        &self,
        entity: ObjectID,
        slot: usize,
        value: Matrix2,
        force_apply: bool,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleValueUniformCommands>().unwrap();
        commands.0.push(ECommand::Mat2(entity, slot, value, force_apply));

        self
    }

    fn set_uniform_vec4(
        &self,
        entity: ObjectID,
        slot: usize,
        value: Vector4,
        force_apply: bool,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleValueUniformCommands>().unwrap();
        commands.0.push(ECommand::Vec4(entity, slot, value, force_apply));

        self
    }

    fn set_uniform_vec2(
        &self,
        entity: ObjectID,
        slot: usize,
        value: Vector2,
        force_apply: bool,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleValueUniformCommands>().unwrap();
        commands.0.push(ECommand::Vec2(entity, slot, value, force_apply));

        self
    }

    fn set_uniform_float(
        &self,
        entity: ObjectID,
        slot: usize,
        value: Number,
        force_apply: bool,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleValueUniformCommands>().unwrap();
        commands.0.push(ECommand::Float(entity, slot, value, force_apply));

        self
    }

    fn set_uniform_int(
        &self,
        entity: ObjectID,
        slot: usize,
        value: i32,
        force_apply: bool,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleValueUniformCommands>().unwrap();
        commands.0.push(ECommand::Int(entity, slot, value, force_apply));

        self
    }

    fn set_uniform_uint(
        &self,
        entity: ObjectID,
        slot: usize,
        value: u32,
        force_apply: bool,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleValueUniformCommands>().unwrap();
        commands.0.push(ECommand::Uint(entity, slot, value, force_apply));

        self
    }
}