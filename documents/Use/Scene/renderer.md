
# Renderer

* 某个 Viewer 的 指定Pass 的 DrawObj 收集器

## Renderer 数据

* Component
    * [Renderer]
    * [RendererRenderTarget]
    * [RendererDrawCallRecord]
    * [ViewerRenderersInfo]
* System
    * [ActionSetRenderer](../../../crates/pi_scene_context/src/renderers/prelude.rs)
    * [OpsRendererCreate](../../../crates/pi_scene_context/src/renderers/command.rs)
        * 对 Viewer 创建指定 Pass 的 Renderer
    * [OpsRendererConnect](../../../crates/pi_scene_context/src/renderers/command.rs)
        * 设置 Renderer 与其他 渲染列表/渲染图节点 链接更新
    * [OpsRendererTarget](../../../crates/pi_scene_context/src/renderers/command.rs)
        * 设置 Renderer 对应渲染目标
    * [OpsRendererCommand](../../../crates/pi_scene_context/src/renderers/command.rs)
        * 设置 Renderer 渲染状态