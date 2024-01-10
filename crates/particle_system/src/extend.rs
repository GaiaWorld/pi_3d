use pi_scene_shell::prelude::*;
use pi_scene_math::{*, coordiante_system::CoordinateSytem3, vector::TToolMatrix};

use crate::{
    emitter::*,
    interpolation::*,
    iparticle_system_config::{IParticleSystemConfig, IShape, TParamType},
    modifier::*,
    tools::TBurstData,
    base::*,
};

pub fn format(cmds: &mut EntityCommands, config: &IParticleSystemConfig) {
    let mut base = ParticleCalculatorBase {
        looping: config.looping == 1,
        prewarm: config.prewarm,
        delay: (config.start_delay * 1000.) as u32,
        duration: (config.duration * 1000.) as u32 ,
        maxcount: config.max_particles as usize,
        scaling_space: config.scaling_mode.mode(),
        simulation_space: config.simulation_space_is_world.mode(),
        render_alignment: config.render_alignment,
        render_mode: config.render_mode,
        stretched_length_scale: if config.stretched_length_scale == 0.0 {
            1.0
        } else {
            config.stretched_length_scale
        },
        stretched_velocity_scale: config.stretched_velocity_scale,
        pivot: Vector3::zeros(),
    };
    if let Some(render_pivot) = config.render_pivot {
        base.pivot = Vector3::new(render_pivot[0], render_pivot[1], render_pivot[2]);
    }
    cmds.insert(base);

    format_shape(cmds, Some(&config.shape));
    format_emission(cmds, config);

    // startLifetime
    let mut interpolation = FloatInterpolation::new(0.);
    parse_float_interpolation(&mut interpolation, &Some(config.lifetime.clone()), TParamType::TParamStartLifetime, 1.0,);
    cmds.insert(ParticleCalculatorStartLifetime(interpolation));

    // startSpeed
    let mut interpolation = FloatInterpolation::new(0.);
    parse_float_interpolation(&mut interpolation, &Some(config.start_speed.clone()), TParamType::TParamStartSpeed, 1.,);
    cmds.insert(ParticleCalculatorStartSpeed(interpolation));
    
    // startSize
    let mut interpolation = StartSize::default();
    StartSize::format(&config.start_size, &mut interpolation);
    cmds.insert(ParticleCalculatorStartSize(interpolation));

    // startRotation
    let mut interpolation = StartRotation::default();
    StartRotation::format(&config.start_rotation, &mut interpolation);
    cmds.insert(ParticleCalculatorStartRotation(interpolation));

    // startColor
    let mut interpolation = StartColor::default();
    StartColor::format(&config.start_color, &mut interpolation);
    cmds.insert(ParticleCalculatorStartColor(interpolation));

    // gravity
    let mut interpolation = FloatInterpolation::new(0.);
    parse_float_interpolation(&mut interpolation, &Some(config.gravity.clone()), TParamType::TParamGravity, 1.0, );
    cmds.insert(ParticleCalculatorGravity(interpolation));

    // VelocityOverLifetime
    if let Some(velocity_over_lifetime) = &config.velocity_over_lifetime {
        let mut interpolation = VelocityOverLifetime::default();
        TranslationInterpolate::format(velocity_over_lifetime, &mut interpolation.translation_interpolate);
        interpolation.is_local_space = if let Some(velocity_over_lifetime_is_local) = config.velocity_over_lifetime_is_local { velocity_over_lifetime_is_local != 0 } else { false };
        cmds.insert(ParticleCalculatorVelocityOverLifetime(interpolation));
    }
    // Orbit
    if let Some(orbtial_velocity) = &(config.orbtial_velocity) {
        let mut interpolation = TranslationInterpolate::default();
        TranslationInterpolate::format(orbtial_velocity, &mut interpolation);
        cmds.insert(ParticleCalculatorOrbitVelocity(interpolation));
    }
    if let Some(orbital_offset) = &config.orbital_offset {
        let mut interpolation = TranslationInterpolate::default();
        TranslationInterpolate::format(orbital_offset, &mut interpolation);
        cmds.insert(ParticleCalculatorOrbitOffset(interpolation));
    }
    if config.orbital_radial.is_some() {
        let mut interpolation = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut interpolation, &config.orbital_radial, TParamType::TParamStartSpeed, 1.0);
        cmds.insert(ParticleCalculatorOrbitRadial(interpolation));
    }
    if config.speed_modifier.is_some() {
        let mut interpolation = SpeedModifier::default();
        parse_float_interpolation(&mut interpolation.speed_modifier, &config.speed_modifier, TParamType::TParamStartSpeed, 1.0);
        cmds.insert(ParticleCalculatorSpeedModifier(interpolation));
    }
    // ForceOverLifetime
    if let Some(force_over_lifetime) = &config.force_over_lifetime {
        let mut interpolation = ForceOverLifetime::default();
        TranslationInterpolate::format(force_over_lifetime, &mut interpolation.translation_interpolate);
        interpolation.is_local_space = if let Some(force_space_is_local) = config.force_space_is_local { force_space_is_local != 0 } else { false };
        cmds.insert(ParticleCalculatorForceOverLifetime(interpolation));
    }
    // limitVelocityOverLifetime
    if config.limit_velocity_over_lifetime.is_some() {
        let mut interpolation = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut interpolation, &config.limit_velocity_over_lifetime, TParamType::TParamLimitVelocityOverLifetime, 1.0);
        let dampen = if let Some(val) = config.limit_velocity_over_lifetime_dampen { val } else { 0. };
        cmds.insert(ParticleCalculatorLimitVelocityOverLifetime(LimitVelocityOverLifetime { interpolation, dampen }));
    }

    // SizeOverLifetime
    if let Some(size_over_lifetime) = &config.size_over_lifetime {
        let mut interpolation = SizeOverLifetime::default();
        ScalingInterpolate::format(size_over_lifetime, &mut interpolation.scaling_interpolate);
        cmds.insert(ParticleCalculatorSizeOverLifetime(interpolation));
    }
    // SizeBySpeed
    if let Some(size_by_speed) = &config.size_by_speed {
        let mut interpolation = SizeBySpeed::default();
        ScalingInterpolate::format(&ParamInfo::OneParamInfo(size_by_speed.0.clone()), &mut interpolation.scaling_interpolate);
        interpolation.range_x = size_by_speed.1;
        interpolation.range_y = size_by_speed.2;
        cmds.insert(ParticleCalculatorSizeBySpeed(interpolation));
    }

    // RotationOverLifetime
    if let Some(rotation_over_lifetime) = &config.rotation_over_lifetime {
        let mut interpolation = RotationOverLifetime::default();
        RotationInterpolate::format(rotation_over_lifetime, &mut interpolation.rotation_interpolate);
        cmds.insert(ParticleCalculatorRotationOverLifetime(interpolation));
    }
    // RotationBySpeed
    if let Some(rotation_by_speed) = &config.rotation_by_speed {
        let mut interpolation = RotationBySpeed::default();
        RotationInterpolate::format(&ParamInfo::OneParamInfo(rotation_by_speed.0.clone()), &mut interpolation.rotation_interpolate);
        interpolation.range_x = rotation_by_speed.1;
        interpolation.range_y = rotation_by_speed.2;
        cmds.insert(ParticleCalculatorRotationBySpeed(interpolation));
    }
    
    // ColorOverLifetime
    if let Some(color_over_lifetime) = &config.color_over_lifetime {
        let mut interpolation = ColorOverLifetime::default();
        Color4Interpolate::format(color_over_lifetime, &mut interpolation.color4_interpolate);
        cmds.insert(ParticleCalculatorColorOverLifetime(interpolation));
    }
    // ColorBySpeed
    if let Some(color_by_speed) = &config.color_by_speed {
        let mut interpolation = ColorBySpeed::default();
        Color4Interpolate::format(&color_by_speed.0, &mut interpolation.color4_interpolate);
        interpolation.range_x = color_by_speed.1;
        interpolation.range_y = color_by_speed.2;
        cmds.insert(ParticleCalculatorColorBySpeed(interpolation));
    }

    // Texture Sheet
    if let Some(texture_sheet) = &(config.texture_sheet) {
        let mut frame_over_time = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut frame_over_time, &Some(texture_sheet.frame_over_time.clone()), TParamType::TParamTextureSheet, 1.0);
        let mut start_frame = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut start_frame, &Some(texture_sheet.start_frame.clone()), TParamType::TParamTextureSheet, 1.0);
        let mut t_sheet = TextureSheet::new(frame_over_time, start_frame);
        t_sheet.anim_mode = texture_sheet.anim_mode.clone();
        t_sheet.custom_row = texture_sheet.custom_row;
        t_sheet.cycles = texture_sheet.cycles;
        t_sheet.row_mode = texture_sheet.row_mode.clone();
        t_sheet.set_tiles_x(texture_sheet.tiles_x);
        t_sheet.set_tiles_y(texture_sheet.tiles_y);
        t_sheet.time_mode = texture_sheet.time_mode.clone();
        cmds.insert(ParticleCalculatorTextureSheet(t_sheet));
    }
    
    // Custom Data V4
    if let Some(custom1) = &(config.custom1) {
        let mut x = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut x, &Some(custom1[0].clone()), TParamType::TParamStartLifetime, 1.0);
        let mut y = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut y, &Some(custom1[1].clone()), TParamType::TParamStartLifetime, 1.0);
        let mut z = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut z, &Some(custom1[2].clone()), TParamType::TParamStartLifetime, 1.0);
        let mut w = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut w, &Some(custom1[3].clone()), TParamType::TParamStartLifetime, 1.0);

        cmds.insert(ParticleCalculatorCustomV4 { x, y, z, w });
    }
    // Trail
    if let Some(trail) = &(config.trail) {
        let mut ps_trail = TrailModifier::new();
        // ps_trail.set_mode(trail.mode);
        ps_trail.mode = trail.mode;
        ps_trail.ratio = trail.ratio;
        parse_float_interpolation(
            &mut ps_trail.lifetime,
            &Some(trail.lifetime.clone()),
            TParamType::None,
            1.0,
        );
        ps_trail.ribbon_count = trail.ribbon_count;
        ps_trail.attach_ribbons_to_transfoem = trail.attach_rtt == 1;
        ps_trail.minimun_vertex_distance = trail.min_dist;
        ps_trail.use_world_space = trail.world_space == 1;
        ps_trail.die_with_particle = trail.die_with == 1;
        ps_trail.size_affects_width = trail.size_awidth == 1;
        ps_trail.texture_mode = trail.tex_mode;
        ps_trail.size_affects_lifetime = trail.size_alifetime == 1;
        ps_trail.inherit_particle_color = trail.inherit_color == 1;
        parse_color4_gradient(
            &mut ps_trail.color_over_lifetime.color4_interpolate.gradient,
            Some(&trail.color_over_life),
            TParamType::None,
        );
        parse_float_interpolation(
            &mut ps_trail.width_over_trail,
            &Some(trail.width_over_trail.clone()),
            TParamType::None,
            1.0,
        );
        parse_color4_gradient(
            &mut ps_trail.color_over_trail.color4_interpolate.gradient,
            Some(&trail.color_over_trail),
            TParamType::None,
        );
        cmds.insert(ParticleCalculatorTrail(ps_trail));
    }
}

