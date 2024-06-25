
use std::{collections::VecDeque, f32::consts::PI};

use pi_scene_shell::prelude::*;
use pi_scene_math::Vector3;


#[derive(Resource, Default)]
pub struct SingleBall(pub Option<Handle<EVertexBufferRange>>, pub Option<Handle<EVertexBufferRange>>);

pub struct BallBuilder;
impl BallBuilder {
    pub const KEY_BUFFER_COLOR4:    &'static str = "BallColor4";
    pub const KEY_BUFFER_POSITION:  &'static str = "BallPosition";
    pub const KEY_BUFFER_NORMAL:    &'static str = "BallNormal";
    pub const KEY_BUFFER_UV:        &'static str = "BallUV";
    pub const KEY_BUFFER_INDICES:   &'static str = "BallIndices";
    pub const KEY_BUFFER:           &'static str = "BallVertices";
    // const POSITION_OFFSET:      usize = 0;
    // const POSITION_SIZE:        usize = 72 * 4;

    pub fn attrs_meta() -> Vec<VertexBufferDesc> {
        let key = KeyVertexBuffer::from(Self::KEY_BUFFER);
        vec![
            VertexBufferDesc::vertices(
                key,
                VertexBufferDescRange::default(),
                vec![
                    EVertexAttribute::Buildin(EBuildinVertexAtribute::Position, wgpu::VertexFormat::Float32x3),
                    EVertexAttribute::Buildin(EBuildinVertexAtribute::Normal, wgpu::VertexFormat::Float32x3),
                    EVertexAttribute::Buildin(EBuildinVertexAtribute::UV, wgpu::VertexFormat::Float32x2),
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
}

#[derive(Clone, Copy)]
pub struct BallParam {
    pub sectors: usize,
    pub stacks: usize,
}

// pub trait InterfaceBall {
//     fn new_ball(&self, scene: ObjectID, sectors: usize, stacks: usize) -> ObjectID;
// }

// impl InterfaceBall for Engine {
//     fn new_ball(&self, scene: ObjectID, sectors: usize, stacks: usize) -> ObjectID {
//         let entity = self.new_object();
//         let world = self
//             .add_to_scene(entity, scene)
//             .as_transform_node(entity)
//             .transform_parent(entity, scene)
//             .as_mesh(entity)
//             .world();

//         let device = world.get_resource::<RenderDevice>().unwrap();
//         let queue = world.get_resource::<RenderQueue>().unwrap();
//         println!(">>>>>>>>>>>>>>>>>>>> 0");

//         let (positions, normals, indices, uvs) = generate_sphere(sectors, stacks);

//         println!(">>>>>>>>>>>>>>>>>>>> 1");
//         let flag = String::from("#") + sectors.to_string().as_str() + "#" + stacks.to_string().as_str();
//         let keypos = KeyVertexBuffer::from(String::from(BallBuilder::KEY_BUFFER_POSITION) + flag.as_str());
//         self.create_vertex_buffer(keypos.clone(), bytemuck::cast_slice(positions.as_slice()).iter().map(|v| *v).collect::<Vec<u8>>());

//         let keynormal = KeyVertexBuffer::from(String::from(BallBuilder::KEY_BUFFER_NORMAL) + flag.as_str());
//         self.create_vertex_buffer(keynormal.clone(), bytemuck::cast_slice(normals.as_slice()).iter().map(|v| *v).collect::<Vec<u8>>());
        
//         let keyuv = KeyVertexBuffer::from(String::from(BallBuilder::KEY_BUFFER_UV) + flag.as_str());
//         self.create_vertex_buffer(keyuv.clone(), bytemuck::cast_slice(uvs.as_slice()).iter().map(|v| *v).collect::<Vec<u8>>());

//         let key = KeyVertexBuffer::from(String::from(BallBuilder::KEY_BUFFER_INDICES) + flag.as_str());
//         self.create_vertex_buffer(key.clone(), bytemuck::cast_slice(indices.as_slice()).iter().map(|v| *v).collect::<Vec<u8>>());

//         self.use_geometry(
//             entity,
//             vec![
//                 VertexBufferDesc::vertices(keypos, None, vec![VertexAttribute { kind: EVertexDataKind::Position, format: wgpu::VertexFormat::Float32x3 }]),
//                 VertexBufferDesc::vertices(keynormal, None, vec![VertexAttribute { kind: EVertexDataKind::Normal, format: wgpu::VertexFormat::Float32x3 }]),
//                 VertexBufferDesc::vertices(keyuv, None, vec![VertexAttribute { kind: EVertexDataKind::UV, format: wgpu::VertexFormat::Float32x2 }]),
//             ],
//             Some(
//                 IndicesBufferDesc { format: wgpu::IndexFormat::Uint16, buffer_range: None, buffer: key }
//             )
//         );

//         entity
//     }
// }

pub struct PluginBallBuilder;
impl Plugin for PluginBallBuilder {
    // fn init(
    //     &mut self,
    //     engine: &mut Engine,
    //     stages: &mut pi_scene_shell::run_stage::RunStage,
    // ) -> Result<(), ErrorPlugin> {

    //     Ok(())
    // }

    fn build(&self, app: &mut App) {
        let asset_mgr = app.world.get_resource::<ShareAssetMgr<EVertexBufferRange>>().unwrap().clone();

        let device = app.world.get_resource::<PiRenderDevice>().unwrap().0.clone();
        let queue = app.world.get_resource::<PiRenderQueue>().unwrap().0.clone();
        let mut allocator = app.world.get_resource_mut::<VertexBufferAllocator3D>().unwrap();

        let (positions, normals, indces, uvs) = generate_sphere(&BallParam { sectors: 16, stacks: 16 });

        let mut singequad = SingleBall::default();

        let count = positions.len() / 3;
        let mut temp = Vec::with_capacity(count * (3 + 3 + 2));
        for i in 0..count {
            temp.push(positions[i * 3 + 0]); temp.push(positions[i * 3 + 1]); temp.push(positions[i * 3 + 2]);
            temp.push(normals[i * 3 + 0]); temp.push(normals[i * 3 + 1]); temp.push(normals[i * 3 + 2]);
            temp.push(uvs[i * 2 + 0]); temp.push(uvs[i * 2 + 1]);
        }

        let key = KeyVertexBuffer::from(BallBuilder::KEY_BUFFER);
        if let Some(bufferrange) = allocator.create_not_updatable_buffer(&device, &queue, &bytemuck::cast_slice(&temp).iter().map(|v| *v).collect::<Vec<u8>>(), None) {
            if let Ok(range) = asset_mgr.insert(key.asset_u64(), bufferrange) {
                singequad.0 = Some(range);
            }
        }
        let key = KeyVertexBuffer::from(BallBuilder::KEY_BUFFER_INDICES);
        if let Some(bufferrange) = allocator.create_not_updatable_buffer_for_index(&device, &queue, &bytemuck::cast_slice(&indces).iter().map(|v| *v).collect::<Vec<u8>>()) {
            if let Ok(range) = asset_mgr.insert(key.asset_u64(), bufferrange) {
                singequad.1 = Some(range);
            }
        }

        app.insert_resource(singequad);
    }
}

struct Triangle {
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3,
}

impl Triangle {
    pub fn compute_normal(&self) -> Vec<f32> {
        let a = self.a.normalize();
        let b = self.b.normalize();
        let c = self.c.normalize();
        [a.as_slice(), b.as_slice(), c.as_slice()].concat()
    }
}

/**
 * @brief 面细分法 八分之一个球
 * @param resolution 分辨率
 */
fn generate_sphere2(mut resolution: f32) -> (Vec<f32>, Vec<f32>, Vec<u32>, Vec<f32>) {
    let mut triangles = VecDeque::new();
    triangles.push_back(Triangle {
        a: Vector3::new(0.0, 1.0, 0.0),
        b: Vector3::new(0.0, 0.0, 1.0),
        c: Vector3::new(1.0, 0.0, 0.0),
    });

    // 这里平方一下，dist_square 的时候 就不用每次开平方了
    resolution *= resolution;
    loop {
        let dist = dist_square(triangles[0].a, triangles[0].b);
        // log::debug!("dist: {}", dist);
        // 当三角形各边长度都不大于resolution时就不再进一步细分
        if dist > resolution {
            let t = triangles.pop_front().unwrap();

            let d = mid_arc_point(t.a, t.b);
            let e = mid_arc_point(t.b, t.c);
            let f = mid_arc_point(t.c, t.a);

            triangles.push_back(Triangle { a: t.a, b: d, c: f });
            triangles.push_back(Triangle { a: d, b: t.b, c: e });
            triangles.push_back(Triangle { a: d, b: f, c: e });
            triangles.push_back(Triangle { a: f, b: e, c: t.c });
        } else {
            break;
        }
    }

    let len = triangles.len();
    // 每个三角形 3 个点，一共 8个象限
    let mut positions = Vec::with_capacity(len * 9 * 8);
    let mut normals = Vec::with_capacity(len * 9 * 8);
    let mut indices = Vec::with_capacity(len * 3 * 8);
    let mut uvs = Vec::with_capacity(len * 6 * 8);

    let mut index = 0;
    for tri in triangles {
        let start = index * 24;
        let mut data = [tri.a.as_slice(), tri.b.as_slice(), tri.c.as_slice()].concat();
        let mut normal = tri.compute_normal();
        let mut uv = compute_uv(&normal);
        // 第一象限
        positions.append(&mut data);
        normals.append(&mut normal);
        uvs.append(&mut uv);

        // 第二象限
        data[0] = -data[0];
        data[3] = -data[3];
        data[6] = -data[6];

        normal[0] = -normal[0];
        normal[3] = -normal[3];
        normal[6] = -normal[6];
        positions.append(&mut data);
        normals.append(&mut normal);

        let mut uv = compute_uv(&normal);
        uvs.append(&mut uv);

        // 第三象限
        data[1] = -data[1];
        data[4] = -data[4];
        data[7] = -data[7];

        normal[1] = -normal[1];
        normal[4] = -normal[4];
        normal[7] = -normal[7];
        positions.append(&mut data);
        normals.append(&mut normal);

        let mut uv = compute_uv(&normal);
        uvs.append(&mut uv);

        // 第四象限
        data[0] = -data[0];
        data[3] = -data[3];
        data[6] = -data[6];

        normal[0] = -normal[0];
        normal[3] = -normal[3];
        normal[6] = -normal[6];
        positions.append(&mut data);
        normals.append(&mut normal);

        let mut uv = compute_uv(&normal);
        uvs.append(&mut uv);

        // 第五象限
        data[2] = -data[2];
        data[5] = -data[5];
        data[8] = -data[8];

        normal[2] = -normal[2];
        normal[5] = -normal[5];
        normal[8] = -normal[8];
        positions.append(&mut data);
        normals.append(&mut normal);

        let mut uv = compute_uv(&normal);
        uvs.append(&mut uv);

        // 第六象限
        data[0] = -data[0];
        data[3] = -data[3];
        data[6] = -data[6];

        normal[0] = -normal[0];
        normal[3] = -normal[3];
        normal[6] = -normal[6];
        positions.append(&mut data);
        normals.append(&mut normal);

        let mut uv = compute_uv(&normal);
        uvs.append(&mut uv);

        // 第七象限
        data[1] = -data[1];
        data[4] = -data[4];
        data[7] = -data[7];

        normal[1] = -normal[1];
        normal[4] = -normal[4];
        normal[7] = -normal[7];
        positions.append(&mut data);
        normals.append(&mut normal);

        let mut uv = compute_uv(&normal);
        uvs.append(&mut uv);

        // 第八象限
        data[0] = -data[0];
        data[3] = -data[3];
        data[6] = -data[6];

        normal[0] = -normal[0];
        normal[3] = -normal[3];
        normal[6] = -normal[6];
        positions.append(&mut data);
        normals.append(&mut normal);

        let mut uv = compute_uv(&normal);
        uvs.append(&mut uv);

        for i in 0..8 * 3 {
            indices.push(start + i);
        }

        index += 1;
    }

    return (positions, normals, indices, uvs);
}

fn mid_arc_point(a: Vector3, b: Vector3) -> Vector3 {
    let mut c = a + b;
    let length = c.norm();
    c[0] /= length;
    c[1] /= length;
    c[2] /= length;

    c
}

fn dist_square(a: Vector3, b: Vector3) -> f32 {
    let c = a - b;
    return c[0] * c[0] + c[1] * c[1] + c[2] * c[2];
}

fn compute_uv(normalize: &[f32]) -> Vec<f32> {
    let au = normalize[0].atan2(normalize[2]) / (2.0 * PI) + 0.5;
    let av = normalize[2] * 0.5 + 0.5;

    let bu = normalize[3].atan2(normalize[5]) / (2.0 * PI) + 0.5;
    let bv = normalize[4] * 0.5 + 0.5;

    let cu = normalize[6].atan2(normalize[8]) / (2.0 * PI) + 0.5;
    let cv = normalize[7] * 0.5 + 0.5;

    vec![au, av, bu, bv, cu, cv]
}

/**
 * @brief 面细分法 经纬细分
 * @param sectors 分辨率
 */
pub fn generate_sphere(param: &BallParam) -> (Vec<f32>, Vec<f32>, Vec<u16>, Vec<f32>) {
    // Largely inspired from http://www.songho.ca/opengl/gl_sphere.html
    let radius = 0.5;
    let sectors: usize = param.sectors;
    let stacks: usize = param.stacks;

    let sectorsf32 = sectors as f32;
    let stacksf32 = stacks as f32;
    let length_inv = 1. / radius;
    let sector_step = 2. * PI / sectorsf32;
    let stack_step = PI / stacksf32;

    let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(stacks * sectors);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(stacks * sectors);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(stacks * sectors);
    let mut indices: Vec<u16> = Vec::with_capacity(stacks * sectors * 2 * 3);

    for i in 0..stacks + 1 {
        let stack_angle = PI / 2. - (i as f32) * stack_step;
        let xy = radius * stack_angle.cos();
        let z = radius * stack_angle.sin();

        for j in 0..sectors + 1 {
            let sector_angle = (j as f32) * sector_step;
            let x = xy * sector_angle.cos();
            let y = xy * sector_angle.sin();

            vertices.push([x, y, z]);
            normals.push([x * length_inv, y * length_inv, z * length_inv]);
            uvs.push([(j as f32) / sectorsf32, (i as f32) / stacksf32]);
        }
    }

    // indices
    //  k1--k1+1
    //  |  / |
    //  | /  |
    //  k2--k2+1
    for i in 0..stacks {
        let mut k1 = i * (sectors + 1);
        let mut k2 = k1 + sectors + 1;
        for _j in 0..sectors {
            if i != 0 {
                indices.push(k1 as u16);
                indices.push((k1 + 1) as u16);
                indices.push(k2 as u16);
            }
            if i != stacks - 1 {
                indices.push((k1 + 1) as u16);
                indices.push((k2 + 1) as u16);
                indices.push(k2 as u16);
            }
            k1 += 1;
            k2 += 1;
        }
    }

    // log::error!("Normals: {:?}", normals);
    return (vertices.concat(), normals.concat(), indices, uvs.concat());
}
