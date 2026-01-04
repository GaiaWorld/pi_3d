
use core::f32;
use std::ops::Range;

use pi_scene_shell::prelude::*;

use crate::{
    geometry::vertex_buffer_useinfo::*,
    prelude::*,
};

use super::{*, instanced_buffer::*, types::ModelInstanceAttributes, };

/// 单一实例缓冲区更新系统 - 处理使用单一缓冲区的实例化网格更新
///
/// # 功能说明
/// 此系统专门处理那些配置为使用单一实例缓冲区的实例化网格，当实例源引用发生变化时：
/// 1. 收集所有活动实例的数据
/// 2. 按渲染队列和透明度进行排序
/// 3. 更新GPU缓冲区数据
///
/// # 参数说明
/// - `actives`: 查询所有活动的实例网格组件（包含启用状态、渲染队列参数、裁剪标志、全局矩阵和局部位置）
/// - `instanceattributes`: 查询实例的模型属性数据（位置、颜色、缩放等自定义属性）
/// - `changeds`: 监听实例源引用变化的组件变更事件
/// - `sources`: 查询实例源的相关配置和状态信息
/// - `dispoeds`: 查询实体的销毁状态
/// - `geometrys`: 查询几何体的实例化信息
/// - `slots`: 查询和修改顶点缓冲区槽位信息
/// - `instancedcache`: 实例缓冲区分配器（只读引用）
/// - `allocator`: 顶点缓冲区分配器（可变引用）
/// - `device`: 渲染设备资源
/// - `queue`: 渲染命令队列
/// - `temp`: 临时数据容器，用于排序和收集实例数据
/// - `entitysets`: 用于过滤重复的实体变更事件
pub fn sys_tick_instanced_buffer_update_single(
    actives: Query<(&GlobalEnable, &RenderQueueSortParam, &AbstructMeshCullingFlag, &GlobalMatrix, &LocalPosition), With<InstanceMesh>>,
    instanceattributes: Query<&ModelInstanceAttributes>,
    changeds: ComponentChanged<InstanceSourceRefs>,
    mut sources: Query<
        (
            Entity, &EInstanceSortMode, &InstanceSourceRefs, &GeometryID, &MeshInstanceState, &mut InstancedSortedCollection
        )
    >,
    dispoeds: Query<&DisposeReady>,
    geometrys: Query<&InstancedInfoComp>,
    mut slots: Query<(&AssetDescVBSlots, &mut AssetResVBSlots, &mut LoadedKeyVBSlots, &mut FlagGeometryDirty)>,
    instancedcache: Res<InstanceBufferAllocator>,
    mut allocator: ResMut<VertexBufferAllocator3D>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
    mut temp: ResMut<TmpCommonVec>,
    entitysets: Res<EntityFilterForComponentChanged>,
    // mut performance: ResMut<Performance>,
) {
    // 性能监控代码（已注释）
    // performance.systems.push(String::from("sys_tick_instanced_buffer_update_single"));

    // 从实体过滤器中获取当前帧的实体集合，用于避免重复处理
    let mut entities = entitysets.pop();

    // 遍历所有发生实例源引用变化的实体
    changeds.iter().for_each(|entity| {
        // 如果实体已经在当前帧处理过，则跳过，避免重复处理
        if !entities.insert(entity) { return; }

        // 获取实例源的相关配置信息，如果获取失败则跳过
        if let Ok((idsource, sortmode, instances, idgeo, meshinsstate, mut instancessortinfos)) = sources.get_mut(*entity) {
            // 检查实例源是否已被销毁，如果已销毁则跳过处理
            if let Ok(disposed) = dispoeds.get(idsource) {
                if disposed.0 == true { return; }

                // 检查是否配置为使用单一实例缓冲区，如果不是则跳过
                if meshinsstate.use_single_instancebuffer == false { return; }

                // 获取几何体的实例化信息，如果获取失败则跳过
                if let Ok(InstancedInfoComp(Some(buffer))) = geometrys.get(idgeo.0) {
                    // 重置实例排序信息集合，清理上一帧的数据
                    instancessortinfos.reset();
                    // 设置每个实例的数据大小（字节数）
                    instancessortinfos.sizeperinstance = buffer.bytes_per_instance as u16;
                    // 设置是否使用单一实例缓冲区的标志
                    instancessortinfos.use_single_instancebuffer = meshinsstate.use_single_instancebuffer;

                    // 清空临时数据容器，准备收集本帧的实例数据
                    temp.clear();

                    // 收集实例信息并按渲染队列排序
                    // u32::MAX 表示最大批处理实例数量限制（无限）
                    if collect_instance_info(sortmode, instances, &mut instancessortinfos, &actives, &dispoeds, &instanceattributes, &mut temp, u32::MAX) {
                        // 获取收集到的实例数据切片，用于上传到GPU
                        let collected = instancessortinfos.data.as_slice();
                        let instancedinfo = buffer;

                        // 获取几何体对应的顶点缓冲区槽位信息
                        if let Ok((desclist, mut buffer, mut keys, mut flag)) = slots.get_mut(idgeo.0) {
                            // 根据实例化信息的槽位配置，获取对应的缓冲区引用
                            // 同时更新缓冲区版本键和脏标记
                            let buffer = match instancedinfo.slot() {
                                EVertexBufferSlot::Slot01 => {
                                    if let Some(buffer) = &mut buffer[0] {
                                        keys.0[0] = desclist.key(0); // 更新版本键
                                        *flag = FlagGeometryDirty;   // 标记几何体数据为脏
                                        &mut buffer.0
                                    } else { return; }
                                },
                                EVertexBufferSlot::Slot02 => {
                                    if let Some(buffer) = &mut buffer[1] {
                                        keys.0[1] = desclist.key(1);
                                        *flag = FlagGeometryDirty;
                                        &mut buffer.0
                                    } else { return; }
                                },
                                EVertexBufferSlot::Slot03 => {
                                    if let Some(buffer) = &mut buffer[2] {
                                        keys.0[2] = desclist.key(2);
                                        *flag = FlagGeometryDirty;
                                        &mut buffer.0
                                    } else { return; }
                                },
                                EVertexBufferSlot::Slot04 => {
                                    if let Some(buffer) = &mut buffer[3] {
                                        keys.0[3] = desclist.key(3);
                                        *flag = FlagGeometryDirty;
                                        &mut buffer.0
                                    } else { return; }
                                },
                                EVertexBufferSlot::Slot05 => {
                                    if let Some(buffer) = &mut buffer[4] {
                                        keys.0[4] = desclist.key(4);
                                        *flag = FlagGeometryDirty;
                                        &mut buffer.0
                                    } else { return; }
                                },
                                EVertexBufferSlot::Slot06 => {
                                    if let Some(buffer) = &mut buffer[5] {
                                        keys.0[5] = desclist.key(5);
                                        *flag = FlagGeometryDirty;
                                        &mut buffer.0
                                    } else { return; }
                                },
                                EVertexBufferSlot::Slot07 => {
                                    if let Some(buffer) = &mut buffer[6] {
                                        keys.0[6] = desclist.key(6);
                                        *flag = FlagGeometryDirty;
                                        &mut buffer.0
                                    } else { return; }
                                },
                                EVertexBufferSlot::Slot08 => {
                                    if let Some(buffer) = &mut buffer[7] {
                                        keys.0[7] = desclist.key(7);
                                        *flag = FlagGeometryDirty;
                                        &mut buffer.0
                                    } else { return; }
                                },
                                _ => { return; } // 不支持的槽位，直接返回
                            };

                            // 更新单一实例缓冲区，将收集的数据上传到GPU
                            update_instanced_buffer_for_single(buffer, &collected, &instancedcache, &mut allocator, &device, &queue);
                        }
                    }
                }
            }
        }
    });

    // 将处理过的实体集合重新放回过滤器，供下一帧使用
    entitysets.push(entities);
}


