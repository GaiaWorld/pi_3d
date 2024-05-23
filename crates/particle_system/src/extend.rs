use pi_render::components;
use pi_scene_context::pass::pi_world::editor::EntityEditor;
use pi_scene_shell::{add_component, prelude::*};

use crate::{
    emitter::*,
    interpolation::*,
    iparticle_system_config::{IParticleSystemConfig, IShape, TParamType},
    modifier::*,
    tools::TBurstData,
    base::*,
};

pub fn format(
    entity: Entity, 
    // world: &World,
    // cmds: &mut EntityCommands, 
    editor: &mut EntityEditor,
    config: &IParticleSystemConfig) {

    // let mut components_index = vec![];
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
    // components_index.push((world.init_component()));
    // alter1.alter(entity, (base,));
    add_component(editor, entity, base);

    let shapeemitter = format_shape(Some(&config.shape));
    let emission = format_emission(config);

    // startLifetime
    let mut interpolation = FloatInterpolation::new(0.);
    parse_float_interpolation(&mut interpolation, &Some(config.lifetime.clone()), TParamType::TParamStartLifetime, 1.0,);
    let startlifetime = ParticleCalculatorStartLifetime(interpolation);
    // cmds.insert(ParticleCalculatorStartLifetime(interpolation));

    // startSpeed
    let mut interpolation = FloatInterpolation::new(0.);
    parse_float_interpolation(&mut interpolation, &Some(config.start_speed.clone()), TParamType::TParamStartSpeed, 1.,);
    let startspeed = ParticleCalculatorStartSpeed(interpolation);
    // cmds.insert(ParticleCalculatorStartSpeed(interpolation));
    
    // startSize
    let mut interpolation = StartSize::default();
    StartSize::format(&config.start_size, &mut interpolation);
    let startsize = ParticleCalculatorStartSize(interpolation);
    // cmds.insert(ParticleCalculatorStartSize(interpolation));

    // startRotation
    let mut interpolation = StartRotation::default();
    StartRotation::format(&config.start_rotation, &mut interpolation);
    let startrotation = ParticleCalculatorStartRotation(interpolation);
    // cmds.insert(ParticleCalculatorStartRotation(interpolation));

    // startColor
    let mut interpolation = StartColor::default();
    StartColor::format(&config.start_color, &mut interpolation);
    let startcolor = ParticleCalculatorStartColor(interpolation);
    // cmds.insert(ParticleCalculatorStartColor(interpolation));

    // gravity
    let mut interpolation = FloatInterpolation::new(0.);
    parse_float_interpolation(&mut interpolation, &Some(config.gravity.clone()), TParamType::TParamGravity, 1.0, );
    let gravity = ParticleCalculatorGravity(Gravity { interpolation }, Vector3::new(0., -9.8, 0.));
    // cmds.insert(ParticleCalculatorGravity(Gravity { interpolation }, Vector3::new(0., -9.8, 0.)));
    // alter2.alter(entity, /* (base,)); */
    // /* cmds.insert( */(ParticleCalculatorStartModifiers {
    //     emission,
    //     shapeemitter,
    //     startcolor,
    //     startlifetime,
    //     startrotation,
    //     startsize,
    //     startspeed,
    //     gravity,
    // },));
    add_component(editor, entity, ParticleCalculatorStartModifiers {
        emission,
        shapeemitter,
        startcolor,
        startlifetime,
        startrotation,
        startsize,
        startspeed,
        gravity,
    }).unwrap();
    // VelocityOverLifetime
    let mut velocity = None;
    if let Some(velocity_over_lifetime) = &config.velocity_over_lifetime {
        let mut interpolation = VelocityOverLifetime::default();
        TranslationInterpolate::format(velocity_over_lifetime, &mut interpolation.translation_interpolate);
        interpolation.is_local_space = if let Some(velocity_over_lifetime_is_local) = config.velocity_over_lifetime_is_local { velocity_over_lifetime_is_local != 0 } else { false };
        velocity = Some(ParticleCalculatorVelocityOverLifetime(interpolation));
    }
    // Orbit
    let mut interpolation = TranslationInterpolate::default();
    if let Some(orbtial_velocity) = &(config.orbtial_velocity) {
        TranslationInterpolate::format(orbtial_velocity, &mut interpolation);
    }
    let orbitvelocity = ParticleCalculatorOrbitVelocity(interpolation);
    // cmds.insert(ParticleCalculatorOrbitVelocity(interpolation));
    let mut interpolation = TranslationInterpolate::default();
    if let Some(orbital_offset) = &config.orbital_offset {
        TranslationInterpolate::format(orbital_offset, &mut interpolation);
    }
    let orbitoffset = ParticleCalculatorOrbitOffset(interpolation);
    // cmds.insert(ParticleCalculatorOrbitOffset(interpolation));
    let mut interpolation = FloatInterpolation::new(0.);
    if config.orbital_radial.is_some() {
        parse_float_interpolation(&mut interpolation, &config.orbital_radial, TParamType::TParamStartSpeed, 1.0);
    }
    let orbitradial = ParticleCalculatorOrbitRadial(interpolation);
    // cmds.insert(ParticleCalculatorOrbitRadial(interpolation));

    let mut speed = None;
    if config.speed_modifier.is_some() {
        let mut interpolation = SpeedModifier::default();
        parse_float_interpolation(&mut interpolation.speed_modifier, &config.speed_modifier, TParamType::TParamStartSpeed, 1.0);
        speed = Some(ParticleCalculatorSpeedModifier(interpolation));
    }
    // ForceOverLifetime
    let mut interpolation = ForceOverLifetime::default();
    if let Some(force_over_lifetime) = &config.force_over_lifetime {
        TranslationInterpolate::format(force_over_lifetime, &mut interpolation.translation_interpolate);
        interpolation.is_local_space = if let Some(force_space_is_local) = config.force_space_is_local { force_space_is_local != 0 } else { false };
    }
    let force = ParticleCalculatorForceOverLifetime(interpolation);
    // cmds.insert(ParticleCalculatorForceOverLifetime(interpolation));

    // limitVelocityOverLifetime
    let mut limitvelocity = None;
    if config.limit_velocity_over_lifetime.is_some() {
        let mut interpolation = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut interpolation, &config.limit_velocity_over_lifetime, TParamType::TParamLimitVelocityOverLifetime, 1.0);
        let dampen = if let Some(val) = config.limit_velocity_over_lifetime_dampen { val } else { 0. };
        limitvelocity = Some(ParticleCalculatorLimitVelocityOverLifetime(LimitVelocityOverLifetime { interpolation, dampen }));
    }

    // SizeOverLifetime
    let mut size = None;
    if let Some(size_over_lifetime) = &config.size_over_lifetime {
        let mut interpolation = SizeOverLifetime::default();
        ScalingInterpolate::format(size_over_lifetime, &mut interpolation.scaling_interpolate);
        size = Some(ParticleCalculatorSizeOverLifetime(interpolation));
        // cmds.insert(ParticleCalculatorSizeOverLifetime(interpolation));
    }
    // SizeBySpeed
    let mut sizebyspeed = None;
    if let Some(size_by_speed) = &config.size_by_speed {
        let mut interpolation = SizeBySpeed::default();
        ScalingInterpolate::format(&ParamInfo::OneParamInfo(size_by_speed.0.clone()), &mut interpolation.scaling_interpolate);
        interpolation.range_x = size_by_speed.1;
        interpolation.range_y = size_by_speed.2;
        sizebyspeed = Some(ParticleCalculatorSizeBySpeed(interpolation));
    }

    // RotationOverLifetime
    let mut rotation = None;
    if let Some(rotation_over_lifetime) = &config.rotation_over_lifetime {
        let mut interpolation = RotationOverLifetime::default();
        RotationInterpolate::format(rotation_over_lifetime, &mut interpolation.rotation_interpolate);
        rotation = Some(ParticleCalculatorRotationOverLifetime(interpolation));
    }
    // RotationBySpeed
    let mut rotationbyspeed = None;
    if let Some(rotation_by_speed) = &config.rotation_by_speed {
        let mut interpolation = RotationBySpeed::default();
        RotationInterpolate::format(&ParamInfo::OneParamInfo(rotation_by_speed.0.clone()), &mut interpolation.rotation_interpolate);
        interpolation.range_x = rotation_by_speed.1;
        interpolation.range_y = rotation_by_speed.2;
        rotationbyspeed= Some(ParticleCalculatorRotationBySpeed(interpolation));
    }
    
    // ColorOverLifetime
    let mut color = None;
    if let Some(color_over_lifetime) = &config.color_over_lifetime {
        let mut interpolation = ColorOverLifetime::default();
        Color4Interpolate::format(color_over_lifetime, &mut interpolation.color4_interpolate);
        color = Some(ParticleCalculatorColorOverLifetime(interpolation));
    }
    // ColorBySpeed
    let mut colorbyspeed = None;
    if let Some(color_by_speed) = &config.color_by_speed {
        let mut interpolation = ColorBySpeed::default();
        Color4Interpolate::format(&color_by_speed.0, &mut interpolation.color4_interpolate);
        interpolation.range_x = color_by_speed.1;
        interpolation.range_y = color_by_speed.2;
        colorbyspeed = Some(ParticleCalculatorColorBySpeed(interpolation));
    }


    // Texture Sheet
    let mut texturesheet = None;
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
        texturesheet = Some(ParticleCalculatorTextureSheet(t_sheet));
    }
    // alter3.alter(entity, /* components) */
    // /* cmds.insert */(ParticleCalculatorOverLifetime {
    //     orbitoffset,
    //     orbitradial,
    //     orbitvelocity,
    //     force,
    //     size,
    //     sizebyspeed,
    //     velocity,
    //     color,
    //     colorbyspeed,
    //     rotation,
    //     rotationbyspeed,
    //     speed,
    //     limitvelocity,
    //     texturesheet
    // },));
    add_component(editor, entity, ParticleCalculatorOverLifetime {
        orbitoffset,
        orbitradial,
        orbitvelocity,
        force,
        size,
        sizebyspeed,
        velocity,
        color,
        colorbyspeed,
        rotation,
        rotationbyspeed,
        speed,
        limitvelocity,
        texturesheet
    }).unwrap();
    
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

        // cmds.insert(ParticleCalculatorCustomV4 { x, y, z, w });
        // alter4.alter(entity, (ParticleCalculatorCustomV4 { x, y, z, w },));
        add_component(editor, entity, ParticleCalculatorCustomV4 { x, y, z, w }).unwrap();
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
        // alter5.alter(entity, (ParticleCalculatorTrail (ps_trail),));
        add_component(editor, entity, ParticleCalculatorTrail (ps_trail)).unwrap();
        // cmds.insert(ParticleCalculatorTrail(ps_trail));
    }
}

