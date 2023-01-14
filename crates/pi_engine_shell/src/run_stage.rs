use std::vec::Drain;

use pi_atom::Atom;
use pi_ecs::prelude::StageBuilder;
use pi_hash::XHashMap;

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

#[derive(Debug, Clone, Copy)]
///
/// * 在运行阶段之上封装了 章节管理
/// * 每章节可以有多个阶段,章节内部的阶段间有顺序
/// * 每章节间有顺序
/// * 一个章节内阶段结束才能进入下个章节
/// * 当 一个System需要等待多个System的结束, 且编码时无法确定依赖的System时, 应该将该System放入下一章节
pub enum ERunStageChap {
    Command,
    Logic01,
    Logic02,
    Uniform,
}

pub struct RunStage {
    command: RunStageSub,
    logic01: RunStageSub,
    logic02: RunStageSub,
    uniform_update: RunStageSub,
    list: Vec<StageBuilder>,
}
impl RunStage {
    pub fn new() -> Self {
        Self { command: RunStageSub::new(), uniform_update: RunStageSub::new(), logic01: RunStageSub::new(), logic02: RunStageSub::new(), list: vec![]}
    }
    /// * 获取System在指定章节内的 阶段
    /// * 当未能查找到 自身依赖的 System 的注册信息时会在编译时报错, 给出了出错的 System 及 依赖的 System 注册名称
    pub fn query_stage<T: TSystemStageInfo>(&mut self, chap: ERunStageChap) -> &mut StageBuilder {
        match chap {
            ERunStageChap::Command => {
                self.command.query_stage::<T>()
            },
            ERunStageChap::Logic01 => {
                self.command.query_stage::<T>()
            },
            ERunStageChap::Logic02 => {
                self.command.query_stage::<T>()
            },
            ERunStageChap::Uniform => {
                self.uniform_update.query_stage::<T>()
            },
        }
    }
    pub fn drain(&mut self) -> Drain<StageBuilder> {
        self.command.drain().for_each(|item| {
            self.list.push(item);
        });

        self.logic01.drain().for_each(|item| {
            self.list.push(item);
        });

        self.logic02.drain().for_each(|item| {
            self.list.push(item);
        });

        self.uniform_update.drain().for_each(|item| {
            self.list.push(item);
        });

        self.list.drain(..)
    }
    pub fn log(&self) {
        
        
        let root_dir = std::env::current_dir().unwrap();
        
        let file_name = "Stages.txt";

        let mut text = String::from("");

        text += self.command.log().as_str();
        text += "\r\n--------------------------------------------------\r\n";

        text += self.uniform_update.log().as_str();

        std::fs::write(root_dir.join(file_name), text);
    }
    pub fn levels(&self) {
        
        
        let root_dir = std::env::current_dir().unwrap();
        
        let file_name = "Levels.txt";

        let mut text = String::from("");

        text += self.command.log().as_str();
        text += "\r\n--------------------------------------------------\r\n";

        text += self.uniform_update.log().as_str();

        std::fs::write(root_dir.join(file_name), text);
    }
}

struct RunStageSub {
    stages: Vec<StageBuilder>,
    flags: Vec<LevelFlag>,
    flag_counter: LevelFlag,
    sys_flags: XHashMap<KeySystem, LevelFlag>,
    sys_childs: XHashMap<KeySystem, Vec<LevelFlag>>,
}

