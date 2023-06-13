use nalgebra::AbstractRotation;
use pi_scene_math::Vector3;

use crate::{
    interpolation::{Color4Gradient, FloatInterpolation, IInterpolation},
    iparticle_system_config::EInterpolationCurveMode,
    particle::Particle, normalize,
};

use super::{
    base::Color4Interpolate, color_over_lifetime::ColorOverLifetime,
    trial_geometry::TrailGeometryModifier,
};

#[derive(Debug, Clone, Copy)]
pub enum ETrailMode {
    Particles = 0,
}

#[derive(Debug, Clone, Copy)]
pub enum ETrailTextureMode {
    Stretch = 0,
    Tiled = 1,
    DistributePerSegment = 2,
    RepeatPerSegment = 3,
}
pub struct TrailModifier {
    pub _mode: ETrailMode,

    /**
     * 可生成路径轨迹的粒子的百分比
     */
    pub ratio: f32,

    /**
     * 使用粒子多少生命百分比内的路径点
     */
    pub lifetime: FloatInterpolation,

    /**
     * ribbon 模式 间隔多少个粒子进行连接
     */
    pub ribbonCount: f32,

    /**
     * ribbon 末尾是否连接到粒子系统的位置
     */
    pub attachRibbonsToTransfoem: bool,

    /**
     * 新路径点与作为轨迹用的最后一个点之间距离超过多少，才将新作为轨迹用的点
     */
    pub minimunVertexDistance: f32,

    _useWorldSpace: bool,

    /**
     * 粒子的Size是否实时影响拖尾宽度
     */
    pub sizeAffectsWidth: bool,
    /**
     * 粒子的Size是否影响拖尾的 Lifetime 参数
     */
    pub sizeAffectsLifetime: bool,
    /**
     * 粒子的实时颜色是否影响拖尾颜色的整体变化
     */
    pub inheritParticleColor: bool,
    /**
     * 拖尾的颜色随生命增长的整体变化
     */
    pub colorOverLifetime: ColorOverLifetime,
    /**
     * 拖尾的初始宽度
     */
    pub widthOverTrail: FloatInterpolation,
    /**
     * 拖尾颜色在路径线上的分布情况
     */
    pub colorOverTrail: ColorOverLifetime,

    positions: Vec<f32>,
    indices: Vec<f32>,
    colors: Vec<f32>,
    uvs: Vec<f32>,

    geometry: Vec<f32>,
    pub mesh: Vec<f32>,

    maxKeyPoint: f32,
    modifyCall: Box<dyn Fn(Vector3, &mut Vec<Particle>, &mut TrailModifier)>,

    /**
     * 拖尾是否在粒子死亡时即死亡
     */
    pub dieWithParticle: bool,

    pub _textureMode: ETrailTextureMode,
    pub _enabled: bool,
}

impl TrailModifier {
    // const tempPoint: Vector3 =  Vector3::zeros();
    // const tempDirection: Vector3 =  Vector3::zeros();
    // const tempExtend: Vector3 =  Vector3::zeros();
    // const tempView: Vector3 = Vector3::zeros();
    // const tempColor4A: [f32;4] = [1.; 4];
    // const tempColor4B: [f32;4] = [1.; 4];
    // const counter: f32 = 0.;

    /**
     *
     *  * 渲染模式
     *      * Particles
     *      * 渲染为粒子的路径线
     */
    pub fn set_mode(&mut self, value: ETrailMode) {
        self._mode = value;
        self.updateModify();
    }

    pub fn get_mode(&self) -> ETrailMode {
        return self._mode;
    }

    /// 生成轨迹点的坐标是否使用其当时的世界坐标
    pub fn set_worldSpace(&mut self, value: bool) {
        self._useWorldSpace = value;
        self.updateModify();
    }
    pub fn get_worldSpace(&self) -> bool {
        return self._useWorldSpace;
    }

    /**
     * 拖尾的纹理平铺方式
     */
    pub fn set_textureMode(&mut self, value: ETrailTextureMode) {
        self._textureMode = value;
    }
    pub fn get_TextureMode(&self) -> ETrailTextureMode {
        return self._textureMode;
    }

