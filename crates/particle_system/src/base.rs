
use pi_assets::asset::{Asset, Size};
use pi_engine_shell::prelude::*;
use pi_scene_math::*;


pub type TCurveTime = Number;
pub type TCurveValue = Number;
pub type TCurveInTangent = Number;
pub type TCurveOutTangent = Number;
pub type TCurveScalar = Number;

pub enum TCurveMode {
    /**
     * 静态数值
     */
    Constant,
    /**
     * 静态数值随机 - XYZ 随机值相同
     */
    TwoConstants,
    /**
     * 曲线插值
     */
    Curve,
    /**
     * 曲线插值
     */
    TwoCurves,
    Random,
}

pub enum TGradienMode {
    /**
     * 静态数值
     */
    Color,
    /**
     * 静态数值随机 - XYZ 随机值相同
     */
    TwoColors,
    /**
     * 曲线插值
     */
    Gradient,
    /**
     * 曲线插值
     */
    TwoGradients,
    Random,
}

pub struct ICurveKey(TCurveTime, TCurveValue, TCurveInTangent, TCurveOutTangent, TCurveMode);

pub struct ICurve(Vec<ICurveKey>, TCurveScalar);

pub struct ParticleSystemCalculatorID(pub Entity, pub usize);
impl Drop for ParticleSystemCalculatorID {
    fn drop(&mut self) {
        
    }
}
impl Asset for ParticleSystemCalculatorID {
    type Key = u64;
}
impl Size for ParticleSystemCalculatorID {
    fn size(&self) -> usize {
        self.1
    }
}

// pub struct ParticleSystemCalculatorManager(pub ShareAssetMgr<>)

#[derive(Component)]
pub struct ParticleSysBursts { }

#[derive(Component)]
pub struct ParticleSysRateOverTime { }

#[derive(Component)]
pub struct ParticleSysEmitterShape { }

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
pub struct ParticleSysRenderPivot { }

#[derive(Component)]
pub struct ParticleSysRenderMode { }

#[derive(Component)]
pub struct ParticleSysRenderAlignment { }

#[derive(Component)]
pub struct ParticleSysRenderSimulationSpace { }

#[derive(Component)]
pub struct ParticleSysRenderScalingSpace { }

#[derive(Component)]
pub struct ParticleSysLooping { }

#[derive(Component)]
pub struct ParticleSysPrewarm { }

#[derive(Component)]
pub struct ParticleSysDuration { }


#[derive(Component)]
pub struct ParticleSysEmittedLoop { }

#[derive(Component)]
pub struct ParticleSysEmittingProgress { }

#[derive(Component)]
pub struct ParticleAge(Vec<f32>);

#[derive(Component)]
pub struct ParticleLifetime(Vec<f32>);

#[derive(Component)]
pub struct ParticleStartColor(Vec<Vector4>);

#[derive(Component)]
pub struct ParticleStartScaling(Vec<Vector3>);

#[derive(Component)]
pub struct ParticleStartRotation(Vec<Vector3>);


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
pub struct ParticleVelocity(Vec<Vector3>);

#[derive(Component)]
pub struct ParticleDirection(Vec<Vector3>);

#[derive(Component)]
pub struct ParticleUV(Vec<Vector4>);


#[derive(Component)]
pub struct ParticleGlobalPosList(Vec<Vec<Vector3>>);

#[derive(Component)]
pub struct ParticleLocalPosList(Vec<Vec<Vector3>>);
