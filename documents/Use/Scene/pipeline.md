# Pipeline

## 渲染状态配置

* ColorFormat
  * 渲染目标的色彩格式
  * 每个 Pass 对应一个配置
  * 在 Scene 中为所有 Pass 做一个配置
  * Viewer 在配置 Pass 时需要注意 各 Pass 的 ColorFormat 是否相同
* DepthStencilFormat
  * 渲染目标的深度模板格式
  * 每个 Pass 对应一个配置
  * 在 Scene 中为所有 Pass 做一个配置
  * Viewer 在配置 Pass 时需要注意 各 Pass 的 DepthStencilFormat 是否相同
* Blend
  * 是否支持半透明混合
  * 每个 Pass 对应一个配置
  * 在 Scene 中为所有 Pass 做一个配置

* ClearColor
  * 渲染时 渲染目标的清屏颜色
  * 每个 Viewer 对应一个配置, 传递到 Renderer, 最终在 RenderPass 使用
* AutoClearColor
  * 渲染时 是否执行渲染目标的 颜色清屏
  * 每个 Viewer 对应一个配置, 传递到 Renderer, 最终在 RenderPass 使用
* ClearDepth
  * 渲染时 渲染目标的清屏深度
  * 每个 Viewer 对应一个配置, 传递到 Renderer, 最终在 RenderPass 使用
* AutoClearDepth
  * 渲染时 是否执行渲染目标的 深度清屏
  * 每个 Viewer 对应一个配置, 传递到 Renderer, 最终在 RenderPass 使用
* ClearStencil
  * 渲染时 渲染目标的清屏模板
  * 每个 Viewer 对应一个配置, 传递到 Renderer, 最终在 RenderPass 使用
* AutoClearStencil
  * 渲染时 是否执行渲染目标的 模板清屏
  * 每个 Viewer 对应一个配置, 传递到 Renderer, 最终在 RenderPass 使用

* DepthWrite
  * 是否 写入深度
  * 每个 Model 配置
  * 创建 Pipeline 时 需检查对应Pass是否支持
* DepthCompare
  * 深度测试配置
  * 每个 Model 配置
  * 创建 Pipeline 时 需检查对应Pass是否支持
* DepthBias
  * 深度偏移
  * 每个 Model 配置
  * 创建 Pipeline 时 需检查对应Pass是否支持
* Stencil
  * 模板配置
  * 每个 Model 配置
  * 创建 Pipeline 时 需检查对应Pass是否支持
* Blend
  * 半透明混合模式
  * 每个 Model 配置
  * 创建 Pipeline 时 需检查对应Pass是否支持

* CullMode
  * 面剔除模式
  * 每个 Model 配置
* PolygonMode
  * 多边形绘制模式
  * 每个 Model 配置
* FrontFace
  * 正面背面检测模式
  * 每个 Model 配置

## Vertices

* 