
use pi_animation::{type_animation_context::{TypeAnimationContext, AnimationContextAmount}, animation_group_manager::AnimationGroupManagerDefault, animation::AnimationInfo, animation_group::AnimationGroupID, animation_listener::EAnimationEvent, curve_frame_event::CurveFrameEvent};
use pi_assets::{asset::{Handle}};
use pi_atom::Atom;
use pi_curves::curve::{frame::{FrameDataValue, KeyFrameDataTypeAllocator, KeyFrameCurveValue}, frame_curve::FrameCurve, FrameIndex};
use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;
use pi_slotmap::DefaultKey;

use crate::{ flags::SceneID};

pub struct TypeFrameCurve<F: FrameDataValue+ 'static>(pub FrameCurve<F>);
impl<F: FrameDataValue+ 'static> pi_assets::asset::Asset for TypeFrameCurve<F> {
    type Key = Atom;

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
pub struct TypeAnimeContext<D: FrameDataValue + 'static> {
    pub ctx: TypeAnimationContext<D, AssetTypeFrameCurve<D>>,
}

#[derive(Debug, Default, Component)]
pub struct AnimationGroups {
    pub map: XHashMap<Atom, AnimationGroupID>,
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
    pub runtimeinfos: pi_animation::runtime_info::RuntimeInfoMap<ObjectID>,
    pub dispose_animations: Vec<AnimationInfo>,
    pub dispose_animationgroups: Vec<(SceneID, AnimationGroupID)>,
    pub group_records: XHashMap<DefaultKey, (Entity, Atom, CurveFrameEvent<AnimeFrameEventData>, u8)>,
}
impl GlobalAnimeAbout {
    pub const CURVE_FRAME_EVENT_FRAMES: u16 = 60000;
    pub fn record_group(&mut self,  id_obj: Entity, key_group: &Atom, id_group: DefaultKey) {
        self.group_records.insert(id_group, (id_obj, key_group.clone(), CurveFrameEvent::new(Self::CURVE_FRAME_EVENT_FRAMES as KeyFrameCurveValue), 0));
    }
    pub fn add_frame_event(&mut self,  id_group: DefaultKey, percent: f32, data: AnimeFrameEventData) {
        if let Some(record) = self.group_records.get_mut(&id_group) {
            record.2.add((percent as KeyFrameCurveValue * Self::CURVE_FRAME_EVENT_FRAMES as KeyFrameCurveValue) as FrameIndex, data);
        }
    }
    pub fn add_frame_event_listen(&mut self,  id_group: DefaultKey) {
        if let Some(listen) = self.group_records.get_mut(&id_group) {
            listen.3 = listen.3 | TagGroupListen::FRAME;
        }
    }
    pub fn add_start_listen(&mut self,  id_group: DefaultKey) {
        if let Some(listen) = self.group_records.get_mut(&id_group) {
            listen.3 = listen.3 | TagGroupListen::START;
        }
    }
    pub fn add_end_listen(&mut self,  id_group: DefaultKey) {
        if let Some(listen) = self.group_records.get_mut(&id_group) {
            listen.3 = listen.3 | TagGroupListen::END;
        }
    }
    pub fn add_loop_listen(&mut self,  id_group: DefaultKey) {
        if let Some(listen) = self.group_records.get_mut(&id_group) {
            listen.3 = listen.3 | TagGroupListen::LOOP;
        }
    }
    pub fn remove(&mut self, id_group: &DefaultKey) {
        self.group_records.remove(id_group);
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct GlobalAnimeEvents(Vec<(Entity, usize, u8, u32)>);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct SceneAnimationContextMap(XHashMap<Entity, SceneAnimationContext>);
impl SceneAnimationContextMap {
    pub fn init_scene(&mut self, idscene: Entity) {
        self.0.insert(idscene, SceneAnimationContext::new());
    }
    pub fn remove_scene(&mut self, idscene: &Entity) -> Option<SceneAnimationContext> {
        self.0.remove(idscene)
    }
    pub fn create_group(
        &mut self,
        id_scene: Entity,
    ) -> Option<DefaultKey> {
        let id_group = if let Some(mut ctx) = self.0.get_mut(&id_scene) {
            ctx.0.create_animation_group()
        } else {
            return None;
        };

        Some(id_group)
    }
}

#[derive(Component)]
pub struct SceneAnimationContext(pub(crate) AnimationContextAmount<ObjectID, AnimationGroupManagerDefault<ObjectID>>, pub(crate) Vec<AnimationGroupID>);
impl SceneAnimationContext {
    pub fn new() -> Self {
        Self(
            AnimationContextAmount::<ObjectID, AnimationGroupManagerDefault<ObjectID>>::default(
                AnimationGroupManagerDefault::<ObjectID>::default()
            ),
            vec![]
        )
    }
}