pub use crate::{
    bindgroup::*,
    transforms::prelude::*,
    scene::prelude::*,
    cameras::prelude::*,
    layer_mask::prelude::*,
    renderers::prelude::*,
    pass::*,
    skeleton::{
        command_sys::*,
        prelude::*,
    },
    materials::{prelude::*, command_sys::*},
    meshes::{prelude::*, command_sys::*},
    geometry::prelude::*,
    state::*,
    animation::prelude::*,
    flags::*,
    object::*,
    viewer::prelude::*,
    cullings::prelude::*,
    light::prelude::*,
    shadow::prelude::*,
    sprite::*,
};

#[derive(SystemParam)]
pub struct TypeAnimeContexts<'w> {
    pub position: ResMut<'w, TypeAnimeContext<LocalPosition>>,
    pub euler: ResMut<'w, TypeAnimeContext<LocalEulerAngles>>,
    pub quaternion: ResMut<'w, TypeAnimeContext<LocalRotationQuaternion>>,
    pub scaling: ResMut<'w, TypeAnimeContext<LocalScaling>>,
    pub enable: ResMut<'w, TypeAnimeContext<Enable>>,
    pub camerafov: ResMut<'w, TypeAnimeContext<CameraFov>>,
    pub camerasize: ResMut<'w, TypeAnimeContext<CameraOrthSize>>,
    pub indices_range: ResMut<'w, TypeAnimeContext<IndiceRenderRange>>,

    pub float: ResMut<'w, TypeAnimeContext<AnimatorableFloat>>,
    pub vec2s: ResMut<'w, TypeAnimeContext<AnimatorableVec2>>,
    pub vec3s: ResMut<'w, TypeAnimeContext<AnimatorableVec3>>,
    pub vec4s: ResMut<'w, TypeAnimeContext<AnimatorableVec4>>,
    pub uints: ResMut<'w, TypeAnimeContext<AnimatorableUint>>,
    pub _ints: ResMut<'w, TypeAnimeContext<AnimatorableSint>>,
}
impl<'w> MemSize for TypeAnimeContexts<'w> {
    fn memsize(&self) -> usize {
        self.position.memsize()
        + self.euler.memsize()
        + self.quaternion.memsize()
        + self.scaling.memsize()
        + self.enable.memsize()
        + self.camerafov.memsize()
        + self.camerasize.memsize()
        + self.indices_range.memsize()
        + self.float.memsize()
        + self.vec2s.memsize()
        + self.vec3s.memsize()
        + self.vec4s.memsize()
        + self.uints.memsize()
        + self._ints.memsize()
    }
}

