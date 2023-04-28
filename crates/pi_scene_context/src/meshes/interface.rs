use pi_engine_shell::prelude::*;
use pi_scene_math::Vector4;


pub trait InterfaceMesh {
    fn create_mesh(
        & self,
        scene: ObjectID,
    ) -> ObjectID;

    fn as_mesh(
        & self,
        object: ObjectID,
    ) -> & Self;
    
    fn create_instanced_mesh(
        & self,
        scene: ObjectID,
        source: ObjectID,
    ) -> ObjectID;

    fn set_instance_color(
        & self,
        instance: ObjectID,
        color: Vector4,
    ) -> &Self;
    
    fn set_instance_tilloff(
        & self,
        instance: ObjectID,
        value: Vector4,
    ) -> &Self;

    fn cast_shadow(
        &self,
        instance: ObjectID,
        value: bool,
    ) -> &Self;
    
    fn receive_shadow(
        &self,
        instance: ObjectID,
        value: bool,
    ) -> &Self;
}
// impl InterfaceMesh for crate::engine::Engine {
//     fn create_mesh(
//         & self,
//         scene: ObjectID,
//     ) -> ObjectID {

//         let entity = self.new_object();

//         self.add_to_scene(entity, scene);
//         self.as_transform_node(entity);
//         self.transform_parent(entity, scene);

//         self.as_mesh(entity);

//         entity
//     }
    
//     fn create_instanced_mesh(
//         & self,
//         scene: ObjectID,
//         source: ObjectID,
//     ) -> ObjectID {
//         let world = self.world();

//         let entity = self.new_object();

//         self.add_to_scene(entity, scene);
//         self.as_transform_node(entity);
//         self.transform_parent(entity, scene);

//         let commands = world.get_resource_mut::<SingleInstanceMeshCreateCommandList>().unwrap();
//         commands.list.push(EInstanceMeshCreateCommand::CreateInstance(source, entity));

//         entity
//     }
    
//     fn set_instance_color(
//         & self,
//         instance: ObjectID,
//         color: Vector4,
//     ) -> &Self {
//         let commands = self.world().get_resource_mut::<SingleInstanceMeshModifyCommandList>().unwrap();
//         commands.list.push(EInstanceMeshModifyCommand::InstanceColor(instance, color));

//         self
//     }
    
//     fn set_instance_tilloff(
//         & self,
//         instance: ObjectID,
//         value: Vector4,
//     ) -> &Self {
//         let commands = self.world().get_resource_mut::<SingleInstanceMeshModifyCommandList>().unwrap();
//         commands.list.push(EInstanceMeshModifyCommand::InstanceTillOff(instance, value));

//         self
//     }

//     fn as_mesh(
//         & self,
//         object: ObjectID,
//     ) -> & Self {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleMeshCreateCommandList>().unwrap();
//         commands.list.push(EMeshCreateCommand::Create(object));

//         self.render_sort(object, TransparentSortParam::opaque());
//         self.render_mode(object, ERenderMode::Opaque);
//         self.disable_blend(object);
//         self.disable_depth_stencil(object);
//         self.layer_mask(object, LayerMask::default());
//         self.cull_mode(object, ECullMode::Back);
//         self.polygon_mode(object, EPolygonMode::Fill);
//         self.front_face(object, EFrontFace::Ccw);

//         self
//     }

//     fn cast_shadow(
//         &self,
//         instance: ObjectID,
//         value: bool,
//     ) -> &Self {
//         todo!()
//     }

//     fn receive_shadow(
//         &self,
//         instance: ObjectID,
//         value: bool,
//     ) -> &Self {
//         todo!()
//     }
// }