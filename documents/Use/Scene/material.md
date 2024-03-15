# 材质 Material

## Action

* [ActionSetMaterial](../../../crates/pi_scene_context/src/materials/prelude.rs)
* [OpsMaterialCreate](../../../crates/pi_scene_context/src/materials/command.rs)
  * 创建材质
* [OpsMaterialUse](../../../crates/pi_scene_context/src/materials/command.rs)
  * 应用材质
* [OpsUniformMat4...](../../../crates/pi_scene_context/src/materials/command.rs)
  * 设置 数值Uniform
* [OpsUniformTexture](../../../crates/pi_scene_context/src/materials/command.rs)
  * 设置 纹理Uniform
* [OpsUniformTextureFromRenderTarget](../../../crates/pi_scene_context/src/materials/command.rs)
  * 从 RT 设置 纹理Uniform
* [OpsTargetAnimationUniform](../../../crates/pi_scene_context/src/materials/command.rs)
  * 设置 数值Uniform动画


## **重要** 材质使用的Shader元数据必须在使用前注册好

* 为了初始化材质实体的原型 不受Shader元数据的异步影响
* 同时也使得可动画的材质属性 作用的 uniform 能在初始化时一次确定
* Shader元数据 作为资源注册到World 这个操作正好是同步的因此可以满足上面的需求
* 顶点Shader中必须将z值转换到 0-1 `通过调整投影矩阵`