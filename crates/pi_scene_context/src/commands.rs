use std::mem::replace;

use pi_ecs::prelude::Component;

pub trait TCommandList<T: Clone + Component>{
    fn list(&mut self) -> &mut Vec<T>;
    fn push(&mut self, val: T) {
        self.list().push(val);
    }
    fn reset(&mut self) -> Vec<T> {
        replace(self.list(), vec![])
    }
}