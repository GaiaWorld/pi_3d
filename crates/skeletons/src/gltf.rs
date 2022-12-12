// use pi_3d_loader::interface::InterfaceGLTFLoader;

// struct GltfFactory {}

// impl GltfFactory for InterfaceGLTFLoader {
//     /// 创建 节点 - Node
//     /// * `scaling` - [f32, f32, f32] - scale
//     /// * `rotation` - [f32, f32, f32] - rotation (Euler Angle)
//     /// * `rotation_quaterion` - [f32, f32, f32, f32] - rotation (Quaterion)
//     /// * `matrix` - [f32; 16] - matrix
//     fn gltf_create_node<T: Clone>(
//         &self,
//         translation: Option<[f32; 3]>,
//         scaling: Option<[f32; 3]>,
//         rotation: Option<[f32; 3]>,
//         rotation_quaterion: Option<[f32; 4]>,
//         matrix: Option<[[f32; 4]; 4]>,
//     ) -> T {
//         1
//     }

//     /// 赋予节点 层级信息 - node.
//     /// TODO
//     fn gltf_layer_mask<T: Clone>(&self, entity: T, layer: u32){

//     }

//     /// 赋予节点 包围盒信息 -
//     /// * `center` `extend` - boundingbox
//     fn gltf_bounding_info<T: Clone>(&self, entity: T, min: [f32; 3], max: [f32; 3]){

//     }

//     /// 创建Buffer
//     fn gltf_create_buffer(&self, buffer_id: Atom, data: Vec<u8>){

//     }

//     /// 检查Buffer是否存在
//     fn gltf_check_buffer(&self, buffer_id: &Atom) -> bool{

//     }

//     /// 设置目标的网格描述
//     fn gltf_geometry<T: Clone>(&self, entity: T, desc: Vec<GLTFVertexBufferDesc>){

//     }

//     /// 使用 Verteices Buffer
//     fn gltf_apply_vertices_buffer<T: Clone>(
//         &self,
//         entity: T,
//         kind: EGLTFVertexDataKind,
//         buffer_id: Atom,
//         range: Range<wgpu::BufferAddress>,
//         format: wgpu::VertexFormat,
//     ){

//     }

//     /// 使用 Indeices Buffer
//     fn gltf_apply_indices_buffer<T: Clone>(
//         &self,
//         entity: T,
//         buffer_id: Atom,
//         range: Range<wgpu::BufferAddress>,
//         format: wgpu::IndexFormat,
//     );

//     /// 创建 纹理
//     fn gltf_create_texture(&mut self, path: Atom){

//     }

//     /// 设置材质指定纹理采样
//     fn gltf_material_texture_sampler<T: Clone>(
//         &mut self,
//         materialid: T,
//         texture_slot_name: Atom,
//         has_alpha: Option<bool>,
//         mag_filter: Option<u8>,
//         min_filter: Option<u8>,
//         wrap_u: Option<u8>,
//         wrap_v: Option<u8>,
//         format: wgpu::TextureFormat,
//     ){

//     }

//     /// 创建 基础材质
//     /// 1. 测试
//     /// 2. 没有实际支持的时候直接使用为DefaultMaterial
//     fn gltf_default_material<T: Clone>(&self) -> T{
//         1
//     }

//     /// 绑定材质
//     fn gltf_use_material<T: Clone, T1: Clone>(&self, entity: T, materialid: T1){

//     }

//     fn gltf_apply_skin<T: Clone>(&self, entity: T, bones: Vec<T>){

//     }
// }