pub fn format_shape(cmds: &mut EntityCommands, shape: Option<&IShape>) {
    let emitter = if let Some(shape) = &shape {
        let mut _pos = Vector3::zeros();
        let mut _rotation = Vector3::new(1., 1., 1.);
        let mut _scale = Vector3::zeros();
        let mut _randomize = None;
        let mut _align_dir = 0;
        let mut shape_emitter: ShapeEmitter = match shape {
            // 2
            IShape::ShapeBox(shape) => {
                let mut temp = BoxShapeEmitter::new();
                temp.emit_mode = if let Some(mode) = &shape.box_emit_mode {
                    *mode
                } else {
                    EBoxShapeMode::Volume
                };
                if let Some(val) = &shape.position { _pos.copy_from_slice(val); } else { _pos.copy_from_slice(&[0., 0., 0.]) };
                if let Some(val) = &shape.rotation { _rotation.copy_from_slice(val); } else { _rotation.copy_from_slice(&[0., 0., 0.]) };
                if let Some(val) = &shape.scale { _scale.copy_from_slice(val); } else { _scale.copy_from_slice(&[1., 1., 1.]) };
                // _pos = shape.position.clone();
                // _rotation = shape.rotation.clone();
                // _scale = shape.scale.clone();
                _randomize = shape.randomize.clone();
                _align_dir = shape.align_dir;
                CoordinateSytem3::matrix4_compose_euler_angle(&_scale, &_rotation, &_pos, &mut temp.local_matrix);
                ShapeEmitter::Box(temp)
            }
            // 3
            IShape::ShapeCircle(shape) => {
                let mut temp = CircleShapeEmitter::new(shape.radius, shape.radius_thickness);
                let (mode, value, spread, speed) = match &shape.arc {
                    crate::iparticle_system_config::IShapeArc::IShapeArcRandom(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcLoop(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcPingPong(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcBurstSpread(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                };
                temp.arc_mode = mode;
                temp.arc_value = value * std::f32::consts::PI / 180.;
                temp.arc_spread = spread;
                temp.arc_speed = speed;

                if let Some(val) = &shape.position { _pos.copy_from_slice(val); } else { _pos.copy_from_slice(&[0., 0., 0.]) };
                if let Some(val) = &shape.rotation { _rotation.copy_from_slice(val); } else { _rotation.copy_from_slice(&[0., 0., 0.]) };
                if let Some(val) = &shape.scale { _scale.copy_from_slice(val); } else { _scale.copy_from_slice(&[1., 1., 1.]) };
                // _pos = shape.position.clone();
                // _rotation = shape.rotation.clone();
                // _scale = shape.scale.clone();
                _randomize = shape.randomize.clone();
                _align_dir = shape.align_dir;
                CoordinateSytem3::matrix4_compose_euler_angle(&_scale, &_rotation, &_pos, &mut temp.local_matrix);
                ShapeEmitter::Circle(temp)
            }
            // 0
            IShape::ShapeCone(shape) => {
                let mut temp =
                    ConeShapeEmitter::new(shape.radius, shape.angle * std::f32::consts::PI / 180.);
                temp.radius_range = shape.radius_thickness;
                temp.set_height(shape.height);
                temp.height_range = if shape.emit_as_volume { 1.0 } else { 0.0 };
                let (mode, value, spread, speed) = match &shape.arc {
                    crate::iparticle_system_config::IShapeArc::IShapeArcRandom(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcLoop(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcPingPong(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcBurstSpread(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                };

                temp.arc_mode = mode;
                temp.arc_value = value * std::f32::consts::PI / 180.;
                temp.arc_spread = spread;
                temp.arc_speed = speed;

                if let Some(val) = &shape.position { _pos.copy_from_slice(val); } else { _pos.copy_from_slice(&[0., 0., 0.]) };
                if let Some(val) = &shape.rotation { _rotation.copy_from_slice(val); } else { _rotation.copy_from_slice(&[0., 0., 0.]) };
                if let Some(val) = &shape.scale { _scale.copy_from_slice(val); } else { _scale.copy_from_slice(&[1., 1., 1.]) };
                // _pos = shape.position.clone();
                // _rotation = shape.rotation.clone();
                // _scale = shape.scale.clone();
                _randomize = shape.randomize.clone();
                _align_dir = shape.align_dir;
                CoordinateSytem3::matrix4_compose_euler_angle(&_scale, &_rotation, &_pos, &mut temp.local_matrix);
                ShapeEmitter::Cone(temp)
            }
            //5
            IShape::ShapeEdge(shape) => {
                let mut temp = EdgeShapeEmitter::new();
                temp.size = shape.radius;
                let (mode, value, spread, speed) = match &shape.arc {
                    crate::iparticle_system_config::IShapeArc::IShapeArcRandom(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcLoop(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcPingPong(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcBurstSpread(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                };
                temp.arc_mode = mode;
                temp.arc_value = value * std::f32::consts::PI / 180.;
                temp.arc_spread = spread;
                temp.arc_speed = speed;

                if let Some(val) = &shape.position { _pos.copy_from_slice(val); } else { _pos.copy_from_slice(&[0., 0., 0.]) };
                if let Some(val) = &shape.rotation { _rotation.copy_from_slice(val); } else { _rotation.copy_from_slice(&[0., 0., 0.]) };
                if let Some(val) = &shape.scale { _scale.copy_from_slice(val); } else { _scale.copy_from_slice(&[1., 1., 1.]) };
                // _pos = shape.position.clone();
                // _rotation = shape.rotation.clone();
                // _scale = shape.scale.clone();
                _randomize = shape.randomize.clone();
                _align_dir = shape.align_dir;
                CoordinateSytem3::matrix4_compose_euler_angle(&_scale, &_rotation, &_pos, &mut temp.local_matrix);
                ShapeEmitter::Edge(temp)
            }
            //4
            IShape::ShapeHemisphere(shape) => {
                let mut temp = HemisphereShapeEmitter::new(shape.radius, shape.radius_thickness);
                let (mode, value, spread, speed) = match &shape.arc {
                    crate::iparticle_system_config::IShapeArc::IShapeArcRandom(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcLoop(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcPingPong(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcBurstSpread(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                };
                temp.arc_mode = mode;
                temp.arc_value = value * std::f32::consts::PI / 180.;
                temp.arc_spread = spread;
                temp.arc_speed = speed;

                if let Some(val) = &shape.position { _pos.copy_from_slice(val); } else { _pos.copy_from_slice(&[0., 0., 0.]) };
                if let Some(val) = &shape.rotation { _rotation.copy_from_slice(val); } else { _rotation.copy_from_slice(&[0., 0., 0.]) };
                if let Some(val) = &shape.scale { _scale.copy_from_slice(val); } else { _scale.copy_from_slice(&[1., 1., 1.]) };
                // _pos = shape.position.clone();
                // _rotation = shape.rotation.clone();
                // _scale = shape.scale.clone();
                _randomize = shape.randomize.clone();
                _align_dir = shape.align_dir;
                CoordinateSytem3::matrix4_compose_euler_angle(&_scale, &_rotation, &_pos, &mut temp.local_matrix);
                ShapeEmitter::Hemisphere(temp)
            }
            // 6
            IShape::ShapeRectangle(shape) => {
                let mut temp = RectangleShapeEmitter::new();

                if let Some(val) = &shape.position { _pos.copy_from_slice(val); } else { _pos.copy_from_slice(&[0., 0., 0.]) };
                if let Some(val) = &shape.rotation { _rotation.copy_from_slice(val); } else { _rotation.copy_from_slice(&[0., 0., 0.]) };
                if let Some(val) = &shape.scale { _scale.copy_from_slice(val); } else { _scale.copy_from_slice(&[1., 1., 1.]) };
                // _pos = shape.position.clone();
                // _rotation = shape.rotation.clone();
                // _scale = shape.scale.clone();
                _randomize = shape.randomize.clone();
                _align_dir = shape.align_dir;
                CoordinateSytem3::matrix4_compose_euler_angle(&_scale, &_rotation, &_pos, &mut temp.local_matrix);
                ShapeEmitter::Rectangle(temp)
            }
            // 1
            IShape::ShapeSphere(shape) => {
                let mut temp = SphereShapeEmitter::new(shape.radius, shape.radius_thickness);
                let (mode, value, spread, speed) = match &shape.arc {
                    crate::iparticle_system_config::IShapeArc::IShapeArcRandom(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcLoop(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcPingPong(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcBurstSpread(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                };

                temp.arc_mode = mode;
                temp.arc_value = value * std::f32::consts::PI / 180.;
                temp.arc_spread = spread;
                temp.arc_speed = speed;

                if let Some(val) = &shape.position { _pos.copy_from_slice(val); } else { _pos.copy_from_slice(&[0., 0., 0.]) };
                if let Some(val) = &shape.rotation { _rotation.copy_from_slice(val); } else { _rotation.copy_from_slice(&[0., 0., 0.]) };
                if let Some(val) = &shape.scale { _scale.copy_from_slice(val); } else { _scale.copy_from_slice(&[1., 1., 1.]) };
                // _pos = shape.position.clone();
                // _rotation = shape.rotation.clone();
                // _scale = shape.scale.clone();
                _randomize = shape.randomize.clone();
                _align_dir = shape.align_dir;
                CoordinateSytem3::matrix4_compose_euler_angle(&_scale, &_rotation, &_pos, &mut temp.local_matrix);
                ShapeEmitter::Sphere(temp)
            }
            IShape::Point() => {
                ShapeEmitter::Point(PointShapeEmitter::new())
            },
        };

        // if let (Some(pos), Some(rotation), Some(scale)) = (_pos, _rotation, _scale) {
        //     shape_emitter.position(Vector3::new(pos[0], pos[1], pos[2]));
        //     shape_emitter.rotation(Vector3::new(rotation[0], rotation[1], rotation[2]));
        //     shape_emitter.scaling(Vector3::new(scale[0], scale[1], scale[2]));
        // }

        shape_emitter.align_direction(_align_dir != 0);

        if let Some(randomize) = &_randomize {
            shape_emitter.randomize_direction(randomize[0]);
            shape_emitter.spherize_direction(randomize[1]);
            shape_emitter.randomize_position(randomize[2]);
        }

        shape_emitter
    } else {
        ShapeEmitter::Point(PointShapeEmitter::new())
    };

    cmds.insert(ParticleCalculatorShapeEmitter(emitter));
}

pub fn format_emission(cmds: &mut EntityCommands, config: &IParticleSystemConfig) {
    // bursts
    let mut bursts: Vec<TBurstData> = vec![];
    if let Some(v) = &config.emission.1 {
        let len = v.len();
        for i in 0..len {
            bursts.push(v[i].clone());
        }
    }
    let emission = ParticleCalculatorEmission { 
        bursts,
        rateovertime: FloatInterpolation::new(config.emission.0)
    };

    cmds.insert(emission);
}
