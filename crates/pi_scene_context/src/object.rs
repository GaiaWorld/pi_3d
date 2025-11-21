pub use pi_scene_shell::prelude::*;

use crate::prelude::*;

pub type OpsDispose = OpsDisposeReady;
pub type ActionListDispose = ActionListDisposeReady;

// 类型别名：提高代码可读性和维护性
type IndexOfItemsListForSameAlphaIndex = usize; // 同一透明度索引下项目列表的索引
type ItemInfo = (Entity, (Number, Number, Number)); // 项目信息：实体ID和坐标(x,y,z)
type ItemInInfoList = Vec<ItemInfo>; // 项目信息列表
type IndexOfItemInInfoList = usize; // 项目信息在列表中的索引

/// 临时向量容器 - 用于实例渲染的高性能临时数据管理
///
/// # 功能概述
/// 此结构体专门用于管理实例渲染时的临时数据，支持：
/// - 按透明度分组的实例管理
/// - 高效的实例排序和遍历
/// - 内存复用以减少分配开销
/// - 对象池模式提高性能
///
/// # 数据结构设计
/// - alphaindexarr: 按透明度索引排序的数组，用于快速查找
/// - same_alphaindex_items: 存储各透明度组的实例数据和排序信息
/// - pool0: 对象池，用于复用已分配的内存空间
///
/// # 性能优化特性
/// 1. 使用二分查找快速定位透明度组 (O(log n))
/// 2. 对象池复用内存，减少频繁分配/释放
/// 3. 预分配容量避免动态扩容
/// 4. 批量操作提高效率
#[derive(Resource, Default)]
pub struct TmpCommonVec {
    /// 透明度索引数组：存储透明度值到存储位置的映射
    /// 保持排序状态，支持二分查找，结构：(透明度值, 在same_alphaindex_items中的索引)
    alphaindexarr: Vec<(i32, IndexOfItemsListForSameAlphaIndex)>,

    /// 同透明度组的实例数据集合
    /// 每个元素是一个元组：(排序向量, 实体数据向量)
    /// - 排序向量: Vec<(排序参数, 实体索引)>, 用于深度排序
    /// - 实体数据向量: Vec<(实体ID, 坐标)>, 存储实际的实例信息
    same_alphaindex_items: Vec<(Vec<(Number, IndexOfItemInInfoList)>, ItemInInfoList)>,

