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

#[derive(Debug)]
pub struct SceneID(pub ObjectID);
#[derive(Debug)]
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
                        format: wgpu::TextureFormat::Bgra8Unorm,
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
                        format: wgpu::TextureFormat::Bgra8Unorm,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })
                ]
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PrimitiveState {
    pub state: wgpu::PrimitiveState,
}
impl Default for PrimitiveState {
    fn default() -> Self {
        Self {
            state: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                polygon_mode: wgpu::PolygonMode::Fill,
                ..Default::default()
            }
        }
    }
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
            depth: false,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ERenderMode {
    Opaque = 1,
    Skybox = 2,
    AlphaTest = 3,
    Transparent = 4,
}

#[derive(Debug, Clone, Copy)]
pub struct RenderSortParam {
    /// 渲染类型
    pub mode: ERenderMode,
    /// 同 渲染类型 中的 渲染分组
    pub group: u8,
    /// 同 渲染分组 中的 渲染顺序
    pub index: u32,
}
impl RenderSortParam {
    pub fn opaque() -> Self {
        Self {
            mode: ERenderMode::Opaque,
            group: 0,
            index: 2000,
        }
    }
    pub fn transparent() -> Self {
        Self {
            mode: ERenderMode::Transparent,
            group: 0,
            index: 3000,
        }
    }
    pub fn skybox() -> Self {
        Self {
            mode: ERenderMode::Skybox,
            group: 0,
            index: 2000,
        }
    }
    pub fn alpha_test() -> Self {
        Self {
            mode: ERenderMode::AlphaTest,
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