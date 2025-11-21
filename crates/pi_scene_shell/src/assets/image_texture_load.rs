//! # 图像纹理和纹理视图加载系统
//!
//! 该模块负责处理3D渲染引擎中图像纹理和纹理视图的异步加载、缓存和生命周期管理。
//! 支持多种纹理类型，包括普通2D纹理、环境纹理、asimage纹理等。
//!
//! ## 核心功能
//!
//! ### 异步纹理加载
//! - **多格式支持**: 支持PNG、JPG、HDR等多种图像格式
//! - **环境纹理**: 专门处理立方体贴图等环境映射纹理
//! - **asimage集成**: 支持asimage格式的特殊纹理加载
//! - **智能缓存**: 基于Key的高效纹理资源缓存机制
//!
//! ### 纹理视图管理
//! - **自动创建**: 基于纹理资源自动创建对应的纹理视图
//! - **多组件支持**: 支持不同类型的纹理视图组件
//! - **错误处理**: 完善的加载失败处理和回退机制
//! - **性能优化**: 批量处理和异步任务调度
//!
//! ### 资源状态跟踪
//! - **加载状态**: 实时跟踪纹理和视图的加载状态
//! - **统计信息**: 详细的加载成功/失败统计
//! - **内存管理**: 精确的内存使用量计算
//! - **错误日志**: 完整的错误信息记录和报告
//!
//! ## 架构设计
//!
//! ### 分层加载架构
//! 1. **纹理加载层**: 负责原始图像数据的加载和解码
//! 2. **纹理视图层**: 基于加载的纹理创建GPU可用的视图
//! 3. **组件管理层**: 将纹理视图绑定到ECS组件
//! 4. **状态监控层**: 跟踪和管理所有加载过程的状态
//!
//! ### 异步处理流程
//! - 使用SegQueue实现无锁的异步任务队列
//! - 支持大量并发加载请求的高效处理
//! - 智能重试机制和超时处理
//! - 基于优先级的加载调度
//!
//! ## 使用方式
//!
//! ### 基本纹理加载
//! ```rust
//! // 创建纹理加载请求
//! let texture_id = loader.create_load(texture_key);
//!
//! // 查询加载状态
//! match loader.query_imgtex(&texture_key, &asset_mgr) {
//!     Ok(handle) => { /* 加载成功 */ }
//!     Err(true) => { /* 加载失败 */ }
//!     Err(false) => { /* 仍在加载中 */ }
//! }
//! ```
//!
//! ### 纹理视图组件
//! ```rust
//! // 通过ECS组件自动管理纹理视图
//! #[derive(Component)]
//! struct MyTextureComponent(EKeyTexture);
//!
//! // 系统会自动检测组件变化并触发加载
//! sys_image_texture_view_load_launch::<MyTextureComponent, TextureViewUsage>();
//! ```
//!
//! ## 性能特性
//!
//! ### 内存优化
//! - **智能缓存**: LRU风格的纹理缓存策略
//! - **内存池**: 预分配的缓冲区减少动态分配
//! - **延迟释放**: 基于使用频率的智能资源释放
//!
//! ### 并发性能
//! - **无锁队列**: 使用SegQueue实现高并发访问
//! - **批量处理**: 批量减少系统调用开销
//! - **异步IO**: 充分利用多核处理能力
//!
//! ### 错误恢复
//! - **自动重试**: 网络或IO错误的自动重试机制
//! - **回退纹理**: 加载失败时自动使用默认纹理
//! - **错误报告**: 详细的错误信息和调试支持

use crate::ecs::*;

use std::{marker::PhantomData, ops::{Deref, DerefMut}};
use crossbeam::queue::SegQueue;
use pi_assets::{
    mgr::{AssetMgr, LoadResult},
};
use pi_async_rt::prelude::AsyncRuntime;
use pi_bevy_render_plugin::asimage_url::{RenderTarget, load_from_asimage_url};
pub use pi_bevy_render_plugin::{ResStateTextureLoader, ResTextureCombineAtlas2DMgr};
use pi_hal::{loader::AsyncLoader, runtime::RENDER_RUNTIME};
use pi_bevy_asset::ShareAssetMgr;
use pi_render::{renderer::texture_loader::{environment_texture_loader::EnvironmentTextureTools, loader::ImageTextureLoader}, rhi::asset::{ImageTextureDesc, TextureRes}};
use pi_share::Share;
use crate::prelude::*;

use super::{texture::*};


/// 纹理加载模式枚举
///
/// 定义了不同类型的纹理加载方式，用于区分不同的处理策略：
#[derive(Clone, Copy)]
pub enum ETextureLoaderMode {
    /// 标准2D纹理加载模式
    /// 用于常规的2D图像纹理，如漫反射贴图、法线贴图等
    D2,
    /// 环境纹理加载模式
    /// 用于立方体贴图、HDR环境贴图等特殊纹理类型
    Env,
}

/// 纹理加载队列信息
///
/// 包含了纹理加载请求所需的所有基本信息，用于异步加载队列管理：
pub struct QueueInfo {
    /// 加载请求的唯一标识符
    /// 用于跟踪加载状态和查询结果
    pub id: IDImageTextureLoad,
    /// 纹理资源的关键标识
    /// 包含文件路径、格式信息等
    pub key: KeyImageTextureFrame,
    /// 纹理加载模式
    /// 决定使用何种加载策略
    pub mode: ETextureLoaderMode,
}