pub fn format_shape(shape: Option<&IShape>) -> ParticleCalculatorShapeEmitter {
    let emitter = if let Some(shape) = &shape {
        let shape_emitter = match shape {
            // 2
            IShape::ShapeBox(shape) => {
                BoxShapeEmitter::new(&shape)
            }
            // 3
            IShape::ShapeCircle(shape) => {
                CircleShapeEmitter::create(&shape)
            }
            // 0
            IShape::ShapeCone(shape) => {
                ConeShapeEmitter::create(&shape)
            }
            //5
            IShape::ShapeEdge(shape) => {
                EdgeShapeEmitter::create(&shape)
            }
            //4
            IShape::ShapeHemisphere(shape) => {
                HemisphereShapeEmitter::create(&shape)
            }
            // 6
            IShape::ShapeRectangle(shape) => {
                RectangleShapeEmitter::create(&shape)
            }
            // 1
            IShape::ShapeSphere(shape) => {
                SphereShapeEmitter::create(&shape)
            }
            IShape::Point() => {
                PointShapeEmitter::create()
            },
        };

        shape_emitter
    } else {
        PointShapeEmitter::create()
    };
    ParticleCalculatorShapeEmitter(emitter)
}

pub fn format_emission(config: &IParticleSystemConfig) -> ParticleCalculatorEmission {
    // bursts
    let mut bursts: Vec<TBurstData> = vec![];
    if let Some(v) = &config.emission.1 {
        let len = v.len();
        for i in 0..len {
            bursts.push(v[i].clone());
        }
    }
    ParticleCalculatorEmission { 
        bursts,
        rateovertime: FloatInterpolation::new(config.emission.0)
    }
}
