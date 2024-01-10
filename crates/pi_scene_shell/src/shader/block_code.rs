
use pi_atom::Atom;

pub trait TToBlockCodeAtom {
    fn to_block_code(&self) -> BlockCodeAtom;
}

impl TToBlockCodeAtom for pi_render::rhi::shader::BlockCodeAtom {
    fn to_block_code(&self) -> BlockCodeAtom {
        let mut code_define = String::from("");
        for c in self.define.iter() {
            code_define += c.code.as_str();
        }
        
        let mut code_running = String::from("");
        for c in self.running.iter() {
            code_running += c.code.as_str();
        }
        
        BlockCodeAtom {
            define: Atom::from(code_define),
            running: Atom::from(code_running),
        }
    }
}

/// 代码片段
#[derive(Debug, Clone)]
pub struct BlockCodeAtom {
    /// 声明代码
    pub define: Atom,
    /// 运行代码
    pub running: Atom,
}
impl BlockCodeAtom {
    pub fn size(&self) -> usize {
        self.define.as_bytes().len() + self.running.as_bytes().len()
    }
    pub fn to_block_code(&self) -> BlockCode {
        BlockCode {
            define: String::from(self.define.as_str()),
            running: String::from(self.running.as_str()),
        }
    }
}

/// 代码片段
#[derive(Debug, Clone)]
pub struct BlockCode {
    /// 声明代码
    pub define: String,
    /// 运行代码
    pub running: String,
}
impl BlockCode {
    pub fn size(&self) -> usize {
        self.define.as_bytes().len() + self.running.as_bytes().len()
    }
}

/// 代码片段
#[derive(Debug, Clone)]
pub struct CodeSnippet(pub String);
impl CodeSnippet {
    pub fn size(&self) -> usize {
        self.0.as_bytes().len()
    }
}

#[derive(Default)]
pub struct ShaderBlock {
    pub vs_denfine: String,
    pub vs_running: String,
    pub fs_denfine: String,
    pub fs_running: String,
}

pub struct ShaderBlocks(Vec<ShaderBlock>);
impl ShaderBlocks {
    pub fn add(&mut self, key: &ShaderBlockID, value: ShaderBlock) {
        let len = self.0.len() as u32;
        let idx = key.0;
        if len <= idx {
            for _ in len..idx {
                self.0.push(ShaderBlock::default());
            }
        }
        self.0[idx as usize] = value;
    }
    pub fn get(&self, key: &ShaderBlockID) -> Option<&ShaderBlock> {
        self.0.get(key.0 as usize)
    }
}

#[derive(Clone, Copy)]
pub struct ShaderBlockID(u32);
impl ShaderBlockID {
    // pub const 
}
