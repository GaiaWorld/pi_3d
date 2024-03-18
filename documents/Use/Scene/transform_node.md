# TransformNode

* 位移、旋转、缩放

## 位移

* Component
  * LocalPosition
    * 局部的位移
* Interface
  * ```rust
    engine.transform_position(entity, Vector3::new(1., 2., 2.));
    ```

## 旋转

* Component
  * LocalRotation
    * 局部的旋转矩阵
  * LocalRotationQuaternion
    * 局部的旋转四元数
  * LocalEulerAngles
    * 局部的旋转欧拉角(弧度)
* Interface
  * ```rust
    /// 只需使用其中一个,同步转换到其他形式的旋转描述
    engine.transform_rotation_quaternion(entity, Quaternion::identify());
    engine.transform_rotation_euler(entity, Vector3::new(1., 0., 0.));
    ```

## 缩放

* Component
  * LocalScaling
    * 局部的缩放数据
* Interface
  * ```rust
    engine.transform_scaling(entity, Vector3::new(2., 2., 2.));
    ```

## 变换矩阵

* Component
  * LocalMatrix
    * 局部的变换矩阵
  * WorldMatrix
    * 全局的变换矩阵
  * GlobalTransform
    * 全局的变换信息
      * 位移
      * 旋转
      * 缩放
      * 无缩放信息的变换矩阵

## Action

* [ActionSetTransform](../../../crates/pi_scene_context/src/transforms/prelude.rs)
* [OpsTransformNode](../../../crates/pi_scene_context/src/transforms/command.rs)
  * 创建纯节点
* [OpsTransformNodeParent](../../../crates/pi_scene_context/src/transforms/command.rs)
  * 设置父节点
* [OpsTransformNodeLocalPosition](../../../crates/pi_scene_context/src/transforms/command.rs)
  * 局部位移
* [OpsTransformNodeLocalRotationQuaternion](../../../crates/pi_scene_context/src/transforms/command.rs)
  * 局部旋转
* [OpsTransformNodeLocalEuler](../../../crates/pi_scene_context/src/transforms/command.rs)
  * 局部旋转
* [OpsTransformNodeLocalScaling](../../../crates/pi_scene_context/src/transforms/command.rs)
  * 局部缩放
* [OpsNodeEnable](../../../crates/pi_scene_context/src/flags/mod.rs)
  * 自身 Enable

## Animatable - 可动画属性

* [LocalPosition](../../../crates/pi_scene_context/src/transforms/transform_node.rs)
* [LocalEulerAngles](../../../crates/pi_scene_context/src/transforms/transform_node.rs)
* [LocalRotationQuaternion](../../../crates/pi_scene_context/src/transforms/transform_node.rs)
* [LocalScaling](../../../crates/pi_scene_context/src/transforms/transform_node.rs)
* [Enable](../../../crates/pi_scene_context/src/transforms/transform_node.rs)