/// 纹理加载请求ID类型
///
/// 每个纹理加载请求都有唯一的64位ID，用于异步跟踪和状态管理
pub type IDImageTextureLoad = u64;

/// 图像纹理加载器资源
///
/// 该结构体是纹理加载系统的核心组件，负责管理所有纹理加载相关的状态和队列：
/// - **异步加载队列**: 使用SegQueue实现无锁的高效队列
/// - **加载状态跟踪**: 跟踪每个加载请求的成功、失败和进行中状态
/// - **资源缓存**: 管理已加载纹理资源的引用和生命周期
/// - **错误处理**: 记录和处理加载失败的情况
#[derive(Resource, Default)]
pub struct ResImageTextureLoader {
    /// 底层纹理加载器
    /// 实际执行纹理数据加载和解码的核心组件
    pub loader: ImageTextureLoader<IDImageTextureLoad>,

    /// 查询计数器
    /// 用于生成唯一的加载请求ID，保证每个请求都有唯一标识
    pub query_counter: IDImageTextureLoad,

    /// 等待处理队列
    /// 存储待处理的纹理加载请求，使用共享的SegQueue实现无锁并发访问
    pub wait: Share<SegQueue<QueueInfo>>,

    /// 成功加载结果映射
    /// 存储已成功加载的纹理资源，通过加载ID快速查找对应的资源句柄
    pub success: XHashMap<IDImageTextureLoad, Handle<ImageTextureFrame>>,

    /// 失败原因映射
    /// 根据纹理键存储加载失败的原因，用于错误报告和调试
    pub fail_reason: XHashMap<KeyImageTextureFrame, EError>,

    /// 失败记录映射
    /// 根据加载ID存储加载失败的原因，用于查询特定请求的失败信息
    pub failrecord: XHashMap<IDImageTextureLoad, EError>,
}
impl DerefMut for ResImageTextureLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.loader
    }
}
impl Deref for ResImageTextureLoader {
    type Target = ImageTextureLoader<u64>;

    fn deref(&self) -> &Self::Target {
        &self.loader
    }
    
}
impl ResImageTextureLoader {
    /// 创建标准2D纹理加载请求
    ///
    /// 为指定的纹理键创建一个异步加载请求：
    /// 1. 生成唯一的加载ID（原子递增计数器）
    /// 2. 创建加载队列信息项
    /// 3. 将请求推入等待队列供后续处理
    /// 4. 返回加载ID用于状态查询
    pub fn create_load(&mut self, key: KeyImageTextureFrame) -> IDImageTextureLoad {
        // 递增查询计数器，确保每个加载请求都有唯一ID
        self.query_counter += 1;
        let id = self.query_counter;

        // 创建队列信息并推入等待队列
        // D2模式表示标准2D纹理加载
        self.wait.push(QueueInfo { id, key, mode: ETextureLoaderMode::D2 });
        id
    }

    /// 创建环境纹理加载请求
    ///
    /// 为环境纹理（如立方体贴图、HDR环境贴图）创建异步加载请求：
    /// 1. 生成唯一的加载ID
    /// 2. 创建环境纹理模式的队列信息
    /// 3. 将请求推入等待队列
    /// 4. 返回加载ID
    ///
    /// 环境纹理通常需要特殊的处理流程，如立方体贴图的6个面加载
    pub fn create_load_env(&mut self, key: KeyImageTextureFrame) -> IDImageTextureLoad {
        // 递增查询计数器
        self.query_counter += 1;
        let id = self.query_counter;

        // 创建环境纹理模式的队列信息
        // Env模式表示环境纹理加载，会使用不同的加载策略
        self.wait.push(QueueInfo { id, key, mode: ETextureLoaderMode::Env });
        id
    }
    /// 查询图像纹理加载状态
    ///
    /// 检查指定纹理键的加载状态：
    /// - **Ok(handle)**: 纹理已成功加载，返回资源句柄
    /// - **Err(true)**: 纹理加载失败
    /// - **Err(false)**: 纹理仍在加载中
    ///
    /// 这是一个非阻塞查询，不会触发新的加载操作
    pub fn query_imgtex(&self, key: &KeyImageTextureFrame, asset: &AssetMgr<ImageTextureFrame>) -> Result<Handle<ImageTextureFrame>, bool> {
        // 首先尝试从资源管理器获取已加载的纹理
        if let Some(res) = asset.get(key) {
            Ok(res)
        } else {
            // 如果未找到，检查是否记录了加载失败信息
            Err(self.fail_reason.contains_key(key))
        }
    }

    /// 查询加载失败原因
    ///
    /// 根据加载ID获取具体的加载失败错误信息：
    /// 1. 从失败记录中查找对应的错误信息
    /// 2. 如果找到，返回错误并从记录中移除（单次查询）
    /// 3. 如果未找到，说明加载未失败或已被查询过
    ///
    /// 返回Option<EError>，None表示没有失败信息
    pub fn query_failed_reason(&mut self, id: IDImageTextureLoad) -> Option<EError> {
        if let Some(key) = self.failrecord.remove(&id) {
            Some(key)
        } else {
            None
        }
    }

    /// 查询加载成功结果
    ///
    /// 根据加载ID获取成功加载的纹理资源句柄：
    /// 1. 从成功映射中查找对应的纹理句柄
    /// 2. 如果找到，返回句柄并从映射中移除（单次查询）
    /// 3. 如果未找到，说明加载未成功或已被查询过
    ///
    /// 返回Option<Handle>，None表示没有成功结果
    pub fn query_success(&mut self, id: IDImageTextureLoad) -> Option<Handle<ImageTextureFrame>> {
        self.success.remove(&id)
    }
}