/// 多缓冲区实例更新系统 - 处理使用多个缓冲区的实例化网格更新
///
/// # 功能说明
/// 与单一缓冲区系统不同，此系统处理使用多个实例缓冲区的实例化网格，支持：
/// 1. 批量实例处理，受最大批处理数量限制
/// 2. 分布式缓冲区管理
/// 3. 收集阶段与上传阶段分离
///
/// # 性能优化
/// - 收集实例数据但不立即上传，延迟到专门的upload系统中
/// - 支持实例批处理，避免单个缓冲区过大的问题
/// - 使用临时向量优化排序性能
pub fn sys_tick_instanced_buffer_update(
    changeds: ComponentChanged<InstanceSourceRefs>,
    actives: Query<(&GlobalEnable, &RenderQueueSortParam, &AbstructMeshCullingFlag, &GlobalMatrix, &LocalPosition), With<InstanceMesh>>,
    instanceattributes: Query<&ModelInstanceAttributes>,
    mut sources: Query<
        (
            Entity, &EInstanceSortMode, &InstanceSourceRefs, &GeometryID, &MeshInstanceState, &mut InstancedSortedCollection
        ),
    >,
    dispoeds: Query<&DisposeReady>,
    geometrys: Query<&InstancedInfoComp>,
    mut temp: ResMut<TmpCommonVec>,
    engineopt: Res<EngineCustomPlugins>,
    entitysets: Res<EntityFilterForComponentChanged>,
    // mut performance: ResMut<Performance>,
) {
    // 从实体过滤器中获取当前帧的实体集合
    let mut entities = entitysets.pop();

    // 性能监控代码（已注释）
    // performance.systems.push(String::from("sys_tick_instanced_buffer_update"));
    // log::error!("Instance Update");

    // 统计成功处理的实例源数量
    let mut counter = 0;

    // 遍历所有发生实例源引用变化的实体
    changeds.iter().for_each(|entity| {
        // 避免重复处理同一实体
        if !entities.insert(entity) { return; }

        // 获取实例源的配置信息
        if let Ok((idsource, sortmode, instances, idgeo, meshinsstate, mut instancessortinfos)) = sources.get_mut(*entity) {
            // 检查实例源是否已被销毁
            if let Ok(disposed) = dispoeds.get(idsource) {
                if disposed.0 == true { return; }

                // 跳过使用单一缓冲区的实例（它们由另一个系统处理）
                if meshinsstate.use_single_instancebuffer == true { return; }

                // 获取几何体的实例化信息
                if let Ok(InstancedInfoComp(Some(instancedinfo))) = geometrys.get(idgeo.0) {
                    // 禁用渲染的代码（已注释）
                    // *renderenable = RenderGeometryEable(false);

                    // 清空临时数据容器，准备收集实例数据
                    temp.clear();

                    // 重置实例排序信息集合
                    instancessortinfos.reset();
                    // 设置缓冲区使用模式标志
                    instancessortinfos.use_single_instancebuffer = meshinsstate.use_single_instancebuffer;
                    // 设置每个实例的数据大小
                    instancessortinfos.sizeperinstance = instancedinfo.bytes_per_instance as u16;

                    // 收集实例信息，使用引擎配置的最大批处理数量限制
                    if collect_instance_info(sortmode, instances, &mut instancessortinfos, &actives, &dispoeds, &instanceattributes, &mut temp, engineopt.max_instance_batch_count) {
                        // 成功收集到实例数据，增加计数器
                        counter += 1;
                    }

                    // 调试输出（已注释）
                    // log::error!("{:?}", (instancessortinfos.count, instancessortinfos.data.len()));
                }
            }
        }
    });

    // 将处理过的实体集合重新放回过滤器
    entitysets.push(entities);

    // 调试输出（已注释）
    // log::error!("sys_tick_instanced_buffer_update {:?}", (counter));
}


