use crate::ecs::*;

use pi_bevy_render_plugin::{PiRenderSystemSet, FrameState, should_run};

use crate::prelude::{EngineInstant, ErrorRecord};
use crate::prelude::FrameDataPrepare;

// pub struct RunStage {
//     list: Vec<StageBuilder>,
// }
// impl Default for RunStage {
//     fn default() -> Self {
//         Self {
//             list: vec![
//                 StageBuilder::new(),
//                 StageBuilder::new(),
//                 StageBuilder::new(),
//                 StageBuilder::new(),
//                 StageBuilder::new(),

//                 StageBuilder::new(),
//                 StageBuilder::new(),
//                 StageBuilder::new(),
//                 StageBuilder::new(),
//                 StageBuilder::new(),

//                 StageBuilder::new(),
//                 StageBuilder::new(),
//             ]
//         }
//     }
// }
// impl RunStage {
//     const COMMAND: usize = 0;
//     const LOCAL_ROTATION: usize = 1;
//     const BETWEEN_LOCAL_ROTATION_AND_LOCAL_MATRIX: usize = 2;
//     const LOCAL_MATRIX: usize = 3;
//     const BETWEEN_LOCAL_MATRIX_AND_WORLD_MATRIX: usize = 4;
//     const WORLD_MATRIX: usize = 5;
//     const AFTER_WORLD_MATRIX: usize = 6;
//     const UNIFORM_UPDATE: usize = 7;
//     const BETWEEN_UNIFORM_UPDATE_AND_FILTER_CULLING: usize = 8;
//     const FILTER_CULLING: usize = 9;
//     const RENDER_SORT: usize = 10;
//     const DIRTY_STATE: usize = 11;
//     pub fn command_stage(&mut self) -> &mut StageBuilder {
//         self.list.get_mut(Self::COMMAND).unwrap()
//     }
//     pub fn local_rotation_stage(&mut self) -> &mut StageBuilder {
//         self.list.get_mut(Self::LOCAL_ROTATION).unwrap()
//     }
//     pub fn between_local_rotation_and_local_matrix_stage(&mut self) -> &mut StageBuilder {
//         self.list.get_mut(Self::BETWEEN_LOCAL_ROTATION_AND_LOCAL_MATRIX).unwrap()
//     }
//     pub fn local_matrix_stage(&mut self) -> &mut StageBuilder {
//         self.list.get_mut(Self::LOCAL_MATRIX).unwrap()
//     }
//     pub fn between_local_matrix_and_world_matrix_stage(&mut self) -> &mut StageBuilder {
//         self.list.get_mut(Self::BETWEEN_LOCAL_MATRIX_AND_WORLD_MATRIX).unwrap()
//     }
//     pub fn world_matrix(&mut self) -> &mut StageBuilder {
//         self.list.get_mut(Self::WORLD_MATRIX).unwrap()
//     }
//     pub fn after_world_matrix(&mut self) -> &mut StageBuilder {
//         self.list.get_mut(Self::AFTER_WORLD_MATRIX).unwrap()
//     }
//     pub fn uniform_update(&mut self) -> &mut StageBuilder {
//         self.list.get_mut(Self::UNIFORM_UPDATE).unwrap()
//     }
//     pub fn between_uniform_update_and_filter_culling(&mut self) -> &mut StageBuilder {
//         self.list.get_mut(Self::BETWEEN_UNIFORM_UPDATE_AND_FILTER_CULLING).unwrap()
//     }
//     pub fn filter_culling(&mut self) -> &mut StageBuilder {
//         self.list.get_mut(Self::FILTER_CULLING).unwrap()
//     }
//     pub fn render_sort(&mut self) -> &mut StageBuilder {
//         self.list.get_mut(Self::RENDER_SORT).unwrap()
//     }
//     pub fn dirty_state_stage(&mut self) -> &mut StageBuilder {
//         self.list.get_mut(Self::DIRTY_STATE).unwrap()
//     }
//     pub fn drain(&mut self) -> Drain<StageBuilder> {
//         self.list.drain(..)
//     }
// }

pub type KeySystem = &'static str;
pub type LevelFlag = usize;

struct SysPre;
impl TSystemStageInfo for SysPre {
    fn key() -> KeySystem {
        "Root"
    }
    fn depends() -> Vec<KeySystem> {
        vec![]
    }
}

