
use pi_engine_shell::{prelude::*, frame_time::PluginFrameTime,};

#[derive(Component)]
pub struct ParticleSysStartLifetime { }

#[derive(Component)]
pub struct ParticleSysStartColor { }

#[derive(Component)]
pub struct ParticleSysStartSpeed { }

#[derive(Component)]
pub struct ParticleSysStartScaling { }

#[derive(Component)]
pub struct ParticleSysGravity { }

#[derive(Component)]
pub struct ParticleSysStartRotation { }

#[derive(Component)]
pub struct ParticleSysColorOverLifetime { }

#[derive(Component)]
pub struct ParticleSysVelocityOverLifetime { }

#[derive(Component)]
pub struct ParticleSysSizeOverLifetime { }

#[derive(Component)]
pub struct ParticleSysLimitVelocityOverLifetime { }

#[derive(Component)]
pub struct ParticleSysForceOverLifetime { }

#[derive(Component)]
pub struct ParticleSysRotationOverLifetime { }

#[derive(Component)]
pub struct ParticleSysRotationBySpeed { }

#[derive(Component)]
pub struct ParticleSysColorBySpeed { }

#[derive(Component)]
pub struct ParticleSysTextureSheet { }

#[derive(Component)]
pub struct ParticleAge(Vec<f32>);

#[derive(Component)]
pub struct ParticleLifetime(Vec<f32>);

#[derive(Component)]
pub struct ParticleStartWorldMatrix(Vec<Matrix>);

#[derive(Component)]
pub struct ParticleEmitMatrix(Vec<Matrix>);

#[derive(Component)]
pub struct ParticleLocalMatrix(Vec<Matrix>);

#[derive(Component)]
pub struct ParticleLocalRotation(Vec<Vector3>);

#[derive(Component)]
pub struct ParticleLocalPosition(Vec<Vector3>);

#[derive(Component)]
pub struct ParticleLocalScaling(Vec<Vector3>);

#[derive(Component)]
pub struct ParticleColor(Vec<Vector4>);

#[derive(Component)]
pub struct ParticleStartColor(Vec<Vector4>);

#[derive(Component)]
pub struct ParticleStartScaling(Vec<Vector3>);

#[derive(Component)]
pub struct ParticleStartRotation(Vec<Vector3>);

#[derive(Component)]
pub struct ParticleVelocity(Vec<Vector3>);

#[derive(Component)]
pub struct ParticleDirection(Vec<Vector3>);

#[derive(Component)]
pub struct ParticleUV(Vec<Vector4>);


#[derive(Component)]
pub struct ParticleGlobalPosList(Vec<Vec<Vector3>>);

#[derive(Component)]
pub struct ParticleLocalPosList(Vec<Vec<Vector3>>);
