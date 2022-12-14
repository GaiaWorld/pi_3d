
use std::{collections::VecDeque, f32::consts::PI};

use pi_assets::mgr::AssetMgr;
use pi_engine_shell::{object::InterfaceObject, assets::sync_load::{InterfaceAssetSyncCreate, AssetSyncWait}};
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use pi_scene_math::Vector3;
use pi_share::Share;
use render_data_container::{VertexBuffer, EVertexDataFormat, KeyVertexBuffer};
use render_geometry::{indices::{AssetKeyBufferIndices, AssetResBufferIndices, IndicesBufferDesc}, vertex_data::{VertexBufferDesc, VertexAttribute, EVertexDataKind}};

use crate::{
    plugin::{Plugin, ErrorPlugin},
    object::{ObjectID},
    engine::Engine, 
    scene::{ interface::InterfaceScene},
    transforms::interface::InterfaceTransformNode, geometry::{TInterfaceGeomtery, GeometryDesc, indices::InterfaceBufferIndices}
};

use super::interface::InterfaceMesh;


pub struct BallBuilder;
impl BallBuilder {
    const KEY_BUFFER_COLOR4:    &'static str = "BallColor4";
    const KEY_BUFFER_POSITION:  &'static str = "BallPosition";
    const KEY_BUFFER_NORMAL:    &'static str = "BallNormal";
    const KEY_BUFFER_UV:        &'static str = "BallUV";
    const KEY_BUFFER_INDICES:   &'static str = "BallIndices";
    pub fn position(
        device: &RenderDevice,
        queue: &RenderQueue,
        data: &[f32],
    ) -> VertexBuffer {
        let mut position = VertexBuffer::new(false, EVertexDataFormat::F32, false);

        let len = data.len();
        position.update_f32(&data, 0);
        position.update_buffer(device, queue);
        position
    }
    pub fn normal(
        device: &RenderDevice,
        queue: &RenderQueue,
        data: &[f32],
    ) -> VertexBuffer {
        let len = data.len();

        let mut normals = VertexBuffer::new(false, EVertexDataFormat::F32, false);
        normals.update_f32(&data, 0);
        normals.update_buffer(device, queue);
        normals
    }
    pub fn indices(
        device: &RenderDevice,
        queue: &RenderQueue,
        data: &[u16],
    ) -> VertexBuffer {
        let len = data.len();
        let mut indices = VertexBuffer::new(false, EVertexDataFormat::U16, true);
        indices.update_u16(&data, 0);
        indices.update_buffer(device, queue);
        indices
    }

    pub fn uv(
        device: &RenderDevice,
        queue: &RenderQueue,
        data: &[f32],
    ) -> VertexBuffer {
        let len = data.len();
        let mut uvs = VertexBuffer::new(false, EVertexDataFormat::F32, false);
        uvs.update_f32(&data, 0);
        uvs.update_buffer(device, queue);
        uvs
    }
}

pub trait InterfaceBall {
    fn new_ball(&self, scene: ObjectID, sectors: usize, stacks: usize) -> ObjectID;
}

impl InterfaceBall for Engine {
    fn new_ball(&self, scene: ObjectID, sectors: usize, stacks: usize) -> ObjectID {
        let entity = self.new_object();
        let world = self
            .add_to_scene(entity, scene)
            .as_transform_node(entity)
            .transform_parent(entity, scene)
            .as_mesh(entity)
            .world();

        let device = world.get_resource::<RenderDevice>().unwrap();
        let queue = world.get_resource::<RenderQueue>().unwrap();

        let (positions, normals, indices, uvs) = generate_sphere(sectors, stacks);

        let flag = String::from("#") + sectors.to_string().as_str() + "#" + stacks.to_string().as_str();
        let keypos = KeyVertexBuffer::from(String::from(BallBuilder::KEY_BUFFER_POSITION) + flag.as_str());
        self.create_vertex_buffer(keypos.clone(), BallBuilder::position(device, queue, positions.as_slice()));

        let keynormal = KeyVertexBuffer::from(String::from(BallBuilder::KEY_BUFFER_NORMAL) + flag.as_str());
        self.create_vertex_buffer(keynormal.clone(), BallBuilder::normal(device, queue, normals.as_slice()));
        
        let keyuv = KeyVertexBuffer::from(String::from(BallBuilder::KEY_BUFFER_UV) + flag.as_str());
        self.create_vertex_buffer(keyuv.clone(), BallBuilder::uv(device, queue, uvs.as_slice()));

        let key = KeyVertexBuffer::from(String::from(BallBuilder::KEY_BUFFER_INDICES) + flag.as_str());
        self.create_vertex_buffer(key.clone(), BallBuilder::indices(device, queue, indices.as_slice()));

        self.use_geometry(
            entity,
            vec![
                VertexBufferDesc::vertices(keypos, None, vec![VertexAttribute { kind: EVertexDataKind::Position, format: wgpu::VertexFormat::Float32x3 }]),
                VertexBufferDesc::vertices(keynormal, None, vec![VertexAttribute { kind: EVertexDataKind::Normal, format: wgpu::VertexFormat::Float32x3 }]),
                VertexBufferDesc::vertices(keyuv, None, vec![VertexAttribute { kind: EVertexDataKind::UV, format: wgpu::VertexFormat::Float32x2 }]),
            ]
        );
        self.use_indices(entity, IndicesBufferDesc { format: wgpu::IndexFormat::Uint16, buffer_range: None, buffer: key });

        entity
    }
}

pub struct PluginBallBuilder;
impl Plugin for PluginBallBuilder {
    fn init(
        &mut self,
        engine: &mut Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {

        Ok(())
    }
}

#[derive(Debug)]
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
 * @brief ???????????? ??????????????????
 * @param resolution ?????????
 */
fn generate_sphere2(mut resolution: f32) -> (Vec<f32>, Vec<f32>, Vec<u32>, Vec<f32>) {
    let mut triangles = VecDeque::new();
    triangles.push_back(Triangle {
        a: Vector3::new(0.0, 1.0, 0.0),
        b: Vector3::new(0.0, 0.0, 1.0),
        c: Vector3::new(1.0, 0.0, 0.0),
    });

    // ?????????????????????dist_square ????????? ???????????????????????????
    resolution *= resolution;
    loop {
        let dist = dist_square(triangles[0].a, triangles[0].b);
        println!("dist: {}", dist);
        // ????????????????????????????????????resolution???????????????????????????
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
    // ??????????????? 3 ??????????????? 8?????????
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
        // ????????????
        positions.append(&mut data);
        normals.append(&mut normal);
        uvs.append(&mut uv);

        // ????????????
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

        // ????????????
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

        // ????????????
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

        // ????????????
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

        // ????????????
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

        // ????????????
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

        // ????????????
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
 * @brief ???????????? ????????????
 * @param sectors ?????????
 */
fn generate_sphere(sectors: usize, stacks: usize) -> (Vec<f32>, Vec<f32>, Vec<u16>, Vec<f32>) {
    // Largely inspired from http://www.songho.ca/opengl/gl_sphere.html
    let radius = 1.0;

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
                indices.push(k2 as u16);
                indices.push((k1 + 1) as u16);
            }
            if i != stacks - 1 {
                indices.push((k1 + 1) as u16);
                indices.push(k2 as u16);
                indices.push((k2 + 1) as u16);
            }
            k1 += 1;
            k2 += 1;
        }
    }

    return (vertices.concat(), normals.concat(), indices, uvs.concat());
}