    pub fn new() -> Self {
        Self {
            _mode: ETrailMode::Particles,
            ratio: 1.,
            lifetime: FloatInterpolation::new(),
            ribbonCount: 1.,
            attachRibbonsToTransfoem: false,
            minimunVertexDistance: 1.,
            _useWorldSpace: false,
            sizeAffectsWidth: false,
            sizeAffectsLifetime: false,
            inheritParticleColor: false,
            colorOverLifetime: ColorOverLifetime::new(Color4Interpolate::new(
                Color4Gradient::default(),
            )),
            widthOverTrail: FloatInterpolation::new(),
            colorOverTrail: ColorOverLifetime::new(Color4Interpolate::new(
                Color4Gradient::default(),
            )),
            positions: vec![],
            indices: vec![],
            colors: vec![],
            uvs: vec![],
            geometry: vec![],
            mesh: vec![],
            maxKeyPoint: 65000. / 2.,
            modifyCall: Box::new(TrailModifier::forParticlesLocalSpace),
            dieWithParticle: true,
            _textureMode: ETrailTextureMode::Stretch,
            _enabled: false,
        }
        // let id = TrailModifier.counter++;
        // self.geometry = new Geometry(`Trail${id}`, scene, new VertexData(), true);
        // let root = scene.metadata.trailRoot;
        // if (!root) {
        //     root = new TransformNode(`Trails`, scene);
        //     scene.metadata.trailRoot = root;
        // }
        // self.mesh = new Mesh(`Trail${id}`, scene);
        // self.geometry.applyToMesh(self.mesh);
        // self.modifyCall = TrailModifier.forParticlesLocalSpace;
        // self.mesh.setEnabled(false);
        // self.mesh.parent = root;

        // parseFloatInterpolation(self.widthOverTrail, [1, 2, 2, 0.5], null);
        // parseFloatInterpolation(self.lifetime, [1, 2, 1, 0.5], null);
    }

    pub fn set_enable(&mut self, value: bool) {
        self._enabled = value;
        if (!value) {
            // self.mesh.setEnabled(value);
        }
    }

    pub fn dispose(&self) {
        // self.mesh.dispose(true, true);
        // self.geometry.dispose();
    }

    pub fn modify(&mut self, cameraGlobalPos: Vector3, particles: &Vec<&mut Particle>) {
        // (self.modifyCall)(cameraGlobalPos, particles, self);
    }

    pub fn updateModify(&mut self) {
        match (self._mode) {
            ETrailMode::Particles => {
                if (self._useWorldSpace) {
                    self.modifyCall = Box::new(TrailModifier::forParticlesWorldSpace);
                } else {
                    self.modifyCall = Box::new(TrailModifier::forParticlesLocalSpace);
                }
            }
            _ => {
                if (self._useWorldSpace) {
                    self.modifyCall = Box::new(TrailModifier::forRibbonWorldSpace);
                } else {
                    self.modifyCall = Box::new(TrailModifier::forRibbonLocalSpace);
                }
            }
        }
    }

