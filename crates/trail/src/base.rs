use std::sync::Arc;

use pi_scene_shell::prelude::*;
use pi_scene_math::{*, vector::TToolVector3};
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

// #[derive(Component)]
// pub struct TrailMesh(pub Entity);

#[derive(Component)]
pub struct TrailGeometry(pub Entity);

#[derive(Clone)]
pub struct KeyPoint {
    pub pos: Vector3,
    pub axis: Vector3,
    /// 出生时间点
    pub starttime: u32,
    /// 在整个Trail中的长度百分比
    pub distance: Number,
    /// 在整个Trail中的长度百分比
    pub distance_percent: Number,
    pub width: Number,
    pub color: Vector4,
}

#[derive(Default, Component)]
pub struct TrailPoints(pub Vec<PathPoint>, pub Vec<Vector4>, pub Vec<Number>, pub bool);
impl TrailPoints {
    pub fn reset(&mut self) {
        self.0.clear();
        self.1.clear();
        self.2.clear();
    }
    pub fn run(
        &mut self,
        worldmatrix: &Matrix,
        localmatrix: &Matrix,
        colorcontrol: &Vector4,
        colorinterpolator: &Color4Gradient,
        colorinterpolator2: &Color4Gradient,
        sizecontrol: Number,
        widthinterpolator: &FloatInterpolation,
        agecontrol: u32,
        base: &TrailBase,
        randoms: &BaseRandom,
        distancecontrol: Number,
        limit_between_distance: Number,
        trailworldspace: bool,
    ) {
        if base.time <= base.starttime + base.lifetime {
            let mut newpos = Vector3::zeros();
            let mut newaxisx = Vector3::new(1., 0., 0.);
            let mut newsize = Vector3::new(0.5773502691896257 as f32, 0.5773502691896257 as f32, 0.5773502691896257 as f32);

            if trailworldspace {
                coordiante_system::CoordinateSytem3::transform_coordinates(&newpos.clone(), &worldmatrix, &mut newpos);
                coordiante_system::CoordinateSytem3::transform_normal(&newaxisx.clone(), &worldmatrix, &mut newaxisx);
                coordiante_system::CoordinateSytem3::transform_normal(&newsize.clone(), &worldmatrix, &mut newsize);
                // log::warn!("New Point 1: {:?}  {:?}", newaxisx, newpos);
            } else {
                coordiante_system::CoordinateSytem3::transform_coordinates(&newpos.clone(), localmatrix, &mut newpos);
                coordiante_system::CoordinateSytem3::transform_normal(&newaxisx.clone(), localmatrix, &mut newaxisx);
                coordiante_system::CoordinateSytem3::transform_normal(&newsize.clone(), localmatrix, &mut newsize);
                // log::warn!("New Point 2: {:?}  {:?}", localmatrix, newpos);
            }
            let xlen = coordiante_system::CoordinateSytem3::length(&newaxisx);
            if Number::EPSILON < xlen {
                newaxisx.scale_mut(1. / xlen);
            } else {
                newaxisx.copy_from_slice(&[1., 0., 0.]);
            };

            let newaxisz = if let Some(last) = self.0.last() {
                let mut temp = newpos - last.pos;
                let len = coordiante_system::CoordinateSytem3::length(&temp);
                if Number::EPSILON < len {
                    temp.scale_mut(1. / len);
                } else {
                    temp.copy_from(&last.zaxis);
                }
                temp
            } else {
                let len = coordiante_system::CoordinateSytem3::length(&newpos);
                if Number::EPSILON < len {
                    newpos.scale(1. / len)
                } else {
                    Vector3::new(0., 0., 1.)
                }
            };
            // log::warn!("Z: {:?} ", newaxisz);

            let limit_time = if base.time < agecontrol { 0 } else { base.time - agecontrol };
            // log::warn!("limit_time: {:?}  {:?}", limit_time, base);
            let (path, totaldistance) = PathPoints::path_update_point(newpos, newaxisx, newaxisz, base.time, &mut self.0, limit_time, distancecontrol, limit_between_distance);
            self.0 = path;
            // log::warn!("Total Distance: {:?}", totaldistance);
            // log::warn!("Point: {:?}  {:?}  {:?}  {:?}  {:?}  {:?}", limit_time, base, totaldistance, newpos, newaxisx, newaxisz);
            if totaldistance < Number::EPSILON {
                self.3 = false;
            } else {
                let amount = (base.time - base.starttime) as f32 / base.lifetime as f32;
                let mut color = [0., 0., 0., 0.];
                colorinterpolator.interpolate(amount, &mut color, randoms);
                let color = Vector4::new(color[0] * colorcontrol.x, color[1] * colorcontrol.y, color[2] * colorcontrol.z, color[3] * colorcontrol.w);
    
                self.1 = PathPoints::path_color(&self.0, randoms, &color, colorinterpolator2);
                self.2 = PathPoints::path_width(&self.0, randoms, sizecontrol * coordiante_system::CoordinateSytem3::length(&newsize) / f32::sqrt(3.0), widthinterpolator);
                self.3 = true;
            }
        } else {
            self.0.clear();
            self.3 = false;
        }
    }

