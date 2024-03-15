# Mesh

## ModelMatrix

* Component
  * [RenderWorldMatrix](../../../crates/pi_scene_context/src/meshes/model.rs)
    * 渲染时模型矩阵
* System
  * [OpsMeshRenderAlignment](../../../crates/pi_scene_context/src/meshes/command.rs)
    * 设置渲染姿态
  * [sys_render_matrix_for_uniform](../../../crates/pi_scene_context/src/meshes/system.rs)

## 网格数据

* Component
  * [GeometryID](../../../crates/pi_scene_context/src/geometry/vertex_buffer_useinfo.rs)
    * 网格ID
* System
  * [OpsGeomeryCreate](../../../crates/pi_scene_context/src/geometry/command.rs)
    * 使用网格数据

## 材质数据

* Component
  * [PassID](../../../crates/pi_scene_context/src/pass/pass_object.rs)
    * 渲染使用的效果材质的 实体ID, 对Mesh的指定Pass使用指定Material
* System
  * [OpsMaterialUse](../../../crates/pi_scene_context/src/materials/command.rs)

## 实例化相关

* Component
  * [InstanceSourceRefs](../../../crates/pi_scene_context/src/geometry/instance/mod.rs)
    * 记录关联的实例的ID列表
  * [ModelInstanceAttributes](../../../crates/pi_scene_context/src/geometry/instance/types.rs)
    * 存储实例化属性数据的布局信息
* System
  * [ActionSetAbstructMesh](../../../crates/pi_scene_context/src/meshes/prelude.rs)
  * [ActionSetMesh](../../../crates/pi_scene_context/src/meshes/prelude.rs)
  * [ActionSetInstanceMesh](../../../crates/pi_scene_context/src/meshes/prelude.rs)
  * [OpsMeshCreation](../../../crates/pi_scene_context/src/meshes/command.rs)
    * 创建 mesh 时声明实例化信息
  * [OpsInstanceMeshCreation](../../../crates/pi_scene_context/src/meshes/command.rs)
    * 创建 实例化Mesh
  * [OpsInstanceVec4...](../../../crates/pi_scene_context/src/meshes/command.rs) 
    * [sys_act_instance_attribute](../../../crates/pi_scene_context/src/meshes/command_sys.rs)
    * 修改/动画作用 实例化属性
  * [sys_tick_instanced_buffer_update](../../../crates/pi_scene_context/src/geometry/instance/sys_instance.rs)
    * 实例数据收集>更新Buffer

## 骨骼相关

* Component
  * [BindSkinValue](../../../crates/pi_scene_context/src/skeleton/skeleton.rs)
    * 骨骼数据
* System
  * [OpsSkinUse](../../../crates/pi_scene_context/src/skeleton/command.rs)
    * 应用骨骼

## 剔除相关

* Component
  * [LayerMask](../../../crates/pi_scene_context/src/layer_mask/base.rs)
* System
  * [OpsLayerMask](../../../crates/pi_scene_context/src/layer_mask/command.rs)

## 排序相关

* Component
  * [TransparentSortParam](../../../crates/pi_scene_context/src/renderers/render_sort/mod.rs)
* System
  * [OpsRenderQueue](../../../crates/pi_scene_context/src/renderers/render_sort/mod.rs)

## Animatable - 可动画属性

* [InstanceAttributeAnimated]()
* [IndiceRenderRange](../../../crates/pi_scene_context/src/meshes/model.rs)