/// 实例缓冲区上传系统 - 将收集的实例数据上传到GPU
///
/// # 功能说明
/// 此系统负责将多缓冲区更新系统收集的实例数据上传到GPU，与收集系统分离的原因是：
/// 1. 收集和上传分离，提高并行处理能力
/// 2. 批量上传减少GPU状态切换
/// 3. 统一管理GPU命令队列操作
///
/// # 调用时机
/// 在所有实例收集系统执行完成后调用，确保数据完整性和上传效率
pub fn sys_instanced_buffer_upload(
    mut instancedcache: ResMut<InstanceBufferAllocator>,
    queue: Res<PiRenderQueue>,
    // mut performance: ResMut<Performance>,
) {
    // 性能监控代码（已注释）
    // performance.systems.push(String::from("sys_instanced_buffer_upload"));

    // 执行实例缓冲区数据上传
    // 将所有收集的实例数据批量上传到GPU
    instancedcache.upload(&queue);
}

/// 单一实例缓冲区更新函数 - 更新指定的实例缓冲区数据
///
/// # 功能说明
/// 根据新数据的大小决定是复用现有缓冲区还是重新创建缓冲区，优化GPU内存使用：
/// 1. 如果现有缓冲区足够大且未被实例缓冲区分配器使用，则直接更新数据
/// 2. 否则创建新的不可更新缓冲区，避免频繁的内存分配
///
/// # 参数说明
/// - `oldbuffer`: 要更新的旧缓冲区（可变引用）
/// - `collected`: 收集到的实例数据字节数组
/// - `instancedcache`: 实例缓冲区分配器，用于检查缓冲区使用状态
/// - `allocator`: 顶点缓冲区分配器，用于创建新缓冲区
/// - `device`: 渲染设备
/// - `queue`: 渲染命令队列
pub fn update_instanced_buffer_for_single(
    oldbuffer: &mut EVerticesBufferTmp,
    collected: &[u8],
    instancedcache: &InstanceBufferAllocator,
    allocator: &mut VertexBufferAllocator3D,
    device:&RenderDevice,
    queue: &PiRenderQueue,
) {
    match oldbuffer {
        EVerticesBufferTmp::Instance(_) => {
            // 实例类型的缓冲区暂不处理
        },
        EVerticesBufferTmp::Buffer(oldbuffer) => match oldbuffer {
            EVerticesBufferUsage::EVBRange(buffer) => {
                // 判断是否需要创建新缓冲区
                let need_new_buffer = if instancedcache.check(buffer.buffer()) == false {
                    // 缓冲区未被实例缓存使用，检查容量是否足够
                    buffer.buffer().size() < collected.len() as u64
                } else {
                    // 缓冲区被实例缓存使用，需要创建新缓冲区避免冲突
                    true
                };

                if need_new_buffer {
                    // 创建新的不可更新缓冲区，一次性写入数据
                    if let Some(newbuffer) = allocator.create_not_updatable_buffer(device, queue, collected, None) {
                        *buffer = Share::new(newbuffer);
                    }
                } else {
                    // 复用现有缓冲区，直接写入数据
                    queue.write_buffer(buffer.buffer(), 0, collected);
                    *buffer = buffer.clone();
                }
            },
            _ => {
                // 其他类型的缓冲区暂不处理
            },
        },
    }
}

