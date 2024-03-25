
use pi_scene_shell::prelude::*;

pub struct QuadBuilder;
impl QuadBuilder {
    // const KEY_BUFFER_COLOR4:    &'static str = "QuadColor4";
    // const KEY_BUFFER_POSITION:  &'static str = "QuadPosition";
    // const KEY_BUFFER_NORMAL:    &'static str = "QuadNormal";
    // const KEY_BUFFER_UV:        &'static str = "QuadUV";
    const KEY_BUFFER_INDICES:   &'static str = "QuadIndices";
    const KEY_BUFFER:           &'static str = "QuadBuildin";
    // const KEY_BUFFER_COLOR4:    IDAssetVertexBuffer = -2005;
    // const KEY_BUFFER_POSITION:  IDAssetVertexBuffer = -2004;
    // const KEY_BUFFER_NORMAL:    IDAssetVertexBuffer = -2003;
    // const KEY_BUFFER_UV:        IDAssetVertexBuffer = -2002;
    // const KEY_BUFFER_INDICES:   IDAssetVertexBuffer = -2001;
    // const KEY_BUFFER:           IDAssetVertexBuffer = -2000;
    pub fn attrs_meta() -> Vec<VertexBufferDesc> {
        let keypos = KeyVertexBuffer::from(Self::KEY_BUFFER);
        vec![
            VertexBufferDesc::vertices(
                keypos, 
                VertexBufferDescRange::default(),
                vec![
                    EVertexAttribute::Buildin(EBuildinVertexAtribute::Position),
                    EVertexAttribute::Buildin(EBuildinVertexAtribute::Normal),
                    EVertexAttribute::Buildin(EBuildinVertexAtribute::UV),
                    // VertexAttribute { kind: EVertexDataKind::Position, format: wgpu::VertexFormat::Float32x3 },
                    // VertexAttribute { kind: EVertexDataKind::Normal, format: wgpu::VertexFormat::Float32x3 },
                    // VertexAttribute { kind: EVertexDataKind::UV, format: wgpu::VertexFormat::Float32x2 }
                ]
            ),
        ]
    }
    pub fn indices_meta() -> IndicesBufferDesc {
        let key = KeyVertexBuffer::from(Self::KEY_BUFFER_INDICES);
        IndicesBufferDesc { format: wgpu::IndexFormat::Uint16, buffer_range: None, buffer: key }
    }
    pub fn position() -> [f32; 12] {
        [
            -0.5, -0.5, 0.,   
            0.5, -0.5, 0.,   
            0.5, 0.5, 0.,  
            -0.5, 0.5, 0.,  
        ]
    }
    pub fn normal() -> [f32; 12] {
        [
            0., 0., 1.,     
            0., 0., 1.,     
            0., 0., 1.,     
            0., 0., 1.,
        ]
    }
    pub fn vertices() -> [f32; 32] {
        [
            -0.5, -0.5,  0.,      0., 0., 1.,     0., 0.,    
            0.5, -0.5,  0.,      0., 0., 1.,     1., 0.,   
            0.5,  0.5,  0.,      0., 0., 1.,     1., 1.,     
            -0.5, 0.5,  0.,      0., 0., 1.,     0., 1.,
        ]
    }
    pub fn indices() -> [u16;6] {
        [
            0, 1, 2, 0, 2, 3
        ]
    }
    pub fn uvs() -> [f32; 8] {
        [
            0., 0.,   
            1., 0.,   
            1., 1.,     
            0., 1.,
        ]
    }
}

// pub struct ActionQuad;
// impl ActionQuad {
//     pub fn create(
//         app: &mut App,
//         scene: Entity,
//         name: String,
//     ) -> Entity {
//         let mut queue = CommandQueue::default();
//         let mut commands = Commands::new(&mut queue, &app.world);
//         let id_geo = commands.spawn_empty().id();

//         let id_mesh = ActionMesh::create(app, scene, name);
//         ActionMesh::use_geometry(app, id_mesh, QuadBuilder::attrs_meta(), Some(QuadBuilder::indices_meta()));

//         id_mesh
//     }
// }

