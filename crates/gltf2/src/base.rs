use std::{ops::Range, fmt::Debug};

use pi_curves::curve::{frame::FrameDataValue, frame_curve::{FrameCurve, frames::interplate_frame_values_step}};
use pi_engine_shell::prelude::*;
use pi_gltf::animation::Interpolation;
use pi_scene_context::prelude::*;
use pi_node_materials::prelude::*;
use pi_scene_math::*;


#[derive(Clone, Copy)]
pub enum EAnimeCurve {
    FrameValues = 0x00,
    FrameValuesStep = 0x01,
    EasingCurve = 0x02,
    MinMaxCurve = 0x03,
    CubicBezierCurve = 0x04,
    GLTFCubicSpline = 0x05,
}
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

    BoneOffset          = 100,
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
            100 => Some(Self::BoneOffset),
            101 => Some(Self::IndicesRange),
            _ => {
                None
            }
        }
    }
}


#[derive(Clone, Copy)]
pub enum EAmountMode {
    None            ,

    BackIn          ,
    BackOut         ,
    BackInOut       ,

    CircleIn        ,
    CircleOut       ,
    CircleInOut     ,

    CubicIn         ,
    CubicOut        ,
    CubicInOut      ,

    SineIn          ,
    SineOut         ,
    SineInOut       ,

    QuadIn          ,
    QuadOut         ,
    QuadInOut       ,

    QuartIn         ,
    QuartOut        ,
    QuartInOut      ,

    QuintIn         ,
    QuintOut        ,
    QuintInOut      ,

    ExpoIn          ,
    ExpoOut         ,
    ExpoInOut       ,

    ElasticIn       ,
    ElasticOut      ,
    ElasticInOut    ,

    BounceIn        ,
    BounceOut       ,
    BounceInOut     ,

    JumpStart       ,
    JumpEnd         ,
    JumpNone        ,
    JumpBoth        ,

    CubicBezier     ,
}

pub fn number_to_easingmode(val: u8) -> pi_curves::easing::EEasingMode {
    match val {
        /*BackIn          = */ 0x01 => {
            pi_curves::easing::EEasingMode::BackIn
        },
        /*BackOut         = */ 0x02 => {
            pi_curves::easing::EEasingMode::BackOut
        },
        /*BackInOut       = */ 0x03 => {
            pi_curves::easing::EEasingMode::BackInOut
        },
        /*CircleIn        = */ 0x04 => {
            pi_curves::easing::EEasingMode::CircleIn
        },
        /*CircleOut       = */ 0x05 => {
            pi_curves::easing::EEasingMode::CircleOut
        },
        /*CircleInOut     = */ 0x06 => {
            pi_curves::easing::EEasingMode::CircleInOut
        },
        /*CubicIn         = */ 0x07 => {
            pi_curves::easing::EEasingMode::CubicIn
        },
        /*CubicOut        = */ 0x08 => {
            pi_curves::easing::EEasingMode::CubicOut
        },
        /*CubicInOut      = */ 0x09 => {
            pi_curves::easing::EEasingMode::CubicInOut
        },
        /*SineIn          = */ 0x11 => {
            pi_curves::easing::EEasingMode::SineIn
        },
        /*SineOut         = */ 0x12 => {
            pi_curves::easing::EEasingMode::SineOut
        },
        /*SineInOut       = */ 0x13 => {
            pi_curves::easing::EEasingMode::SineInOut
        },
        /*QuadIn          = */ 0x14 => {
            pi_curves::easing::EEasingMode::QuadIn
        },
        /*QuadOut         = */ 0x15 => {
            pi_curves::easing::EEasingMode::QuadOut
        },
        /*QuadInOut       = */ 0x16 => {
            pi_curves::easing::EEasingMode::QuadInOut
        },
        /*QuartIn         = */ 0x17 => {
            pi_curves::easing::EEasingMode::QuartIn
        },
        /*QuartOut        = */ 0x18 => {
            pi_curves::easing::EEasingMode::QuartOut
        },
        /*QuartInOut      = */ 0x19 => {
            pi_curves::easing::EEasingMode::QuartInOut
        },
        /*QuintIn         = */ 0x21 => {
            pi_curves::easing::EEasingMode::QuintIn
        },
        /*QuintOut        = */ 0x22 => {
            pi_curves::easing::EEasingMode::QuintOut
        },
        /*QuintInOut      = */ 0x23 => {
            pi_curves::easing::EEasingMode::QuintInOut
        },
        /*ExpoIn          = */ 0x24 => {
            pi_curves::easing::EEasingMode::ExpoIn
        },
        /*ExpoOut         = */ 0x25 => {
            pi_curves::easing::EEasingMode::ExpoOut
        },
        /*ExpoInOut       = */ 0x26 => {
            pi_curves::easing::EEasingMode::ExpoInOut
        },
        /*ElasticIn       = */ 0x27 => {
            pi_curves::easing::EEasingMode::ElasticIn
        },
        /*ElasticOut      = */ 0x28 => {
            pi_curves::easing::EEasingMode::ElasticOut
        },
        /*ElasticInOut    = */ 0x29 => {
            pi_curves::easing::EEasingMode::ElasticInOut
        },
        /*BounceIn        = */ 0x31 => {
            pi_curves::easing::EEasingMode::BounceIn
        },
        /*BounceOut       = */ 0x32 => {
            pi_curves::easing::EEasingMode::BounceOut
        },
        /*BounceInOut     = */ 0x33 => {
            pi_curves::easing::EEasingMode::BounceInOut
        },
        /*None            = */ _ => {
            pi_curves::easing::EEasingMode::None
        },
    }
}

