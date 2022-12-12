
use std::mem::replace;

use derive_deref::{Deref, DerefMut};
use pi_ecs::{prelude::{Id, ResMut, Query}, query::{Write, With}};
use pi_ecs_macros::setup;
use pi_scene_math::{Number, Matrix, Vector4, Vector2, Matrix2};
use render_shader::shader::{KeyShaderEffect, ResShader, KeyShader};

use crate::{object::{ObjectID, GameObject}, renderers::render_mode::ERenderMode};

use super::uniforms::{mat4::Mat4Uniform, mat2::Mat2Uniform, vec4::Vec4Uniform, vec2::Vec2Uniform, float::FloatUniform, int::IntUniform, uint::UintUniform};

pub trait TMaterial {
    fn render_mode(&self) -> ERenderMode;
}
#[derive(Debug)]
enum ECommand {
    Mat4(ObjectID, usize, Matrix, bool),
    Mat2(ObjectID, usize, Matrix2, bool),
    Vec4(ObjectID, usize, Vector4, bool),
    Vec2(ObjectID, usize, Vector2, bool),
    Float(ObjectID, usize, Number, bool),
    Int(ObjectID, usize, i32, bool),
    Uint(ObjectID, usize, u32, bool),
}

#[derive(Debug, Default)]
pub struct SingleValueUniformCommands(Vec<ECommand>);
pub struct SysValueUniformComand;
#[setup]
impl SysValueUniformComand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleValueUniformCommands>,
        mut mat4: Query<GameObject, Write<Mat4Uniform>, With<Mat4Uniform>>,
        mut mat2: Query<GameObject, Write<Mat2Uniform>, With<Mat2Uniform>>,
        mut vec4: Query<GameObject, Write<Vec4Uniform>, With<Vec4Uniform>>,
        mut vec2: Query<GameObject, Write<Vec2Uniform>, With<Vec2Uniform>>,
        mut float: Query<GameObject, Write<FloatUniform>, With<FloatUniform>>,
        mut int: Query<GameObject, Write<IntUniform>, With<IntUniform>>,
        mut uint: Query<GameObject, Write<UintUniform>, With<UintUniform>>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Mat4  (entity, slot, value, ismust) => {
                    if let Some(mut item) = mat4.get_mut(entity) {
                        if let Some(mut modify) = item.get_mut() {
                            modify.set(slot, value.as_slice());
                            item.notify_modify();
                        } else if ismust {
                            cmds.0.push(cmd);
                        }
                    }
                },
                ECommand::Mat2  (entity, slot, value, ismust) => {
                    if let Some(mut item) = mat2.get_mut(entity) {
                        if let Some(mut modify) = item.get_mut() {
                            modify.set(slot, value.as_slice());
                            item.notify_modify();
                        } else if ismust {
                            cmds.0.push(cmd);
                        }
                    }
                },
                ECommand::Vec4  (entity, slot, value, ismust) => {
                    if let Some(mut item) = vec4.get_mut(entity) {
                        if let Some(mut modify) = item.get_mut() {
                            modify.set(slot, value.as_slice());
                            item.notify_modify();
                        } else if ismust {
                            cmds.0.push(cmd);
                        }
                    }
                },
                ECommand::Vec2  (entity, slot, value, ismust) => {
                    if let Some(mut item) = vec2.get_mut(entity) {
                        if let Some(mut modify) = item.get_mut() {
                            modify.set(slot, value.as_slice());
                            item.notify_modify();
                        } else if ismust {
                            cmds.0.push(cmd);
                        }
                    }
                },
                ECommand::Float (entity, slot, value, ismust) => {
                    if let Some(mut item) = float.get_mut(entity) {
                        if let Some(mut modify) = item.get_mut() {
                            modify.set(slot, value);
                            item.notify_modify();
                        } else if ismust {
                            cmds.0.push(cmd);
                        }
                    }
                },
                ECommand::Int   (entity, slot, value, ismust) => {
                    if let Some(mut item) = int.get_mut(entity) {
                        if let Some(mut modify) = item.get_mut() {
                            modify.set(slot, value);
                            item.notify_modify();
                        } else if ismust {
                            cmds.0.push(cmd);
                        }
                    }
                },
                ECommand::Uint  (entity, slot, value, ismust) => {
                    if let Some(mut item) = uint.get_mut(entity) {
                        if let Some(mut modify) = item.get_mut() {
                            modify.set(slot, value);
                            item.notify_modify();
                        } else if ismust {
                            cmds.0.push(cmd);
                        }
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
#[setup]
impl SysMaterialIDCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleMaterialIDCommandList>,
        mut materials: Query<GameObject, Write<MaterialID>>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                MaterialIDCommand::Use(mat, id) => {
                    match materials.get_mut(mat) {
                        Some(mut mat) => {
                            mat.write(id);
                        },
                        None => todo!(),
                    }
                },
                MaterialIDCommand::UnUse(mat, id) => {
                    
                },
            }
        });
    }
}

pub trait InterfaceMaterial {
    fn use_material(
        & self,
        object: ObjectID,
        material: MaterialID,
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
        material: MaterialID,
    ) {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleMaterialIDCommandList>().unwrap();
        commands.list.push(MaterialIDCommand::Use(object, material));
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