use pi_animation::{curve_frame_event::CurveFrameEvent, animation_listener::{AnimationListener, OnStart, OnLoop, OnFrameEvent}, animation_group::{AnimationGroupID, AnimationGroup}};
use pi_curves::curve::FrameIndex;
use pi_scene_shell::prelude::*;

pub struct AnimationGroupListener {
    pub(crate) listener: XHashMap<Atom, (CurveFrameEvent<Atom>, AnimationListener<Atom>)>,
}

impl AnimationGroupListener {
    pub fn add_frame_event_data(
        &mut self,
        key_group: &Atom,
        id_group: &AnimationGroupID,
        group: &AnimationGroup<Atom>,
        frame: FrameIndex,
        data: Atom,
    ) -> &mut Self {

        let (frame_event, _listener) = if let Some(item) = self.listener.get_mut(key_group) {
            item
        } else {
            self.listener.insert(key_group.clone(), (CurveFrameEvent::<Atom>::new(group.max_frame()), AnimationListener::<Atom>::new(id_group.clone())));
            self.listener.get_mut(key_group).unwrap()
        };

        frame_event.add(frame, data);

        self
    }

    pub fn add_start(
        &mut self,
        key_group: &Atom,
        id_group: &AnimationGroupID,
        group: &AnimationGroup<Atom>,
        call: OnStart,
    ) -> &mut Self {

        let (_frame_event, listener) = if let Some(item) = self.listener.get_mut(key_group) {
            item
        } else {
            self.listener.insert(key_group.clone(), (CurveFrameEvent::<Atom>::new(group.max_frame()), AnimationListener::<Atom>::new(id_group.clone())));
            self.listener.get_mut(key_group).unwrap()
        };

        listener.on_start.push(call);

        self
    }
    
    pub fn add_frame_event(
        &mut self,
        key_group: &Atom,
        id_group: &AnimationGroupID,
        group: &AnimationGroup<Atom>,
        call: OnFrameEvent<Atom>,
    ) -> &mut Self {

        let (_frame_event, listener) = if let Some(item) = self.listener.get_mut(key_group) {
            item
        } else {
            self.listener.insert(key_group.clone(), (CurveFrameEvent::<Atom>::new(group.max_frame()), AnimationListener::<Atom>::new(id_group.clone())));
            self.listener.get_mut(key_group).unwrap()
        };

        listener.on_frame_event.push(call);

        self
    }
    
    pub fn add_loop(
        &mut self,
        key_group: &Atom,
        id_group: &AnimationGroupID,
        group: &AnimationGroup<Atom>,
        call: OnLoop,
    ) -> &mut Self {

        let (_frame_event, listener) = if let Some(item) = self.listener.get_mut(key_group) {
            item
        } else {
            self.listener.insert(key_group.clone(), (CurveFrameEvent::<Atom>::new(group.max_frame()), AnimationListener::<Atom>::new(id_group.clone())));
            self.listener.get_mut(key_group).unwrap()
        };

        listener.on_loop.push(call);

        self
    }
    
    pub fn add_end(
        &mut self,
        key_group: &Atom,
        id_group: &AnimationGroupID,
        group: &AnimationGroup<Atom>,
        call: OnStart,
    ) -> &mut Self {

        let (_frame_event, listener) = if let Some(item) = self.listener.get_mut(key_group) {
            item
        } else {
            self.listener.insert(key_group.clone(), (CurveFrameEvent::<Atom>::new(group.max_frame()), AnimationListener::<Atom>::new(id_group.clone())));
            self.listener.get_mut(key_group).unwrap()
        };

        listener.on_end.push(call);

        self
    }
}