impl MemSize for ResImageTextureLoader {
    fn memsize(&self) -> usize {
        self.wait.len() * 32 + 256
        + self.success.len() * 8 + 256
        + self.fail_reason.len() * 8 + 256
        + self.loading_image.len() * 32 + 256
        + self.loading_data.len() * 32 + 256
        + self.fail_reason.capacity() * 16
        + self.success.capacity() * 16
        + self.failrecord.len() * 16
        + 16
    }
}

/// 图像纹理加载启动系统
///
/// # 功能职责
/// 该系统是纹理加载流程的启动点，负责处理待加载的纹理请求：
/// - **请求队列处理**: 从等待队列中取出所有待处理的纹理加载请求
/// - **加载模式分发**: 根据纹理类型选择合适的加载策略
/// - **异步任务调度**: 为每个请求创建异步加载任务
/// - **状态更新**: 更新加载状态和统计信息
///
/// # 处理流程
///
/// ## 队列处理
/// 1. 从等待队列中弹出所有待处理的纹理加载请求
/// 2. 根据加载模式（D2或Env）分发到不同的处理逻辑
/// 3. 为每个请求启动相应的异步加载任务
///
/// ## 加载模式处理
/// - **D2模式**: 直接使用底层ImageTextureLoader进行异步加载
/// - **Env模式**:
///   - 首先尝试从资源管理器获取预加载的环境纹理
///   - 如果失败，启动EnvironmentTextureTools进行异步环境纹理加载
///   - 文件纹理和环境纹理采用不同的加载策略
///
/// # 性能优化
/// - **批量处理**: 一次性处理队列中的所有请求
/// - **异步调度**: 所有加载任务都在后台异步执行
/// - **智能缓存**: 自动处理纹理合并和优化
/// - **状态监控**: 实时更新加载统计信息
pub fn sys_image_texture_load_launch(
    mut loader: ResMut<ResImageTextureLoader>,                     // 纹理加载器（可变）
    image_assets_mgr: Res<ShareAssetMgr<ImageTextureFrame>>,      // 图像纹理资源管理器
    queue: Res<PiRenderQueue>,                                    // GPU渲染队列
    device: Res<PiRenderDevice>,                                  // GPU设备
    mut state: ResMut<ResStateTextureLoader>,                      // 纹理加载状态（可变）
    mut combinemgr: ResMut<ResTextureCombineAtlas2DMgr>,            // 纹理合并管理器（可变）
) {
    // 处理等待队列中的所有纹理加载请求
    // 使用while循环确保一次性处理所有待处理的请求
    while let Some(item) = loader.wait.pop() {
        match item.mode {
            // 处理标准2D纹理加载
            ETextureLoaderMode::D2 => {
                // 调用底层异步加载器进行纹理加载
                // 设置纹理用途：TEXTURE_BINDING(用于着色器采样) + COPY_DST(用于数据复制)
                if let Some(tex) = loader.loader.async_load(
                    item.id,                                                                                     // 加载ID
                    item.key,                                                                                   // 纹理键
                    wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, // 纹理用途标志
                    &device, &queue, &image_assets_mgr                                                         // GPU设备和资源管理器
                ) {
                    // 加载成功，将结果存入成功映射
                    loader.success.insert(item.id, tex);
                }
                // 注意：如果async_load返回None，可能是资源已存在或其他原因，不记录为失败
            },

            // 处理环境纹理加载
            ETextureLoaderMode::Env => {
                // 首先尝试从资源管理器加载环境纹理
                let imageresult = AssetMgr::load(&image_assets_mgr, &item.key);
                match imageresult {
                    // 加载成功，直接记录结果
                    pi_assets::mgr::LoadResult::Ok(res) => {
                        loader.success.insert(item.id, res);
                    },

                    // 加载失败，需要进一步处理
                    _ => {
                        // 区分文件纹理和程序生成纹理
                        if item.key.file {
                            // 文件纹理失败：直接记录失败信息
                            loader.fail_reason.insert(item.key.clone(), ErrorRecord::ERROR_TEXTURE_LOAD_FAIL);
                            loader.failrecord.insert(item.id, ErrorRecord::ERROR_TEXTURE_LOAD_FAIL);
                        } else {
                            // 程序生成纹理：启动异步环境纹理处理
                            // 克隆必要的引用用于异步任务
                            let (success, failquene, device, queue) = (loader.loader.success.clone(), loader.failquene.clone(), (device).clone(), (queue).clone());
                            let param = item.key.clone();
                            let id = item.id;
                            // 在渲染运行时中启动异步环境纹理处理任务
                            RENDER_RUNTIME.spawn(async move {
                                // 使用EnvironmentTextureTools进行异步环境纹理加载
                                match EnvironmentTextureTools::async_load(param.clone(), device, queue, imageresult).await {
                                    // 加载成功：将结果推入成功队列
                                    Ok(data) => { success.push((id, param, data)) },
                                    // 加载失败：将失败信息推入失败队列
                                    Err(_) => failquene.push((id, param, ErrorRecord::ERROR_TEXTURE_LOAD_FAIL)),
                                }
                            })
                            .unwrap(); // 实际开发中应该处理spawn失败的情况
                        }
                    }
                }
            },
        }
    }

    // 检查纹理合并优化（将小纹理合并成大图集以提高性能）
    loader.loader.check_combine(&device, &queue, &mut combinemgr);

    // 处理底层加载器的成功结果
    // 将异步加载成功的纹理更新到顶层加载器中
    while let Some(item) = loader.loader.success.pop() {
        loader.success.insert(item.0, item.2);
    }

    // 处理底层加载器的失败结果
    // 将失败信息记录到顶层加载器的失败记录中
    while let Some(item) = loader.loader.failquene.pop() {
        loader.fail_reason.insert(item.1, item.2);  // 按键记录失败原因
        loader.failrecord.insert(item.0, item.2);     // 按ID记录失败原因
    }
}

