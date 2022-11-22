use std::{collections::VecDeque, f32::consts::PI};

use pi_engine_shell::object::InterfaceObject;
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use pi_scene_math::Vector3;
use render_data_container::{EVertexDataFormat, GeometryBuffer, GeometryBufferPool};
use render_geometry::geometry::VertexAttributeBufferMeta;

use crate::{
    engine::Engine,
    object::ObjectID,
    plugin::{ErrorPlugin, Plugin},
    resources::SingleGeometryBufferPool,
    scene::interface::InterfaceScene,
    transforms::interface::InterfaceTransformNode,
    vertex_data::{
        indices::{
            AttributeIndices, AttributeIndicesCommand, IDAttributeIndices,
            IDAttributeIndicesCommand, SingleAttributeIndicesCommandList,
            SingleIDAttributeIndicesCommandList,
        },
        normal::{
            AttributeNormal, AttributeNormalCommand, IDAttributeNormal, IDAttributeNormalCommand,
            SingleAttributeNormalCommandList, SingleIDAttributeNormalCommandList,
        },
        position::{
            AttributePosition, AttributePositionCommand, IDAttributePosition,
            IDAttributePositionCommand, SingleAttributePositionCommandList,
            SingleIDAttributePositionCommandList,
        },
        uv::{AttributeUV, AttributeUVCommand, IDAttributeUV, SingleAttributeUVCommandList},
    },
};

use super::interface::InterfaceMesh;

pub struct SingleBaseBall {
    position: IDAttributePosition,
    normal: IDAttributeNormal,
    indices: IDAttributeIndices,
    uv: IDAttributeUV,
}
impl SingleBaseBall {
    pub fn position(&self) -> IDAttributePosition {
        self.position
    }
    pub fn normal(&self) -> IDAttributeNormal {
        self.normal
    }
    pub fn indices(&self) -> IDAttributeIndices {
        self.indices
    }
}

pub struct BallBuilder;
impl BallBuilder {
    pub fn position(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
        data: &[f32],
    ) -> AttributePosition {
        let mut position = GeometryBuffer::new(true, EVertexDataFormat::F32, false);

        let len = data.len();
        position.update_f32(&data, 0);
        position.update_buffer(device, queue);
        let id_position = gbp.insert(position);

        AttributePosition {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_position,
                start: 0,
                end: len * 4,
                data_bytes_size: 3 * 4,
                data_count: len / 3,
            },
        }
    }
    pub fn normal(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
        data: &[f32],
    ) -> AttributeNormal {
        let len = data.len();

        let mut normals = GeometryBuffer::new(true, EVertexDataFormat::F32, false);
        normals.update_f32(&data, 0);
        normals.update_buffer(device, queue);
        let id_normal = gbp.insert(normals);

        AttributeNormal {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_normal,
                start: 0,
                end: len * 4,
                data_bytes_size: 3 * 4,
                data_count: len / 3,
            },
        }
    }
    pub fn indices(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
        data: &[u32],
    ) -> AttributeIndices {
        let len = data.len();
        let mut indices = GeometryBuffer::new(true, EVertexDataFormat::U32, true);
        indices.update_u32(&data, 0);
        indices.update_buffer(device, queue);
        let id_indices = gbp.insert(indices);

        AttributeIndices {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_indices,
                start: 0,
                end: len * 4,
                data_bytes_size: 1 * 4,
                data_count: len,
            },
            format: wgpu::IndexFormat::Uint32,
        }
    }

    pub fn uv(
        device: &RenderDevice,
        queue: &RenderQueue,
        gbp: &mut SingleGeometryBufferPool,
        data: &[f32],
    ) -> AttributeUV {
        let len = data.len();
        let mut uvs = GeometryBuffer::new(true, EVertexDataFormat::F32, true);
        uvs.update_f32(&data, 0);
        uvs.update_buffer(device, queue);
        let id_uv = gbp.insert(uvs);

        AttributeUV {
            meta: VertexAttributeBufferMeta {
                buffer_id: id_uv,
                start: 0,
                end: len * 2 * 4,
                data_bytes_size: 2 * 4,
                data_count: len / 2,
            },
        }
    }
}