    pub fn forParticlesWorldSpace(
        cameraGlobalPos: Vector3,
        particles: &mut Vec<Particle>,
        modifier: &mut TrailModifier,
    ) {
        let count = particles.len();
        let mut pointCount = 0;
        for i in 0..count {
            let particle = &mut particles[i];
            if (particle.trial_lifetime == 0.0) {
                continue;
            }
            let maxAgeAmount = particle.age / particle.lifetime;
            let pointLenght = particle.global_record_list.len();
            if (pointLenght == 0) {
                let tempPoint = particle.direction;
                let tempPoint = normalize(&tempPoint);
                let tempPoint = tempPoint * (0.01);
                // particle.position.subtractToRef(TrailModifier.tempPoint, TrailModifier.tempPoint);
                let tempPoint = particle.position - tempPoint;
                // Vector3.TransformCoordinatesFromFloatsToRef(TrailModifier.tempPoint.x, TrailModifier.tempPoint.y, TrailModifier.tempPoint.z, particle.startWorldMatrix, TrailModifier.tempPoint);
                let tempPoint = particle.start_world_matrix.transform_vector(&tempPoint);
                particle
                    .global_record_list
                    .push([0., tempPoint[0], tempPoint[1], tempPoint[2]]);
            }

            // Vector3.TransformCoordinatesFromFloatsToRef(particle.position.x, particle.position.y, particle.position.z, particle.startWorldMatrix, TrailModifier.tempPoint);
            let tempPoint = particle
                .start_world_matrix
                .transform_vector(&particle.position);
            particle.global_record_list.push([
                maxAgeAmount,
                tempPoint[0],
                tempPoint[1],
                tempPoint[2],
            ]);
            // maxAgeAmount = Math.min(1.0, maxAgeAmount);

            let recordLength = particle.global_record_list.len();
            let mut tempRecordList = vec![];
            let mut lastRecord = particle.global_record_list[recordLength - 1];

            tempRecordList.push([lastRecord[1], lastRecord[2], lastRecord[3]]);
            let mut lastDistanceAboutIndex = recordLength - 1;
            let mut lastRatioAboutIndex = recordLength - 1;
            let waitTimeRatio = particle.trial_lifetime / particle.lifetime;
            for j in (0..=(recordLength - 2)).rev() {
                let currRecord = particle.global_record_list[j];
                let deltaRatio = maxAgeAmount - currRecord[0];
                if (0. <= deltaRatio && deltaRatio <= waitTimeRatio) {
                    let ax = lastRecord[1];
                    let ay = lastRecord[2];
                    let az = lastRecord[3];
                    let bx = currRecord[1];
                    let by = currRecord[2];
                    let bz = currRecord[3];

                    let dx = ax - bx;
                    let dy = ay - by;
                    let dz = az - bz;
                    let len = (dx * dx + dy * dy + dz * dz).sqrt();
                    if (len >= modifier.minimunVertexDistance || currRecord[0] == 0.) {
                        lastRecord = currRecord;
                        lastDistanceAboutIndex = j;
                        tempRecordList.push([lastRecord[1], lastRecord[2], lastRecord[3]]);
                    }
                    lastRatioAboutIndex = j;
                }
            }
            if (tempRecordList.len() == 1) {
                lastRecord = particle.global_record_list[0];
                lastDistanceAboutIndex = 0;
                lastRatioAboutIndex = 0;
                tempRecordList.push([lastRecord[1], lastRecord[2], lastRecord[3]]);
            }

            if (lastDistanceAboutIndex != lastRatioAboutIndex) {
                lastRecord = particle.global_record_list[lastRatioAboutIndex];
                tempRecordList.push([lastRecord[1], lastRecord[2], lastRecord[3]]);
            }
            lastRatioAboutIndex = lastRatioAboutIndex - 1;
            if (lastRatioAboutIndex >= 0) {
                lastRecord = particle.global_record_list[lastRatioAboutIndex];
                tempRecordList.push([lastRecord[1], lastRecord[2], lastRecord[3]]);
            }

            let keyPointCount = tempRecordList.len() - 1;
            let mut width = particle.trial_width * 0.5;
            if (modifier.sizeAffectsWidth) {
                width *= (particle.scaling[0] * particle.scaling[1]
                    + particle.scaling[1] * particle.scaling[1])
                    .sqrt();
            }

            for j in 0..keyPointCount {
                if (pointCount < modifier.maxKeyPoint as usize) {
                    let trailAmount = j / keyPointCount;

                    let tempPoint = Vector3::new(
                        tempRecordList[j][0],
                        tempRecordList[j][1],
                        tempRecordList[j][2],
                    );

                    let tempDirection = Vector3::new(
                        tempRecordList[j + 1][0],
                        tempRecordList[j + 1][1],
                        tempRecordList[j + 1][2],
                    );
                    let tempDirection = tempDirection - tempPoint;
                    let tempDirection = normalize(&tempDirection);

                    let tempView = cameraGlobalPos - tempPoint;
                    let tempView = normalize(&tempView);

                    // Vector3.CrossToRef(TrailModifier.tempDirection, TrailModifier.tempView, TrailModifier.tempExtend);
                    let mut tempExtend = tempDirection.cross(&tempView);
                    let xSquareLength = tempExtend.magnitude_squared();
                    if (xSquareLength == 0.) {
                        tempExtend = Vector3::new(0., 1., 0.);
                    } else {
                        // TrailModifier.tempExtend.normalizeFromLength(Math.sqrt(xSquareLength));
                    }

                    match modifier.widthOverTrail.mode {
                        EInterpolationCurveMode::Curve | EInterpolationCurveMode::TwoCurves => {
                            width *= modifier
                                .widthOverTrail
                                .interpolate(trailAmount as f32, particle.base_random);
                        }
                        _ => {}
                    }

                    let tempExtend = tempExtend * width;

                    TrailGeometryModifier::modifyPosition(
                        tempRecordList[j][0],
                        tempRecordList[j][1],
                        tempRecordList[j][2],
                        tempExtend[0],
                        tempExtend[1],
                        tempExtend[2],
                        &mut modifier.positions,
                        pointCount,
                    );

                    TrailGeometryModifier::modifyUV(
                        trailAmount as f32,
                        &mut modifier.uvs,
                        pointCount,
                    );
                    let mut tempColor4A = [0.; 4];
                    let mut tempColor4B = [0.; 4];

                    modifier
                        .colorOverLifetime
                        .color4Interpolate
                        .gradient
                        .interpolate(
                            maxAgeAmount,
                            &mut tempColor4A,
                            particle.color_over_lifetime_amount,
                        );
                    modifier
                        .colorOverTrail
                        .color4Interpolate
                        .gradient
                        .interpolate(
                            trailAmount as f32,
                            &mut tempColor4B,
                            particle.color_over_lifetime_amount,
                        );
                    tempColor4A[0] *= tempColor4B[0];
                    tempColor4A[1] *= tempColor4B[1];
                    tempColor4A[2] *= tempColor4B[2];
                    tempColor4A[3] *= tempColor4B[3];
                    if (modifier.inheritParticleColor) {
                        tempColor4A[0] *= particle.color[0];
                        tempColor4A[1] *= particle.color[1];
                        tempColor4A[2] *= particle.color[2];
                        tempColor4A[3] *= particle.color[3];
                    }
                    TrailGeometryModifier::modifyColor(
                        tempColor4A[0],
                        tempColor4A[1],
                        tempColor4A[2],
                        tempColor4A[3],
                        &mut modifier.colors,
                        pointCount,
                    );

                    if (j > 0) {
                        TrailGeometryModifier::modifyIndices(&mut modifier.indices, pointCount);
                    }

                    pointCount += 1;
                }
            }

            tempRecordList.clear();
        }

        // modifier.geometry.setVerticesData("position", new Float32Array(modifier.positions), true);
        // modifier.geometry.setVerticesData("uv", new Float32Array(modifier.uvs), true);
        // modifier.geometry.setVerticesData("color", new Float32Array(modifier.colors), true);
        // modifier.geometry.setIndices(new Uint16Array(modifier.indices), pointCount * 2, true);

        // modifier.mesh.setEnabled(modifier._enabled && modifier.indices.length >= 3);

        // modifier.positions.length = 0;
        // modifier.indices.length = 0;
        // modifier.uvs.length = 0;
    }
    pub fn forParticlesLocalSpace(
        camera_global_pos: Vector3,
        particles: &mut Vec<Particle>,
        modifier: &mut TrailModifier,
    ) {
        let count = particles.len();
        let mut point_count: usize = 0;
        for i in 0..count {
            let particle = &mut particles[i];
            if particle.trial_lifetime == 0. {
                continue;
            }
            let max_age_amount = particle.age / particle.lifetime;
            // let pointLenght = particle.localRecordList.length;
            // if (pointLenght == 0) {
            //     TrailModifier.tempPoint.copyFrom(particle.direction);
            //     TrailModifier.tempPoint.normalize();
            //     particle.localRecordList.push([-0.01, particle.position.x - TrailModifier.tempPoint.x, particle.position.y - TrailModifier.tempPoint.y, particle.position.z - TrailModifier.tempPoint.z]);
            // }

            particle.local_record_list.push([
                max_age_amount,
                particle.position[0],
                particle.position[1],
                particle.position[2],
            ]);
            // maxAgeAmount = Math.min(1.0, maxAgeAmount);

            let record_length = particle.local_record_list.len();
            let mut tempRecordList = vec![];
            let mut lastRecord = particle.local_record_list[record_length - 1];

            let mut tempPoint = particle.start_world_matrix.transform_vector(&Vector3::new(
                lastRecord[1],
                lastRecord[2],
                lastRecord[3],
            ));
            tempRecordList.push([tempPoint[0], tempPoint[1], tempPoint[2]]);
            let mut lastDistanceAboutIndex = record_length - 1;
            let mut lastRatioAboutIndex = record_length - 1;
            let waitTimeRatio = particle.trial_lifetime / particle.lifetime;
            for j in (0..=(record_length - 2)).rev() {
                let currRecord = particle.local_record_list[j];
                let deltaRatio = max_age_amount - currRecord[0];
                if (deltaRatio <= waitTimeRatio) {
                    let ax = lastRecord[1];
                    let ay = lastRecord[2];
                    let az = lastRecord[3];
                    let bx = currRecord[1];
                    let by = currRecord[2];
                    let bz = currRecord[3];

                    let dx = ax - bx;
                    let dy = ay - by;
                    let dz = az - bz;
                    let len = (dx * dx + dy * dy + dz * dz).sqrt();
                    if (len >= modifier.minimunVertexDistance || currRecord[0] == 0.) {
                        lastRecord = currRecord;
                        lastDistanceAboutIndex = j;
                        tempPoint = particle.start_world_matrix.transform_vector(&Vector3::new(
                            lastRecord[1],
                            lastRecord[2],
                            lastRecord[3],
                        ));
                        tempRecordList.push([tempPoint[0], tempPoint[1], tempPoint[2]]);
                    }
                    lastRatioAboutIndex = j;
                }
            }
            if (tempRecordList.len() == 1) {
                lastRecord = particle.local_record_list[0];
                lastDistanceAboutIndex = 0;
                lastRatioAboutIndex = 0;
                tempPoint = particle.start_world_matrix.transform_vector(&Vector3::new(
                    lastRecord[1],
                    lastRecord[2],
                    lastRecord[3],
                ));
                tempRecordList.push([tempPoint[0], tempPoint[1], tempPoint[2]]);
            }

            if (lastDistanceAboutIndex != lastRatioAboutIndex) {
                lastRecord = particle.local_record_list[lastRatioAboutIndex];
                // Vector3.TransformCoordinatesFromFloatsToRef(lastRecord[1], lastRecord[2], lastRecord[3], particle.startWorldMatrix, TrailModifier.tempPoint);
                tempPoint = particle.start_world_matrix.transform_vector(&Vector3::new(
                    lastRecord[1],
                    lastRecord[2],
                    lastRecord[3],
                ));
                tempRecordList.push([tempPoint[0], tempPoint[1], tempPoint[2]]);
            }
            lastRatioAboutIndex = lastRatioAboutIndex - 1;
            if (lastRatioAboutIndex >= 0) {
                lastRecord = particle.local_record_list[lastRatioAboutIndex];
                tempPoint = particle.start_world_matrix.transform_vector(&Vector3::new(
                    lastRecord[1],
                    lastRecord[2],
                    lastRecord[3],
                ));
                tempRecordList.push([tempPoint[0], tempPoint[1], tempPoint[2]]);
            }

            let keyPointCount = tempRecordList.len() - 1;
            let mut width = particle.trial_width * 0.5;
            if (modifier.sizeAffectsWidth) {
                width *= (particle.scaling[0] * particle.scaling[0]
                    + particle.scaling[1] * particle.scaling[1])
                    .sqrt();
            }

            for j in 0..keyPointCount {
                if (point_count < modifier.maxKeyPoint as usize) {
                    let trailAmount = j / keyPointCount;

                    tempPoint = Vector3::new(
                        tempRecordList[j][0],
                        tempRecordList[j][1],
                        tempRecordList[j][2],
                    );

                    let temp_direction = Vector3::new(
                        tempRecordList[j + 1][0],
                        tempRecordList[j + 1][1],
                        tempRecordList[j + 1][2],
                    );
                    let temp_direction = temp_direction - tempPoint;
                    let temp_direction = normalize(&temp_direction);

                    let tempView = camera_global_pos - tempPoint;
                    let tempView = normalize(&tempView);

                    let mut tempExtend = temp_direction - tempView;
                    let xSquareLength = tempExtend.magnitude_squared();
                    if (xSquareLength == 0.) {
                        tempExtend = Vector3::new(0., 1., 0.);
                    } else {
                        // TrailModifier.tempExtend.normalizeFromLength(Math.sqrt(xSquareLength));
                    }

                    match modifier.widthOverTrail.mode {
                        EInterpolationCurveMode::Curve | EInterpolationCurveMode::TwoCurves => {
                            width *= modifier
                                .widthOverTrail
                                .interpolate(trailAmount as f32, particle.base_random);
                        }
                        _ => {}
                    }

                    // if (modifier.widthOverTrail.mode == EInterpolationCurveMode.Curve || modifier.widthOverTrail.mode == EInterpolationCurveMode.TwoCurves) {
                    //     width *= modifier.widthOverTrail.interpolate(trailAmount, particle.baseRandom);
                    // }
                    let tempExtend = tempExtend * width;

                    TrailGeometryModifier::modifyPosition(
                        tempRecordList[j][0],
                        tempRecordList[j][1],
                        tempRecordList[j][2],
                        tempExtend[0],
                        tempExtend[1],
                        tempExtend[2],
                        &mut modifier.positions,
                        point_count,
                    );

                    TrailGeometryModifier::modifyUV(
                        trailAmount as f32,
                        &mut modifier.uvs,
                        point_count,
                    );

                    let mut tempColor4A = [0.; 4];
                    let mut tempColor4B = [0.; 4];
                    modifier
                        .colorOverLifetime
                        .color4Interpolate
                        .gradient
                        .interpolate(
                            max_age_amount,
                            &mut tempColor4A,
                            particle.color_over_lifetime_amount,
                        );
                    modifier
                        .colorOverTrail
                        .color4Interpolate
                        .gradient
                        .interpolate(
                            trailAmount as f32,
                            &mut tempColor4B,
                            particle.color_over_lifetime_amount,
                        );
                    tempColor4A[0] *= tempColor4B[0];
                    tempColor4A[1] *= tempColor4B[1];
                    tempColor4A[2] *= tempColor4B[2];
                    tempColor4A[3] *= tempColor4B[3];
                    if (modifier.inheritParticleColor) {
                        tempColor4A[0] *= particle.color[0];
                        tempColor4A[1] *= particle.color[1];
                        tempColor4A[2] *= particle.color[2];
                        tempColor4A[3] *= particle.color[3];
                    }
                    TrailGeometryModifier::modifyColor(
                        tempColor4A[0],
                        tempColor4A[1],
                        tempColor4A[2],
                        tempColor4A[3],
                        &mut modifier.colors,
                        point_count,
                    );

                    if (j > 0) {
                        TrailGeometryModifier::modifyIndices(&mut modifier.indices, point_count);
                    }

                    point_count += 1;
                }
            }

            tempRecordList.clear();
        }

        // modifier.geometry.setVerticesData("position", new Float32Array(modifier.positions), true);
        // modifier.geometry.setVerticesData("uv", new Float32Array(modifier.uvs), true);
        // modifier.geometry.setVerticesData("color", new Float32Array(modifier.colors), true);
        // modifier.geometry.setIndices(new Uint16Array(modifier.indices), pointCount * 2, true);

        // modifier.mesh.setEnabled(modifier._enabled && modifier.indices.length >= 3);

        // modifier.positions.length = 0;
        // modifier.indices.length = 0;
        // modifier.uvs.length = 0;
    }

    pub fn forRibbonWorldSpace(
        cameraGlobalPos: Vector3,
        particles: &mut Vec<Particle>,
        modifier: &mut TrailModifier,
    ) {
    }
    pub fn forRibbonLocalSpace(
        cameraGlobalPos: Vector3,
        particles: &mut Vec<Particle>,
        modifier: &mut TrailModifier,
    ) {
    }

    pub fn modifyGeometry(
        ax: f32,
        ay: f32,
        az: f32,
        bx: f32,
        by: f32,
        bz: f32,
        data: Vec<f32>,
        offset: f32,
    ) {
    }
}
