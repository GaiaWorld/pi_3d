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
