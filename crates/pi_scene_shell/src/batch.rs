use crate::ecs::*;


pub struct BatchParam {
    // 是否使用与相机距离排序
    pub distance: bool,
    // 是否对实例化数据批处理
    pub instance: bool,
    // 是否对静态Mesh批处理
    pub static_mesh: bool,
}
impl Default for BatchParam {
    fn default() -> Self {
        Self { distance: true, instance: true, static_mesh: true }
    }
}

#[derive(Component, Default)]
pub struct BatchParamOpaque(pub BatchParam);

#[derive(Component, Default)]
pub struct BatchParamTransparent(pub BatchParam);