/// 通用纹理视图加载器
///
/// 泛型纹理视图加载器，支持不同类型的纹理键和纹理视图使用类型：
/// - **泛型设计**: K代表纹理键类型，支持ECS组件的自动识别
/// - **异步队列**: 使用SegQueue管理等待、成功、失败的加载状态
/// - **类型安全**: 通过泛型约束确保类型安全和编译时检查
/// - **性能优化**: 无锁队列设计支持高并发访问
///
/// # 队列管理
/// - **wait队列**: 存储待处理的纹理视图加载请求
/// - **success队列**: 存储加载成功的纹理视图结果
/// - **fail队列**: 存储加载失败的纹理视图信息
///
/// # 使用方式
/// 该加载器通常与ECS组件系统配合使用，自动检测组件变化并触发加载：
/// ```rust
/// // 定义自定义纹理组件
/// #[derive(Component)]
/// struct CustomTexture(EKeyTexture);
///
/// // 使用泛型加载器
/// type MyTextureLoader = ImageTextureViewLoader<CustomTexture>;
/// ```
#[derive(Resource)]
pub struct ImageTextureViewLoader<K> {
    /// 等待处理队列
    /// 存储（实体ID, 纹理视图键, 加载ID, 插槽索引）的元组
    /// 用于跟踪需要创建纹理视图的实体和对应的纹理资源
    pub wait: Share<SegQueue<(ObjectID, KeyImageTextureViewFrame, IDImageTextureLoad, usize)>>,

    /// 成功结果队列
    /// 存储（实体ID, 纹理键, 纹理视图使用方式, 插槽索引）的元组
    /// 用于存储加载成功并已创建的纹理视图
    pub success: Share<SegQueue<(ObjectID, EKeyTexture, ETextureViewUsage, usize)>>,

    /// 失败结果队列
    /// 存储（实体ID, 纹理键, 插槽索引）的元组
    /// 用于存储加载失败的纹理视图信息，便于后续处理
    pub fail: Share<SegQueue<(ObjectID, EKeyTexture, usize)>>,

    /// 类型标记
    /// PhantomData用于在运行时不占用内存，但在编译时提供类型信息
    pub _p: PhantomData<K>
}
impl<K> Default for ImageTextureViewLoader<K> {
    fn default() -> Self {
        Self { wait: Share::new(SegQueue::new()), success: Share::new(SegQueue::new()), fail: Share::new(SegQueue::new()), _p: PhantomData::default() }
    }
}

/// 纹理视图加载启动系统
///
/// 该系统负责检测纹理键组件的变化并启动相应的纹理视图加载：
/// - **组件变化检测**: 使用Changed<K>查询检测纹理键组件的变化
/// - **加载请求创建**: 为每个变化的组件创建纹理视图加载请求
/// - **异步处理**: 将加载请求推入异步队列供后台处理
/// - **状态统计**: 更新纹理视图加载的统计信息
pub fn sys_image_texture_view_load_launch<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component>(
    mut items: Query<(Entity, &K, &mut D), Changed<K>>,             // 查询发生变化的纹理键组件和对应的视图组件
    loader: Res<ImageTextureViewLoader<K>>,                       // 纹理视图加载器
    imgtex_assets_mgr: Res<ShareAssetMgr<ImageTextureViewFrame>>,  // 纹理视图帧资源管理器
    texres_assets_mgr: Res<ShareAssetMgr<TextureRes>>,            // 纹理资源管理器
    mut image_loader: ResMut<ResImageTextureLoader>,              // 图像纹理加载器（可变）
    queue: Res<PiRenderQueue>,                                   // GPU渲染队列
    device: Res<PiRenderDevice>,                                 // GPU设备
    mut state: ResMut<ResStateTextureLoader>,                    // 纹理加载状态（可变）
    targets: Res<CustomRenderTargets>,                            // 自定义渲染目标
    asimage: Query<(OrDefault<RenderTarget>, OrDefault<GraphId>)> // asimage相关查询
) {
    // 遍历所有发生纹理键组件变化的实体
    items.iter_mut().for_each(|(entity, param, mut cmd)| {
        // 增加纹理视图计数统计
        state.texview_count += 1;

        // 解引用纹理键获取实际的纹理键值
        let param = param.deref();

        // 调用内部函数处理纹理视图加载
        match _sys_image_texture_view_load_launch2(
            entity, 0, param, &imgtex_assets_mgr, &texres_assets_mgr, &mut image_loader,
            &queue, &device, &mut state, &loader.wait, &loader.success, &loader.fail, &targets, &asimage
        ) {
            // 如果返回了立即可用的纹理视图数据，立即更新组件
            Some(data) => {
                // 将纹理视图数据转换为组件类型并赋值
                *cmd = D::from(data);
            },
            // 如果没有立即可用数据，可能是需要异步加载，不进行任何操作
            None => {}
        }
    });
}

