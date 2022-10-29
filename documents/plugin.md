# Plugin 实现

## 如果有 操作

* 定义 XXXCmommand
  * 声明各种操作
* 定义 SingleXXXCommandList
  * 在全局记录的该操作列表
* 实现操作的 System
  * SysXXXCommand

## 实现 Plugin

* 定义 PluginXXX
  * 实现 `Plugin` trait
  * init 接口中
    * 注册 system
    * 添加 resource

## 定义 Engine 上的 该类操作的接口

* 定义 trait
  * InterfaceXXX

## example - .\src\layer_mask\mod.rs

* ```rust
    #[derive(Debug, Clone, Copy)]
    pub struct LayerMask(pub u32);
    impl Default for LayerMask {
        fn default() -> Self {
            Self(0xFFFFFFFF)
        }
    }
    impl LayerMask {
        pub fn include(&self, other: &Self) -> bool {
            return self.0 & other.0 > 0;
        }
    }

    #[derive(Debug)]
    pub enum LayerMaskCommand {
        Set(ObjectID, LayerMask),
    }
    #[derive(Debug, Default)]
    pub struct SingleLayerMaskCommandList {
        pub list: Vec<LayerMaskCommand>,
    }

    pub struct SysLayerMaskCommand;
    #[setup]
    impl SysLayerMaskCommand {
        #[system]
        pub fn cmd(
            mut cmds: ResMut<SingleLayerMaskCommandList>,
            mut objects: Query<GameObject, Write<LayerMask>>,
        ) {
            cmds.list.drain(..).for_each(|cmd| {
                match cmd {
                    LayerMaskCommand::Set(entity, layer) => {
                        match objects.get_mut(entity) {
                            Some(mut object) => {
                                object.insert_no_notify(layer);
                            },
                            None => todo!(),
                        }
                    },
                }
            });
        }
    }

    pub struct PluginLayerMask;
    impl Plugin for PluginLayerMask {
        fn init(
            engine: &mut crate::engine::Engine,
            stages: &mut crate::run_stage::RunStage,
        ) -> Result<(), crate::plugin::ErrorPlugin> {
            let world = engine.world_mut();

            SysLayerMaskCommand::setup(world, stages.command_stage());

            world.insert_resource(SingleLayerMaskCommandList::default());

            Ok(())
        }
    }

    pub trait InterfaceLayerMask {
        fn layer_mask(
            &mut self,
            object: ObjectID,
            layer: LayerMask,
        ) -> &mut Self;
    }

    impl InterfaceLayerMask for crate::engine::Engine {
        fn layer_mask(
            &mut self,
            object: ObjectID,
            layer: LayerMask,
        ) -> &mut Self {
            let world = self.world_mut();

            let commands = world.get_resource_mut::<SingleLayerMaskCommandList>().unwrap();
            commands.list.push(LayerMaskCommand::Set(object, layer));

            self
        }
    }

    ```