/// 收集实例信息函数 - 收集、排序并组织实例数据用于渲染
///
/// # 功能流程
/// 1. 遍历所有实例，筛选活动的、未销毁的、未被裁剪的实例
/// 2. 根据排序模式计算每个实例的排序参数（用于深度排序）
/// 3. 使用临时向量进行高效排序和分组
/// 4. 按透明度索引和批处理大小限制组织实例数据
/// 5. 计算每组实例的包围盒中心点
///
/// # 性能优化
/// - 使用TmpCommonVec进行高效的实例排序，避免频繁内存分配
/// - 支持批处理，避免单个批次包含过多实例
/// - 按透明度预分组，优化透明渲染顺序
///
/// # 返回值
/// - true: 成功收集到实例数据
/// - false: 没有收集到有效的实例数据
fn collect_instance_info(
    sortmode: &EInstanceSortMode,                           // 排序模式：决定如何排序实例
    instances: &InstanceSourceRefs,                        // 实例源引用：包含要处理的所有实例
    instancessortinfos: &mut InstancedSortedCollection, // 输出：排序后的实例信息集合
    actives: &Query<(&GlobalEnable, &RenderQueueSortParam, &AbstructMeshCullingFlag, &GlobalMatrix, &LocalPosition), With<InstanceMesh>>, // 活动实例查询
    dispoeds: &Query<&DisposeReady>,                       // 销毁状态查询
    instanceattributes: &Query<&ModelInstanceAttributes>,  // 实例属性查询
    temp: &mut TmpCommonVec,                              // 临时排序向量
    max_instance_batch_count: u32,                         // 最大批处理实例数量限制
) -> bool {
    // 从排序模式中提取排序参数配置
    // isglobal: 是否使用全局坐标排序
    // vidx: 排序使用的坐标轴索引 (0=x, 1=y, 2=z)
    // scl: 排序参数的缩放因子
    let (isglobal, vidx, scl) = sortmode.arg_for_sortparam();

    // 遍历所有实例，收集活动实例的信息
    instances.iter().for_each(|(_k, id)| {
        // 获取实例的活动状态和销毁状态
        if let (Ok((enable, instancelayer, culling, gtransform, localpos)), Ok(disposed)) = (actives.get(*id), dispoeds.get(*id)) {
            // 筛选条件：实例必须启用、未销毁、且未被裁剪
            if enable.0 == true && disposed.0 == false && culling.0 {
                // 根据排序模式计算排序参数（通常用于深度排序）
                let sortparam = if isglobal  {
                    // 使用全局空间位置进行排序
                    gtransform.position().as_slice()[vidx] * scl
                } else {
                    // 使用局部空间位置进行排序
                    localpos.0.as_slice()[vidx] * scl
                };

                // 将实例信息添加到临时排序向量中
                temp.push(*id, instancelayer.index, sortparam, gtransform.xyz());
            }
        }
    });

    // 对临时向量中的实例进行排序（按排序参数和透明度索引）
    temp.sort();

    // 如果收集到实例数据，则进行处理
    if  temp.is_empty() == false {
        // 初始化包围盒边界值
        let mut minx = f32::MAX;
        let mut miny = f32::MAX;
        let mut minz = f32::MAX;
        let mut maxx = f32::MIN;
        let mut maxy = f32::MIN;
        let mut maxz = f32::MIN;

        // 初始化批处理控制变量
        let mut tmp_alphaindex = i32::MIN;      // 当前处理的透明度索引
        let mut tmp_instance_start = 0;         // 当前批次的起始实例索引
        let mut tmp_instance_end = 0;           // 当前批次的结束实例索引

        // 按排序顺序遍历所有实例
        temp.iter(|(idinstance, index, xyz)| {
            // 获取实例的属性数据（位置、颜色、缩放等自定义属性）
            if let Ok(instancedata) = instanceattributes.get(*idinstance) {
                // 检查是否需要开始新的批次
                // 条件1: 透明度索引发生变化（需要不同的渲染顺序）
                // 条件2: 当前批次实例数量超过最大批处理限制
                if tmp_alphaindex != *index || tmp_instance_end - tmp_instance_start > max_instance_batch_count {
                    // 保存当前批次的范围信息
                    // (透明度索引, 实例范围, 包围盒中心点)
                    instancessortinfos.ranges.push((
                        tmp_alphaindex,
                        Range { start: tmp_instance_start, end: tmp_instance_end },
                        ((minx + maxx) * 0.5, (miny + maxy) * 0.5, (minz + maxz) * 0.5) // 包围盒中心
                    ));

                    // 开始新批次，重置变量
                    tmp_alphaindex = *index;
                    tmp_instance_start = tmp_instance_end;
                    minx = f32::MAX; // 重置包围盒
                    miny = f32::MAX;
                    minz = f32::MAX;
                    maxx = f32::MIN;
                    maxy = f32::MIN;
                    maxz = f32::MIN;
                }

                // 更新当前批次的包围盒范围
                minx = minx.min(xyz.0);
                miny = miny.min(xyz.1);
                minz = minz.min(xyz.2);
                maxx = maxx.max(xyz.0);
                maxy = maxy.max(xyz.1);
                maxz = maxz.max(xyz.2);

                // 增加批次计数
                tmp_instance_end += 1;

                // 将实例属性数据追加到排序信息的数据缓冲区中
                // 使用unsafe函数进行高效的内存拷贝
                unsafe_vec_append_slice(&mut instancessortinfos.data, instancedata.bytes());
            }
        });

        // 处理最后一个批次（循环外需要手动添加）
        if tmp_instance_start != tmp_instance_end {
            instancessortinfos.ranges.push((
                tmp_alphaindex,
                Range { start: tmp_instance_start, end: tmp_instance_end },
                ((minx + maxx) * 0.5, (miny + maxy) * 0.5, (minz + maxz) * 0.5)
            ));
        }

        // 设置总实例数量
        instancessortinfos.count = tmp_instance_end as u32;
        return true; // 成功收集并处理了实例数据
    } else {
        return false; // 没有收集到有效的实例数据
    }
}