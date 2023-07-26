
use std::{marker::PhantomData, hash::Hash};

use pi_animation::{
    type_animation_context::{TypeAnimationContext, AnimationContextAmount},
    animation_group_manager::AnimationGroupManagerDefault,
    animation_group::AnimationGroupID,
    curve_frame_event::CurveFrameEvent, animation::AnimationInfo, target_animation::TargetAnimation, runtime_info::RuntimeInfoMap
};
use pi_assets::{asset::{Handle}};
use pi_atom::Atom;
use pi_bevy_asset::TAssetCapacity;
use pi_curves::curve::{frame::{FrameDataValue, KeyFrameDataTypeAllocator, KeyFrameCurveValue}, frame_curve::FrameCurve, FrameIndex};
use pi_hash::XHashMap;
use pi_slotmap::DefaultKey;

use bevy::{
    ecs::prelude::*, prelude::{Deref, DerefMut},
};

use super::AnimationGroupParam;

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

#[derive(Resource)]
pub struct GlobalAnimeAbout {
    pub ty_alloc: KeyFrameDataTypeAllocator,
    pub runtimeinfos: pi_animation::runtime_info::RuntimeInfoMap<Entity>,
    pub dispose_animationgroups: Vec<(Entity, AnimationGroupID)>,
    pub group_records: XHashMap<DefaultKey, (Entity, CurveFrameEvent<AnimeFrameEventData>, u8)>,
}
impl GlobalAnimeAbout {
    pub const CURVE_FRAME_EVENT_FRAMES: u16 = 60000;
    pub fn record_group(&mut self,  id_obj: Entity, id_group: DefaultKey) {
        self.group_records.insert(id_group, (id_obj, CurveFrameEvent::new(Self::CURVE_FRAME_EVENT_FRAMES as KeyFrameCurveValue), 0));
    }
    pub fn add_frame_event(&mut self,  id_group: DefaultKey, percent: f32, data: AnimeFrameEventData) {
        if let Some(record) = self.group_records.get_mut(&id_group) {
            record.1.add((percent as KeyFrameCurveValue * Self::CURVE_FRAME_EVENT_FRAMES as KeyFrameCurveValue) as FrameIndex, data);
        }
    }
    pub fn add_frame_event_listen(&mut self,  id_group: DefaultKey) {
        if let Some(listen) = self.group_records.get_mut(&id_group) {
            listen.2 = listen.2 | TagGroupListen::FRAME;
        }
    }
    pub fn add_start_listen(&mut self,  id_group: DefaultKey) {
        if let Some(listen) = self.group_records.get_mut(&id_group) {
            listen.2 = listen.2 | TagGroupListen::START;
        }
    }
    pub fn add_end_listen(&mut self,  id_group: DefaultKey) {
        if let Some(listen) = self.group_records.get_mut(&id_group) {
            listen.2 = listen.2 | TagGroupListen::END;
        }
    }
    pub fn add_loop_listen(&mut self,  id_group: DefaultKey) {
        if let Some(listen) = self.group_records.get_mut(&id_group) {
            listen.2 = listen.2 | TagGroupListen::LOOP;
        }
    }
    pub fn remove(&mut self, id_group: &DefaultKey) {
        self.group_records.remove(id_group);
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct GlobalAnimeEvents(pub Vec<(Entity, AnimationGroupID, u8, u32)>);


#[derive(Resource, Deref, DerefMut, Default)]
pub struct SceneAnimationContextMap(XHashMap<Entity, SceneAnimationContext>);
impl SceneAnimationContextMap {
    pub fn init_scene(&mut self, idscene: Entity) {
        self.0.insert(idscene, SceneAnimationContext::new());
    }
    pub fn remove_scene(&mut self, idscene: &Entity) -> Option<SceneAnimationContext> {
        self.0.remove(idscene)
    }
    pub fn query_group_animations(
        &self,
        idscene: Entity,
        idgroup: DefaultKey,
    ) -> Option<&Vec<TargetAnimation<Entity>>> {
        if let Some(ctx) = self.0.get(&idscene) {
            if let Some(group) = ctx.0.animation_group(idgroup) {
                return Some(group.animations());
            }
        }
        None
    }
    pub fn group_weight(
        &mut self,
        idscene: Entity,
        idgroup: DefaultKey,
        weight: f32,
    ) {
        if let Some(ctx) = self.0.get_mut(&idscene) {
            ctx.0.animation_group_weight(idgroup, weight);
        }
    }
    /// 动画组创建 为 立即执行
    pub fn create_group(
        &mut self,
        id_scene: Entity,
    ) -> Option<DefaultKey> {
        let id_group = if let Some(ctx) = self.0.get_mut(&id_scene) {
            ctx.0.create_animation_group()
        } else {
            return None;
        };

        Some(id_group)
    }
    /// 动画组销毁 为 立即执行
    pub fn delete_group(&mut self, idscene: &Entity, idgroup: DefaultKey) {
        if let Some(ctx) = self.0.get_mut(idscene) {
            ctx.0.del_animation_group(idgroup);
        }
    }
    /// 最外层 的 system 中调用
    pub fn apply_removed_animations<F: FrameDataValue, D: AsRef<FrameCurve<F>>>(&self, typectx: &mut TypeAnimationContext<F, D>) {
        self.0.iter().for_each(|ctx| {
            ctx.1.0.apply_removed_animations(typectx);
        });
    }
    /// 最外层 的 system 中调用 - 在 所有 apply_removed_animations 调用之后
    pub fn clear_removed_animations(&mut self) {
        self.0.iter_mut().for_each(|ctx| {
            ctx.1.0.clear_removed_animations();
        });
    }
    
    ///
    pub fn start_with_progress(
        &mut self,
        id_scene: Entity,
        group: DefaultKey,
        param: AnimationGroupParam,
        delay_ms: pi_animation::base::TimeMS,
        fillmode: pi_animation::base::EFillMode,
    )  {
        if let Some(ctx) = self.0.get_mut(&id_scene) {
            match ctx.0.start_with_progress(group, param.speed, param.loop_mode, param.from, param.to, param.fps, param.amountcalc, delay_ms, fillmode) {
                Ok(_) => {
                    // log::warn!("Start Anime Ok!");
                },
                Err(e) => {
                    // log::warn!("Start Anime faile! {:?}", e);
                },
            }
        }
    }
    ///
    pub fn pause(
        &mut self,
        id_scene: Entity,
        group: DefaultKey,
    )  {
        if let Some(ctx) = self.0.get_mut(&id_scene) {
            ctx.0.pause(group);
        }
    }
    ///
    pub fn stop(
        &mut self,
        id_scene: Entity,
        group: DefaultKey,
    )  {
        if let Some(ctx) = self.0.get_mut(&id_scene) {
            ctx.0.stop(group);
        }
    }
    pub fn add_target_anime(
        &mut self,
        id_scene: Entity,
        target: Entity,
        group: DefaultKey,
        animation: AnimationInfo,
    )  {
        if let Some(ctx) = self.0.get_mut(&id_scene) {
            // log::warn!("add_target_anime Ok!");
            ctx.0.add_target_animation_notype(animation, group, target);
        }
    }
}

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
    mut ctxs: ResMut<SceneAnimationContextMap>,
) {
    ctxs.clear_removed_animations();
}
