
use pi_scene_shell::{prelude::*, frame_time::PluginFrameTime,};

#[derive(Component, Default)]
pub struct ParticleSysStartLifetime { }

#[derive(Component, Default)]
pub struct ParticleSysStartColor { }

#[derive(Component, Default)]
pub struct ParticleSysStartSpeed { }

#[derive(Component, Default)]
pub struct ParticleSysStartScaling { }

#[derive(Component, Default)]
pub struct ParticleSysGravity { }

#[derive(Component, Default)]
pub struct ParticleSysStartRotation { }

#[derive(Component, Default)]
pub struct ParticleSysColorOverLifetime { }

#[derive(Component, Default)]
pub struct ParticleSysVelocityOverLifetime { }

#[derive(Component, Default)]
pub struct ParticleSysSizeOverLifetime { }

#[derive(Component, Default)]
pub struct ParticleSysLimitVelocityOverLifetime { }

#[derive(Component, Default)]
pub struct ParticleSysForceOverLifetime { }

#[derive(Component, Default)]
pub struct ParticleSysRotationOverLifetime { }

#[derive(Component, Default)]
pub struct ParticleSysRotationBySpeed { }

#[derive(Component, Default)]
pub struct ParticleSysColorBySpeed { }

#[derive(Component, Default)]
pub struct ParticleSysTextureSheet { }

#[derive(Component, Default)]
pub struct ParticleAge(Vec<f32>);

#[derive(Component, Default)]
pub struct ParticleLifetime(Vec<f32>);

#[derive(Component, Default)]
pub struct ParticleStartWorldMatrix(Vec<Matrix>);

#[derive(Component, Default)]
pub struct ParticleEmitMatrix(Vec<Matrix>);

#[derive(Component, Default)]
pub struct ParticleLocalMatrix(Vec<Matrix>);

#[derive(Component, Default)]
pub struct ParticleLocalRotation(Vec<Vector3>);

#[derive(Component, Default)]
pub struct ParticleLocalPosition(Vec<Vector3>);

#[derive(Component, Default)]
pub struct ParticleLocalScaling(Vec<Vector3>);

#[derive(Component, Default)]
pub struct ParticleColor(Vec<Vector4>);

#[derive(Component, Default)]
pub struct ParticleStartColor(Vec<Vector4>);

#[derive(Component, Default)]
pub struct ParticleStartScaling(Vec<Vector3>);

#[derive(Component, Default)]
pub struct ParticleStartRotation(Vec<Vector3>);

#[derive(Component, Default)]
pub struct ParticleVelocity(Vec<Vector3>);

#[derive(Component, Default)]
pub struct ParticleDirection(Vec<Vector3>);

#[derive(Component, Default)]
pub struct ParticleUV(Vec<Vector4>);


#[derive(Component, Default)]
pub struct ParticleGlobalPosList(Vec<Vec<Vector3>>);

#[derive(Component, Default)]
pub struct ParticleLocalPosList(Vec<Vec<Vector3>>);