pub trait TTypeAnimeAssetMgr {
    fn position(&self) -> &ShareAssetMgr<TypeFrameCurve<LocalPosition>>;
    fn euler(&self) -> &ShareAssetMgr<TypeFrameCurve<LocalEulerAngles>>;
    fn quaternion(&self) -> &ShareAssetMgr<TypeFrameCurve<LocalRotationQuaternion>>;
    fn scaling(&self) -> &ShareAssetMgr<TypeFrameCurve<LocalScaling>>;
    fn enable(&self) -> &ShareAssetMgr<TypeFrameCurve<Enable>>;
    fn camerafov(&self) -> &ShareAssetMgr<TypeFrameCurve<CameraFov>>;
    fn camerasize(&self) -> &ShareAssetMgr<TypeFrameCurve<CameraOrthSize>>;
    fn indicerange_curves(&self) -> &ShareAssetMgr<TypeFrameCurve<IndiceRenderRange>>;
    fn float(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableFloat>>;
    fn vec2s(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableVec2>>;
    fn vec3s(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableVec3>>;
    fn vec4s(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableVec4>>;
    fn uints(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableUint>>;
    fn _ints(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableSint>>;
}
impl TTypeAnimeAssetMgr for World {
    fn position(&self) -> &ShareAssetMgr<TypeFrameCurve<LocalPosition>> {
       &*self.get_resource::<ShareAssetMgr<TypeFrameCurve<LocalPosition>>>().unwrap()
    }
    fn euler(&self) -> &ShareAssetMgr<TypeFrameCurve<LocalEulerAngles>> {
       &*self.get_resource::<ShareAssetMgr<TypeFrameCurve<LocalEulerAngles>>>().unwrap()
    }
    fn quaternion(&self) -> &ShareAssetMgr<TypeFrameCurve<LocalRotationQuaternion>> {
       &*self.get_resource::<ShareAssetMgr<TypeFrameCurve<LocalRotationQuaternion>>>().unwrap()
    }
    fn scaling(&self) -> &ShareAssetMgr<TypeFrameCurve<LocalScaling>> {
       &*self.get_resource::<ShareAssetMgr<TypeFrameCurve<LocalScaling>>>().unwrap()
    }
    fn enable(&self) -> &ShareAssetMgr<TypeFrameCurve<Enable>> {
       &*self.get_resource::<ShareAssetMgr<TypeFrameCurve<Enable>>>().unwrap()
    }
    fn camerafov(&self) -> &ShareAssetMgr<TypeFrameCurve<CameraFov>> {
       &*self.get_resource::<ShareAssetMgr<TypeFrameCurve<CameraFov>>>().unwrap()
    }
    fn camerasize(&self) -> &ShareAssetMgr<TypeFrameCurve<CameraOrthSize>> {
       &*self.get_resource::<ShareAssetMgr<TypeFrameCurve<CameraOrthSize>>>().unwrap()
    }
    fn indicerange_curves(&self) -> &ShareAssetMgr<TypeFrameCurve<IndiceRenderRange>> {
       &*self.get_resource::<ShareAssetMgr<TypeFrameCurve<IndiceRenderRange>>>().unwrap()
    }
    fn float(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableFloat>> {
       &*self.get_resource::<ShareAssetMgr<TypeFrameCurve<AnimatorableFloat>>>().unwrap()
    }
    fn vec2s(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableVec2>> {
       &*self.get_resource::<ShareAssetMgr<TypeFrameCurve<AnimatorableVec2>>>().unwrap()
    }
    fn vec3s(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableVec3>> {
       &*self.get_resource::<ShareAssetMgr<TypeFrameCurve<AnimatorableVec3>>>().unwrap()
    }
    fn vec4s(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableVec4>> {
       &*self.get_resource::<ShareAssetMgr<TypeFrameCurve<AnimatorableVec4>>>().unwrap()
    }
    fn uints(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableUint>> {
       &*self.get_resource::<ShareAssetMgr<TypeFrameCurve<AnimatorableUint>>>().unwrap()
    }
    fn _ints(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableSint>> {
       &*self.get_resource::<ShareAssetMgr<TypeFrameCurve<AnimatorableSint>>>().unwrap()
    }
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
    pub indicerange_curves: Res<'w, ShareAssetMgr<TypeFrameCurve<IndiceRenderRange>>>,

    pub float: Res<'w, ShareAssetMgr<TypeFrameCurve<AnimatorableFloat>>>,
    pub vec2s: Res<'w, ShareAssetMgr<TypeFrameCurve<AnimatorableVec2>>>,
    pub vec3s: Res<'w, ShareAssetMgr<TypeFrameCurve<AnimatorableVec3>>>,
    pub vec4s: Res<'w, ShareAssetMgr<TypeFrameCurve<AnimatorableVec4>>>,
    pub uints: Res<'w, ShareAssetMgr<TypeFrameCurve<AnimatorableUint>>>,
    pub _ints: Res<'w, ShareAssetMgr<TypeFrameCurve<AnimatorableSint>>>,
}
impl<'w> MemSize for TypeAnimeAssetMgrs<'w> {
    fn memsize(&self) -> usize {
        self.position.size()
        + self.euler.size()
        + self.quaternion.size()
        + self.scaling.size()
        + self.enable.size()
        + self.camerafov.size()
        + self.camerasize.size()
        + self.indicerange_curves.size()
        + self.float.size()
        + self.vec2s.size()
        + self.vec3s.size()
        + self.vec4s.size()
        + self.uints.size()
        + self._ints.size()
    }
}

impl<'w> TTypeAnimeAssetMgr for TypeAnimeAssetMgrs<'w> {
    fn position(&self) -> &ShareAssetMgr<TypeFrameCurve<LocalPosition>> { &*self.position }
    fn euler(&self) -> &ShareAssetMgr<TypeFrameCurve<LocalEulerAngles>> { &*self.euler }
    fn quaternion(&self) -> &ShareAssetMgr<TypeFrameCurve<LocalRotationQuaternion>> { &*self.quaternion }
    fn scaling(&self) -> &ShareAssetMgr<TypeFrameCurve<LocalScaling>> { &*self.scaling }
    fn enable(&self) -> &ShareAssetMgr<TypeFrameCurve<Enable>> { &*self.enable }
    fn camerafov(&self) -> &ShareAssetMgr<TypeFrameCurve<CameraFov>> { &*self.camerafov }
    fn camerasize(&self) -> &ShareAssetMgr<TypeFrameCurve<CameraOrthSize>> { &*self.camerasize }
    fn indicerange_curves(&self) -> &ShareAssetMgr<TypeFrameCurve<IndiceRenderRange>> { &*self.indicerange_curves }
    fn float(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableFloat>> { &*self.float }
    fn vec2s(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableVec2>> { &*self.vec2s }
    fn vec3s(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableVec3>> { &*self.vec3s }
    fn vec4s(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableVec4>> { &*self.vec4s }
    fn uints(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableUint>> { &*self.uints }
    fn _ints(&self) -> &ShareAssetMgr<TypeFrameCurve<AnimatorableSint>> { &*self._ints }
}