#[derive(SystemParam)]
pub struct TypeAnimeContexts<'w> {
    pub position: ResMut<'w, TypeAnimeContext<LocalPosition>>,
    pub euler: ResMut<'w, TypeAnimeContext<LocalEulerAngles>>,
    pub quaternion: ResMut<'w, TypeAnimeContext<LocalRotationQuaternion>>,
    pub scaling: ResMut<'w, TypeAnimeContext<LocalScaling>>,
    pub isactive: ResMut<'w, TypeAnimeContext<Enable>>,
    pub camerafov: ResMut<'w, TypeAnimeContext<CameraFov>>,
    pub camerasize: ResMut<'w, TypeAnimeContext<CameraOrthSize>>,
    pub alpha: ResMut<'w, TypeAnimeContext<Alpha>>,
    pub alphacutoff: ResMut<'w, TypeAnimeContext<Cutoff>>,
    pub maintex_uscale: ResMut<'w, TypeAnimeContext<MainTexUScale>>,
    pub maintex_vscale: ResMut<'w, TypeAnimeContext<MainTexVScale>>,
    pub maintex_uoffset: ResMut<'w, TypeAnimeContext<MainTexUOffset>>,
    pub maintex_voffset: ResMut<'w, TypeAnimeContext<MainTexVOffset>>,
    pub opacitytex_uscale: ResMut<'w, TypeAnimeContext<OpacityTexUScale>>,
    pub opacitytex_vscale: ResMut<'w, TypeAnimeContext<OpacityTexVScale>>,
    pub opacitytex_uoffset: ResMut<'w, TypeAnimeContext<OpacityTexUOffset>>,
    pub opacitytex_voffset: ResMut<'w, TypeAnimeContext<OpacityTexVOffset>>,
    pub masktex_uscale: ResMut<'w, TypeAnimeContext<MaskTexUScale>>,
    pub masktex_vscale: ResMut<'w, TypeAnimeContext<MaskTexVScale>>,
    pub masktex_uoffset: ResMut<'w, TypeAnimeContext<MaskTexUOffset>>,
    pub masktex_voffset: ResMut<'w, TypeAnimeContext<MaskTexVOffset>>,
    pub maskcutoff: ResMut<'w, TypeAnimeContext<MaskCutoff>>,
    pub maincolor: ResMut<'w, TypeAnimeContext<MainColor>>,
    pub lightdiffuse: ResMut<'w, TypeAnimeContext<LightDiffuse>>,
    pub boneoffset: ResMut<'w, TypeAnimeContext<InstanceBoneoffset>>,
    pub indices_range: ResMut<'w, TypeAnimeContext<IndiceRenderRange>>,
}

