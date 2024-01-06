
use std::hash::Hash;

use pi_animation::{
    type_animation_context::{TypeAnimationContext, AnimationContextAmount},
    animation_group_manager::AnimationGroupManagerDefault,
    animation_group::AnimationGroupID,
    curve_frame_event::CurveFrameEvent, runtime_info::RuntimeInfoMap
};
use pi_assets::asset::Handle;
use pi_bevy_asset::TAssetCapacity;
use pi_curves::curve::{frame::{FrameDataValue, KeyFrameDataTypeAllocator, KeyFrameCurveValue}, frame_curve::FrameCurve, FrameIndex};
use pi_hash::XHashMap;
use pi_slotmap::DefaultKey;

use bevy::{
    ecs::prelude::*, prelude::{Deref, DerefMut},
};

#[derive(Clone, Copy, Component)]
/// 标识 Entity 启动了动画, 需要使用记录好的相关数据覆盖对应数据
pub struct FlagAnimationStartResetComp;

pub type KeyAnimeCurve = String;

pub type IDAssetTypeFrameCurve = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, Hash)]
pub struct SceneID(pub Entity);

pub struct TypeFrameCurve<F: FrameDataValue+ 'static>(pub FrameCurve<F>);
impl<F: FrameDataValue+ 'static> pi_assets::asset::Asset for TypeFrameCurve<F> {
    type Key = IDAssetTypeFrameCurve;
    // const TYPE: &'static str = "TypeFrameCurve";
}
impl<F: FrameDataValue+ 'static> pi_assets::asset::Size for TypeFrameCurve<F> {
    fn size(&self) -> usize {
        F::size() * self.0.values.len() + 2 * self.0.frames.len() + self.0.size()
    }
}

pub struct AssetTypeFrameCurve<F: FrameDataValue+ 'static>(pub Handle<TypeFrameCurve<F>>);
impl<F: FrameDataValue+ 'static> From<Handle<TypeFrameCurve<F>>> for AssetTypeFrameCurve<F> {
    fn from(value: Handle<TypeFrameCurve<F>>) -> Self {
        Self(value)
    }
}
impl<F: FrameDataValue+ 'static> AsRef<FrameCurve<F>> for AssetTypeFrameCurve<F> {
    fn as_ref(&self) -> &FrameCurve<F> {
        &self.0.0
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct TypeAnimeContext<D: TAnimatableComp> {
    pub ctx: TypeAnimationContext<D, AssetTypeFrameCurve<D>>,
}
impl<D: TAnimatableComp> TypeAnimeContext<D> {
    pub fn new<T: Clone + PartialEq + Eq + Hash>(ty: usize, runtime_info_map: &mut RuntimeInfoMap<T>) -> Self {
        Self { ctx: TypeAnimationContext::<D, AssetTypeFrameCurve<D>>::new(ty, runtime_info_map) }
    }
}

pub trait TAnimatableComp: Default + FrameDataValue + Component + std::fmt::Debug + TAssetCapacity {

}
pub trait TAnimatableCompRecord<T: TAnimatableComp>: Component {
    fn comp(&self) -> T;
}

#[derive(Debug, Default, Component)]
pub struct AnimationGroups {
    pub map: XHashMap<AnimationGroupID, AnimationGroupID>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TagGroupListen;
impl TagGroupListen {
    pub const START : u8   = 0b0000_0001;
    pub const END   : u8   = 0b0000_0010;
    pub const LOOP  : u8   = 0b0000_0100;
    pub const FRAME : u8   = 0b0000_1000;
}

pub type AnimeFrameEventData = u32;

#[derive(Component)]
pub struct AnimationGroupKey(pub DefaultKey);

#[derive(Component)]
pub struct AnimationGroupScene(pub Entity);

pub enum EAnimatorableEntityType {
    Uniform,
    Attribute,
}

#[derive(Component)]
pub struct AnimatorableUniform;

#[derive(Component)]
pub struct AnimatorableAttribute;

#[derive(Resource)]
pub struct GlobalAnimeAbout {
    pub ty_alloc: KeyFrameDataTypeAllocator,
    pub runtimeinfos: pi_animation::runtime_info::RuntimeInfoMap<Entity>,
    pub dispose_animationgroups: Vec<(Entity, AnimationGroupID)>,
    pub group_records: XHashMap<AnimationGroupID, (Entity, CurveFrameEvent<AnimeFrameEventData>, u8)>,
}
impl GlobalAnimeAbout {
    pub(crate) const CURVE_FRAME_EVENT_FRAMES: u16 = 60000;
    pub(crate) fn record_group(&mut self,  id_group: AnimationGroupID, group: Entity) {
        self.group_records.insert(id_group, (group, CurveFrameEvent::new(Self::CURVE_FRAME_EVENT_FRAMES as KeyFrameCurveValue), 0));
    }
    pub(crate) fn add_frame_event(&mut self,  id_group: AnimationGroupID, percent: f32, data: AnimeFrameEventData) {
        if let Some(record) = self.group_records.get_mut(&id_group) {
            record.1.add((percent as KeyFrameCurveValue * Self::CURVE_FRAME_EVENT_FRAMES as KeyFrameCurveValue) as FrameIndex, data);
        }
    }
    pub(crate) fn add_frame_event_listen(&mut self,  id_group: AnimationGroupID) {
        if let Some(listen) = self.group_records.get_mut(&id_group) {
            listen.2 = listen.2 | TagGroupListen::FRAME;
        }
    }
    pub(crate) fn add_start_listen(&mut self,  id_group: AnimationGroupID) {
        if let Some(listen) = self.group_records.get_mut(&id_group) {
            listen.2 = listen.2 | TagGroupListen::START;
        }
    }
    pub(crate) fn add_end_listen(&mut self,  id_group: AnimationGroupID) {
        if let Some(listen) = self.group_records.get_mut(&id_group) {
            listen.2 = listen.2 | TagGroupListen::END;
        }
    }
    pub(crate) fn add_loop_listen(&mut self,  id_group: AnimationGroupID) {
        if let Some(listen) = self.group_records.get_mut(&id_group) {
            listen.2 = listen.2 | TagGroupListen::LOOP;
        }
    }
    pub(crate) fn remove(&mut self, id_group: &AnimationGroupID) {
        self.group_records.remove(id_group);
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct GlobalAnimeEvents(pub Vec<(Entity, Entity, u8, u32)>);

#[derive(Component)]
pub struct SceneAnimationContext(pub AnimationContextAmount<Entity, AnimationGroupManagerDefault<Entity>>);
impl SceneAnimationContext {
    pub fn new() -> Self {
        Self(
            AnimationContextAmount::<Entity, AnimationGroupManagerDefault<Entity>>::default(
                AnimationGroupManagerDefault::<Entity>::default()
            )
        )
    }
}

pub fn sys_animation_removed_data_clear(
    mut ctxs: Query<&mut SceneAnimationContext>,
) {
    ctxs.iter_mut().for_each(|mut ctx| {
        ctx.0.clear_removed_animations()
    });
}
