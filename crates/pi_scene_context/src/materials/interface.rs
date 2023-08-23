
use pi_engine_shell::prelude::*;

// use super::{
//     shader_effect::{ShaderEffectMeta, AssetKeyShaderEffect, AssetResShaderEffectMeta},
//     command::{EMaterialIDCommand},
//     material::MaterialID,
//     uniforms::sys_uniform::{SingleUniformCommands, EUniformCommand}
// };



// pub trait InterfaceMaterialMeta {
//     fn regist_material_meta(
//         &self,
//         key: KeyShaderMeta,
//         meta: ShaderEffectMeta,
//     ) -> &Self;
//     fn as_material(
//         &self,
//         entity: ObjectID,
//         shader: KeyShaderMeta,
//         pass: EPassTag,
//     ) -> &Self;
// }

// impl InterfaceMaterialMeta for EnginShell {
//     fn regist_material_meta(
//         &self,
//         key: KeyShaderMeta,
//         meta: ShaderEffectMeta,
//     ) -> &Self {
//         let world = self.world();

//         let asset_mgr = world.get_resource::<Share<AssetMgr<ShaderEffectMeta>>>().unwrap();
//         if !asset_mgr.check_asset(&key) {
//             let meta = asset_mgr.create_asset(key.clone(), meta);
//             let wait = world.get_resource_mut::<AssetSyncWait<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>>().unwrap();
//             wait.1.push((key.clone(), meta));
//         }

//         self
//     }

//     fn as_material(
//         &self,
//         entity: ObjectID,
//         shader: KeyShaderMeta,
//         pass: EPassTag,
//     ) -> &Self {
//         let world = self.world();
//         let commands = world.get_resource_mut::<SingleMatCreateCommands>().unwrap();
//         commands.0.push(EMatCreateCommand::Use(entity, shader, pass));
        
//         self
//     }
// }

pub trait InterfaceMaterial {
    fn use_material(
        & self,
        object: ObjectID,
        material: ObjectID,
    );
    fn set_texture(
        &self,
        entity: ObjectID,
        desc: UniformTextureWithSamplerParam,
        force_apply: bool,
    ) -> &Self;
    fn set_uniform_mat4(
        &self,
        entity: ObjectID,
        slot: usize,
        value: pi_scene_math::Matrix,
        force_apply: bool,
    ) -> &Self;
    fn set_uniform_mat2(
        &self,
        entity: ObjectID,
        slot: usize,
        value: pi_scene_math::Matrix2,
        force_apply: bool,
    ) -> &Self;
    fn set_uniform_vec4(
        &self,
        entity: ObjectID,
        slot: usize,
        value: pi_scene_math::Vector4,
        force_apply: bool,
    ) -> &Self;
    fn set_uniform_vec2(
        &self,
        entity: ObjectID,
        slot: usize,
        value: pi_scene_math::Vector2,
        force_apply: bool,
    ) -> &Self;
    fn set_uniform_float(
        &self,
        entity: ObjectID,
        slot: usize,
        value: pi_scene_math::Number,
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

// impl InterfaceMaterial for crate::engine::Engine {
//     fn use_material(
//         & self,
//         object: ObjectID,
//         material: ObjectID,
//     ) {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleMaterialIDCommandList>().unwrap();
//         commands.list.push(EMaterialIDCommand::Use(object, MaterialID(material)));
//     }

//     fn set_texture(
//         &self,
//         entity: ObjectID,
//         desc: UniformTextureWithSamplerParam,
//         force_apply: bool,
//     ) -> &Self {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleUniformCommands>().unwrap();
//         commands.0.push(EUniformCommand::Texture(entity, desc, force_apply));

//         self
//     }

//     fn set_uniform_mat4(
//         &self,
//         entity: ObjectID,
//         slot: usize,
//         value: pi_scene_math::Matrix,
//         force_apply: bool,
//     ) -> &Self {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleUniformCommands>().unwrap();
//         commands.0.push(EUniformCommand::Mat4(entity, slot, value, force_apply));

//         self
//     }

//     fn set_uniform_mat2(
//         &self,
//         entity: ObjectID,
//         slot: usize,
//         value: pi_scene_math::Matrix2,
//         force_apply: bool,
//     ) -> &Self {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleUniformCommands>().unwrap();
//         commands.0.push(EUniformCommand::Mat2(entity, slot, value, force_apply));

//         self
//     }

//     fn set_uniform_vec4(
//         &self,
//         entity: ObjectID,
//         slot: usize,
//         value: pi_scene_math::Vector4,
//         force_apply: bool,
//     ) -> &Self {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleUniformCommands>().unwrap();
//         commands.0.push(EUniformCommand::Vec4(entity, slot, value, force_apply));

//         self
//     }

//     fn set_uniform_vec2(
//         &self,
//         entity: ObjectID,
//         slot: usize,
//         value: pi_scene_math::Vector2,
//         force_apply: bool,
//     ) -> &Self {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleUniformCommands>().unwrap();
//         commands.0.push(EUniformCommand::Vec2(entity, slot, value, force_apply));

//         self
//     }

//     fn set_uniform_float(
//         &self,
//         entity: ObjectID,
//         slot: usize,
//         value: pi_scene_math::Number,
//         force_apply: bool,
//     ) -> &Self {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleUniformCommands>().unwrap();
//         commands.0.push(EUniformCommand::Float(entity, slot, value, force_apply));

//         self
//     }

//     fn set_uniform_int(
//         &self,
//         entity: ObjectID,
//         slot: usize,
//         value: i32,
//         force_apply: bool,
//     ) -> &Self {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleUniformCommands>().unwrap();
//         commands.0.push(EUniformCommand::Int(entity, slot, value, force_apply));

//         self
//     }

//     fn set_uniform_uint(
//         &self,
//         entity: ObjectID,
//         slot: usize,
//         value: u32,
//         force_apply: bool,
//     ) -> &Self {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleUniformCommands>().unwrap();
//         commands.0.push(EUniformCommand::Uint(entity, slot, value, force_apply));

//         self
//     }
// }