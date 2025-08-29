use pi_scene_shell::prelude::*;
use pi_scene_math::{coordiante_system::CoordinateSytem3, vector::TToolVector3, Vector3, Matrix, Number, Point3};
use serde::{Deserialize, Serialize};

use crate::{flags::GlobalEnable, prelude::MeshInstanceState, viewer::prelude::ViewerTransformMatrix};

use super::{bounding::VecBoundingInfoCalc, oct_tree::BoundingOctTree};

pub trait TBoundingInfoCalc {
    fn add_fast(&mut self, key: Entity);
    fn add(&mut self, key: Entity, min: (Number, Number, Number), max: (Number, Number, Number), intersection_treshold: Number, sortindex: i32);
    fn remove(&mut self, key: Entity);
    fn culling<F: TFilter>(&mut self, vp: &Matrix, filter: F, result: &mut Vec<Entity>);
    fn ray_test(
        &self,
        piray: &PiRay,
        result: &mut Option<PickResult>,
        sortparams: &Query<&GlobalEnable>,
    );
    fn entities(&self) -> Vec<Entity>;
    fn size(&self) -> usize;
    fn reset_temp(&mut self);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoundingKey(pub Entity);
impl Default for BoundingKey {
    #[cfg(feature = "use_bevy")]
    fn default() -> Self {
        Self(Entity::from_bits(0))
    }
    #[cfg(not(feature = "use_bevy"))]
    fn default() -> Self {
        Self(Entity::default())
    }
}
impl From<pi_slotmap::KeyData> for BoundingKey {
    #[cfg(feature = "use_bevy")]
    fn from(value: pi_slotmap::KeyData) -> Self {
        let bits = value.as_ffi();
        Self(Entity::from_bits(bits))
    }
    #[cfg(not(feature = "use_bevy"))]
    fn from(value: pi_slotmap::KeyData) -> Self {
        Self(Entity::from(value))
    }
}
impl pi_slotmap::Key for BoundingKey {
    #[cfg(feature = "use_bevy")]
    fn data(&self) -> pi_slotmap::KeyData {
        pi_slotmap::KeyData::from_ffi(self.0.to_bits())
    }
    #[cfg(not(feature = "use_bevy"))]
    fn data(&self) -> pi_slotmap::KeyData {
        self.0.data()
    }

	fn index(&self) -> usize {
		self.0.index() as usize
	}
    
    fn with(_idx: usize) -> Self {
        todo!()
    }
}

impl Null for BoundingKey {
    #[cfg(feature = "use_bevy")]
	fn null() -> Self { Self(Entity::from_bits(u64::null())) }
    #[cfg(not(feature = "use_bevy"))]
	fn null() -> Self { Self(Entity::null()) }

    #[cfg(feature = "use_bevy")]
    fn is_null(&self) -> bool { self.0.to_bits().is_null() }
    #[cfg(not(feature = "use_bevy"))]
    fn is_null(&self) -> bool { self.0.is_null() }
}


/// 检测级别
/// *
#[derive(Default, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum ECullingStrategy {
    /// 不用检测直接通过检测
    None,
    /// 检测 包围球中心 在不在 视锥, 检测 包围球 在不在 视锥
    #[default]
    Optimistic,
    /// 检测 包围球中心 在不在 视锥, 检测 包围球 在不在 视锥, 检测 包围盒 在不在 视锥
    STANDARD,
}

