
use pi_animation::{type_animation_context::{TypeAnimationContext, AnimationContextAmount}, animation_group_manager::AnimationGroupManagerDefault, animation::AnimationInfo, animation_group::AnimationGroupID, animation_listener::EAnimationEvent, curve_frame_event::CurveFrameEvent};
use pi_assets::{asset::{Handle}};
use pi_atom::Atom;
use pi_curves::curve::{frame::{FrameDataValue, KeyFrameDataTypeAllocator, KeyFrameCurveValue}, frame_curve::FrameCurve, FrameIndex};
use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;
use pi_slotmap::DefaultKey;