// fn regist(
//     asset_mgr: Res<ShareAssetMgr<EVertexBufferRange>>,
//     mut data_map: ResMut<VertexBufferDataMap3D>,
// ) {
//     if !ActionVertexBuffer::check(&asset_mgr, KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER)) {
//         ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER), bytemuck::cast_slice(&QuadBuilder::vertices()).iter().map(|v| *v).collect::<Vec<u8>>());
//     }
//     if !ActionVertexBuffer::check(&asset_mgr, KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_INDICES)) {
//         ActionVertexBuffer::create_indices(&mut data_map, KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_INDICES), bytemuck::cast_slice(&QuadBuilder::indices()).iter().map(|v| *v).collect::<Vec<u8>>());
//     }
// }

// pub trait InterfaceQuad {
//     fn regist_quad(
//         &self
//     ) -> &Self;
//     fn new_quad(
//         & self,
//         scene: ObjectID,
//     ) -> ObjectID;
// }

// impl InterfaceQuad for Engine {
//     fn regist_quad(
//         &self
//     ) -> &Self {
//         self.create_vertex_buffer(KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER), bytemuck::cast_slice(&QuadBuilder::vertices()).iter().map(|v| *v).collect::<Vec<u8>>());
//         self.create_vertex_buffer(KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_INDICES), bytemuck::cast_slice(&QuadBuilder::indices()).iter().map(|v| *v).collect::<Vec<u8>>());

//         self
//     }
//     fn new_quad(
//         & self,
//         scene: ObjectID,
//     ) -> ObjectID {

//         let entity = self.new_object();
//         self.add_to_scene(entity, scene)
//                                     .as_transform_node(entity)
//                                     .transform_parent(entity, scene)
//                                     .as_mesh(entity);
        
//         self.use_geometry(
//             entity,
//             QuadBuilder::attrs_meta(),
//             Some(QuadBuilder::indices_meta())
//         );

//         entity
//     }
// }

#[derive(Resource, Default)]
pub struct SingleQuad(pub Option<Handle<EVertexBufferRange>>, pub Option<Handle<EVertexBufferRange>>);

pub struct PluginQuadBuilder;
impl Plugin for PluginQuadBuilder {
    fn build(&self, app: &mut App) {
        let asset_mgr = app.world.get_resource::<ShareAssetMgr<EVertexBufferRange>>().unwrap().clone();
        // let mut data_map = app.world.get_resource_mut::<VertexBufferDataMap3D>().unwrap();
        // if !ActionVertexBuffer::check(&asset_mgr, KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER)) {
        //     ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER), bytemuck::cast_slice(&QuadBuilder::vertices()).iter().map(|v| *v).collect::<Vec<u8>>());
        // }
        // if !ActionVertexBuffer::check(&asset_mgr, KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_INDICES)) {
        //     ActionVertexBuffer::create_indices(&mut data_map, KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_INDICES), bytemuck::cast_slice(&QuadBuilder::indices()).iter().map(|v| *v).collect::<Vec<u8>>());
        // }

        let device = app.world.get_resource::<PiRenderDevice>().unwrap().0.clone();
        let queue = app.world.get_resource::<PiRenderQueue>().unwrap().0.clone();
        let mut allocator = app.world.get_resource_mut::<VertexBufferAllocator3D>().unwrap();
        let mut singequad = SingleQuad::default();
        let key = KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER);
        if let Some(bufferrange) = allocator.create_not_updatable_buffer(&device, &queue, &bytemuck::cast_slice(&QuadBuilder::vertices()).iter().map(|v| *v).collect::<Vec<u8>>(), None) {
            if let Ok(range) = asset_mgr.insert(key.asset_u64(), bufferrange) {
                singequad.0 = Some(range);
            }
        }
        let key = KeyVertexBuffer::from(QuadBuilder::KEY_BUFFER_INDICES);
        if let Some(bufferrange) = allocator.create_not_updatable_buffer_for_index(&device, &queue, &bytemuck::cast_slice(&QuadBuilder::indices()).iter().map(|v| *v).collect::<Vec<u8>>()) {
            if let Ok(range) = asset_mgr.insert(key.asset_u64(), bufferrange) {
                singequad.1 = Some(range);
            }
        }
        app.insert_resource(singequad);
        // app.add_startup_system(regist);
    }
}