pub trait TSystemStageInfo {
    fn key() -> KeySystem {
        std::any::type_name::<Self>()
    }
    fn depends() -> Vec<KeySystem> {
        vec![
            SysPre::key()
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
///
/// * 在运行阶段之上封装了 章节管理
/// * 每章节可以有多个阶段,章节内部的阶段间有顺序
/// * 每章节间有顺序
/// * 一个章节内阶段结束才能进入下个章节
/// * 当 一个System需要等待多个System的结束, 且编码时无法确定依赖的System时, 应该将该System放入下一章节
pub enum ERunStageChap {
    New,
    // 场景中的 节点, Mesh, Light, Camera [一级实体]
    Initial,
    _InitialApply,
    AnimeAmount,
    Anime,
    Uniform,
    Dispose,
    _DisposeApply,
    StateCheck,
}

pub struct PluginRunstage;
impl Plugin for PluginRunstage {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, ERunStageChap::New);
        app.configure_set(Update, ERunStageChap::Initial.after(ERunStageChap::New));
        app.configure_set(Update, ERunStageChap::_InitialApply.after(ERunStageChap::Initial));
        app.configure_set(Update, ERunStageChap::AnimeAmount.in_set(FrameDataPrepare).after(ERunStageChap::_InitialApply));
        app.configure_set(Update, ERunStageChap::Anime.in_set(FrameDataPrepare).after(ERunStageChap::AnimeAmount));
        app.configure_set(Update, ERunStageChap::Uniform.in_set(FrameDataPrepare).after(ERunStageChap::Anime));
        app.configure_set(Update, ERunStageChap::Dispose.after(ERunStageChap::Uniform));
        app.configure_set(Update, ERunStageChap::_DisposeApply.after(ERunStageChap::Dispose));
        app.configure_set(Update, ERunStageChap::StateCheck.after(ERunStageChap::_DisposeApply).before(PiRenderSystemSet));

        app.insert_resource(ErrorRecord(vec![], false));

        app.insert_resource(RunState3D::default());

#[cfg(feature = "use_bevy")]
{
    app.add_systems(Update, apply_deferred.in_set(ERunStageChap::_InitialApply));
    app.add_systems(Update, apply_deferred.in_set(ERunStageChap::_DisposeApply));
}

        app.insert_resource(RunSystemRecord::default());
        app.add_systems(Update, sys_reset_system_record.in_set(ERunStageChap::StateCheck));

        app.insert_resource(EngineInstant(pi_time::Instant::now()));
    }
}

#[derive(Resource)]
pub struct RunState3D(u32);
impl Default for RunState3D {
    fn default() -> Self {
        Self(Self::ANIMATION)
    }
}
impl RunState3D {
    pub const USE_LIGHTING: u32 = 1 << 0;
    pub const USE_SHADOW: u32 = 1 << 1;
    pub const ANIMATION: u32 = 1 << 2;
    pub fn with_lighting(&mut self, flag: bool) {
        if flag {
            self.0 = self.0 | Self::USE_LIGHTING;
        } else {
            self.0 = self.0 - (self.0 & Self::USE_LIGHTING);
        }
    }
    pub fn with_shadow(&mut self, flag: bool) {
        if flag {
            self.0 = self.0 | Self::USE_SHADOW;
        } else {
            self.0 = self.0 - (self.0 & Self::USE_SHADOW);
        }
    }
    pub fn with_animation(&mut self, flag: bool) {
        if flag {
            self.0 = self.0 | Self::ANIMATION;
        } else {
            self.0 = self.0 - (self.0 & Self::ANIMATION);
        }
    }
}

pub fn should_run_with_lighting(
    // state: Res<FrameState>,
    state3d: Res<RunState3D>,
) -> bool {
    // should_run(state) && 
    (state3d.0 & RunState3D::USE_LIGHTING) == RunState3D::USE_LIGHTING
}

pub fn should_run_with_animation(
    // state: Res<FrameState>,
    state3d: Res<RunState3D>,
) -> bool {
    // should_run(state) && 
    (state3d.0 & RunState3D::ANIMATION) == RunState3D::ANIMATION
}

#[derive(Default, Resource)]
pub struct RunSystemRecord(pub Vec<String>);

pub fn sys_reset_system_record(mut record: ResMut<RunSystemRecord>) {
    // #[cfg(not(target_arch = "wasm32"))]
    // {
    //     let mut txt = String::from("");
    //     record.0.iter().for_each(|name| {
    //         txt += name.as_str();
    //         txt += "\n";
    //     });
    //     let root_dir = std::env::current_dir().unwrap();
    //     let file_name: String = String::from("systems.md");
    //     let _ = std::fs::write(root_dir.join(file_name), txt.as_str());
    // }

    record.0.clear();
}