pub enum BallBuilderCommand {
    Base(
        ObjectID,
        IDAttributePosition,
        IDAttributeNormal,
        IDAttributeIndices,
    ),
}

pub struct SingleBallBuilderCommandList {
    pub list: Vec<BallBuilderCommand>,
}
pub trait InterfaceBall {
    fn new_ball(&self, scene: ObjectID) -> ObjectID;
}

impl InterfaceBall for Engine {
    fn new_ball(&self, scene: ObjectID) -> ObjectID {
        let entity = self.new_object();
        let world = self
            .add_to_scene(entity, scene)
            .as_transform_node(entity)
            .transform_parent(entity, scene)
            .as_mesh(entity)
            .world();

        let base_Ball = world.get_resource_mut::<SingleBaseBall>().unwrap();
        let commands = world
            .get_resource_mut::<SingleIDAttributePositionCommandList>()
            .unwrap();
        commands.list.push(IDAttributePositionCommand::Create(
            entity,
            base_Ball.position(),
        ));
        let commands = world
            .get_resource_mut::<SingleIDAttributeNormalCommandList>()
            .unwrap();
        commands
            .list
            .push(IDAttributeNormalCommand::Create(entity, base_Ball.normal()));
        let commands = world
            .get_resource_mut::<SingleIDAttributeIndicesCommandList>()
            .unwrap();
        commands.list.push(IDAttributeIndicesCommand::Create(
            entity,
            base_Ball.indices(),
        ));

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
        let position_id = engine.new_object();
        let normal_id = engine.new_object();
        let indices_id = engine.new_object();
        let uv_id = engine.new_object();

        let world = engine.world_mut();

        let device = world.get_resource::<RenderDevice>().unwrap();
        let queue = world.get_resource::<RenderQueue>().unwrap();
        let gbp = world
            .get_resource_mut::<SingleGeometryBufferPool>()
            .unwrap();

        let (positions, normals, indices, uvs) = generate_sphere(36, 18);

        let position = BallBuilder::position(device, queue, gbp, &positions);
        let normal = BallBuilder::normal(device, queue, gbp, &normals);
        let indices = BallBuilder::indices(device, queue, gbp, &indices);
        let uvs = BallBuilder::uv(device, queue, gbp, &uvs);

        let commands = world
            .get_resource_mut::<SingleAttributePositionCommandList>()
            .unwrap();
        commands
            .list
            .push(AttributePositionCommand::Create(position_id, position));

        let commands = world
            .get_resource_mut::<SingleAttributeNormalCommandList>()
            .unwrap();
        commands
            .list
            .push(AttributeNormalCommand::Create(normal_id, normal));

        let commands = world
            .get_resource_mut::<SingleAttributeIndicesCommandList>()
            .unwrap();
        commands
            .list
            .push(AttributeIndicesCommand::Create(indices_id, indices));

        let commands = world
            .get_resource_mut::<SingleAttributeUVCommandList>()
            .unwrap();
        commands.list.push(AttributeUVCommand::Create(uv_id, uvs));

        world.insert_resource::<SingleBaseBall>(SingleBaseBall {
            position: IDAttributePosition(position_id),
            normal: IDAttributeNormal(normal_id),
            indices: IDAttributeIndices(indices_id),
            uv: IDAttributeUV(uv_id),
        });

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
        println!("dist: {}", dist);
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
fn generate_sphere(sectors: usize, stacks: usize) -> (Vec<f32>, Vec<f32>, Vec<u32>, Vec<f32>) {
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
    let mut indices: Vec<u32> = Vec::with_capacity(stacks * sectors * 2 * 3);

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
                indices.push(k1 as u32);
                indices.push(k2 as u32);
                indices.push((k1 + 1) as u32);
            }
            if i != stacks - 1 {
                indices.push((k1 + 1) as u32);
                indices.push(k2 as u32);
                indices.push((k2 + 1) as u32);
            }
            k1 += 1;
            k2 += 1;
        }
    }

    return (vertices.concat(), normals.concat(), indices, uvs.concat());
}