/// 记录 TransformNode 对应的射线命中盒子
#[derive(Component, Clone, Copy)]
pub struct Collider {
    pub minimum: Vector3,
    pub maximum: Vector3,
    pub intersection_treshold: Number,
    pub sortindex: i32,
}
impl Default for Collider {
    fn default() -> Self {
        Self { minimum: Vector3::new(-0.5, -0.5, -0.5), maximum: Vector3::new(0.5, 0.5, 0.5), intersection_treshold: 0., sortindex: i32::MIN }
    }
}
impl Collider {
    pub fn minmax(&self, matrix: &Matrix, temp: &mut Vector3) -> ((Number, Number, Number), (Number, Number, Number), Number, i32) {
        CoordinateSytem3::transform_normal_floats(1., 1., 1., matrix, temp);
        let radius = self.minimum.metric_distance(&self.maximum).abs() * 0.5 * temp.x.max(temp.y).max(temp.z);

        CoordinateSytem3::transform_coordinates(&self.minimum, matrix, temp);
        let min = (temp.x, temp.y, temp.z);
        CoordinateSytem3::transform_coordinates(&self.maximum, matrix, temp);
        let max = (temp.x, temp.y, temp.z);
        (
            (Number::min(min.0, max.0), Number::min(min.1, max.1), Number::min(min.2, max.2)),
            (Number::max(min.0, max.0), Number::max(min.1, max.1), Number::max(min.2, max.2)),
            radius * (1.0 + self.intersection_treshold),
            self.sortindex
        )
    }
}

/// 记录 Mesh 对应的网格剔除Box信息
#[derive(Component)]
pub struct GeometryBounding {
    pub minimum: Vector3,
    pub maximum: Vector3,
}
impl Default for GeometryBounding {
    fn default() -> Self {
        Self { minimum: Vector3::new(-0.5, -0.5, -0.5), maximum: Vector3::new(0.5, 0.5, 0.5) }
    }
}
impl GeometryBounding {
    pub fn minmax(&self, matrix: &Matrix) -> ((Number, Number, Number), (Number, Number, Number)) {
        let mut temp = Vector3::zeros();
        CoordinateSytem3::transform_coordinates(&self.minimum, matrix, &mut temp);
        let min = (temp.x, temp.y, temp.z);
        CoordinateSytem3::transform_coordinates(&self.maximum, matrix, &mut temp);
        let max = (temp.x, temp.y, temp.z);
        (
            (Number::min(min.0, max.0), Number::min(min.1, max.1), Number::min(min.2, max.2)),
            (Number::max(min.0, max.0), Number::max(min.1, max.1), Number::max(min.2, max.2))
        )
    }
}

pub trait TFilter {
    fn filter(&self, entity: Entity) -> bool;
    fn query(&self, entity: Entity) -> bool;
    fn iter(&self) -> std::collections::btree_set::Iter<Entity>;
}

/// 标识 Mesh 的网格剔除信息是否需要更新
#[derive(Component, Default)]
pub struct ItemCullingDirty;

/// 标识 Mesh 的网格剔除模式
#[derive(Component, Default)]
pub struct GeometryCullingMode(pub ECullingStrategy);


#[derive(Debug, Clone, Copy)]
pub struct PiRay {
    pub origin: (Number, Number, Number),
    pub far: (Number, Number, Number),
    pub direction: (Number, Number, Number),
}

#[derive(Debug, Clone, Copy)]
pub struct PickResult {
    pub target: Entity,
    pub min: (Number, Number, Number),
    pub max: (Number, Number, Number),
    pub sortindex: i32,
    pub bybounding: bool,
    pub pickdetail: Option<(Number, Number, Number)>,
}