#[derive(SystemParam)]
pub struct TypeAnimeAssetMgrs<'w> {
    pub position: Res<'w, ShareAssetMgr<TypeFrameCurve<LocalPosition>>>,
    pub euler: Res<'w, ShareAssetMgr<TypeFrameCurve<LocalEulerAngles>>>,
    pub quaternion: Res<'w, ShareAssetMgr<TypeFrameCurve<LocalRotationQuaternion>>>,
    pub scaling: Res<'w, ShareAssetMgr<TypeFrameCurve<LocalScaling>>>,
    pub enable: Res<'w, ShareAssetMgr<TypeFrameCurve<Enable>>>,
    pub camerafov: Res<'w, ShareAssetMgr<TypeFrameCurve<CameraFov>>>,
    pub camerasize: Res<'w, ShareAssetMgr<TypeFrameCurve<CameraOrthSize>>>,
    pub alpha: Res<'w, ShareAssetMgr<TypeFrameCurve<Alpha>>>,
    pub alphacutoff: Res<'w, ShareAssetMgr<TypeFrameCurve<Cutoff>>>,
    pub mainuscl_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexUScale>>>,
    pub mainvscl_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexVScale>>>,
    pub mainuoff_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexUOffset>>>,
    pub mainvoff_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexVOffset>>>,
    pub opacityuscl_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexUScale>>>,
    pub opacityvscl_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexVScale>>>,
    pub opacityuoff_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexUOffset>>>,
    pub opacityvoff_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexVOffset>>>,
    pub maskuscl_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexUScale>>>,
    pub maskvscl_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexVScale>>>,
    pub maskuoff_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexUOffset>>>,
    pub maskvoff_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexVOffset>>>,
    pub maskcutoff_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<MaskCutoff>>>,
    pub maincolor_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<MainColor>>>,
    pub lightdiffuse_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<LightDiffuse>>>,
    pub boneoff_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<InstanceBoneoffset>>>,
    pub indicerange_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<IndiceRenderRange>>>,
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
impl TValue<1> for Alpha {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for Cutoff {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<3> for MainColor {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32; let z = data[offset + 2] as f32;
        Self(Vector3::new(x, y, z))
    }
}
impl TValue<1> for MainTexUScale {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for MainTexVScale {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for MainTexUOffset {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for MainTexVOffset {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}

impl TValue<1> for MaskTexUScale {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for MaskTexVScale {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for MaskTexUOffset {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for MaskTexVOffset {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}

impl TValue<1> for OpacityTexUScale {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for OpacityTexVScale {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for OpacityTexUOffset {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for OpacityTexVOffset {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
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

impl TValue<1> for MaskCutoff {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}

impl TValue<3> for LightDiffuse {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32; let z = data[offset + 2] as f32;
        Self(Vector3::new(x, y, z))
    }
}

impl TValue<1> for InstanceBoneoffset {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as u32;
        Self(x)
    }
}

impl TValue<2> for IndiceRenderRange {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as u32; let y = data[offset + 1] as u32;
        Self(Some(Range { start: x, end: y, }))
    }
}

pub fn curve<const N: usize, T: TValue<N> + FrameDataValue + Debug>(
    data: &[f32],
    frames: usize,
    mode: EAnimeCurve,
) -> FrameCurve<T> {
    let vs = N; let vs2 = N * 2; let vs3 = N * 3;
    let design_frame_per_second = data[0] as FrameIndex;
    let curve = match mode {
        EAnimeCurve::FrameValues => {
            let mut curve = FrameCurve::<T>::curve_frame_values(design_frame_per_second);
            let head = 1;
            let step = vs;
            // let frames = (data.len() - head) / step;
            let datastart = head + frames;
            for i in 0..frames {
                let index = datastart + i * step;
                let frame = data[head + i] as FrameIndex;
                // log::warn!("Frame {:?}, data: {:?}", frame, T::newn(data, index + 1));
                curve.curve_frame_values_frame(frame, T::newn(data, index));
            }
            curve
        },
        EAnimeCurve::FrameValuesStep => {
            let mut curve = FrameCurve::<T>::curve_frame_values(design_frame_per_second);
            let head = 1;
            let step = vs;
            // let frames = (data.len() - head) / step;
            let datastart = head + frames;
            for i in 0..frames {
                let index = head + i * step;
                let frame = data[index + 0] as FrameIndex;
                curve.curve_frame_values_frame(frame, T::newn(data, index));
            }
            curve.call = interplate_frame_values_step;
            curve
        },
        EAnimeCurve::EasingCurve => {
            let frame_count = data[1] as FrameIndex;
            let mode = number_to_easingmode(data[2] as u8);
            let head = 3;
            let from = T::newn(data, head + 0);
            let scalar = T::newn(data, head + vs);
            let curve = FrameCurve::<T>::curve_easing(
                from,
                scalar,
                frame_count,
                design_frame_per_second, mode
            );
            curve
        },
        EAnimeCurve::MinMaxCurve => {
            let design_frame_per_second = data[0] as FramePerSecond;
            let from = T::newn(data, 1);
            let to = T::newn(data, 1 + vs);
            let head = 1 + vs2;
            let mut curve = FrameCurve::<T>::curve_minmax_curve(from, to, design_frame_per_second);
            let step = 3;
            // let frames = (data.len() - head) / step;
            let datastart = head + frames;
            for i in 0..frames {
                let frame = data[head + i] as FrameIndex;
                let index = datastart + i * step;
                let intangent  = data[index + 0] as f32;
                let value = data[index + 1] as f32;
                let outtangent = data[index + 2] as f32;
                curve.curve_minmax_curve_frame(frame, value, intangent, outtangent);
            }
            curve
        },
        EAnimeCurve::CubicBezierCurve => {
            let frame_count = data[1] as FrameIndex;
            let mut head = 2;
            let from = T::newn(data, head);
            let scalar = T::newn(data, head + vs);
            head = head + vs2;
            let x1 = data[head] as f32; let y1 = data[head + 1] as f32; let x2 = data[head + 2] as f32; let y2 = data[head + 3] as f32; 
            let curve = FrameCurve::<T>::curve_cubic_bezier(
                from,
                scalar,
                frame_count,
                design_frame_per_second,
                x1 as f32, y1 as f32, x2 as f32, y2 as f32
            );
            curve
        },
        EAnimeCurve::GLTFCubicSpline => {
            let mut curve = FrameCurve::<T>::curve_cubic_spline(design_frame_per_second);
            let head = 1;
            let step = vs3;
            // let frames = (data.len() - head) / step;
            let datastart = head + frames;
            for i in 0..frames {
                let frame = data[head + i] as FrameIndex;
                let index = datastart + i * step;
                let intangent = T::newn(data, index);
                let value = T::newn(data, index + vs);
                let outtangent = T::newn(data, index + vs2);
                curve.curve_cubic_splice_frame(frame, value, intangent, outtangent);
            }
            curve
        },
    };
    curve
}

pub fn curve_gltf<const N: usize, T: TValue<N> + FrameDataValue + Debug>(
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
                // log::warn!("Frame {:?}, data: {:?}", frame, T::newn(values, index));
                curve.curve_frame_values_frame(frame, T::newn(values, index));
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
            let mut curve = FrameCurve::<T>::curve_frame_values(design_frame_per_second);
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
            let mut curve = FrameCurve::<T>::curve_frame_values(design_frame_per_second);
            let step = vs2;
            for i in 0..frames {
                let frame = (times[i] * (design_frame_per_second as f32)) as FrameIndex;
                let index = i * step;
                let intangent = T::newn(values, index);
                let value = T::newn(values, index + vs);
                let outtangent = intangent.clone();
                // log::warn!("Frame {:?}, data: {:?}", frame, T::newn(data, index + 1));
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
        EAnimePropertyType::Alpha               => cmds.alpha.get(&key).is_some(),
        EAnimePropertyType::MainColor           => cmds.maincolor_curves.get(&key).is_some(),
        EAnimePropertyType::MainTexUScale       => cmds.mainuscl_curves.get(&key).is_some(),
        EAnimePropertyType::MainTexVScale       => cmds.mainvscl_curves.get(&key).is_some(),
        EAnimePropertyType::MainTexUOffset      => cmds.mainuoff_curves.get(&key).is_some(),
        EAnimePropertyType::MainTexVOffset      => cmds.mainvoff_curves.get(&key).is_some(),
        EAnimePropertyType::OpacityTexUScale    => cmds.opacityuscl_curves.get(&key).is_some(),
        EAnimePropertyType::OpacityTexVScale    => cmds.opacityvscl_curves.get(&key).is_some(),
        EAnimePropertyType::OpacityTexUOffset   => cmds.opacityuoff_curves.get(&key).is_some(),
        EAnimePropertyType::OpacityTexVOffset   => cmds.opacityvoff_curves.get(&key).is_some(),
        EAnimePropertyType::AlphaCutoff         => cmds.alphacutoff.get(&key).is_some(),
        EAnimePropertyType::CameraFov           => cmds.camerafov.get(&key).is_some(),
        EAnimePropertyType::CameraOrthSize      => cmds.camerasize.get(&key).is_some(),
        EAnimePropertyType::LightDiffuse        => cmds.lightdiffuse_curves.get(&key).is_some(),
        EAnimePropertyType::MaskTexUScale       => cmds.maskuscl_curves.get(&key).is_some(),
        EAnimePropertyType::MaskTexVScale       => cmds.maskvscl_curves.get(&key).is_some(),
        EAnimePropertyType::MaskTexUOffset      => cmds.maskuoff_curves.get(&key).is_some(),
        EAnimePropertyType::MaskTexVOffset      => cmds.maskvoff_curves.get(&key).is_some(),
        EAnimePropertyType::MaskCutoff          => cmds.maskcutoff_curves.get(&key).is_some(),
        EAnimePropertyType::Enable              => cmds.enable.get(&key).is_some(),
        EAnimePropertyType::BoneOffset          => cmds.boneoff_curves.get(&key).is_some(),
        EAnimePropertyType::IndicesRange        => cmds.indicerange_curves.get(&key).is_some(),
        EAnimePropertyType::Intensity           => {false},
        EAnimePropertyType::CellId              => {false},
    }
}

pub fn p3d_anime_curve_create(cmds: &TypeAnimeAssetMgrs, key: IDAssetTypeFrameCurve, property: EAnimePropertyType, data: &[f32], frames: f64, mode: EAnimeCurve) {
    let frames = frames as usize;
    match property {
        EAnimePropertyType::LocalPosition       => {
            let v = curve::<3, LocalPosition>(data, frames, mode);
            cmds.position.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::LocalScaling        => {
            let v = curve::<3, LocalScaling>(data, frames, mode);
            cmds.scaling.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::LocalRotation    => {
            let v = curve::<4, LocalRotationQuaternion>(data, frames, mode);
            cmds.quaternion.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::LocalEulerAngles    => {
            let v = curve::<3, LocalEulerAngles>(data, frames, mode);
            cmds.euler.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::Alpha               => {
            let v = curve::<1, Alpha>(data, frames, mode);
            cmds.alpha.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MainColor           => {
            let v = curve::<3, MainColor>(data, frames, mode);
            cmds.maincolor_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MainTexUScale       => {
            let v = curve::<1, MainTexUScale>(data, frames, mode);
            cmds.mainuscl_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MainTexVScale       => {
            let v = curve::<1, MainTexVScale>(data, frames, mode);
            cmds.mainvscl_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MainTexUOffset      => {
            let v = curve::<1, MainTexUOffset>(data, frames, mode);
            cmds.mainuoff_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MainTexVOffset      => {
            let v = curve::<1, MainTexVOffset>(data, frames, mode);
            cmds.mainvoff_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::OpacityTexUScale    => {
            let v = curve::<1, OpacityTexUScale>(data, frames, mode);
            cmds.opacityuscl_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::OpacityTexVScale    => {
            let v = curve::<1, OpacityTexVScale>(data, frames, mode);
            cmds.opacityvscl_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::OpacityTexUOffset   => {
            let v = curve::<1, OpacityTexUOffset>(data, frames, mode);
            cmds.opacityuoff_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::OpacityTexVOffset   => {
            let v = curve::<1, OpacityTexVOffset>(data, frames, mode);
            cmds.opacityvoff_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::AlphaCutoff         => {
            let v = curve::<1, Cutoff>(data, frames, mode);
            cmds.alphacutoff.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::CameraFov           => {
            let v = curve::<1, CameraFov>(data, frames, mode);
            cmds.camerafov.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::CameraOrthSize      => {
            let v = curve::<1, CameraOrthSize>(data, frames, mode);
            cmds.camerasize.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::LightDiffuse        => {
            let v = curve::<3, LightDiffuse>(data, frames, mode);
            cmds.lightdiffuse_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MaskTexUScale       => {
            let v = curve::<1, MaskTexUScale>(data, frames, mode);
            cmds.maskuscl_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MaskTexVScale       => {
            let v = curve::<1, MaskTexVScale>(data, frames, mode);
            cmds.maskvscl_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MaskTexUOffset      => {
            let v = curve::<1, MaskTexUOffset>(data, frames, mode);
            cmds.maskuoff_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MaskTexVOffset      => {
            let v = curve::<1, MaskTexVOffset>(data, frames, mode);
            cmds.maskvoff_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MaskCutoff          => {
            let v = curve::<1, MaskCutoff>(data, frames, mode);
            cmds.maskcutoff_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::Enable            => {
            let v = curve::<1, Enable>(data, frames, mode);
            cmds.enable.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::BoneOffset          => {
            let v = curve::<1, InstanceBoneoffset>(data, frames, mode);
            cmds.boneoff_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::IndicesRange        => {
            let v = curve::<2, IndiceRenderRange>(data, frames, mode);
            cmds.indicerange_curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::Intensity => {
            
        },
        EAnimePropertyType::CellId => {
            
        },
    }
}