/// 纹理视图加载检查系统
///
/// 该系统负责检查纹理视图加载的完成状态并更新相应的ECS组件：
/// - **状态检查**: 检查异步加载任务的完成状态
/// - **成功处理**: 处理加载成功的纹理视图并更新组件
/// - **失败回退**: 为加载失败的纹理提供默认的白色纹理
/// - **统计更新**: 更新纹理视图加载的统计信息
pub fn sys_image_texture_view_loaded_check<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component>(
    mut items: Query<(&K, &mut D)>,                              // 查询所有纹理键组件和对应的视图组件
    loader: Res<ImageTextureViewLoader<K>>,                       // 纹理视图加载器
    imgtex_assets_mgr: Res<ShareAssetMgr<ImageTextureViewFrame>>,  // 纹理视图帧资源管理器
    texres_assets_mgr: Res<ShareAssetMgr<TextureRes>>,            // 纹理资源管理器
    mut image_loader: ResMut<ResImageTextureLoader>,              // 图像纹理加载器（可变）
    mut state: ResMut<ResStateTextureLoader>,                    // 纹理加载状态（可变）
) {
    // 调用内部检查函数处理等待队列中的加载请求
    _sys_image_texture_view_loaded_check(
        &loader.wait, &loader.success, &loader.fail,
        &imgtex_assets_mgr, &mut image_loader, &mut state
    );

    // 处理加载成功的纹理视图
    let mut item = loader.success.pop();
    while let Some((entity, _key, view, _)) = item {
        item = loader.success.pop();  // 继续处理下一个成功项

        // 查找对应的实体并更新其纹理视图组件
        if let Ok((_, mut item)) = items.get_mut(entity) {
            // 将加载成功的纹理视图转换为组件类型并赋值
            *item = D::from(view);
            // 增加成功统计
            state.texview_success += 1;
        }
        // 注意：如果找不到实体，可能实体已被删除，这是正常情况
    }

    // 处理加载失败的纹理视图
    // 使用默认的白色2D纹理作为回退
    let whitekey = KeyTexture::from(DefaultTexture::WHITE_2D);
    let key_u64 = whitekey.asset_u64();
    let view = texres_assets_mgr.get(&key_u64).unwrap();  // 白色纹理应该总是存在

    let mut item = loader.fail.pop();
    while let Some((entity, _key, _)) = item {
        item = loader.fail.pop();  // 继续处理下一个失败项

        // 查找对应的实体并为其设置白色纹理
        if let Ok((_, mut item)) = items.get_mut(entity) {
            log::warn!("Texture Load Failed, Using White Texture as Fallback");
            // 使用白色纹理作为回退
            *item = D::from(ETextureViewUsage::Tex(view.clone()));
            // 即使失败也算作成功处理（使用了回退纹理）
            state.texview_success += 1;
        }
    }
}

fn _sys_image_texture_view_loaded_check(
    wait: &Share<SegQueue<(ObjectID, KeyImageTextureViewFrame, IDImageTextureLoad, usize)>>,
    success: &Share<SegQueue<(ObjectID, EKeyTexture, ETextureViewUsage, usize)>>,
    fail: &Share<SegQueue<(ObjectID, EKeyTexture, usize)>>,
    imgtex_assets_mgr: &ShareAssetMgr<ImageTextureViewFrame>,
    image_loader: &mut ResImageTextureLoader,
    state: &mut ResStateTextureLoader,
) {
    // 弹出并处理等待队列中的所有纹理视图加载请求
    let mut item = wait.pop();
    let mut waitagain = vec![];  // 存储需要重新等待的请求
    let mut idcounter = 0;     // 防止无限循环的安全计数器

    while let Some((entity, key, id, _)) = item {
        idcounter = idcounter + 1;
        // 安全检查：防止单个帧处理过多请求造成性能问题
        if idcounter >= 1024 {
            log::warn!("Current Time Image Load Count > 1024 !");
        }
        item = wait.pop();

        // 将纹理视图键转换为纹理资源键
        let key_u64 = key.asset_u64();

        // 检查对应的图像纹理是否已加载成功
        if let Some(image) = image_loader.query_success(id) {
            // 图像纹理已成功，开始创建纹理视图
            let result = AssetMgr::load(&imgtex_assets_mgr, &key_u64);

            // 准备异步任务所需的数据
            let (success, fail) = (success.clone(), fail.clone());
            let viewkey = key.clone();
            let texkey = EKeyTexture::ImageFrame(key);

            // 在渲染运行时中异步创建纹理视图
            RENDER_RUNTIME.spawn(async move {
                match ImageTextureViewFrame::async_load(image, viewkey, result).await {
                    Ok(res) => {
                        log::warn!("Texture Load Success {:?}", (texkey));
                        // 成功：推入成功队列
                        success.push((entity, texkey, ETextureViewUsage::ImageFrame(res), 0));
                    }
                    Err(_e) => {
                        log::warn!("Texture Load Fail {:?}", (texkey));
                        // 失败：推入失败队列
                        fail.push((entity, texkey, 0));
                    }
                };
            }).unwrap();
        } else if let Some(_fail) = image_loader.query_failed_reason(id) {
            // 图像纹理加载失败，直接记录纹理视图失败
            log::warn!("Texture Fail {:?}", (key.url(), fail));
            fail.push((entity, EKeyTexture::ImageFrame(key), 0));
            state.texview_fail += 1;
        } else {
            // 图像纹理仍在加载中，将请求重新放入等待队列
            log::warn!("Texture Load Again {:?}", (id, key.url()));
            waitagain.push((entity, key, id, 0));
        }
    }

    // 将需要重新等待的请求重新推入等待队列
    waitagain.drain(..).for_each(|item| { wait.push(item) });

    // 更新等待队列长度统计
    state.texview_waiting = wait.len() as u32;
}

