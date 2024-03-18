# Geometry

## Components

* [BundleGeometry](../../../crates/pi_scene_context/src/geometry/prelude.rs)

## Action

* [ActionSetMaterial](../../../crates/pi_scene_context/src/geometry/prelude.rs)
* [OpsGeomeryCreate](../../../crates/pi_scene_context/src/geometry/command.rs)
    * 创建网格 - 使用Buffer描述列表

## System

* [GeometryVBLoader](../../../crates/pi_scene_context/src/geometry/base.rs)
* [sys_vertex_buffer_slots_loaded](../../../crates/pi_scene_context/src/geometry/sys_vertex_buffer_use.rs)
    * Buffer 加载成功
* [sys_geometry_enable](../../../crates/pi_scene_context/src/geometry/sys_vertex_buffer_use.rs)
    * 所有 Buffer 加载成功, Geometry 加载成功