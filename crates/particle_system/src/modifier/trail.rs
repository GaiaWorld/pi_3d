use pi_scene_math::Vector3;

use crate::{
    interpolation::{Color4Gradient, FloatInterpolation, IInterpolation},
    iparticle_system_config::EInterpolationCurveMode,
    tools::*,
    particle::Particle,
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
    pub ribbon_count: f32,

    /**
     * ribbon 末尾是否连接到粒子系统的位置
     */
    pub attach_ribbons_to_transfoem: bool,

    /**
     * 新路径点与作为轨迹用的最后一个点之间距离超过多少，才将新作为轨迹用的点
     */
    pub minimun_vertex_distance: f32,

    _use_world_space: bool,

    /**
     * 粒子的Size是否实时影响拖尾宽度
     */
    pub size_affects_width: bool,
    /**
     * 粒子的Size是否影响拖尾的 Lifetime 参数
     */
    pub size_affects_lifetime: bool,
    /**
     * 粒子的实时颜色是否影响拖尾颜色的整体变化
     */
    pub inherit_particle_color: bool,
    /**
     * 拖尾的颜色随生命增长的整体变化
     */
    pub color_over_lifetime: ColorOverLifetime,
    /**
     * 拖尾的初始宽度
     */
    pub width_over_trail: FloatInterpolation,
    /**
     * 拖尾颜色在路径线上的分布情况
     */
    pub color_over_trail: ColorOverLifetime,

    positions: Vec<f32>,
    indices: Vec<f32>,
    colors: Vec<f32>,
    uvs: Vec<f32>,

    _geometry: Vec<f32>,
    pub mesh: Vec<f32>,

    max_key_point: f32,
    // modify_call: Box<dyn Fn(Vector3, &mut Vec<Particle>, &mut TrailModifier)>,

    /**
     * 拖尾是否在粒子死亡时即死亡
     */
    pub die_with_particle: bool,

    pub _texture_mode: ETrailTextureMode,
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
        self.update_modify();
    }

    pub fn get_mode(&self) -> ETrailMode {
        return self._mode;
    }

    /// 生成轨迹点的坐标是否使用其当时的世界坐标
    pub fn set_world_space(&mut self, value: bool) {
        self._use_world_space = value;
        self.update_modify();
    }
    pub fn get_world_space(&self) -> bool {
        return self._use_world_space;
    }

    /**
     * 拖尾的纹理平铺方式
     */
    pub fn set_texture_mode(&mut self, value: ETrailTextureMode) {
        self._texture_mode = value;
    }
    pub fn get_texture_mode(&self) -> ETrailTextureMode {
        return self._texture_mode;
    }

    pub fn new() -> Self {
        Self {
            _mode: ETrailMode::Particles,
            ratio: 1.,
            lifetime: FloatInterpolation::new(0.),
            ribbon_count: 1.,
            attach_ribbons_to_transfoem: false,
            minimun_vertex_distance: 1.,
            _use_world_space: false,
            size_affects_width: false,
            size_affects_lifetime: false,
            inherit_particle_color: false,
            color_over_lifetime: ColorOverLifetime::default(),
            width_over_trail: FloatInterpolation::new(0.),
            color_over_trail: ColorOverLifetime::default(),
            positions: vec![],
            indices: vec![],
            colors: vec![],
            uvs: vec![],
            _geometry: vec![],
            mesh: vec![],
            max_key_point: 65000. / 2.,
            // modify_call: Box::new(TrailModifier::for_particles_local_space),
            die_with_particle: true,
            _texture_mode: ETrailTextureMode::Stretch,
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
        if !value {
            // self.mesh.setEnabled(value);
        }
    }

    pub fn dispose(&self) {
        // self.mesh.dispose(true, true);
        // self.geometry.dispose();
    }

    pub fn modify(&mut self, _camera_global_pos: Vector3, _particles: &Vec<&mut Particle>) {
        // (self.modifyCall)(cameraGlobalPos, particles, self);
    }

    #[allow(unreachable_patterns)]
    pub fn update_modify(&mut self) {
        match self._mode {
            ETrailMode::Particles => {
                if self._use_world_space {
                    // self.modify_call = Box::new(TrailModifier::for_particles_world_space);
                } else {
                    // self.modify_call = Box::new(TrailModifier::for_particles_local_space);
                }
            }
            _ => {
                if self._use_world_space {
                    // self.modify_call = Box::new(TrailModifier::for_ribbon_world_space);
                } else {
                    // self.modify_call = Box::new(TrailModifier::for_ribbon_local_space);
                }
            }
        }
    }

    pub fn for_particles_world_space(
        camera_global_pos: Vector3,
        particles: &mut Vec<Particle>,
        modifier: &mut TrailModifier,
        randoms: &BaseRandom,
    ) {
        let count = particles.len();
        let mut point_count = 0;
        for i in 0..count {
            let particle = &mut particles[i];
            if particle.trial_lifetime == 0.0 {
                continue;
            }
            let max_age_amount = particle.age / particle.lifetime;
            let point_lenght = particle.global_record_list.len();
            if point_lenght == 0 {
                let temp_point = particle.direction;
                let temp_point = normalize(&temp_point);
                let temp_point = temp_point * (0.01);
                // particle.position.subtractToRef(TrailModifier.tempPoint, TrailModifier.tempPoint);
                let temp_point = particle.position - temp_point;
                // Vector3.TransformCoordinatesFromFloatsToRef(TrailModifier.tempPoint.x, TrailModifier.tempPoint.y, TrailModifier.tempPoint.z, particle.startWorldMatrix, TrailModifier.tempPoint);
                let temp_point = particle.start_world_matrix.transform_vector(&temp_point);
                particle
                    .global_record_list
                    .push([0., temp_point[0], temp_point[1], temp_point[2]]);
            }

            // Vector3.TransformCoordinatesFromFloatsToRef(particle.position.x, particle.position.y, particle.position.z, particle.startWorldMatrix, TrailModifier.tempPoint);
            let temp_point = particle
                .start_world_matrix
                .transform_vector(&particle.position);
            particle.global_record_list.push([
                max_age_amount,
                temp_point[0],
                temp_point[1],
                temp_point[2],
            ]);
            // maxAgeAmount = Math.min(1.0, maxAgeAmount);

            let record_length = particle.global_record_list.len();
            let mut temp_record_list = vec![];
            let mut last_record = particle.global_record_list[record_length - 1];

            temp_record_list.push([last_record[1], last_record[2], last_record[3]]);
            let mut last_distance_about_index = record_length as i32 - 1;
            let mut last_ratio_about_index = record_length as i32- 1;
            let wait_time_ratio = particle.trial_lifetime / particle.lifetime;
            for j in (0..=(record_length - 2)).rev() {
                let curr_record = particle.global_record_list[j];
                let delta_ratio = max_age_amount - curr_record[0];
                if 0. <= delta_ratio && delta_ratio <= wait_time_ratio {
                    let ax = last_record[1];
                    let ay = last_record[2];
                    let az = last_record[3];
                    let bx = curr_record[1];
                    let by = curr_record[2];
                    let bz = curr_record[3];

                    let dx = ax - bx;
                    let dy = ay - by;
                    let dz = az - bz;
                    let len = (dx * dx + dy * dy + dz * dz).sqrt();
                    if len >= modifier.minimun_vertex_distance || curr_record[0] == 0. {
                        last_record = curr_record;
                        last_distance_about_index = j as i32;
                        temp_record_list.push([last_record[1], last_record[2], last_record[3]]);
                    }
                    last_ratio_about_index = j as i32;
                }
            }
            if temp_record_list.len() == 1 {
                last_record = particle.global_record_list[0];
                last_distance_about_index = 0;
                last_ratio_about_index = 0;
                temp_record_list.push([last_record[1], last_record[2], last_record[3]]);
            }

            if last_distance_about_index != last_ratio_about_index {
                last_record = particle.global_record_list[last_ratio_about_index as usize];
                temp_record_list.push([last_record[1], last_record[2], last_record[3]]);
            }
            last_ratio_about_index = last_ratio_about_index - 1;
            if last_ratio_about_index >= 0 {
                last_record = particle.global_record_list[last_ratio_about_index as usize];
                temp_record_list.push([last_record[1], last_record[2], last_record[3]]);
            }

            let key_point_count = temp_record_list.len() - 1;
            let mut width = particle.trial_width * 0.5;
            if modifier.size_affects_width {
                width *= (particle.scaling[0] * particle.scaling[1]
                    + particle.scaling[1] * particle.scaling[1])
                    .sqrt();
            }

            for j in 0..key_point_count {
                if point_count < modifier.max_key_point as usize {
                    let trail_amount = j / key_point_count;

                    let temp_point = Vector3::new(
                        temp_record_list[j][0],
                        temp_record_list[j][1],
                        temp_record_list[j][2],
                    );

                    let temp_direction = Vector3::new(
                        temp_record_list[j + 1][0],
                        temp_record_list[j + 1][1],
                        temp_record_list[j + 1][2],
                    );
                    let temp_direction = temp_direction - temp_point;
                    let temp_direction = normalize(&temp_direction);

                    let temp_view = camera_global_pos - temp_point;
                    let temp_view = normalize(&temp_view);

                    // Vector3.CrossToRef(TrailModifier.tempDirection, TrailModifier.tempView, TrailModifier.tempExtend);
                    let mut temp_extend = temp_direction.cross(&temp_view);
                    let x_square_length = temp_extend.magnitude_squared();
                    if x_square_length == 0. {
                        temp_extend = Vector3::new(0., 1., 0.);
                    } else {
                        // TrailModifier.tempExtend.normalizeFromLength(Math.sqrt(xSquareLength));
                    }

                    match modifier.width_over_trail.mode {
                        EInterpolationCurveMode::Curve | EInterpolationCurveMode::TwoCurves => {
                            width *= modifier
                                .width_over_trail
                                .interpolate(trail_amount as f32, particle.base_random);
                        }
                        _ => {}
                    }

                    let temp_extend = temp_extend * width;

                    TrailGeometryModifier::modify_position(
                        temp_record_list[j][0],
                        temp_record_list[j][1],
                        temp_record_list[j][2],
                        temp_extend[0],
                        temp_extend[1],
                        temp_extend[2],
                        &mut modifier.positions,
                        point_count,
                    );

                    TrailGeometryModifier::modify_uv(
                        trail_amount as f32,
                        &mut modifier.uvs,
                        point_count,
                    );
                    let mut temp_color4_a = [0.; 4];
                    let mut temp_color4_b = [0.; 4];

                    modifier
                        .color_over_lifetime
                        .color4_interpolate
                        .gradient
                        .interpolate(
                            max_age_amount,
                            &mut temp_color4_a,
                            randoms,
                        );
                    modifier
                        .color_over_trail
                        .color4_interpolate
                        .gradient
                        .interpolate(
                            trail_amount as f32,
                            &mut temp_color4_b,
                            randoms,
                        );
                    temp_color4_a[0] *= temp_color4_b[0];
                    temp_color4_a[1] *= temp_color4_b[1];
                    temp_color4_a[2] *= temp_color4_b[2];
                    temp_color4_a[3] *= temp_color4_b[3];
                    if modifier.inherit_particle_color {
                        temp_color4_a[0] *= particle.color[0];
                        temp_color4_a[1] *= particle.color[1];
                        temp_color4_a[2] *= particle.color[2];
                        temp_color4_a[3] *= particle.color[3];
                    }
                    TrailGeometryModifier::modify_color(
                        temp_color4_a[0],
                        temp_color4_a[1],
                        temp_color4_a[2],
                        temp_color4_a[3],
                        &mut modifier.colors,
                        point_count,
                    );

                    if j > 0 {
                        TrailGeometryModifier::modify_indices(&mut modifier.indices, point_count);
                    }

                    point_count += 1;
                }
            }

            temp_record_list.clear();
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
    pub fn for_particles_local_space(
        camera_global_pos: Vector3,
        particles: &mut Vec<Particle>,
        modifier: &mut TrailModifier,
        randoms: &BaseRandom,
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
            let mut temp_record_list = vec![];
            let mut last_record = particle.local_record_list[record_length - 1];

            let mut temp_point = particle.start_world_matrix.transform_vector(&Vector3::new(
                last_record[1],
                last_record[2],
                last_record[3],
            ));
            temp_record_list.push([temp_point[0], temp_point[1], temp_point[2]]);
            let mut last_distance_about_index = record_length as i32 - 1;
            let mut last_ratio_about_index = record_length as i32 - 1;
            let wait_time_ratio = particle.trial_lifetime / particle.lifetime;
            for j in (0..=(record_length - 2)).rev() {
                let curr_record = particle.local_record_list[j];
                let delta_ratio = max_age_amount - curr_record[0];
                if delta_ratio <= wait_time_ratio {
                    let ax = last_record[1];
                    let ay = last_record[2];
                    let az = last_record[3];
                    let bx = curr_record[1];
                    let by = curr_record[2];
                    let bz = curr_record[3];

                    let dx = ax - bx;
                    let dy = ay - by;
                    let dz = az - bz;
                    let len = (dx * dx + dy * dy + dz * dz).sqrt();
                    if len >= modifier.minimun_vertex_distance || curr_record[0] == 0. {
                        last_record = curr_record;
                        last_distance_about_index = j as i32;
                        temp_point = particle.start_world_matrix.transform_vector(&Vector3::new(
                            last_record[1],
                            last_record[2],
                            last_record[3],
                        ));
                        temp_record_list.push([temp_point[0], temp_point[1], temp_point[2]]);
                    }
                    last_ratio_about_index = j as i32;
                }
            }
            if temp_record_list.len() == 1 {
                last_record = particle.local_record_list[0];
                last_distance_about_index = 0;
                last_ratio_about_index = 0;
                temp_point = particle.start_world_matrix.transform_vector(&Vector3::new(
                    last_record[1],
                    last_record[2],
                    last_record[3],
                ));
                temp_record_list.push([temp_point[0], temp_point[1], temp_point[2]]);
            }

            if last_distance_about_index != last_ratio_about_index{
                last_record = particle.local_record_list[last_ratio_about_index as usize];
                // Vector3.TransformCoordinatesFromFloatsToRef(lastRecord[1], lastRecord[2], lastRecord[3], particle.startWorldMatrix, TrailModifier.tempPoint);
                temp_point = particle.start_world_matrix.transform_vector(&Vector3::new(
                    last_record[1],
                    last_record[2],
                    last_record[3],
                ));
                temp_record_list.push([temp_point[0], temp_point[1], temp_point[2]]);
            }
            last_ratio_about_index = last_ratio_about_index - 1;
            if last_ratio_about_index >= 0 {
                last_record = particle.local_record_list[last_ratio_about_index as usize];
                temp_point = particle.start_world_matrix.transform_vector(&Vector3::new(
                    last_record[1],
                    last_record[2],
                    last_record[3],
                ));
                temp_record_list.push([temp_point[0], temp_point[1], temp_point[2]]);
            }

            let key_point_count = temp_record_list.len() - 1;
            let mut width = particle.trial_width * 0.5;
            if modifier.size_affects_width {
                width *= (particle.scaling[0] * particle.scaling[0]
                    + particle.scaling[1] * particle.scaling[1])
                    .sqrt();
            }

            for j in 0..key_point_count {
                if point_count < modifier.max_key_point as usize {
                    let trail_amount = j / key_point_count;

                    temp_point = Vector3::new(
                        temp_record_list[j][0],
                        temp_record_list[j][1],
                        temp_record_list[j][2],
                    );

                    let temp_direction = Vector3::new(
                        temp_record_list[j + 1][0],
                        temp_record_list[j + 1][1],
                        temp_record_list[j + 1][2],
                    );
                    let temp_direction = temp_direction - temp_point;
                    let temp_direction = normalize(&temp_direction);

                    let temp_view = camera_global_pos - temp_point;
                    let temp_view = normalize(&temp_view);

                    let mut temp_extend = temp_direction - temp_view;
                    let x_square_length = temp_extend.magnitude_squared();
                    if x_square_length == 0. {
                        temp_extend = Vector3::new(0., 1., 0.);
                    } else {
                        // TrailModifier.tempExtend.normalizeFromLength(Math.sqrt(xSquareLength));
                    }

                    match modifier.width_over_trail.mode {
                        EInterpolationCurveMode::Curve | EInterpolationCurveMode::TwoCurves => {
                            width *= modifier
                                .width_over_trail
                                .interpolate(trail_amount as f32, particle.base_random);
                        }
                        _ => {}
                    }

                    // if (modifier.widthOverTrail.mode == EInterpolationCurveMode.Curve || modifier.widthOverTrail.mode == EInterpolationCurveMode.TwoCurves) {
                    //     width *= modifier.widthOverTrail.interpolate(trailAmount, particle.baseRandom);
                    // }
                    let temp_extend = temp_extend * width;

                    TrailGeometryModifier::modify_position(
                        temp_record_list[j][0],
                        temp_record_list[j][1],
                        temp_record_list[j][2],
                        temp_extend[0],
                        temp_extend[1],
                        temp_extend[2],
                        &mut modifier.positions,
                        point_count,
                    );

                    TrailGeometryModifier::modify_uv(
                        trail_amount as f32,
                        &mut modifier.uvs,
                        point_count,
                    );

                    let mut temp_color4_a = [0.; 4];
                    let mut temp_color4_b = [0.; 4];
                    modifier
                        .color_over_lifetime
                        .color4_interpolate
                        .gradient
                        .interpolate(
                            max_age_amount,
                            &mut temp_color4_a,
                            randoms,
                        );
                    modifier
                        .color_over_trail
                        .color4_interpolate
                        .gradient
                        .interpolate(
                            trail_amount as f32,
                            &mut temp_color4_b,
                            randoms,
                        );
                    temp_color4_a[0] *= temp_color4_b[0];
                    temp_color4_a[1] *= temp_color4_b[1];
                    temp_color4_a[2] *= temp_color4_b[2];
                    temp_color4_a[3] *= temp_color4_b[3];
                    if modifier.inherit_particle_color {
                        temp_color4_a[0] *= particle.color[0];
                        temp_color4_a[1] *= particle.color[1];
                        temp_color4_a[2] *= particle.color[2];
                        temp_color4_a[3] *= particle.color[3];
                    }
                    TrailGeometryModifier::modify_color(
                        temp_color4_a[0],
                        temp_color4_a[1],
                        temp_color4_a[2],
                        temp_color4_a[3],
                        &mut modifier.colors,
                        point_count,
                    );

                    if j > 0 {
                        TrailGeometryModifier::modify_indices(&mut modifier.indices, point_count);
                    }

                    point_count += 1;
                }
            }

            temp_record_list.clear();
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

    pub fn for_ribbon_world_space(
        _camera_global_pos: Vector3,
        _particles: &mut Vec<Particle>,
        _modifier: &mut TrailModifier,
    ) {
    }
    pub fn for_ribbon_local_space(
        _camera_global_pos: Vector3,
        _particles: &mut Vec<Particle>,
        _modifier: &mut TrailModifier,
    ) {
    }

    pub fn modify_geometry(
        _ax: f32,
        _ay: f32,
        _az: f32,
        _bx: f32,
        _by: f32,
        _bz: f32,
        _data: Vec<f32>,
        _offset: f32,
    ) {
    }
}
