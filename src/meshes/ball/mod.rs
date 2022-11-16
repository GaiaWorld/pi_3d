use std::collections::VecDeque;

use pi_render::rhi::{device::RenderDevice, RenderQueue};
use pi_scene_math::Vector3;
use render_data_container::{EVertexDataFormat, GeometryBuffer, GeometryBufferPool};
use render_geometry::geometry::VertexAttributeBufferMeta;

use crate::{
    default_render::interface::InterfaceDefaultMaterial,
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
    },
};

use super::interface::InterfaceMesh;

pub struct SingleBaseBall {
    position: IDAttributePosition,
    normal: IDAttributeNormal,
    indices: IDAttributeIndices,
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
    fn new_Ball(&self, scene: ObjectID) -> ObjectID;
}

impl InterfaceBall for Engine {
    fn new_Ball(&self, scene: ObjectID) -> ObjectID {
        let entity = self.new_object();
        let world = self
            .add_to_scene(entity, scene)
            .as_transform_node(entity)
            .transform_parent(entity, scene)
            .as_mesh(entity)
            .use_default_material(entity)
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
        engine: &mut Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), ErrorPlugin> {
        let position_id = engine.new_object();
        let normal_id = engine.new_object();
        let indices_id = engine.new_object();
        let world = engine.world_mut();

        let device = world.get_resource::<RenderDevice>().unwrap();
        let queue = world.get_resource::<RenderQueue>().unwrap();
        let gbp = world
            .get_resource_mut::<SingleGeometryBufferPool>()
            .unwrap();

        let ball = generate_sphere(0.1);

        let len = ball.len();
        // 每个三角形 3 个点，一共 8个象限
        let mut positions = Vec::with_capacity(len * 9 * 8);
        let mut normals = Vec::with_capacity(len * 3 * 8);
        let mut indices = Vec::with_capacity(len * 3 * 8);

        let mut index = 0;
        for tri in ball {
            let start = index * 24;
            let mut data = [tri.a.as_slice(), tri.b.as_slice(), tri.c.as_slice()].concat();
            let normal = tri.compute_normal();

            // 第一象限
            positions.append(&mut data);
            normals.append(&mut vec![normal[0], normal[1], normal[2]]);

            // 第二象限
            data[0] = -data[0];
            data[3] = -data[3];
            data[6] = -data[6];
            positions.append(&mut data);
            normals.append(&mut vec![-normal[0], normal[1], normal[2]]);

            // 第三象限
            data[1] = -data[1];
            data[4] = -data[4];
            data[7] = -data[7];
            positions.append(&mut data);
            normals.append(&mut vec![-normal[0], -normal[1], normal[2]]);

            // 第四象限
            data[0] = -data[0];
            data[3] = -data[3];
            data[6] = -data[6];
            positions.append(&mut data);
            normals.append(&mut vec![normal[0], -normal[1], normal[2]]);

            // 第五象限
            data[2] = -data[2];
            data[5] = -data[5];
            data[8] = -data[8];
            positions.append(&mut data);
            normals.append(&mut vec![normal[0], normal[1], -normal[2]]);

            // 第六象限
            data[0] = -data[0];
            data[3] = -data[3];
            data[6] = -data[6];
            positions.append(&mut data);
            normals.append(&mut vec![-normal[0], normal[1], -normal[2]]);

            // 第七象限
            data[1] = -data[1];
            data[4] = -data[4];
            data[7] = -data[7];
            positions.append(&mut data);
            normals.append(&mut vec![-normal[0], -normal[1], -normal[2]]);

            // 第八象限
            data[0] = -data[0];
            data[3] = -data[3];
            data[6] = -data[6];
            positions.append(&mut data);
            normals.append(&mut vec![normal[0], -normal[1], -normal[2]]);

            for i in 0..8 * 3 {
                indices.push(start + i);
            }
        }

        let position = BallBuilder::position(device, queue, gbp, &positions);
        let normal = BallBuilder::normal(device, queue, gbp, &normals);
        let indices = BallBuilder::indices(device, queue, gbp, &indices);

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

        world.insert_resource::<SingleBaseBall>(SingleBaseBall {
            position: IDAttributePosition(position_id),
            normal: IDAttributeNormal(normal_id),
            indices: IDAttributeIndices(indices_id),
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
    pub fn compute_normal(&self) -> Vector3 {
        let ab = self.b - self.a;
        let bc = self.c - self.b;

        return ab.cross(&bc).normalize();
    }
}

/**
 * @brief 面细分法 八分之一个球
 * @param resolution 分辨率
 */
fn generate_sphere(mut resolution: f32) -> VecDeque<Triangle> {
    let mut triangles = VecDeque::new();
    triangles.push_back(Triangle {
        a: Vector3::new(0.0, 1.0, 0.0),
        b: Vector3::new(1.0, 0.0, 0.0),
        c: Vector3::new(0.0, 0.0, 1.0),
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

            triangles.push_back(Triangle { a: t.a, b: f, c: d });
            triangles.push_back(Triangle { a: t.b, b: d, c: e });
            triangles.push_back(Triangle { a: t.c, b: e, c: f });
            triangles.push_back(Triangle { a: d, b: e, c: f });
        } else {
            break;
        }
    }

    return triangles;
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
