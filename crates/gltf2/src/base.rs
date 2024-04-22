use std::ops::Range;

use pi_curves::curve::{frame::FrameDataValue, frame_curve::{FrameCurve, frames::interplate_frame_values_step}};
use pi_scene_shell::prelude::*;
use pi_gltf::animation::Interpolation;
use pi_scene_context::prelude::*;
use pi_scene_math::*;

#[derive(Clone, Copy)]
pub enum EAnimePropertyType {
    LocalPosition       =  0,
    LocalRotation       =  1,
    LocalScaling        =  2, 
    MainTexUScale       =  3, 
    MainTexVScale       =  4,
    MainTexUOffset      =  5, 
    MainTexVOffset      =  6,
    Alpha               =  7, 
    MainColor           =  8, 
    CameraOrthSize      =  9, 
    CameraFov           = 10,
    Enable              = 11,
    LocalEulerAngles    = 12,
    Intensity           = 13,
    LightDiffuse        = 14,
    AlphaCutoff         = 15,
    CellId              = 16,
    OpacityTexUScale    = 17,
    OpacityTexVScale    = 18,
    OpacityTexUOffset   = 19,
    OpacityTexVOffset   = 20,
    MaskCutoff          = 21,
    MaskTexUScale       = 22,
    MaskTexVScale       = 23,
    MaskTexUOffset      = 24,
    MaskTexVOffset      = 25,
    
    MainTexTilloff      = 50,
    MaskTexTilloff      = 51,
    OpacityTexTilloff      = 52,

    // BoneOffset          = 100,
    IndicesRange        = 101,
}
impl EAnimePropertyType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
             0 => Some(Self::LocalPosition),
             1 => Some(Self::LocalRotation),
             2 => Some(Self::LocalScaling),
             3 => Some(Self::MainTexUScale),
             4 => Some(Self::MainTexVScale),
             5 => Some(Self::MainTexUOffset),
             6 => Some(Self::MainTexVOffset),
             7 => Some(Self::Alpha),
             8 => Some(Self::MainColor),
             9 => Some(Self::CameraOrthSize),
            10 => Some(Self::CameraFov),
            11 => Some(Self::Enable),
            12 => Some(Self::LocalEulerAngles),
            13 => Some(Self::Intensity),
            14 => Some(Self::LightDiffuse),
            15 => Some(Self::AlphaCutoff),
            16 => Some(Self::CellId),
            17 => Some(Self::OpacityTexUScale),
            18 => Some(Self::OpacityTexVScale),
            19 => Some(Self::OpacityTexUOffset),
            20 => Some(Self::OpacityTexVOffset),
            21 => Some(Self::MaskCutoff),
            22 => Some(Self::MaskTexUScale),
            23 => Some(Self::MaskTexVScale),
            24 => Some(Self::MaskTexUOffset),
            25 => Some(Self::MaskTexVOffset),
            
            50 => Some(Self::MainTexTilloff),
            51 => Some(Self::MaskTexTilloff),
            52 => Some(Self::OpacityTexTilloff),
            // 100 => Some(Self::BoneOffset),
            101 => Some(Self::IndicesRange),
            _ => {
                None
            }
        }
    }
}


pub fn interpolation_from_u8(val: u8) -> Option<Interpolation> {
    match val {
        1 => { Some(Interpolation::Linear) },
        2 => { Some(Interpolation::Step) },
        3 => { Some(Interpolation::CubicSpline) },
        4 => { Some(Interpolation::PICUBICSPLINE) },
        _ => { None },
    }
}
pub trait TValue<const N: usize> {
    fn newn(data: &[f32], offset: usize) -> Self;
}
impl TValue<3> for LocalScaling {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32; let z = data[offset + 2] as f32;
        Self(Vector3::new(x, y, z))
    }
}
impl TValue<3> for LocalEulerAngles {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32; let z = data[offset + 2] as f32;
        Self(Vector3::new(x, y, z))
    }
}
impl TValue<3> for LocalPosition {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32; let z = data[offset + 2] as f32;
        Self(Vector3::new(x, y, z))
    }
}
impl TValue<4> for LocalRotationQuaternion {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32; let z = data[offset + 2] as f32; let w = data[offset + 3] as f32;
        Self::create(x, y, z, w)
    }
}
impl TValue<1> for Enable {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}

