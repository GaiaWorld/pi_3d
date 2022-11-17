use std::vec::Drain;

use pi_ecs::prelude::StageBuilder;

pub struct RunStage {
    list: Vec<StageBuilder>,
}
impl Default for RunStage {
    fn default() -> Self {
        Self {
            list: vec![
                StageBuilder::new(),
                StageBuilder::new(),
                StageBuilder::new(),
                StageBuilder::new(),
                StageBuilder::new(),

                StageBuilder::new(),
                StageBuilder::new(),
                StageBuilder::new(),
                StageBuilder::new(),
                StageBuilder::new(),

                StageBuilder::new(),
                StageBuilder::new(),
            ]
        }
    }
}
impl RunStage {
    const COMMAND: usize = 0;
    const LOCAL_ROTATION: usize = 1;
    const BETWEEN_LOCAL_ROTATION_AND_LOCAL_MATRIX: usize = 2;
    const LOCAL_MATRIX: usize = 3;
    const BETWEEN_LOCAL_MATRIX_AND_WORLD_MATRIX: usize = 4;
    const WORLD_MATRIX: usize = 5;
    const AFTER_WORLD_MATRIX: usize = 6;
    const UNIFORM_UPDATE: usize = 7;
    const BETWEEN_UNIFORM_UPDATE_AND_FILTER_CULLING: usize = 8;
    const FILTER_CULLING: usize = 9;
    const RENDER_SORT: usize = 10;
    const DIRTY_STATE: usize = 11;
    pub fn command_stage(&mut self) -> &mut StageBuilder {
        self.list.get_mut(Self::COMMAND).unwrap()
    }
    pub fn local_rotation_stage(&mut self) -> &mut StageBuilder {
        self.list.get_mut(Self::LOCAL_ROTATION).unwrap()
    }
    pub fn between_local_rotation_and_local_matrix_stage(&mut self) -> &mut StageBuilder {
        self.list.get_mut(Self::BETWEEN_LOCAL_ROTATION_AND_LOCAL_MATRIX).unwrap()
    }
    pub fn local_matrix_stage(&mut self) -> &mut StageBuilder {
        self.list.get_mut(Self::LOCAL_MATRIX).unwrap()
    }
    pub fn between_local_matrix_and_world_matrix_stage(&mut self) -> &mut StageBuilder {
        self.list.get_mut(Self::BETWEEN_LOCAL_MATRIX_AND_WORLD_MATRIX).unwrap()
    }
    pub fn world_matrix(&mut self) -> &mut StageBuilder {
        self.list.get_mut(Self::WORLD_MATRIX).unwrap()
    }
    pub fn after_world_matrix(&mut self) -> &mut StageBuilder {
        self.list.get_mut(Self::AFTER_WORLD_MATRIX).unwrap()
    }
    pub fn uniform_update(&mut self) -> &mut StageBuilder {
        self.list.get_mut(Self::UNIFORM_UPDATE).unwrap()
    }
    pub fn between_uniform_update_and_filter_culling(&mut self) -> &mut StageBuilder {
        self.list.get_mut(Self::BETWEEN_UNIFORM_UPDATE_AND_FILTER_CULLING).unwrap()
    }
    pub fn filter_culling(&mut self) -> &mut StageBuilder {
        self.list.get_mut(Self::FILTER_CULLING).unwrap()
    }
    pub fn render_sort(&mut self) -> &mut StageBuilder {
        self.list.get_mut(Self::RENDER_SORT).unwrap()
    }
    pub fn dirty_state_stage(&mut self) -> &mut StageBuilder {
        self.list.get_mut(Self::DIRTY_STATE).unwrap()
    }
    pub fn drain(&mut self) -> Drain<StageBuilder> {
        self.list.drain(..)
    }
}