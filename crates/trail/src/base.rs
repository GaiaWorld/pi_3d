use std::{ops::Sub, sync::Arc};

use pi_assets::asset::Handle;
use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector3, Number, Vector4, Vector2, coordiante_system::CoordinateSytem3, vector::TToolVector3, Matrix};
use pi_wy_rng::WyRng;

#[derive(Component)]
pub struct ColorOverTrail(pub Color4Gradient);

#[derive(Component)]
pub struct WidthOverTrail(pub FloatInterpolation);

#[derive(Component)]
pub struct TrailMinimunVertexDistance(pub Number);

#[derive(Component)]
pub struct TrailWorldPlace(pub bool);

#[derive(Component)]
pub struct TrailLinkedTransform(pub Entity);

#[derive(Component)]
pub struct TrailAgeControl(pub u32);

#[derive(Component)]
pub struct TrailColor(pub Vector4);

#[derive(Component)]
pub struct TrailSize(pub f32);

#[derive(Component)]
pub struct TrailRandom(pub WyRng);

#[derive(Component)]
pub struct TrailMesh(pub Entity);

#[derive(Component)]
pub struct TrailGeometry(pub Entity);

#[derive(Clone)]
pub struct KeyPoint {
    pub pos: Vector3,
    pub axis: Vector3,
    /// 出生时间点
    pub starttime: u32,
    /// 在整个Trial中的长度百分比
    pub distance_percent: Number,
    pub width: Number,
    pub color: Vector4,
}

#[derive(Default, Component)]
pub struct TrailPoints(pub Vec<KeyPoint>);
impl TrailPoints {
    pub fn run(
        &mut self,
        newpos: &Vector3,
        newaxis: &Vector3,
        colorcontrol: &Vector4,
        colorinterpolator: &Color4Gradient,
        colorinterpolator2: &Color4Gradient,
        sizecontrol: Number,
        widthinterpolator: &FloatInterpolation,
        agecontrol: u32,
        base: &TrailBase,
        randoms: &BaseRandom,
        minimumdistance: Number,
        worldmatrix: &Matrix,
        trailworldspace: bool,
    ) {
        if base.time <= base.starttime + base.lifetime {
            let amount = (base.time - base.starttime) as f32 / base.lifetime as f32;
            let mut color = [0., 0., 0., 0.];
            colorinterpolator.interpolate(amount, &mut color, randoms);
            let color = Vector4::new(color[0] * colorcontrol.x, color[1] * colorcontrol.y, color[2] * colorcontrol.z, color[3] * colorcontrol.w);
    
            let width = sizecontrol * widthinterpolator.interpolate(amount, randoms.base);
    
            let lastpos = if let Some(pos) = self.0.pop() {
                pos
            } else {
                let pos = newaxis.scale(0.000001);
                let pos = newpos.sub(pos);
                KeyPoint {
                    pos,
                    axis: newaxis.clone(),
                    starttime: base.time,
                    width,
                    color,
                    distance_percent: 0.,
                }
            };
            let mut newkey = KeyPoint {
                pos: newpos.clone(),
                axis: newaxis.clone(),
                starttime: base.time,
                width,
                color,
                distance_percent: 0.,
            };
            if trailworldspace {
                CoordinateSytem3::transform_coordinates(newpos, worldmatrix, &mut newkey.pos);
                CoordinateSytem3::transform_normal(newaxis, worldmatrix, &mut newkey.axis);
            }

            if let Some(lastkey) = self.0.last() {
                let distance = CoordinateSytem3::distance(&lastkey.pos, &newkey.pos);
                if minimumdistance <= distance {
                    self.0.push(newkey.clone());
                }
            } else {
                self.0.push(lastpos);
            }
            self.0.push(newkey);
        }

        let mut points = vec![];
        self.0.drain(..).for_each(|item| {
            if base.time <= item.starttime + agecontrol {
                points.push(item);
            }
        });

        let count = points.len();
        if 0 < count {
            points[0].distance_percent = 0.;
        }
        if 1 < count {
            let mut lastpos = Vector3::zeros();
            lastpos.copy_from(&points[0].pos);
            points[0].distance_percent = 0.;

            let mut distance = 0.;
            for i in 1..count {
                distance += CoordinateSytem3::distance(&lastpos, &points[i].pos);
                points[i].distance_percent = distance;
                lastpos.copy_from(&points[i].pos);
            }

            for i in 0..count {
                points[i].distance_percent /= distance;
            }

            self.0 = points;
        }
    }