impl TValue<1> for CameraFov {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}

impl TValue<1> for CameraOrthSize {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}

impl TValue<2> for IndiceRenderRange {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as u32; let y = data[offset + 1] as u32;
        Self(Some(Range { start: x, end: y, }))
    }
}
impl TValue<1> for AnimatorableFloat {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<2> for AnimatorableVec2 {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32;
        Self(Vector2::new(x, y))
    }
}
impl TValue<3> for AnimatorableVec3 {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32; let z = data[offset + 2] as f32;
        Self(Vector3::new(x, y, z))
    }
}
impl TValue<4> for AnimatorableVec4 {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32; let z = data[offset + 2] as f32; let w = data[offset + 3] as f32;
        Self(Vector4::new(x, y, z, w))
    }
}
impl TValue<1> for AnimatorableUint {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset] as u32)
    }
}
impl TValue<1> for AnimatorableSint {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset] as i32)
    }
}

pub fn curve_gltf<const N: usize, T: TValue<N> + FrameDataValue>(
    times: &[f32],
    values: &[f32],
    design_frame_per_second: FramePerSecond,
    mode: pi_gltf::animation::Interpolation,
) -> FrameCurve<T> {
    let vs = N; let vs2 = N * 2; let vs3 = N * 3;
    let frames = times.len();
    // log::warn!("Frames: {:?}", frames);
    match mode {
        pi_gltf::animation::Interpolation::Linear => {
            let mut curve = FrameCurve::<T>::curve_frame_values(design_frame_per_second);
            let step = vs;
            for i in 0..frames {
                let frame = (times[i] * (design_frame_per_second as f32)) as FrameIndex;
                let index = i * step;
                let value = T::newn(values, index);
                // log::warn!("Frame {:?}, data: {:?}", frame, T::newn(values, index));
                // if step == 1 {
                //     log::warn!("Frame: {:?}, {:?}", frame, value);
                // }
                curve.curve_frame_values_frame(frame, value);
            }
            curve
        },
        pi_gltf::animation::Interpolation::Step => {
            let mut curve = FrameCurve::<T>::curve_frame_values(design_frame_per_second);
            let step = vs;
            for i in 0..frames {
                let frame = (times[i] * (design_frame_per_second as f32)) as FrameIndex;
                let index = i * step;
                // log::warn!("Frame {:?}, data: {:?}", frame, T::newn(data, index + 1));
                curve.curve_frame_values_frame(frame, T::newn(values, index));
            }
            curve.call = interplate_frame_values_step;
            curve
        },
        pi_gltf::animation::Interpolation::CubicSpline => {
            let mut curve = FrameCurve::<T>::curve_cubic_spline(design_frame_per_second);
            let step = vs3;
            for i in 0..frames {
                let frame = (times[i] * (design_frame_per_second as f32)) as FrameIndex;
                let index = i * step;
                let intangent = T::newn(values, index);
                let value = T::newn(values, index + vs);
                let outtangent = T::newn(values, index + vs2);
                // log::warn!("Frame {:?}, data: {:?}", frame, T::newn(data, index + 1));
                curve.curve_cubic_splice_frame(frame, value, intangent, outtangent);
            }
            curve
        },
        pi_gltf::animation::Interpolation::PICUBICSPLINE => {
            let mut curve = FrameCurve::<T>::curve_cubic_spline(design_frame_per_second);
            let step = vs2;
            for i in 0..frames {
                let frame = (times[i] * (design_frame_per_second as f32)) as FrameIndex;
                let index = i * step;
                let intangent = T::newn(values, index);
                let value = T::newn(values, index + vs);
                let outtangent = intangent.clone();
                // log::warn!("Frame {:?}, data: {:?}", frame, T::newn(data, index + 1));
                // if step == 8 {
                //     log::warn!("Frame: {:?}, {:?}, {:?}", frame, intangent, value);
                // }
                curve.curve_cubic_splice_frame(frame, value, intangent, outtangent);
            }
            curve
        },
    }
}

