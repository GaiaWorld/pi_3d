use crate::object::ObjectID;


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

pub struct CullingFlag(pub bool);

pub struct SceneID(pub ObjectID);
pub struct CameraID(pub usize);

pub struct RenderBlend {
    pub enable: bool,
    pub src_color: wgpu::BlendFactor,
    pub dst_color: wgpu::BlendFactor,
    pub src_alpha: wgpu::BlendFactor,
    pub dst_alpha: wgpu::BlendFactor,
    pub opt_color: wgpu::BlendOperation,
    pub opt_alpha: wgpu::BlendOperation,
}
impl Default for RenderBlend {
    fn default() -> Self {
        Self {
            enable: false,
            src_color: wgpu::BlendFactor::One,
            dst_color: wgpu::BlendFactor::OneMinusSrcAlpha,
            src_alpha: wgpu::BlendFactor::One,
            dst_alpha: wgpu::BlendFactor::OneMinusSrcAlpha,
            opt_color: wgpu::BlendOperation::Add,
            opt_alpha: wgpu::BlendOperation::Add,
        }
    }
}
impl RenderBlend {
    pub fn combine(&mut self) {
        self.enable = true;
    }
}

pub struct RenderTargetState {
    // pub state: wgpu::ColorTargetState,
}
impl RenderTargetState {
    pub fn color_target(
        blend: &RenderBlend,
    ) -> [Option<wgpu::ColorTargetState>;1] {
        match blend.enable {
            true => {
                [
                    Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        blend: Some(
                            wgpu::BlendState {
                                color: wgpu::BlendComponent {
                                    src_factor: blend.src_color,
                                    dst_factor: blend.dst_color,
                                    operation: blend.opt_color,
                                },
                                alpha: wgpu::BlendComponent {
                                    src_factor: blend.src_alpha,
                                    dst_factor: blend.dst_alpha,
                                    operation: blend.opt_alpha,
                                },
                            }
                        ),
                        write_mask: wgpu::ColorWrites::ALL,
                    })
                ]
            },
            false => {
                [
                    Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })
                ]
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PrimitiveState {
    pub state: wgpu::PrimitiveState,
}

#[derive(Debug, Clone, Copy)]
pub struct RenderDepthAndStencil {
    pub depth: bool,
    pub stencil: bool,
    pub depth_compare: wgpu::CompareFunction,
}
impl Default for RenderDepthAndStencil {
    fn default() -> Self {
        Self {
            depth: true,
            stencil: false,
            depth_compare: wgpu::CompareFunction::LessEqual,
        }
    }
}
impl RenderDepthAndStencil {
    pub fn state(
        &self
    ) -> Option<wgpu::DepthStencilState> {
        match (self.depth, self.stencil) {
            (true, true) => {
                Some(
                    wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth24PlusStencil8,
                        depth_write_enabled: true,
                        depth_compare: self.depth_compare,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }
                )
            },
            (true, false) => {
                Some(
                    wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth24PlusStencil8,
                        depth_write_enabled: true,
                        depth_compare: self.depth_compare,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }
                )
            },
            (false, true) => {
                Some(
                    wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth24PlusStencil8,
                        depth_write_enabled: false,
                        depth_compare: self.depth_compare,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }
                )
            },
            (false, false) => None,
        }
    }
}

pub struct RenderLayerMask(pub u32);
impl Default for RenderLayerMask {
    fn default() -> Self {
        Self(0xFFFFFFFF)
    }
}
impl RenderLayerMask {
    pub fn include(&self, other: &Self) -> bool {
        return self.0 & other.0 > 0;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderMode {
    Opaque = 1,
    Skybox = 2,
    AlphaTest = 3,
    Transparent = 4,
}

#[derive(Debug)]
pub struct RenderSortParam {
    /// 渲染类型
    pub mode: RenderMode,
    /// 同 渲染类型 中的 渲染分组
    pub group: u8,
    /// 同 渲染分组 中的 渲染顺序
    pub index: u32,
}
impl RenderSortParam {
    pub fn opaque() -> Self {
        Self {
            mode: RenderMode::Opaque,
            group: 0,
            index: 2000,
        }
    }
    pub fn transparent() -> Self {
        Self {
            mode: RenderMode::Transparent,
            group: 0,
            index: 3000,
        }
    }
    pub fn skybox() -> Self {
        Self {
            mode: RenderMode::Skybox,
            group: 0,
            index: 2000,
        }
    }
    pub fn alpha_test() -> Self {
        Self {
            mode: RenderMode::AlphaTest,
            group: 0,
            index: 2450,
        }
    }
}
impl PartialEq for RenderSortParam {
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode && self.group == other.group && self.index == other.index
    }
}
impl Eq for RenderSortParam {
    fn assert_receiver_is_total_eq(&self) {

    }
}
impl PartialOrd for RenderSortParam {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.mode.partial_cmp(&other.mode) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.group.partial_cmp(&other.group) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.index.partial_cmp(&other.index)
    }
}
impl Ord for RenderSortParam {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}