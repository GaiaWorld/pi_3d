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