    pub fn data(
        &self,
        trailworldspace: bool,
        worldmatrix: &Matrix,
        datavertices: &mut Vec<f32>,
        maxverticeslen: usize,
    ) -> bool {
        let count = self.0.len();
        if 1 < count {
            if trailworldspace == false {
                let mut pos = Vector3::zeros();
                let mut axis = Vector3::zeros();
                let mut idx = 0;
                self.0.iter().for_each(|item| {
                    if maxverticeslen < datavertices.len() + TrailBuffer::FLOAT_PER_VERTEX as usize * (2 + 2) {
                        return;
                    }
                    CoordinateSytem3::transform_coordinates(&item.pos, worldmatrix, &mut pos);
                    CoordinateSytem3::transform_normal(&item.axis, worldmatrix, &mut axis);
                    if idx == 0 {
                        datavertices.push(pos.x); datavertices.push(pos.y); datavertices.push(pos.z);
                        datavertices.push(item.color.x); datavertices.push(item.color.y); datavertices.push(item.color.z); datavertices.push(0.);
                        datavertices.push(axis.x); datavertices.push(axis.y); datavertices.push(axis.z);
                        datavertices.push(0.); datavertices.push(item.distance_percent);
                        datavertices.push(pos.x); datavertices.push(pos.y); datavertices.push(pos.z);
                        datavertices.push(item.color.x); datavertices.push(item.color.y); datavertices.push(item.color.z); datavertices.push(0.);
                        datavertices.push(axis.x); datavertices.push(axis.y); datavertices.push(axis.z);
                        datavertices.push(0.); datavertices.push(item.distance_percent);
                    }
                    {
                        datavertices.push(pos.x); datavertices.push(pos.y); datavertices.push(pos.z);
                        datavertices.push(item.color.x); datavertices.push(item.color.y); datavertices.push(item.color.z); datavertices.push(item.color.w);
                        datavertices.push(axis.x); datavertices.push(axis.y); datavertices.push(axis.z);
                        datavertices.push(item.width * 1.); datavertices.push(item.distance_percent);
                    }
                    {
                        datavertices.push(pos.x); datavertices.push(pos.y); datavertices.push(pos.z);
                        datavertices.push(item.color.x); datavertices.push(item.color.y); datavertices.push(item.color.z); datavertices.push(item.color.w);
                        datavertices.push(axis.x); datavertices.push(axis.y); datavertices.push(axis.z);
                        datavertices.push(item.width * -1.); datavertices.push(item.distance_percent);
                    }
                    idx += 1;
                    if idx == count {
                        datavertices.push(pos.x); datavertices.push(pos.y); datavertices.push(pos.z);
                        datavertices.push(item.color.x); datavertices.push(item.color.y); datavertices.push(item.color.z); datavertices.push(0.);
                        datavertices.push(axis.x); datavertices.push(axis.y); datavertices.push(axis.z);
                        datavertices.push(0.); datavertices.push(item.distance_percent);
                        datavertices.push(pos.x); datavertices.push(pos.y); datavertices.push(pos.z);
                        datavertices.push(item.color.x); datavertices.push(item.color.y); datavertices.push(item.color.z); datavertices.push(0.);
                        datavertices.push(axis.x); datavertices.push(axis.y); datavertices.push(axis.z);
                        datavertices.push(0.); datavertices.push(item.distance_percent);
                    }
                });
            } else {
                let mut idx = 0;
                self.0.iter().for_each(|item| {
                    if maxverticeslen < datavertices.len() + TrailBuffer::FLOAT_PER_VERTEX as usize * (2 + 2) {
                        return;
                    }
                    if idx == 0 {
                        datavertices.push(item.pos.x); datavertices.push(item.pos.y); datavertices.push(item.pos.z);
                        datavertices.push(item.color.x); datavertices.push(item.color.y); datavertices.push(item.color.z); datavertices.push(0.);
                        datavertices.push(item.axis.x); datavertices.push(item.axis.y); datavertices.push(item.axis.z);
                        datavertices.push(0.); datavertices.push(item.distance_percent);
                        datavertices.push(item.pos.x); datavertices.push(item.pos.y); datavertices.push(item.pos.z);
                        datavertices.push(item.color.x); datavertices.push(item.color.y); datavertices.push(item.color.z); datavertices.push(0.);
                        datavertices.push(item.axis.x); datavertices.push(item.axis.y); datavertices.push(item.axis.z);
                        datavertices.push(0.); datavertices.push(item.distance_percent);
                    }
                    {
                        datavertices.push(item.pos.x); datavertices.push(item.pos.y); datavertices.push(item.pos.z);
                        datavertices.push(item.color.x); datavertices.push(item.color.y); datavertices.push(item.color.z); datavertices.push(item.color.w);
                        datavertices.push(item.axis.x); datavertices.push(item.axis.y); datavertices.push(item.axis.z);
                        datavertices.push(item.width * 1.); datavertices.push(item.distance_percent);
                    }
                    {
                        datavertices.push(item.pos.x); datavertices.push(item.pos.y); datavertices.push(item.pos.z);
                        datavertices.push(item.color.x); datavertices.push(item.color.y); datavertices.push(item.color.z); datavertices.push(item.color.w);
                        datavertices.push(item.axis.x); datavertices.push(item.axis.y); datavertices.push(item.axis.z);
                        datavertices.push(item.width * -1.); datavertices.push(item.distance_percent);
                    }
                    idx += 1;
                    if idx == count {
                        datavertices.push(item.pos.x); datavertices.push(item.pos.y); datavertices.push(item.pos.z);
                        datavertices.push(item.color.x); datavertices.push(item.color.y); datavertices.push(item.color.z); datavertices.push(0.);
                        datavertices.push(item.axis.x); datavertices.push(item.axis.y); datavertices.push(item.axis.z);
                        datavertices.push(0.); datavertices.push(item.distance_percent);
                        datavertices.push(item.pos.x); datavertices.push(item.pos.y); datavertices.push(item.pos.z);
                        datavertices.push(item.color.x); datavertices.push(item.color.y); datavertices.push(item.color.z); datavertices.push(0.);
                        datavertices.push(item.axis.x); datavertices.push(item.axis.y); datavertices.push(item.axis.z);
                        datavertices.push(0.); datavertices.push(item.distance_percent);
                    }
                });
            }
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Component)]
pub struct TrailBase {
    /// 启动时间点
    pub starttime: u32,
    /// 实时时间点
    pub time: u32,
    /// 生命时长
    pub lifetime: u32,
}
impl TrailBase {
    pub fn new(lifetime: u32) -> Self {
        Self { starttime: 0, time: 0, lifetime }
    }
    pub fn update(&mut self, delta_ms: u32) {
        self.time += delta_ms;
    }
}

pub struct TrailBuffer {
    pub vertices: Vec<f32>,
    pub count: u32,
    pub maxcount: u32,
    buffer: (Arc<NotUpdatableBufferRange>, u32, u32),
    pub key: KeyVertexBuffer,
}
impl TrailBuffer {
    pub const MAX_COUNT: u32 = 1024 * 1024;
    pub const FLOAT_PER_VERTEX: u32 = (3 + 4 + 3 + 2);
    pub const SIZE_PER_VERTEX: u32 = Self::FLOAT_PER_VERTEX * 4;
    pub fn buffer_desc(&self) -> VertexBufferDesc {
        VertexBufferDesc {
            key: self.key.clone(),
            range: None,
            attrs: vec![
                VertexAttribute { kind: EVertexDataKind::Position, format: wgpu::VertexFormat::Float32x3 },
                VertexAttribute { kind: EVertexDataKind::Color4, format: wgpu::VertexFormat::Float32x4 },
                VertexAttribute { kind: EVertexDataKind::TrailAxis, format: wgpu::VertexFormat::Float32x3 },
                VertexAttribute { kind: EVertexDataKind::TrailInfo, format: wgpu::VertexFormat::Float32x2 },
            ],
            step_mode: wgpu::VertexStepMode::Vertex,
            kind: EInstanceKind::None,
        }
    }
    pub fn buffer(&self) -> Arc<NotUpdatableBufferRange> {
        self.buffer.0.clone()
    }
    pub fn new(
        maxcount: u32, 
        allocator: &mut VertexBufferAllocator,
        device: &RenderDevice,
        queue: &RenderQueue,
    ) -> Option<Self> {
        // let maxcount = maxcount.min(Self::MAX_COUNT);

        let size = Self::SIZE_PER_VERTEX * maxcount;
        let mut data = Vec::with_capacity(size as usize);
        for _ in 0..size {
            data.push(0);
        }
        if let Some(buffer) = allocator.create_not_updatable_buffer_pre(device, queue, &data, None) {
            Some(Self {
                vertices: vec![],
                count: 0,
                maxcount,
                buffer: (buffer, 0, size),
                key: KeyVertexBuffer::from("@SingleTrialBuffer#@#@"),
            })
        } else {
            None
        }
    }
    /// 收集 Trail 返回当前Trail 对应 buffer 的 start end
    pub fn collect(
        &mut self,
        trailpoints: &TrailPoints,
        trailworldspace: bool,
        worldmatrix: &Matrix,
    ) -> (u32, u32) {
        let last_count = self.vertices.len() as u32;
        trailpoints.data(trailworldspace, worldmatrix, &mut self.vertices, (self.maxcount * Self::FLOAT_PER_VERTEX) as usize);
        let new_count = self.vertices.len() as u32;

        (last_count * 4, new_count * 4)
    }
    pub fn after_collect(
        &mut self,
        queue: &RenderQueue,
    ) {
        if 0 < self.vertices.len()  {
            let buffer = self.buffer.0.buffer();
            queue.write_buffer(buffer, 0, bytemuck::cast_slice(&self.vertices));
            self.vertices.clear();
        }
    }
}

impl TAssetCapacity for TrailBuffer {
    const ASSET_TYPE: &'static str = "TRAIL_BUFFER";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 1024 * 1024, max: 1024 * 1024, timeout: 1000  }
    }
}

