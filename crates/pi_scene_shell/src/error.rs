
use crate::{ecs::*, prelude::MemSize};

use derive_deref::{Deref, DerefMut};
use pi_render::renderer::errors::ErrorRecord;

pub type EError = u32;

#[derive(Resource, Deref, DerefMut)]
pub struct ResErrorRecord(pub(crate) ErrorRecord);
impl MemSize for ResErrorRecord {
    fn memsize(&self) -> usize {
        self.0.0.capacity() * 4 + 8 + 24
    }
}