#[derive(Component)]
pub enum SceneColliderPool {
    List(VecBoundingInfoCalc),
    QuadTree(),
    OctTree(BoundingOctTree),
}
impl Default for SceneColliderPool {
    fn default() -> Self {
        Self::List(VecBoundingInfoCalc::default())
    }
}
impl SceneColliderPool {
    pub const MODE_LIST: u8 = 0;
    pub const MODE_QUAD_TREE: u8 = 1;
    pub const MODE_OCTREE: u8 = 2;
    pub fn create_vec() -> Self {
        Self::List(VecBoundingInfoCalc::default())
    }
    pub fn create_oct(
        mins: (Number, Number, Number),
        maxs: (Number, Number, Number),
        max_loose: (Number, Number, Number), 
        min_loose: (Number, Number, Number),
        adjust_min: usize,
        adjust_max: usize,
        deep: usize,
    ) -> Self {
        let max = Vector3::new(max_loose.0, max_loose.1, max_loose.2);
        let min = Vector3::new(min_loose.0, min_loose.1, min_loose.2);

        let tree = OctTree::new(
            Aabb::new(
                Point3::new(mins.0, mins.1, mins.2),
                Point3::new(maxs.0, maxs.1, maxs.2),
            ),
            max,
            min,
            adjust_min,
            adjust_max,
            deep,
        );
        Self::OctTree(BoundingOctTree::new(tree))
    }
    pub fn remove(&mut self, entity: Entity) {
        match self {
            SceneColliderPool::List(items) => items.remove(entity),
            SceneColliderPool::QuadTree() => todo!(),
            SceneColliderPool::OctTree(items) => items.remove(entity),
        }
    }
    pub fn set(&mut self, entity: Entity, info: &Collider, matrix: &Matrix, temp: &mut Vector3) {

        match self {
            SceneColliderPool::List(items) => {
                let (min, max, intersection_treshold, sortindex) = info.minmax(matrix, temp);
                items.add(entity, min, max, intersection_treshold, sortindex)
            },
            SceneColliderPool::QuadTree() => {
                
            },
            SceneColliderPool::OctTree(items) => {
                let (min, max, intersection_treshold, sortindex) = info.minmax(matrix, temp);
                items.add(entity, min, max, intersection_treshold, sortindex)
            },
        }
    }
    pub fn ray_test(
        &self,
        ray: &PiRay,
        result: &mut Option<PickResult>,
        sortparams: &Query<&GlobalEnable>,
    ) {
        match self {
            SceneColliderPool::List(item) => item.ray_test(ray, result, sortparams),
            SceneColliderPool::QuadTree() => todo!(),
            SceneColliderPool::OctTree(item) => item.ray_test(ray, result, sortparams),
        }
    }
    pub fn entities(&self) -> Vec<Entity> {
        match self {
            SceneColliderPool::List(items) => items.entities(),
            SceneColliderPool::QuadTree() => vec![],
            SceneColliderPool::OctTree(items) => items.entities(),
        }
    }
    pub fn size(&self) -> usize {
        match self {
            SceneColliderPool::List(items) => items.size(),
            SceneColliderPool::QuadTree() => 0,
            SceneColliderPool::OctTree(items) => items.size(),
        }
    }
}


