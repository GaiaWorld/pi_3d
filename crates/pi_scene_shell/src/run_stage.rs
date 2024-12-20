use crate::ecs::*;

use pi_bevy_render_plugin::{PiRenderDevice, PiRenderSystemSet};

use crate::prelude::{ActionList, DeviceLimits3D, EngineInstant, ErrorRecord, MemSize};
use crate::prelude::FrameDataPrepare;

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
    D3,
    New,
    // 场景中的 节点, Mesh, Light, Camera [一级实体]
    Create,
    Modify,
    Dispose,
    _Dispose,
    Culling,
    Culled,
    Collect,
    StateCheck,
}

pub struct PluginRunstage;
impl Plugin for PluginRunstage {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, ERunStageChap::D3.run_if(runif_3d));
        app.configure_set(Update, ERunStageChap::New            .in_set(ERunStageChap::D3));
        app.configure_set(Update, ERunStageChap::Create         .in_set(ERunStageChap::D3).after(ERunStageChap::New));
        app.configure_set(Update, ERunStageChap::Modify         .in_set(ERunStageChap::D3).after(ERunStageChap::Create));
        app.configure_set(Update, ERunStageChap::Dispose        .in_set(ERunStageChap::D3).in_set(FrameDataPrepare).after(ERunStageChap::Modify));
        app.configure_set(Update, ERunStageChap::_Dispose        .in_set(ERunStageChap::D3).after(ERunStageChap::Dispose));
        app.configure_set(Update, ERunStageChap::Culling        .in_set(ERunStageChap::D3).after(ERunStageChap::_Dispose));
        app.configure_set(Update, ERunStageChap::Culled         .in_set(ERunStageChap::D3).after(ERunStageChap::Culling));
        app.configure_set(Update, ERunStageChap::Collect        .in_set(ERunStageChap::D3).in_set(FrameDataPrepare).after(ERunStageChap::Culled));
        app.configure_set(Update, ERunStageChap::StateCheck     .in_set(ERunStageChap::D3).in_set(FrameDataPrepare).after(ERunStageChap::Collect).before(PiRenderSystemSet));

        app.insert_resource(ErrorRecord(vec![], false));

        app.insert_resource(RunState3D::default());

        let device = app.world.get_resource::<PiRenderDevice>().unwrap();
        let limits = device.limits();
        app.insert_resource(DeviceLimits3D(limits));

#[cfg(feature = "use_bevy")]
{
    app.add_systems(Update, apply_deferred.in_set(ERunStageChap::Modify));
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

pub fn runif_3d(
    state: Res<EngineCustomPlugins>,
) -> bool {
    state.active
}

#[derive(Resource, Clone)]
pub struct EngineCustomPlugins {
    pub particle_system: bool,
    pub lighting: bool,
    pub shadowmapping: bool,
    pub directshadowmapping: bool,
    pub pointshadowmapping: bool,
    pub spotshadowmapping: bool,
    pub skeleton: bool,
    pub combinebuffersize: usize,
    pub active: bool,
    // 禁用 UBO
    pub disenable_ubo: bool,
    // 禁用 材质数组
    pub disenable_material_array: bool,
    // 材质数组 最大长度
    pub maxlen_material_array: u32,
    // 一个批次实例化数目的最大值
    pub max_instance_batch_count: u32,
    // 纹理最大尺寸
    pub max_texture_size: u32,
}
impl MemSize for EngineCustomPlugins {
    fn memsize(&self) -> usize {
        16 * 4
    }
}
impl EngineCustomPlugins {
    pub fn new(param: &[u32]) -> Self {
        Self {
            active:                     param[ 0] != 0,
            particle_system:            param[ 1] != 0,
            lighting:                   param[ 2] != 0,
            shadowmapping:              param[ 3] != 0,
            skeleton:                   param[ 4] != 0,
            directshadowmapping:        param[ 5] != 0,
            pointshadowmapping:         param[ 6] != 0,
            spotshadowmapping:          param[ 7] != 0,
            combinebuffersize:          param[ 8] as usize,
            disenable_ubo:              param[ 9] != 0,
            disenable_material_array:   param[10] != 0,
            maxlen_material_array:      param[11],
            max_instance_batch_count:   param[12],
            max_texture_size:           param[13],
        }
    }
} 
impl Default for EngineCustomPlugins {
    fn default() -> Self {
        Self {
            particle_system: true,
            lighting: true,
            shadowmapping: true,
            skeleton: true,
            directshadowmapping: true,
            pointshadowmapping: true,
            spotshadowmapping: true,
            combinebuffersize: 1 * 1024 * 1024,
            active: true,
            disenable_ubo: false,
            disenable_material_array: false,
            maxlen_material_array: 512,
            max_instance_batch_count: u32::MAX,
            max_texture_size: 2048,
        }
    }
}