    pub fn data(
        &self,
        trailworldspace: bool,
        parentmatrix: &Matrix,
        datavertices: &mut Vec<f32>,
        maxverticeslen: usize,
    ) -> bool {
        let count = self.0.len();
        if 1 < count {
            let mut pos = Vector3::zeros();
            let mut axisz = Vector3::zeros();
            let mut axisx = Vector3::zeros();
            let mut color = Vector4::zeros();
            let mut width;
            let basesize = Vector3::new(0.5773502691896257 as f32, 0.5773502691896257 as f32, 0.5773502691896257 as f32);
            for idx in 0..count {
                if maxverticeslen < datavertices.len() + TrailBuffer::FLOAT_PER_VERTEX as usize * (2 + 2) {
                    break;
                }

                let index = count - idx - 1;
                let item = &self.0[index];
                let mut sizetemp = 1.;
                if trailworldspace == false {
                    // log::warn!("World");
                    coordiante_system::CoordinateSytem3::transform_coordinates(&item.pos, parentmatrix, &mut pos);
                    coordiante_system::CoordinateSytem3::transform_normal(&item.zaxis, parentmatrix, &mut axisz);
                    coordiante_system::CoordinateSytem3::transform_normal(&item.xaxis, parentmatrix, &mut axisx);
                    let mut newsize = Vector3::zeros();
                    coordiante_system::CoordinateSytem3::transform_normal(&basesize, parentmatrix, &mut newsize);
                    sizetemp = coordiante_system::CoordinateSytem3::length(&newsize);
                } else {
                    // log::warn!("Collect {:?} {:?}", item.xaxis, item.pos);
                    pos.copy_from(&item.pos);
                    axisz.copy_from(&item.zaxis);
                    axisx.copy_from(&item.xaxis);
                }
                color.copy_from(&self.1[index]);
                width = self.2[index] * sizetemp;
                if idx == 0 {
                    datavertices.push(pos.x); datavertices.push(pos.y); datavertices.push(pos.z);
                    datavertices.push(color.x); datavertices.push(color.y); datavertices.push(color.z); datavertices.push(0.);
                    datavertices.push(axisx.x); datavertices.push(axisx.y); datavertices.push(axisx.z);
                    datavertices.push(axisz.x); datavertices.push(axisz.y); datavertices.push(axisz.z);
                    datavertices.push(0.); datavertices.push(item.distance_percent);
                    datavertices.push(pos.x); datavertices.push(pos.y); datavertices.push(pos.z);
                    datavertices.push(color.x); datavertices.push(color.y); datavertices.push(color.z); datavertices.push(0.);
                    datavertices.push(axisx.x); datavertices.push(axisx.y); datavertices.push(axisx.z);
                    datavertices.push(axisz.x); datavertices.push(axisz.y); datavertices.push(axisz.z);
                    datavertices.push(0.); datavertices.push(item.distance_percent);
                }
                {
                    datavertices.push(pos.x); datavertices.push(pos.y); datavertices.push(pos.z);
                    datavertices.push(color.x); datavertices.push(color.y); datavertices.push(color.z); datavertices.push(color.w);
                    datavertices.push(axisx.x); datavertices.push(axisx.y); datavertices.push(axisx.z);
                    datavertices.push(axisz.x); datavertices.push(axisz.y); datavertices.push(axisz.z);
                    datavertices.push(width * 1.); datavertices.push(item.distance_percent);
                    datavertices.push(pos.x); datavertices.push(pos.y); datavertices.push(pos.z);
                    datavertices.push(color.x); datavertices.push(color.y); datavertices.push(color.z); datavertices.push(color.w);
                    datavertices.push(axisx.x); datavertices.push(axisx.y); datavertices.push(axisx.z);
                    datavertices.push(axisz.x); datavertices.push(axisz.y); datavertices.push(axisz.z);
                    datavertices.push(width * -1.); datavertices.push(item.distance_percent);
                }
                if idx == count-1 {
                    datavertices.push(pos.x); datavertices.push(pos.y); datavertices.push(pos.z);
                    datavertices.push(color.x); datavertices.push(color.y); datavertices.push(color.z); datavertices.push(0.);
                    datavertices.push(axisx.x); datavertices.push(axisx.y); datavertices.push(axisx.z);
                    datavertices.push(axisz.x); datavertices.push(axisz.y); datavertices.push(axisz.z);
                    datavertices.push(0.); datavertices.push(item.distance_percent);
                    datavertices.push(pos.x); datavertices.push(pos.y); datavertices.push(pos.z);
                    datavertices.push(color.x); datavertices.push(color.y); datavertices.push(color.z); datavertices.push(0.);
                    datavertices.push(axisx.x); datavertices.push(axisx.y); datavertices.push(axisx.z);
                    datavertices.push(axisz.x); datavertices.push(axisz.y); datavertices.push(axisz.z);
                    datavertices.push(0.); datavertices.push(item.distance_percent);
                }
            }
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug, Component)]
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
    pub const FLOAT_PER_VERTEX: u32 = (3 + 4 + 3 + 3 + 2);
    pub const SIZE_PER_VERTEX: u32 = Self::FLOAT_PER_VERTEX * 4;
    pub fn buffer_desc(&self) -> VertexBufferDesc {
        VertexBufferDesc::new(
            self.key.clone(),
            VertexBufferDescRange::default(),
            vec![
                EVertexAttribute::Buildin(EBuildinVertexAtribute::Position),
                EVertexAttribute::Buildin(EBuildinVertexAtribute::Color4),
                EVertexAttribute::Buildin(EBuildinVertexAtribute::TrailAxisX),
                EVertexAttribute::Buildin(EBuildinVertexAtribute::TrailAxisZ),
                EVertexAttribute::Buildin(EBuildinVertexAtribute::Trail),
                // VertexAttribute { kind: EVertexDataKind::Color4, format: wgpu::VertexFormat::Float32x4 },
                // VertexAttribute { kind: EVertexDataKind::TrailAxisX, format: wgpu::VertexFormat::Float32x3 },
                // VertexAttribute { kind: EVertexDataKind::TrailAxisZ, format: wgpu::VertexFormat::Float32x3 },
                // VertexAttribute { kind: EVertexDataKind::TrailInfo, format: wgpu::VertexFormat::Float32x2 },
            ],
            false,
        )
    }
    pub fn buffer_desc_billboard(&self) -> VertexBufferDesc {
        VertexBufferDesc::new(
            self.key.clone(),
            VertexBufferDescRange::default(),
            vec![
                EVertexAttribute::Buildin(EBuildinVertexAtribute::Position),
                EVertexAttribute::Buildin(EBuildinVertexAtribute::Color4),
                EVertexAttribute::Buildin(EBuildinVertexAtribute::TrailAxisX),
                EVertexAttribute::Buildin(EBuildinVertexAtribute::TrailAxisZ),
                EVertexAttribute::Buildin(EBuildinVertexAtribute::TrailBillboard),
                // VertexAttribute { kind: EVertexDataKind::Color4, format: wgpu::VertexFormat::Float32x4 },
                // VertexAttribute { kind: EVertexDataKind::TrailAxisX, format: wgpu::VertexFormat::Float32x3 },
                // VertexAttribute { kind: EVertexDataKind::TrailAxisZ, format: wgpu::VertexFormat::Float32x3 },
                // VertexAttribute { kind: EVertexDataKind::TrailInfo, format: wgpu::VertexFormat::Float32x2 },
            ],
            false,
        )
    }
    pub fn buffer(&self) -> Arc<NotUpdatableBufferRange> {
        self.buffer.0.clone()
    }
    pub fn new(
        maxbytes: u32, 
        allocator: &mut VertexBufferAllocator,
        device: &RenderDevice,
        queue: &RenderQueue,
    ) -> Option<Self> {
        let maxcount = maxbytes / Self::SIZE_PER_VERTEX;

        let size = maxbytes;
        let mut data = Vec::with_capacity(size as usize);
        for _ in 0..size {
            data.push(0);
        }
        log::error!("TrailBuffer {}", data.len());
        if let Some(buffer) = allocator.create_not_updatable_buffer_pre(device, queue, &data, None) {
            Some(Self {
                vertices: vec![],
                count: 0,
                maxcount: maxcount,
                buffer: (buffer, 0, size),
                key: KeyVertexBuffer::from("@SingleTrailBuffer#@#@"),
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
        parentmatrix: &Matrix,
    ) -> (u32, u32) {
        let last_count = self.vertices.len() as u32;
        trailpoints.data(trailworldspace, parentmatrix, &mut self.vertices, (self.maxcount * Self::FLOAT_PER_VERTEX) as usize);
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
            // log::error!("Buffer: {}", self.vertices.len());
            self.vertices.clear();
        }
    }
}

impl TAssetCapacity for TrailBuffer {
    const ASSET_TYPE: &'static str = "TRAIL_BUFFER";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 1024 * 1024, max: 1 * 1024 * 1024, timeout: 1000  }
    }
}

#[test]
fn test_trail() {
    let mut trailpoints = TrailPoints::default();
    let mut time = 0;
    
    let mut newpos: Vector3 = Vector3::new(0., 0., 0.);
    // let mut newaxis: Vector3 = Vector3::new(0., 0., 1.);
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
            &worldmatrix, &worldmatrix,
            &colorcontrol, &colorinterpolator, &colorinterpolator2,
            sizecontrol, &widthinterpolator,
            agecontrol, &base, &randoms, 1000., minimumdistance,
            trailworldspace
        );

        trailpoints.data(trailworldspace, &worldmatrix, &mut dataposition, TrailBuffer::MAX_COUNT as usize);

        println!("{:?}", dataposition);
        println!("{:?}", datacolor);
        println!("{:?}", datatrailaxis);
        println!("{:?}", datatrailinfo);
    }
}