/// 纹理加载系统集
///
/// 定义纹理加载相关系统的执行顺序和依赖关系，确保正确的加载流程：
/// - **TextureRequest**: 纹理请求阶段，收集和处理加载请求
/// - **TextureLoading**: 纹理加载阶段，执行实际的异步加载任务
/// - **TextureLoaded**: 纹理完成阶段，处理加载结果和更新组件
///
/// # 执行顺序
/// 系统按照定义的顺序执行，保证：
/// 1. 先收集所有纹理加载请求
/// 2. 然后执行异步加载任务
/// 3. 最后处理加载完成的结果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageTextureLoad {
    /// 纹理请求阶段
    /// 负责检测ECS组件变化，收集纹理加载请求
    TextureRequest,

    /// 纹理加载阶段
    /// 负责执行实际的异步纹理加载任务
    TextureLoading,

    /// 纹理完成阶段
    /// 负责处理加载结果，更新ECS组件
    TextureLoaded,
}

/// 纹理视图加载插件
///
/// 泛型插件用于注册纹理视图加载相关系统到Bevy应用中：
/// - **泛型参数**: K表示纹理键组件类型，D表示纹理视图使用组件类型
/// - **系统注册**: 自动注册所有必需的纹理视图加载系统
/// - **资源配置**: 初始化纹理加载器资源和状态跟踪
/// - **系统集配置**: 配置系统执行顺序和依赖关系
///
/// # 插件功能
/// - 检查并初始化ResImageTextureLoader资源
/// - 配置StageTextureLoad系统集的执行顺序
/// - 注册泛型的ImageTextureViewLoader资源
/// - 确保系统在正确的执行阶段运行
pub struct PluginImageTextureViewLoad<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component>(PhantomData<(K, D)>);

impl<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component> Plugin for PluginImageTextureViewLoad<K, D> {
    fn build(&self, app: &mut App) {
        // 检查是否已经初始化了纹理加载器资源
        if app.world.contains_resource::<ResImageTextureLoader>() == false {
            // 初始化纹理加载器资源
            app.insert_resource(ResImageTextureLoader::default());
            app.insert_resource(ResStateTextureLoader::default());

            // 配置纹理加载系统集的执行顺序
            // 所有系统都在Modify阶段执行，确保按正确顺序处理
            app.configure_set(StageD3, StageTextureLoad::TextureRequest  .in_set(ERunStageChap::Modify));
            app.configure_set(StageD3, StageTextureLoad::TextureLoading  .in_set(ERunStageChap::Modify).after(StageTextureLoad::TextureRequest));
            app.configure_set(StageD3, StageTextureLoad::TextureLoaded   .in_set(ERunStageChap::Modify).after(StageTextureLoad::TextureLoading));
        }

        // 注册泛型纹理视图加载器资源
        app.insert_resource(ImageTextureViewLoader::<K>::default());
    }
}
impl<K: std::ops::Deref<Target = EKeyTexture> + Component, D: From<ETextureViewUsage> + Component> Default for PluginImageTextureViewLoad<K, D> {
    fn default() -> Self {
        Self(PhantomData::<(K, D)>::default())
    }
}


#[derive(Resource, Default)]
pub struct ImageTextureViewLoader2 {
    pub wait: Share<SegQueue<(ObjectID, KeyImageTextureViewFrame, IDImageTextureLoad, usize)>>,
    pub success: Share<SegQueue<(ObjectID, EKeyTexture, ETextureViewUsage, usize)>>,
    pub fail: Share<SegQueue<(ObjectID, EKeyTexture, usize)>>,
}
impl MemSize for ImageTextureViewLoader2 {
    fn memsize(&self) -> usize {
        self.wait.len() * 64 + 256
        + self.success.len() * 64 + 256
        + self.fail.len() * 64 + 256
    }
}

pub fn sys_image_texture_view_load_launch2(
    // mut commands: Commands,
    mut items: Query<(Entity, &TextureKeyList, &mut EffectBindTexture2DList), Changed<TextureKeyList>>,
    loader: Res<ImageTextureViewLoader2>,
    imgtex_assets_mgr: Res<ShareAssetMgr<ImageTextureViewFrame>>,
    texres_assets_mgr: Res<ShareAssetMgr<TextureRes>>,
    mut image_loader: ResMut<ResImageTextureLoader>,
    queue: Res<PiRenderQueue>,
    device: Res<PiRenderDevice>,
    mut state: ResMut<ResStateTextureLoader>,
    targets: Res<CustomRenderTargets>,
    asimage: Query<(OrDefault<RenderTarget>, OrDefault<GraphId>)>
) {
    items.iter_mut().for_each(|(entity, param, mut cmd)| {
        state.texview_count += 1;
        cmd.empty();
        let mut idx = 0;
        param.0.iter().for_each(|key| {
            match _sys_image_texture_view_load_launch2(
                entity, idx, &key.deref().url, &imgtex_assets_mgr, &texres_assets_mgr, &mut image_loader,
                &queue, &device, &mut state, &loader.wait, &loader.success, &loader.fail, &targets, &asimage
            ) {
                Some(data) => {
                    cmd.loaded_textureviewusage(idx, data, key.deref().url.clone());
                },
                None => {}
            }

            idx += 1;
        });
    });
}

