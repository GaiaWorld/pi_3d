# Camera

## 相机数据

* Component
    * [BundleCamera](../../../crates/pi_scene_context/src/cameras/command.rs)
        * 组件
* System
    * [ActionSetCamera](../../../crates/pi_scene_context/src/cameras/prelude.rs)
    * [OpsCameraCreation](../../../crates/pi_scene_context/src/cameras/command.rs)
        * 创建相机
    * [OpsCameraMode](../../../crates/pi_scene_context/src/cameras/command.rs)
        * 相机模式

## 剔除相关

* Component
    * [ForceIncludeModelList](../../../crates/pi_scene_context/src/viewer/base.rs)
        * 强制包含
    * [ModelListAfterCulling](../../../crates/pi_scene_context/src/viewer/base.rs)
        * 剔除结果
* System
    * [OpsViewerForceInclude](../../../crates/pi_scene_context/src/viewer/command.rs)
        * 设置指定节点强制包含在Viewer的剔除结果列表中
    * [sys_tick_viewer_culling](../../../crates/pi_scene_context/src/viewer/sys_culling.rs)
        * sys_tick_viewer_culling::<TargetCameraParam, CameraParam, StateCamera>

## Animatable - 可动画属性

* [CameraFov](../../../crates/pi_scene_context/src/viewer/camera.rs)
* [CameraOrthSize](../../../crates/pi_scene_context/src/viewer/camera.rs)