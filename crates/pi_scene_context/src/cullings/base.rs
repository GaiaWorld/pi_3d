
use pi_scene_shell::prelude::*;
use pi_scene_math::{coordiante_system::CoordinateSytem3, vector::TToolVector3, Vector3, Matrix, Number, Point3};

use crate::{viewer::prelude::ViewerTransformMatrix, prelude::MeshInstanceState};

use super::{oct_tree::BoundingOctTree, bounding::VecBoundingInfoCalc};

pub trait TBoundingInfoCalc {
    fn add_fast(&mut self, key: Entity);
    fn add(&mut self, key: Entity, min: (Number, Number, Number), max: (Number, Number, Number));
    fn remove(&mut self, key: Entity);
    fn culling<F: TFilter>(&self, vp: &Matrix, filter: F, result: &mut Vec<Entity>);
    fn ray_test(
        &self,
        org: Vector3,
        dir: Vector3,
        result: &mut Option<Entity>,
    );
    fn entities(&self) -> Vec<Entity>;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoundingKey(pub Entity);
impl Default for BoundingKey {
    fn default() -> Self {
        Self(Entity::from_bits(0))
    }
}
impl From<pi_slotmap::KeyData> for BoundingKey {
    fn from(value: pi_slotmap::KeyData) -> Self {
        let bits = value.as_ffi();
        Self(Entity::from_bits(bits))
    }
}
impl pi_slotmap::Key for BoundingKey {
    fn data(&self) -> pi_slotmap::KeyData {
        pi_slotmap::KeyData::from_ffi(self.0.to_bits())
    }

	fn index(&self) -> usize {
		self.0.index() as usize
	}
}

impl Null for BoundingKey {
	fn null() -> Self { Self(Entity::from_bits(u64::null())) }

    fn is_null(&self) -> bool { self.0.to_bits().is_null() }
}


/// 检测级别
/// *
#[derive(Default)]
pub enum ECullingStrategy {
    /// 不用检测直接通过检测
    None,
    /// 检测 包围球中心 在不在 视锥, 检测 包围球 在不在 视锥
    #[default]
    Optimistic,
    /// 检测 包围球中心 在不在 视锥, 检测 包围球 在不在 视锥, 检测 包围盒 在不在 视锥
    STANDARD,
}

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
}


#[derive(Component, Default)]
pub struct GeometryCullingMode(pub ECullingStrategy);


#[derive(Component)]
pub enum SceneBoundingPool {
    List(VecBoundingInfoCalc),
    QuadTree(),
    OctTree(BoundingOctTree),
}
impl SceneBoundingPool {
    pub const MODE_LIST: u8 = 0;
    pub const MODE_QUAD_TREE: u8 = 1;
    pub const MODE_OCTREE: u8 = 2;
    pub fn create_vec() -> Self {
        Self::List(VecBoundingInfoCalc::default())
    }
    pub fn create_oct(min: (Number, Number, Number), max: (Number, Number, Number), adjust_min: usize, adjust_max: usize, deep: usize) -> Self {
        let tree = OctTree::new(
            Aabb::new(
                Point3::new(min.0, min.1, min.2),
                Point3::new(max.0, max.1, max.2),
            ),
            Vector3::new(max.0, max.1, max.2),
            Vector3::new(min.0, min.1, min.2),
            adjust_min,
            adjust_max,
            deep
        );
        Self::OctTree(BoundingOctTree { fast: XHashSet::default(), tree })
    }
    pub fn remove(&mut self, entity: Entity) {
        match self {
            SceneBoundingPool::List(items) => items.remove(entity),
            SceneBoundingPool::QuadTree() => todo!(),
            SceneBoundingPool::OctTree(items) => items.remove(entity),
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
                        items.add(entity, min, max)
                    },
                    ECullingStrategy::STANDARD => {
                        // log::warn!("00000");
                        let (min, max) = info.minmax(matrix);
                        items.add(entity, min, max)
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
                        items.add(entity, min, max)
                    },
                    ECullingStrategy::STANDARD => {
                        let (min, max) = info.minmax(matrix);
                        items.add(entity, min, max)
                    },
                }
            },
        }
    }
    pub fn culling<F: TFilter>(&self, transform: &ViewerTransformMatrix, filter: F, result: &mut Vec<Entity>) {
        let transform = transform.0.clone();
        match self {
            SceneBoundingPool::List(item) => {
                item.culling(&transform, filter, result);
            },
            SceneBoundingPool::QuadTree() => todo!(),
            SceneBoundingPool::OctTree(item) => {
                println!("================otctree");
                item.culling(&transform, filter, result);
            },
        }
    }

    pub fn ray_test(
        &self,
        org: Vector3,
        dir: Vector3,
        result: &mut Option<Entity>,
    ) {
        match self {
            SceneBoundingPool::List(item) => item.ray_test(org, dir, result),
            SceneBoundingPool::QuadTree() => todo!(),
            SceneBoundingPool::OctTree(item) => item.ray_test(org, dir, result),
        }
    }
    pub fn entities(&self) -> Vec<Entity> {
        match self {
            SceneBoundingPool::List(items) => items.entities(),
            SceneBoundingPool::QuadTree() => vec![],
            SceneBoundingPool::OctTree(items) => items.entities(),
        }
    }
}

#[derive(Component)]
pub struct BoundingBoxDisplay {
    pub mesh: Entity,
    pub display: bool,
}
impl BoundingBoxDisplay {
    pub const ATTRIBUTE_MINIMUM: &str = "BoxMinimum";
    pub const ATTRIBUTE_MAXIMUM: &str = "BoxMaximum";
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
