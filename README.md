# pi_3d

[![Crates.io](https://img.shields.io/crates/v/pi_3d.svg)](https://crates.io/crates/pi_3d)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

**pi_3d** 是一个基于 **ECS（Entity Component System）** 架构的现代3D渲染引擎，使用 **wgpu** 作为图形后端，支持跨平台渲染。项目版本为 **1.3.8**，是一个功能完整、架构清晰的3D场景渲染框架。

## 🚀 特性

### 核心特性
- ✅ **ECS架构**: 基于 pi_world 的高性能组件系统
- ✅ **跨平台渲染**: 基于 wgpu，支持 Vulkan、Metal、OpenGL、WebGPU
- ✅ **现代渲染管线**: PBR、阴影、后处理效果
- ✅ **节点材质系统**: 灵活的模块化材质构建
- ✅ **高性能粒子系统**: 完整的粒子生命周期管理
- ✅ **骨骼动画**: 支持复杂的骨骼动画系统
- ✅ **资源管理**: 异步资源加载和生命周期管理

### 渲染特性
- 🎨 **多通道渲染**: 阴影、预深度、不透明、透明等渲染通道
- 🔆 **光照系统**: 方向光、点光源、聚光灯
- 🌊 **材质系统**: PBR、无光照、节点材质
- 💫 **粒子系统**: 发射器、轨迹、生命周期管理
- 🎭 **动画系统**: 骨骼动画、属性动画、场景动画
- 📦 **实例化渲染**: 高效的批量对象渲染

## 📦 项目结构

```
pi_3d/
├── crates/                          # 核心模块
│   ├── pi_scene_context/            # 场景管理核心
│   │   ├── scene/                   # 场景管理
│   │   ├── transforms/              # 变换系统
│   │   ├── cameras/                 # 相机系统
│   │   ├── cullings/                # 剔除系统
│   │   ├── renderers/               # 渲染器管理
│   │   ├── meshes/                  # 网格管理
│   │   ├── materials/               # 材质系统
│   │   ├── animation/               # 动画系统
│   │   ├── light/                   # 光照系统
│   │   ├── shadow/                  # 阴影系统
│   │   └── skeleton/                # 骨骼系统
│   ├── pi_scene_shell/              # 渲染引擎外壳
│   │   ├── engine_shell/            # 引擎核心
│   │   ├── pipeline/                # 渲染管线
│   │   ├── batch/                   # 批次处理
│   │   └── bind_groups/             # 绑定组管理
│   ├── pi_node_materials/           # 节点材质系统
│   ├── pi_particle_system/          # 粒子系统
│   ├── pi_gltf2_load/               # GLTF2资源加载
│   ├── pi_mesh_builder/             # 网格构建器
│   ├── pi_trail_renderer/           # 轨迹渲染器
│   ├── unlit_material/              # 无光照材质
│   ├── pi_standard_material/        # 标准材质
│   └── pi_pbr/                      # PBR材质
├── examples/                        # 示例代码
│   ├── base.rs                      # 基础示例
│   ├── light.rs                     # 光照示例
│   ├── pbr_material.rs              # PBR材质示例
│   └── ...                          # 更多示例
└── src/                             # 主要入口文件
    └── lib.rs                       # 库入口
```

## 🏗️ 架构设计

### ECS架构

pi_3d 使用高性能的 ECS（Entity Component System）架构，提供灵活的组件管理和高效的系统执行：

#### 核心组件类型
- **场景组件**: 场景创建、销毁、时间管理
- **变换组件**: 位置、旋转、缩放、层级关系
- **相机组件**: 透视/正交相机、视锥体裁剪
- **网格组件**: 几何体、材质、渲染状态
- **光照组件**: 方向光、点光源、聚光灯
- **动画组件**: 骨骼动画、属性动画
- **粒子组件**: 粒子发射器、粒子系统
- **渲染组件**: 渲染器、渲染目标

#### 系统编排
```rust
// 运行阶段定义
- First阶段     // 初始化
- Update阶段    // 主要逻辑更新
- PostUpdate阶段 // 后处理更新
- End阶段       // 清理和资源释放
```

### 渲染管线架构

#### 多通道渲染
项目支持复杂的多通道渲染管线：

```rust
// 渲染通道定义
PASS_SHADOW: PassTag::PASS_TAG_01,       // 阴影通道
PASS_PRE_DEPTH: PassTag::PASS_TAG_02,    // 预深度通道
PASS_OPAQUE: PassTag::PASS_TAG_03,       // 不透明通道
PASS_HIGHLIGHT: PassTag::PASS_TAG_04,    // 高亮通道
PASS_SKY_WATER: PassTag::PASS_TAG_06,    // 天空/水面通道
PASS_TRANSPARENT: PassTag::PASS_TAG_07,  // 透明通道
```

#### 渲染组织结构
```
不透明渲染阶段
├── 材质一 渲染
└── 材质二 渲染

半透明渲染阶段
├── 层级 1
│   ├── 材质一 渲染
│   └── 材质二 渲染
├── 层级 2
│   ├── 材质一 渲染
│   └── 材质二 渲染
└── ...
└── 层级 4000
    ├── 材质一 渲染
    └── 材质二 渲染
```

## 🎯 核心功能模块

### 1. 场景管理 (pi_scene_context)

3D场景的核心上下文管理模块，提供完整的场景生命周期管理：

**主要功能**:
- 场景创建、销毁、管理
- 变换系统（位置、旋转、缩放）
- 相机系统（透视、正交）
- 视锥体裁剪系统
- 渲染器管理
- 网格和材质管理
- 动画系统
- 光照和阴影系统
- 骨骼动画系统

### 2. 渲染引擎外壳 (pi_scene_shell)

渲染管线的运行时管理，负责渲染流程的控制和优化：

**主要功能**:
- 渲染管线的生命周期管理
- 资源加载和释放
- 批次处理优化
- 性能监控
- 自定义渲染目标
- 绑定组管理
- 着色器管理

### 3. 材质系统 (pi_node_materials)

基于节点的模块化材质系统，支持灵活的材质构建：

**核心特性**:
- 节点式材质构建器
- 材质块的组合和依赖
- 丰富的材质节点类型

**主要材质节点**:
- `BlockMainTexture`: 主纹理节点
- `BlockOpacity`: 透明度节点
- `BlockEmissiveTexture`: 自发光纹理
- `BlockFresnel`: 菲涅尔效果
- `BlockMixTexture`: 混合纹理
- `BlockFog`: 雾效节点
- `BlockColorSpace`: 颜色空间转换

### 4. 粒子系统 (pi_particle_system)

高性能粒子系统，支持复杂的粒子效果：

**核心系统**:
- `sys_particle_active`: 粒子激活管理
- `sys_prewarm`: 预热系统
- `sys_emission`: 发射系统
- `sys_over_lifetime`: 生命周期系统
- `sys_direction`: 方向系统
- `sys_update_buffer`: 缓冲区更新

**粒子特性**:
- 支持多种发射形状
- 生命周期和速度调制
- 颜色、大小、旋转变化
- 轨迹和拖尾效果
- 基于速度的效果

### 5. 资源管理 (pi_gltf2_load)

GLTF2资源加载器，支持现代3D资源格式：

**特点**:
- GLTF2格式解析
- 集成动画系统
- 支持粒子系统导入
- 异步资源加载

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- 支持 wgpu 的图形驱动
- 对于 Web 平台，需要 WebGPU 支持

### 安装依赖

```toml
[dependencies]
pi_3d = "1.3"
```

### 基础示例

```rust
use pi_3d::*;

fn main() {
    // 初始化应用
    let (mut app, window, event_loop) = test_plugins();

    // 设置场景
    app.add_systems(Startup, setup_scene);

    // 运行循环
    run_loop(app, window, event_loop);
}

fn setup_scene(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    // ... 其他参数
) {
    // 创建场景
    let scene = commands.spawn_empty_id();
    actions.scene.create.push(OpsSceneCreation::ops(
        scene,
        SceneBoundingPool::MODE_LIST,
        SceneColliderPool::MODE_LIST,
        [-9999, -9999, -9999, 9999, 9999, 9999, 0, 0, 0]
    ));

    // 创建相机
    let camera = commands.spawn_empty_id();
    actions.transform.tree.push(OpsTransformNodeParent::ops(camera, scene));
    actions.camera.create.push(OpsCameraCreation::ops(scene, camera, camera));

    // 创建3D对象
    let mesh = commands.spawn_empty_id();
    actions.transform.tree.push(OpsTransformNodeParent::ops(mesh, scene));
    actions.mesh.create.push(OpsMeshCreation::ops(scene, mesh, MeshInstanceState::default()));
    // ... 更多设置
}
```

## 📊 性能优化

### 渲染优化
- **实例化渲染**: 支持大量相同对象的批量渲染
- **批次处理**: 自动合并渲染调用
- **视锥体裁剪**: 减少不必要的渲染
- **遮挡剔除**: 基于深度信息的剔除

### 内存管理
- **内存池**: 减少动态内存分配
- **固定大小缓冲区分配器**: 高效的GPU内存管理
- **资源生命周期管理**: 自动清理未使用的资源

### 性能监控
- **实时统计**: 监控渲染调用、内存使用等
- **性能分析**: 内置性能分析工具
- **调试信息**: 丰富的调试输出

## 🔧 配置选项

### 特性标志

```toml
[features]
default = []
use_bevy = []           # 启用 Bevy 集成
use_pi_ecs = []         # 启用 pi_ecs 集成
dhat-heap = []          # 堆内存分析
dhat-ad-hoc = []        # 临时内存分析
```

### 编译优化

发布版本已启用多种优化：
```toml
[profile.release]
debug = true
strip = true
lto = true
```

## 📚 依赖关系

### 核心依赖

| 依赖库 | 版本 | 描述 |
|--------|------|------|
| pi_world | 0.2 | ECS框架 |
| wgpu | 0.3 | 图形后端 |
| pi_render | 0.3 | 渲染引擎 |
| pi_hal | 0.3 | 硬件抽象层 |
| pi_animation | 0.2 | 动画系统 |
| pi_scene_math | 0.0.* | 数学库 |

### 支持库

- **serde**: 序列化支持
- **parry3d**: 3D碰撞检测
- **nalgebra**: 数学计算
- **crossbeam**: 并发处理
- **futures**: 异步编程

## 🌐 跨平台支持

### 支持平台
- ✅ Windows (Vulkan/D3D12/OpenGL)
- ✅ Linux (Vulkan/OpenGL)
- ✅ macOS (Metal/OpenGL)
- ✅ Web (WebGPU)
- ✅ Android (Vulkan)
- ✅ iOS (Metal)

### WebGPU 支持

项目完全支持 WebGPU，提供：
- 现代化的 Web 图形API
- 原生性能的 3D 渲染
- 跨浏览器兼容性

## 📖 示例项目

项目包含丰富的示例代码：

### 基础示例
- `cube`: 基础立方体渲染
- `unlit_cube`: 无光照材质立方体
- `sprite`: 精灵渲染

### 进阶示例
- `lighting_*`: 各种光照效果
- `anime_*`: 动画系统演示
- `particles/*`: 粒子系统效果
- `pbr/*`: PBR材质渲染
- `gltf_load`: GLTF模型加载

### 性能测试
- `drawlistperformance`: 渲染性能测试
- `instance_*`: 实例化渲染测试

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

### 开发环境设置

1. 克隆项目
```bash
git clone https://github.com/GaiaWorld/pi_3d.git
cd pi_3d
```

2. 安装 Rust 工具链
```bash
rustup update stable
```

3. 运行示例
```bash
cargo run --example cube
```

### 代码规范
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 添加适当的注释和文档

## 📄 许可证

本项目采用双许可证：

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

## 🙏 致谢

感谢以下开源项目的支持：

- [wgpu](https://github.com/gfx-rs/wgpu) - 现代 Rust 图形抽象
- [bevy](https://github.com/bevyengine/bevy) - 数据驱动的游戏引擎
- [nalgebra](https://github.com/dimforge/nalgebra) - 线性代数库
- [parry](https://github.com/dimforge/parry) - 2D/3D 碰撞检测库

---

**pi_3d** - 现代、高性能的 Rust 3D 渲染引擎 🚀

如果项目对您有帮助，请给我们一个 ⭐ Star！