/// FrameCurve
/// * `FrameValues` data: [design_frame_per_second, [times, ...], [(x, y, ..), x, y, ..) ...]]
/// * `FrameValuesStep` data: [design_frame_per_second, [times, ...], [(x, y, ..), (x, y, ..) ...]]
/// * `EasingCurve` data: [design_frame_per_second, total_frame, mode, (x, y, ..), (x, y, ..)]
/// * `MinMaxCurve` data: [design_frame_per_second, (x, y, ..), (x, y, ..), [times, ...], [(f32, it, ot), (f32, it, ot) ...]]
/// * `CubicBezierCurve` data: [design_frame_per_second, total_frame, (x, y, ..), (x, y, ..), (x1, y1, x2, y2)]
/// * `GLTFCubicSpline` data: [design_frame_per_second, [times, ...], [(x, y, ..), (x, y, ..), (x, y, ..), ...]]
pub fn p3d_anime_curve_query(cmds: &TypeAnimeAssetMgrs, key: IDAssetTypeFrameCurve, property: EAnimePropertyType) -> bool {
    match property {
        EAnimePropertyType::LocalPosition       => cmds.position.get(&key).is_some(),
        EAnimePropertyType::LocalScaling        => cmds.scaling.get(&key).is_some(),
        EAnimePropertyType::LocalRotation       => cmds.quaternion.get(&key).is_some(),
        EAnimePropertyType::LocalEulerAngles    => cmds.euler.get(&key).is_some(),
        EAnimePropertyType::Enable              => cmds.enable.get(&key).is_some(),
        // EAnimePropertyType::BoneOffset          => cmds.boneoff_curves.get(&key).is_some(),
        EAnimePropertyType::CameraFov           => cmds.camerafov.get(&key).is_some(),
        EAnimePropertyType::CameraOrthSize      => cmds.camerasize.get(&key).is_some(),
        EAnimePropertyType::IndicesRange        => cmds.indicerange_curves.get(&key).is_some(),
        EAnimePropertyType::Intensity           => {false},
        EAnimePropertyType::CellId              => {false},
        
        EAnimePropertyType::Alpha               => cmds.float.get(&key).is_some(),
        EAnimePropertyType::MainColor           => cmds.vec3s.get(&key).is_some(),
        EAnimePropertyType::MainTexUScale       => cmds.float.get(&key).is_some(),
        EAnimePropertyType::MainTexVScale       => cmds.float.get(&key).is_some(),
        EAnimePropertyType::MainTexUOffset      => cmds.float.get(&key).is_some(),
        EAnimePropertyType::MainTexVOffset      => cmds.float.get(&key).is_some(),
        EAnimePropertyType::OpacityTexUScale    => cmds.float.get(&key).is_some(),
        EAnimePropertyType::OpacityTexVScale    => cmds.float.get(&key).is_some(),
        EAnimePropertyType::OpacityTexUOffset   => cmds.float.get(&key).is_some(),
        EAnimePropertyType::OpacityTexVOffset   => cmds.float.get(&key).is_some(),
        EAnimePropertyType::AlphaCutoff         => cmds.float.get(&key).is_some(),
        EAnimePropertyType::LightDiffuse        => cmds.vec3s.get(&key).is_some(),
        EAnimePropertyType::MaskTexUScale       => cmds.float.get(&key).is_some(),
        EAnimePropertyType::MaskTexVScale       => cmds.float.get(&key).is_some(),
        EAnimePropertyType::MaskTexUOffset      => cmds.float.get(&key).is_some(),
        EAnimePropertyType::MaskTexVOffset      => cmds.float.get(&key).is_some(),
        EAnimePropertyType::MaskCutoff          => cmds.float.get(&key).is_some(),
        
        EAnimePropertyType::MainTexTilloff      => cmds.vec4s.get(&key).is_some(),
        EAnimePropertyType::MaskTexTilloff      => cmds.vec4s.get(&key).is_some(),
        EAnimePropertyType::OpacityTexTilloff   => cmds.vec4s.get(&key).is_some(),
    }
}
