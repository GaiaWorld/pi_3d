
use pi_scene_shell::prelude::*;
use pi_scene_math::{Number, Vector3};

use crate::{
    layer_mask::prelude::*, transforms::prelude::BundleTransformNode,
};

use super::{
    target_camera::*,
    camera::*
};

pub struct OpsCameraCreation(pub(crate) Entity, pub(crate) Entity);
impl OpsCameraCreation {
    pub fn ops(scene: Entity, entity: Entity) -> Self {
        Self(scene, entity)
    }
}
pub type ActionListCameraCreate = ActionList<OpsCameraCreation>;

pub struct OpsCameraMode(pub(crate) Entity, pub(crate) EFreeCameraMode);
impl OpsCameraMode {
    pub fn ops(camera: Entity, as_orthograhic: bool) -> Self {
        if as_orthograhic {
            Self(camera, EFreeCameraMode::Orthograhic)
        } else {
            Self(camera, EFreeCameraMode::Perspective)
        }
    }
}
pub type ActionListCameraMode = ActionList<OpsCameraMode>;

pub struct OpsCameraActive(pub(crate) Entity, pub(crate) bool);
impl OpsCameraActive {
    pub fn ops(camera: Entity, active: bool) -> Self {
        Self(camera, active)
    }
}
pub type ActionListCameraActive = ActionList<OpsCameraActive>;


pub struct OpsCameraFixedMode(pub(crate) Entity, pub(crate) EFixedMode);
impl OpsCameraFixedMode {
    pub fn ops(camera: Entity, as_horizontal: bool) -> Self {
        if as_horizontal {
            Self(camera, EFixedMode::HorizontalFixed)
        } else {
            Self(camera, EFixedMode::VerticalFixed)
        }
    }
}
pub type ActionListCameraFixedMode = ActionList<OpsCameraFixedMode>;

pub struct OpsCameraNearFar(pub(crate) Entity, pub(crate) CameraNearFar);
impl OpsCameraNearFar {
    pub fn ops(camera: Entity, near: Number, far: Number) -> Self {
        Self(camera, CameraNearFar(near, far))
    }
}
pub type ActionListCameraNearFar = ActionList<OpsCameraNearFar>;

pub struct OpsCameraFov(pub(crate) Entity, pub(crate) CameraFov);
impl OpsCameraFov {
    pub fn ops(camera: Entity, fov: Number) -> Self {
        Self(camera, CameraFov(fov))
    }
}
pub type ActionListCameraFov = ActionList<OpsCameraFov>;

pub struct OpsCameraOrthSize(pub(crate) Entity, pub(crate) CameraOrthSize);
impl OpsCameraOrthSize {
    pub fn ops(camera: Entity, size: Number) -> Self {
        Self(camera, CameraOrthSize(size))
    }
}
pub type ActionListCameraOrthSize = ActionList<OpsCameraOrthSize>;

pub struct OpsCameraAspect(pub(crate) Entity, pub(crate) f32);
impl OpsCameraAspect {
    pub fn ops(camera: Entity, value: f32) -> Self {
        Self(camera, value)
    }
}
pub type ActionListCameraAspect = ActionList<OpsCameraAspect>;

// pub struct OpsCameraPixelSize(pub(crate) Entity, pub(crate) u32, pub(crate) u32);
// impl OpsCameraPixelSize {
//     pub fn ops(camera: Entity, w: u32, h: u32) -> Self {
//         Self(camera, w, h)
//     }
// }
// pub type ActionListCameraPixelSize = ActionList<OpsCameraPixelSize>;

// pub struct OpsCameraToScreen(pub(crate) Entity, pub(crate) bool);
// impl OpsCameraToScreen {
//     pub fn ops(camera: Entity, val: bool) -> Self {
//         Self(camera, val)
//     }
// }
// pub type ActionListCameraToScreen = ActionList<OpsCameraToScreen>;

pub struct OpsCameraTarget(pub(crate) Entity, pub(crate) Vector3);
impl OpsCameraTarget {
    pub fn ops(camera: Entity, x: Number, y: Number, z: Number) -> Self {
        Self(camera, Vector3::new(x, y, z))
    }
}
pub type ActionListCameraTarget = ActionList<OpsCameraTarget>;

// pub struct OpsCameraRendererInit(pub(crate) Entity, pub(crate) Entity, pub(crate) String, pub(crate) PassTagOrders, pub(crate) ColorFormat, pub(crate) DepthStencilFormat, pub(crate) RenderTargetMode, pub(crate) u8);
// impl OpsCameraRendererInit {
//     pub fn ops(
//         camera: Entity,
//         renderer: Entity,
//         name: String,
//         orders: PassTagOrders,
//         render_target_color_format: ColorFormat,
//         render_target_depth_stencil_format: DepthStencilFormat,
//         mode: RenderTargetMode,
//     ) -> Self {
//         Self(camera, renderer, name, orders, render_target_color_format, render_target_depth_stencil_format, mode, 0)
//     }
// }
// pub type ActionListCameraRenderer = ActionList<OpsCameraRendererInit>;

// pub struct OpsCameraRendererModify(Entity, String, )

pub struct BundleCamera(
    BundleTransformNode,
    Camera,
    EFreeCameraMode,
    EFixedMode,
    CameraNearFar,
    CameraFov,
    CameraOrthSize,
    RecordCameraFov,
    RecordCameraOrthSize,
    LayerMask,
    CameraUp,
    CameraTarget,
);