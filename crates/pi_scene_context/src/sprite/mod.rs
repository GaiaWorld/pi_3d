mod sprite;
mod command;
mod system;

use pi_scene_shell::prelude::*;
use system::{sys_create_sprite, sys_modify_sprite};

pub use sprite::*;
pub use command::*;

use crate::prelude::{sys_act_instance_attribute, StageModel};


pub struct PluginSprite;
impl crate::Plugin for PluginSprite {

    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListSpriteCreate::default());
        app.insert_resource(ActionListSpriteModify::default());
        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<TextureFrameAtlas>();
        app.insert_resource(TextureFrameAtlasManager::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout));
        app.insert_resource(ResSpriteFrames::default());

        // let a = app.world.get_resource::<ActionListSpriteCreate>();
        // log::error!("ActionListSpriteCreate : {}", a.is_some());

        // let a = app.world.get_resource::<ResSpriteFrames>();
        // log::error!("SpriteFrame : {}", a.is_some());


        #[cfg(feature = "use_bevy")]
        app.add_systems(StageD3, 
            (
                sys_create_sprite.in_set(StageModel::InstanceCreate),
                sys_modify_sprite.after(sys_act_instance_attribute).in_set(StageModel::AbstructMeshCommand),
            )
        );
        
        #[cfg(not(feature = "use_bevy"))]
        app
        .add_systems(StageD3, sys_create_sprite.in_set(StageModel::InstanceCreate))
        .add_systems(StageD3, sys_modify_sprite.before(sys_act_instance_attribute).in_set(StageModel::AbstructMeshCommand))
        ;
    }
}