use pi_atom::Atom;
use pi_engine_shell::prelude::*;

pub mod enable;


pub struct SceneID01;
pub struct SceneID02;
pub struct SceneID03;
pub struct SceneID04;

pub struct SceneCameraID01;
pub struct SceneCameraID02;
pub struct SceneCameraID03;
pub struct SceneCameraID04;
pub struct SceneCameraID05;
pub struct SceneCameraID06;

#[derive(Debug, Component)]
pub struct CullingFlag(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, Hash)]
pub struct SceneMainCameraID(pub Option<Entity>);

#[derive(Debug, Component)]
pub struct CameraID(pub usize);
#[derive(Component)]
pub struct UniqueName(pub Atom);

#[derive(Debug, Component, Default)]
pub struct RecordEnable(pub Enable);
impl TAnimatableCompRecord<Enable> for RecordEnable {
    fn comp(&self) -> Enable {
        self.0.clone()
    }
}

#[derive(Debug, Component, Clone)]
pub struct Enable(pub f32);
impl Enable {
    pub fn bool(&self) -> bool {
        self.0 > 0.5
    }
}
impl pi_curves::curve::frame::FrameDataValue for Enable {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = (cubed - (_2 * squared)) + amount;
        let part4 = cubed - squared;

        let result = (((value1.0 * part1) + (value2.0 * part2)) + (tangent1.0 * part3)) + (tangent2.0 * part4);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        3 * 4
    }
}
impl Default for Enable {
    fn default() -> Self {
        Self(1.)
    }
}
impl TAssetCapacity for Enable {
    const ASSET_TYPE: &'static str = "AnimeCurveEnable";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 500 * 1024 , max: 1024 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for Enable {}

pub type PluginAnimeNodeEnable    = PluginTypeAnime<Enable, RecordEnable>;

#[derive(Component)]
pub struct GlobalEnable(pub bool);

pub struct OpsNodeEnable(pub(crate) Entity, pub(crate) Enable, pub u8);
impl OpsNodeEnable {
    pub fn ops(entity: Entity, val: bool) -> Self {
        let val = if val {
            1.
        } else {
            0.
        };
        Self(entity, Enable(val), 0)
    }
}
pub type ActionListNodeEnable = ActionList<OpsNodeEnable>;

pub fn sys_act_node_enable(
    mut cmds: ResMut<ActionListNodeEnable>,
    mut items: Query<(&mut Enable, &mut RecordEnable)>,
) {
    cmds.drain().drain(..).for_each(|OpsNodeEnable(entity, val, count)| {
        if let Ok((mut node, mut record)) = items.get_mut(entity) {
            record.0 = val.clone();
            *node = val;
        } else {
            if count < 2 {
                cmds.push(OpsNodeEnable(entity, val, count + 1));
            }
        }
    });
}

pub struct PluginFlags;
impl Plugin for PluginFlags {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListNodeEnable::default());
        app.add_systems(Update, 
            sys_act_node_enable.in_set(ERunStageChap::Command)
        );
    }
}