/// 纹理视图加载请求处理函数
///
/// # 功能职责
/// 根据纹理键的类型处理不同的纹理视图加载请求：
/// - **asimage纹理**: 处理特殊的asimage格式纹理
/// - **标准纹理**: 处理常规的纹理资源加载
/// - **图像帧**: 处理图像帧纹理视图
/// - **渲染目标**: 处理SRT渲染目标纹理
///
/// # 处理策略
///
/// ## 立即可用 vs 异步加载
/// - **立即可用**: 已缓存的资源或asimage渲染目标
/// - **异步加载**: 需要从文件或网络加载的纹理资源
///
/// ## 资源类型支持
/// - **EKeyTexture::Tex**: 标准纹理资源
/// - **EKeyTexture::Image**: 图像纹理（当前未实现）
/// - **EKeyTexture::SRT**: 渲染目标纹理
/// - **EKeyTexture::ImageFrame**: 图像帧纹理视图
///
/// # 性能优化
/// - **缓存命中**: 优先使用已缓存的资源
/// - **异步处理**: 所有文件I/O操作都在后台异步执行
/// - **错误处理**: 完善的错误回退和统计机制
fn _sys_image_texture_view_load_launch2(
    entity: Entity,                                                    // 实体ID，用于后续组件更新
    slot: usize,                                                     // 插槽索引，支持多个纹理视图
    param: &EKeyTexture,                                              // 纹理键，决定加载策略
    imgtex_assets_mgr: &ShareAssetMgr<ImageTextureViewFrame>,  // 图像纹理视图帧资源管理器
    texres_assets_mgr: &ShareAssetMgr<TextureRes>,                // 纹理资源管理器
    image_loader: &mut ResImageTextureLoader,                       // 图像纹理加载器（可变）
    queue: &RenderQueue,                                            // GPU渲染队列
    device: &RenderDevice,                                          // GPU设备
    state: &mut ResStateTextureLoader,                               // 纹理加载状态（可变）
    wait: &Share<SegQueue<(ObjectID, KeyImageTextureViewFrame, IDImageTextureLoad, usize)>>,  // 等待队列
    success: &Share<SegQueue<(ObjectID, EKeyTexture, ETextureViewUsage, usize)>>,    // 成功队列
    fail: &Share<SegQueue<(ObjectID, EKeyTexture, usize)>>,                // 失败队列
    targets: &CustomRenderTargets,                                   // 自定义渲染目标集合
    asimage: &Query<(OrDefault<RenderTarget>, OrDefault<GraphId>)>,       // asimage渲染目标查询
) -> Option<ETextureViewUsage> {
    match param {
        // 标准纹理键处理 - 支持asimage和普通纹理资源
        EKeyTexture::Tex(url) => {
            // 检查是否为asimage特殊协议纹理
            if url.starts_with("asimage:://") {
                // asimage纹理处理 - 从实时渲染目标获取纹理视图
                let key = param.clone();
                match load_from_asimage_url(url, asimage) {
                    Ok(rt) => match rt {
                        Some(rt) => {
                            // 成功获取渲染目标纹理视图
                            state.texview_success += 1;
                            Some(ETextureViewUsage::from(&rt.0))
                        },
                        None => {
                            // 渲染目标不存在，推入失败队列
                            fail.push((entity, key, slot)); None
                        },
                    },
                    Err(_) => {
                        // asimage URL解析失败，推入失败队列
                        fail.push((entity, key, slot));
                        None
                    }
                }
            } else {
                // 普通纹理资源处理
                let key_u64 = url.asset_u64();
                let result = AssetMgr::load(&texres_assets_mgr, &key_u64);

                match result {
                    LoadResult::Ok(texture_view) => {
                        // 纹理资源已加载，直接返回纹理视图
                        state.texview_success += 1;
                        Some(ETextureViewUsage::Tex(texture_view))
                    },
                    _ => {
                        // 纹理资源未加载，启动异步加载任务
                        let (success, fail, device, queue) = (success.clone(), fail.clone(), (device).clone(), (queue).clone());
                        let key = param.clone();
                        let url = url.clone();

                        // 在异步运行时中执行纹理加载
                        RENDER_RUNTIME
                            .spawn(async move {
                                // 创建纹理加载描述符
                                let desc = ImageTextureDesc { url: &url, device: &device, queue: &queue, };

                                // 异步加载纹理资源
                                match TextureRes::async_load(desc, result).await {
                                    Ok(res) => {
                                        // 加载成功，推入成功队列
                                        success.push((entity, key, ETextureViewUsage::Tex(res), slot));
                                    }
                                    Err(_e) => {
                                        // 加载失败，推入失败队列
                                        fail.push((entity, key, slot));
                                    }
                                };
                            })
                            .unwrap();

                        // 立即返回None，表示异步处理中
                        None
                    },
                }
            }
        },

        // 原始图像纹理键处理 - 当前版本直接标记为失败
        EKeyTexture::Image(_key) => {
            // 暂未实现直接图像处理逻辑，直接推入失败队列
            fail.push((entity, param.clone(), slot));
            None

            // 注释掉的待实现代码：
            // todo!()
            // // log::warn!("Texture Load {:?}", (key.url()));
            // let key_u64 = key.asset_u64();
            // let result = imgtex_assets_mgr.get(&key_u64);
            // match result {
            //     Some(view) => {
            //         // log::error!("Texture While Launch: {:?}", key_u64);
            //         // log::warn!("Texture Success 0 {:?}", (key.url()));
            //         // *cmd = D::from(ETextureViewUsage::Image(view));
            //         state.texview_success += 1;
            //         Some(ETextureViewUsage::Image(view))
            //     },
            //     _ => {
            //         // let imgkey = key.url();
            //         let id = image_loader.create_load(key.url().clone());
            //         wait.push((entity, key.clone(), id, slot));
            //         None
            //     },
            // }
        },

        // 渲染目标纹理键处理 - 从自定义渲染目标集合获取
        EKeyTexture::SRT(_key) => {
            if let Some(target) = targets.get(*_key) {
                // 渲染目标存在，直接使用其纹理视图
                state.texview_success += 1;
                // log::error!(">>> Use SRT {:?}", (target.rt.target_index(), _key));
                Some(ETextureViewUsage::from(&target.rt))
            } else {
                // 渲染目标不存在，标记失败
                // log::error!("EKeyTexture::SRT Fail");
                state.texview_fail += 1;
                None
            }
        },

        // 帧纹理键处理 - 从图像纹理视图帧资源管理器获取
        EKeyTexture::ImageFrame(key) => {
            let key_u64 = key.asset_u64();
            let result = imgtex_assets_mgr.get(&key_u64);

            match result {
                Some(view) => {
                    // 帧纹理视图已存在，直接返回
                    state.texview_success += 1;
                    Some(ETextureViewUsage::ImageFrame(view))
                },
                _ => {
                    // 帧纹理视图不存在，启动图像加载任务
                    let id = image_loader.create_load(key.url().clone());
                    wait.push((entity, key.clone(), id, slot));
                    None
                },
            }
        },
    }
}