#[derive(Component)]
pub enum SceneBoundingPool {
    None,
    List(VecBoundingInfoCalc),
    QuadTree(),
    OctTree(BoundingOctTree),
}
impl Default for SceneBoundingPool {
    fn default() -> Self {
        Self::None
    }
}
impl SceneBoundingPool {
    pub const MODE_LIST: u8 = 1;
    pub const MODE_OCTREE: u8 = 2;
    pub const MODE_QUAD_TREE: u8 = 3;
    pub fn create_vec() -> Self {
        Self::List(VecBoundingInfoCalc::default())
    }
    pub fn create_oct(
        mins: (Number, Number, Number),
        maxs: (Number, Number, Number),
        max_loose: (Number, Number, Number), 
        min_loose: (Number, Number, Number),
        adjust_min: usize,
        adjust_max: usize,
        deep: usize,
    ) -> Self {
        let max = Vector3::new(max_loose.0, max_loose.1, max_loose.2);
        let min = Vector3::new(min_loose.0, min_loose.1, min_loose.2);

        let tree = OctTree::new(
            Aabb::new(
                Point3::new(mins.0, mins.1, mins.2),
                Point3::new(maxs.0, maxs.1, maxs.2),
            ),
            max,
            min,
            adjust_min,
            adjust_max,
            deep,
        );
        Self::OctTree(BoundingOctTree::new(tree))
    }
    pub fn remove(&mut self, entity: Entity) {
        match self {
            SceneBoundingPool::List(items) => items.remove(entity),
            SceneBoundingPool::QuadTree() => todo!(),
            SceneBoundingPool::OctTree(items) => items.remove(entity),
            SceneBoundingPool::None => {}
        }
    }
    pub fn set(&mut self, entity: Entity, info: &GeometryBounding, mode: &GeometryCullingMode, matrix: &Matrix) {

        match self {
            SceneBoundingPool::List(items) => {
                match mode.0 {
                    ECullingStrategy::None => {
                        items.add_fast(entity);
                    },
                    ECullingStrategy::Optimistic => {
                        // log::warn!("{:?}", (entity, &matrix.0));
                        let (min, max) = info.minmax(matrix);
                        items.add(entity, min, max, 0., i32::MIN)
                    },
                    ECullingStrategy::STANDARD => {
                        // log::warn!("00000");
                        let (min, max) = info.minmax(matrix);
                        items.add(entity, min, max, 0., i32::MIN)
                    },
                }
            },
            SceneBoundingPool::QuadTree() => {
                
            },
            SceneBoundingPool::OctTree(items) => {
                match mode.0 {
                    ECullingStrategy::None => {
                        items.add_fast(entity);
                    },
                    ECullingStrategy::Optimistic => {
                        let (min, max) = info.minmax(matrix);
                        items.add(entity, min, max, 0., i32::MIN)
                    },
                    ECullingStrategy::STANDARD => {
                        let (min, max) = info.minmax(matrix);
                        items.add(entity, min, max, 0., i32::MIN)
                    },
                }
            },
            SceneBoundingPool::None => {}
        }
    }
    pub fn culling<F: TFilter>(&mut self, transform: &ViewerTransformMatrix, filter: F, result: &mut Vec<Entity>) {
        let transform = transform.0.clone();
        match self {
            SceneBoundingPool::List(item) => {
                item.culling(&transform, filter, result);
            },
            SceneBoundingPool::QuadTree() => todo!(),
            SceneBoundingPool::OctTree(item) => {
                // println!("================otctree");
                item.culling(&transform, filter, result);
            },
            SceneBoundingPool::None => {
                filter.iter().for_each(|entity| {
                    if filter.query(*entity) {
                        result.push(*entity);
                    }
                });
            }
        }
    }
    pub fn ray_test(
        &self,
        ray: &PiRay,
        result: &mut Option<PickResult>,
        sortparams: &Query<&GlobalEnable>,
    ) {
        match self {
            SceneBoundingPool::List(item) => item.ray_test(ray, result, sortparams),
            SceneBoundingPool::QuadTree() => todo!(),
            SceneBoundingPool::OctTree(item) => item.ray_test(ray, result, sortparams),
            SceneBoundingPool::None => {}
        }
    }
    pub fn entities(&self) -> Vec<Entity> {
        match self {
            SceneBoundingPool::List(items) => items.entities(),
            SceneBoundingPool::QuadTree() => vec![],
            SceneBoundingPool::OctTree(items) => items.entities(),
            SceneBoundingPool::None => { vec![] }
        }
    }
    pub fn size(&self) -> usize {
        match self {
            SceneBoundingPool::List(items) => items.size(),
            SceneBoundingPool::QuadTree() => 0,
            SceneBoundingPool::OctTree(items) => items.size(),
            SceneBoundingPool::None => { 0}
        }
    }
    pub fn reset_temp(&mut self) {
        match self {
            SceneBoundingPool::List(items) => items.reset_temp(),
            SceneBoundingPool::QuadTree() => {},
            SceneBoundingPool::OctTree(items) => items.reset_temp(),
            SceneBoundingPool::None => { }
        }
    }
}

#[derive(Component)]
pub struct BoundingBoxDisplay {
    pub mesh: Entity,
    pub display: bool,
}
impl Default for BoundingBoxDisplay {
    fn default() -> Self {
        Self { mesh: Entity::default(), display: false }
    }
}
impl BoundingBoxDisplay {
    pub const ATTRIBUTE_MINIMUM: &'static str = "BoxMinimum";
    pub const ATTRIBUTE_MAXIMUM: &'static str = "BoxMaximum";
    pub fn mesh_state() -> MeshInstanceState {
        MeshInstanceState {
            instances: vec![
                CustomVertexAttribute::new(Atom::from(Self::ATTRIBUTE_MAXIMUM), Atom::from(""), ECustomVertexType::Vec3, None),
                CustomVertexAttribute::new(Atom::from(Self::ATTRIBUTE_MINIMUM), Atom::from("A_POSITION = 0.5 * (BoxMaximum + BoxMinimum) + A_POSITION * (BoxMaximum - BoxMinimum);"), ECustomVertexType::Vec3, None),
            ],
            instance_matrix: true,
            use_single_instancebuffer: true,
        }
    }
}