    /// 对象池：用于复用已分配的内存空间
    /// 当调用clear()时，将空的存储空间推入池中供下次复用
    /// 避免频繁的内存分配和释放，提高性能
    pool0: Vec<(Vec<(Number, IndexOfItemInInfoList)>, ItemInInfoList)>,
}
impl TmpCommonVec {
    /// 计算当前向量的预估内存占用大小
    /// 用于性能监控和内存预分配
    pub fn size(&self) -> usize {
        self.alphaindexarr.capacity() * 12 + self.same_alphaindex_items.capacity() * 128
    }
    /// 向临时向量中添加一个实例实体
    ///
    /// # 参数
    /// - entity: 实例的实体ID
    /// - alphaindex: 透明度索引，用于分组排序
    /// - sortparam: 排序参数，通常用于深度排序
    /// - xyz: 实例的世界坐标位置
    ///
    /// # 逻辑说明
    /// 1. 使用二分查找在alphaindexarr中查找对应的透明度组
    /// 2. 如果找到现有组，直接使用；如果未找到，创建新组
    /// 3. 从对象池pool0中复用或创建新的存储空间
    /// 4. 将实体信息添加到对应组的数据结构中
    pub fn push(&mut self, entity: Entity, alphaindex: i32, sortparam: Number, xyz: (Number, Number, Number)) {
        // 使用二分查找在alphaindexarr中查找对应的透明度索引
        // alphaindexarr保持排序状态，便于后续快速查找和遍历
        let info = match self.alphaindexarr.binary_search_by(|a| a.0.cmp(&alphaindex)) {
            Ok(idx) => {
                // 找到现有透明度组，获取对应的存储索引
                &mut self.same_alphaindex_items[self.alphaindexarr[idx].1]
            },
            Err(idx) => {
                // 未找到对应透明度组，需要创建新组
                let i = self.same_alphaindex_items.len();
                // 在正确位置插入新的透明度索引记录，保持数组有序
                self.alphaindexarr.insert(idx, (alphaindex, i));
                // 尝试从对象池获取复用的存储空间，避免频繁内存分配
                let item = self.pool0.pop().unwrap_or((Vec::with_capacity(128), Vec::with_capacity(128)));
                // 将存储空间添加到主数组中
                self.same_alphaindex_items.push(item);
                // 返回新创建的存储空间的可变引用
                &mut self.same_alphaindex_items[i]
            },
        };

        // 获取当前组中现有项目的数量，作为新项目的索引
        let idx = info.1.len();
        // 将实体信息和位置坐标添加到实体数据向量中
        info.1.push((entity, xyz));
        // 将排序参数和实体索引添加到排序向量中，用于后续排序操作
        info.0.push((sortparam, idx));
    }
    /// 对所有透明度组内的实例进行排序
    ///
    /// # 排序逻辑
    /// 1. 遍历每个透明度组
    /// 2. 对组内的排序向量按照sortparam进行排序
    /// 3. 使用partial_cmp处理可能的NaN值情况
    ///
    /// # 注意
    /// 此方法只对组内排序，不改变组间的顺序
    /// 组间顺序由alphaindexarr的插入顺序决定
    pub fn sort(&mut self) {
        // 遍历所有透明度组，对每组内部的实例进行排序
        self.same_alphaindex_items.iter_mut().for_each(|item| {
            // item.0是排序向量，存储了(sortparam, 实体索引)
            // 按照sortparam进行排序，用于实现正确的渲染顺序（如深度排序）
            item.0.sort_by(|a, b| {
                // 使用partial_cmp处理浮点数的比较，避免NaN值导致的panic
                // 如果比较结果为None，返回Equal作为默认值
                if let Some(o) = a.0.partial_cmp(&b.0) { o } else { std::cmp::Ordering::Equal }
            });
        });
    }
    /// 按照排序顺序遍历所有实例数据
    ///
    /// # 泛型参数
    /// - F: 闭包类型，接收 (实体引用, 透明度索引引用, 坐标引用)
    ///
    /// # 遍历顺序
    /// 1. 按透明度索引顺序遍历各个组
    /// 2. 在每个组内按照排序后的顺序遍历实例
    /// 3. 通过排序向量中的索引找到对应的实体数据
    pub fn iter<F: FnMut((&Entity, &i32, &(Number, Number, Number)))>(&self, mut f: F) {
        // 遍历透明度索引数组，按透明度顺序处理各个组
        self.alphaindexarr.iter().for_each(|(alphaidex, idx)| {
            // 获取对应透明度组的所有实例数据
            let infos = &self.same_alphaindex_items[*idx];

            // 遍历组内的排序向量，按照排序顺序处理实例
            infos.0.iter().for_each(|(_, i)| {
                // 通过排序向量中存储的索引，获取实际的实体数据
                let (entity, xyz) = &infos.1[*i];

                // 调用用户提供的闭包，传递实体引用、透明度索引和坐标
                f((entity, alphaidex, xyz));
            });
        });
    }
    /// 清空所有数据并重置向量状态
    ///
    /// # 内存管理策略
    /// 1. 不释放已分配的内存，只是重置长度为0
    /// 2. 将使用过的存储空间回收到对象池中，供下次复用
    /// 3. 使用unsafe代码直接设置长度，避免逐个删除元素的开销
    ///
    /// # 性能优势
    /// - 避免频繁的内存分配和释放
    /// - 对象池复用减少内存碎片
    /// - 批量操作提高性能
    pub fn clear(&mut self) {
        unsafe {
            // 直接将透明度索引数组长度设为0，保留已分配的内存容量
            self.alphaindexarr.set_len(0);

            // 逐个处理实例数据组，将存储空间回收到对象池
            while let Some(mut item) = self.same_alphaindex_items.pop() {
                // 清空排序向量，保留容量
                item.0.set_len(0);
                // 清空实体数据向量，保留容量
                item.1.set_len(0);
                // 将清空的存储空间推入对象池，供下次push时复用
                if self.pool0.len() < 64 { self.pool0.push(item); }
            }
        }
    }
    /// 检查容器是否为空（没有任何实例数据）
    ///
    /// # 返回值
    /// - true: 如果没有存储任何实例数据
    /// - false: 如果至少有一个实例数据
    ///
    /// # 注意
    /// 检查的是same_alphaindex_items而不是alphaindexarr，
    /// 因为alphaindexarr可能为空但same_alphaindex_items仍有数据
    pub fn is_empty(&self) -> bool {
        self.same_alphaindex_items.is_empty()
    }
    /// 统计容器中存储的实例总数
    ///
    /// # 计算方式
    /// 遍历所有透明度组，累加每个组中实例的数量
    ///
    /// # 返回值
    /// 所有透明度组中实例的总数量
    ///
    /// # 性能说明
    /// - O(n)时间复杂度，n为透明度组的数量
    /// - 实时计算，不缓存结果
    pub fn count(&self) -> usize {
        let mut count = 0;
        // 遍历所有透明度组
        self.same_alphaindex_items.iter().for_each(|i| {
            // 累加每个组中实例的数量（i.0.len()为排序向量的长度，等于实例数量）
            count += i.0.len();
        });
        // 返回总实例数量
        return count;
    }
}