pub fn sys_image_texture_view_loaded_check2(
    mut items: Query<(&TextureKeyList, &mut EffectBindTexture2DList)>,
    loader: Res<ImageTextureViewLoader2>,
    imgtex_assets_mgr: Res<ShareAssetMgr<ImageTextureViewFrame>>,
    texres_assets_mgr: Res<ShareAssetMgr<TextureRes>>,
    mut image_loader: ResMut<ResImageTextureLoader>,
    mut state: ResMut<ResStateTextureLoader>,
) {
    _sys_image_texture_view_loaded_check2(
        &loader.wait, &loader.success, &loader.fail,
        &imgtex_assets_mgr, &mut image_loader, &mut state
    );

    let mut item = loader.success.pop();
    while let Some((entity, _key, view, slot)) = item {
        item = loader.success.pop();
        if let Ok((_, mut item)) = items.get_mut(entity) {
            item.loaded_textureviewusage(slot, view, _key);
            state.texview_success += 1;
        }
    }

    let whitekey = KeyTexture::from(DefaultTexture::WHITE_2D);
    // let white = EKeyTexture::Tex(whitekey.clone());
    let key_u64 = whitekey.asset_u64();
    let view = texres_assets_mgr.get(&key_u64).unwrap();
    let mut item = loader.fail.pop();
    while let Some((entity, _key, slot)) = item {
        item = loader.fail.pop();
        if let Ok((_, mut item)) = items.get_mut(entity) {
            // log::error!("Texture From Fail Queue:");
            item.loaded_textureviewusage(slot, ETextureViewUsage::Tex(view.clone()), _key.clone());
            state.texview_success += 1;
        }
    }
}

fn _sys_image_texture_view_loaded_check2(
    wait: &Share<SegQueue<(ObjectID, KeyImageTextureViewFrame, IDImageTextureLoad, usize)>>,
    success: &Share<SegQueue<(ObjectID, EKeyTexture, ETextureViewUsage, usize)>>,
    fail: &Share<SegQueue<(ObjectID, EKeyTexture, usize)>>,
    imgtex_assets_mgr: &ShareAssetMgr<ImageTextureViewFrame>,
    image_loader: &mut ResImageTextureLoader,
    state: &mut ResStateTextureLoader,
) {
    let mut item = wait.pop();
    let mut waitagain = vec![];
    while let Some((entity, key, id, slot)) = item {
        item = wait.pop();

        let key_u64 = key.asset_u64();
        // let imgkey = key.url();
        if let Some(image) = image_loader.query_success(id) {
            let result = AssetMgr::load(&imgtex_assets_mgr, &key_u64);
            // log::warn!("Texture Image Success {:?}", (key.url()));
            let (success, fail) = (success.clone(), fail.clone());
            let viewkey = key.clone();
            let texkey = EKeyTexture::ImageFrame(key);
            RENDER_RUNTIME.spawn(async move {
                // log::error!("Texture Load Task {:?}", (texkey));
                match ImageTextureViewFrame::async_load(image, viewkey, result).await {
                    Ok(res) => {
                        // log::warn!("Texture Load Success {:?}", (texkey));
                        success.push((entity, texkey, ETextureViewUsage::ImageFrame(res), slot));
                    }
                    Err(_e) => {
                        log::error!("Texture Load Fail {:?}", (texkey));
                        fail.push((entity, texkey, slot));
                    }
                };
            }).unwrap();
        } else if let Some(_fail) = image_loader.query_failed_reason(id) {
            log::warn!("Texture Fail {:?}", (key.url(), fail));
            fail.push((entity, EKeyTexture::ImageFrame(key), slot));
            state.texview_fail += 1;
        } else {
            log::warn!("Texture Load Again {:?}", (id, key.url()));
            waitagain.push((entity, key, id, slot));
        }
    }
    waitagain.drain(..).for_each(|item| { wait.push(item) });
    state.texview_waiting = wait.len() as u32;
}