impl RunStageSub {
    pub fn new() -> Self {
        let mut result = Self {
            stages: vec![],
            flags: vec![],
            flag_counter: 0,
            sys_flags: XHashMap::default(),
            sys_childs: XHashMap::default(),
        };

        result.query_stage::<SysPre>();

        result
    }
    ///
    /// * 当未能查找到 自身依赖的 System 的注册信息时会在编译时报错, 给出了出错的 System 及 依赖的 System 注册名称
    pub fn query_stage<T: TSystemStageInfo>(&mut self) -> &mut StageBuilder {
        let sys_key = T::key();
        let parent_keys = T::depends();

        // parent_keys.iter().for_each(|key| {
        //     self._query_stage(key.clone(), vec![]);
        // });

        self._query_stage(sys_key, parent_keys)
    }
    fn _query_stage(&mut self, sys_key: KeySystem, parent_keys: Vec<KeySystem>) -> &mut StageBuilder {
        // 是否已经注册
        if let Some(flag) = self.sys_flags.get(&sys_key) {
            let len = self.flags.len();
            let mut level = usize::MAX;
            // 父节点的level - 寻找有效的最大值
            for index in 0..len {
                if &self.flags[index] == flag {
                    level = index;
                }
            }
            self.stages.get_mut(level).unwrap()
        } else {
            // 寻找父节点的level
            let mut parent_level = usize::MAX;
            parent_keys.iter().for_each(|parent_key| {
                if let Some(flag) = self.sys_flags.get(parent_key) {
                    let len = self.flags.len();

                    // 父节点的level - 寻找有效的最大值
                    for level in 0..len {
                        if &self.flags[level] == flag {
                            if parent_level == usize::MAX {
                                parent_level = level;
                            } else {
                                parent_level = parent_level.max(level);
                            }
                        }
                    }
                } else {
                    panic!("Parent Not Regist: {:?}, {:?}", sys_key, parent_key);
                }
            });

            // 寻找子节点的level
            let mut child_level = usize::MAX;
            if let Some(childs) = self.sys_childs.get(&sys_key) {
                childs.iter().for_each(|flag| {
                    let len = self.flags.len();

                    // 子节点的Level - 寻找最小值
                    for level in 0..len {
                        if &self.flags[level] == flag {
                            if child_level == usize::MAX {
                                child_level = level;
                            } else {
                                child_level = child_level.min(level);
                            }
                        }
                    }
                });
            }

            if parent_level == usize::MAX {
                // 未找到父节点, 也未找到子节点 - 使用最后一个
                if child_level == usize::MAX {
                    let len = self.flags.len();
                    let level = if len == 0 {
                        let flag = self.flag_counter;
                        let stage = StageBuilder::new();
                        self.stages.push(stage);
                        self.flags.push(flag);
                        self.flag_counter += 1;
                        0
                    } else {
                        len - 1
                    };
                    
                    // let flag = self.flag_counter;
                    // let stage = StageBuilder::new();
                    // self.stages.push(stage);
                    // self.flags.push(flag);
                    // self.flag_counter += 1;
                    // let level = self.flags.len() - 1;

                    let flag = self.flags.get(level).unwrap();
                    self.sys_flags.insert(sys_key.clone(), *flag);
                    self.sys_childs.insert(sys_key, vec![]);
                    self.stages.get_mut(level).unwrap()
                // 未找到父节点, 找到子节点 - 使用子节点前一个
                } else {
                    let level = if child_level == 0 {
                        let flag = self.flag_counter;
                        let stage = StageBuilder::new();
                        self.stages.insert(0, stage);
                        self.flags.insert(0, flag);
                        self.flag_counter += 1;
                        0
                    } else {
                        child_level - 1
                    };
                    
                    // let flag = self.flag_counter;
                    // let stage = StageBuilder::new();
                    // self.stages.insert(child_level, stage);
                    // self.flags.insert(child_level, flag);
                    // self.flag_counter += 1;
                    // let level = child_level;
                    

                    let flag = self.flags.get(level).unwrap();
                    self.sys_flags.insert(sys_key.clone(), *flag);
                    self.sys_childs.insert(sys_key, vec![]);
                    self.stages.get_mut(level).unwrap()
                }
            // 找到父节点 - 使用父节点后一个
            } else {
                let level = parent_level + 1;
                if self.flags.len() <= level {
                    let flag = self.flag_counter;
                    let stage = StageBuilder::new();
                    self.stages.push(stage);
                    self.flags.push(flag);
                    self.flag_counter += 1;
                }
                // let flag = self.flag_counter;
                // let stage = StageBuilder::new();
                // self.stages.insert(parent_level + 1, stage);
                // self.flags.insert(parent_level + 1, flag);
                // self.flag_counter += 1;
                

                // if sys_key == "pi_scene_context::transforms::command::SysTransformNodeCommand" {
                //     log::debug!("{:?}", sys_key);
                //     log::debug!("{:?},{:?}, {:?}", self.flags, self.flags.get(level).unwrap(), flag);
                //     log::debug!("{:?}", parent_keys);
                //     log::debug!("{:?}, {:?}, {:?}", parent_level, child_level, level);
                // }

                let flag = self.flags.get(level).unwrap();
                self.sys_flags.insert(sys_key.clone(), *flag);
                self.sys_childs.insert(sys_key, vec![]);
                self.stages.get_mut(level).unwrap()
            }
        }

    }
    pub fn drain(&mut self) -> Drain<StageBuilder> {
        self.stages.drain(..)
    }

    pub fn log(&self) -> String {

        let mut text = String::from("");

        text += "Stages: Flags: ";
        text += self.flags.len().to_string().as_str();
        text += ", Levels: ";
        text += self.stages.len().to_string().as_str();
        text += "\r\n";

        let mut tempspace = String::from("  ");
        for flag in self.flags.iter() {
            tempspace += "  ";

            self.sys_flags.iter().for_each(|(k, v)| {
                if v == flag {
                    text += tempspace.as_str();
                    text += k;
                    text += "\r\n";
                }
            });
        }

        text
    }
    pub fn levels(&self) -> String {

        let mut text = String::from("");

        // text += "Stages: Flags: ";
        // text += self.flags.len().to_string().as_str();
        // text += ", Levels: ";
        // text += self.stages.len().to_string().as_str();
        // text += "\r\n";

        // let mut temp_syss = vec![];
        // let childs = self.sys_childs.get(&SysPre::key()).unwrap();

        // let mut tempspace = String::from("  ");
        // for flag in self.flags.iter() {
        //     tempspace += "  ";

        //     self.sys_flags.iter().for_each(|(k, v)| {
        //         if v == flag {
        //             text += tempspace.as_str();
        //             text += k;
        //             text += "\r\n";
        //         }
        //     });
        // }

        text
    }
}