#[test]
fn test_trail() {
    let mut trailpoints = TrailPoints::default();
    let mut time = 0;
    
    let mut newpos: Vector3 = Vector3::new(0., 0., 0.);
    let mut newaxis: Vector3 = Vector3::new(0., 0., 1.);
    let colorcontrol: Vector4 = Vector4::new(1., 1., 1., 1.);
    let colorinterpolator: Color4Gradient = Color4Gradient::default();
    let colorinterpolator2: Color4Gradient = Color4Gradient::default();
    let sizecontrol: Number = 1.;
    let widthinterpolator: FloatInterpolation = FloatInterpolation::new(1.);
    let agecontrol: u32 = 500;
    let mut base: TrailBase = TrailBase::new(1000);
    let randoms: BaseRandom = BaseRandom::default();
    let minimumdistance: Number = 0.2;
    let worldmatrix: Matrix = Matrix::identity();
    let trailworldspace: bool = true;

    let mut dataposition =  Vec::<f32>::default();
    let mut datacolor =  Vec::<f32>::default();
    let mut datatrailaxis =  Vec::<f32>::default();
    let mut datatrailinfo =  Vec::<f32>::default();

    for _ in 0..4 {
        dataposition.clear();
        datacolor.clear();
        datatrailaxis.clear();
        datatrailinfo.clear();
        newpos.z += 0.1;
        time += 100;
        base.update(time);
        trailpoints.run(
            &newpos, &newaxis,
            &colorcontrol, &colorinterpolator, &colorinterpolator2,
            sizecontrol, &widthinterpolator,
            agecontrol, &base, &randoms, minimumdistance,
            &worldmatrix, trailworldspace
        );

        trailpoints.data(trailworldspace, &worldmatrix, &mut dataposition, TrailBuffer::MAX_COUNT as usize);

        println!("{:?}", dataposition);
        println!("{:?}", datacolor);
        println!("{:?}", datatrailaxis);
        println!("{:?}", datatrailinfo);
    }
}