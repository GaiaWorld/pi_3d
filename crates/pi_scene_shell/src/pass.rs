use std::ops::Deref;

// use bevy_ecs::prelude::Component;

pub type PassTagValue = u16;

#[derive(Debug, Clone, Copy,  PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PassTag(PassTagValue);
impl Deref for PassTag {
    type Target = PassTagValue;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl PassTag {
    pub const PASS_01: u16 = 0b0000_0000_0000_0001;
    pub const PASS_02: u16 = 0b0000_0000_0000_0010;
    pub const PASS_03: u16 = 0b0000_0000_0000_0100;
    pub const PASS_04: u16 = 0b0000_0000_0000_1000;
    pub const PASS_05: u16 = 0b0000_0000_0001_0000;
    pub const PASS_06: u16 = 0b0000_0000_0010_0000;
    pub const PASS_07: u16 = 0b0000_0000_0100_0000;
    pub const PASS_08: u16 = 0b0000_0000_1000_0000;
    pub const PASS_TAG_01: PassTag = PassTag(0b0000_0000_0000_0001);
    pub const PASS_TAG_02: PassTag = PassTag(0b0000_0000_0000_0010);
    pub const PASS_TAG_03: PassTag = PassTag(0b0000_0000_0000_0100);
    pub const PASS_TAG_04: PassTag = PassTag(0b0000_0000_0000_1000);
    pub const PASS_TAG_05: PassTag = PassTag(0b0000_0000_0001_0000);
    pub const PASS_TAG_06: PassTag = PassTag(0b0000_0000_0010_0000);
    pub const PASS_TAG_07: PassTag = PassTag(0b0000_0000_0100_0000);
    pub const PASS_TAG_08: PassTag = PassTag(0b0000_0000_1000_0000);
    // pub const PASS_TAG_09: PassTag = PassTag(0b0000_0001_0000_0000);
    // pub const PASS_TAG_10: PassTag = PassTag(0b0000_0010_0000_0000);
    // pub const PASS_TAG_11: PassTag = PassTag(0b0000_0100_0000_0000);
    // pub const PASS_TAG_12: PassTag = PassTag(0b0000_1000_0000_0000);
    // pub const PASS_TAG_13: PassTag = PassTag(0b0001_0000_0000_0000);
    // pub const PASS_TAG_14: PassTag = PassTag(0b0010_0000_0000_0000);
    // pub const PASS_TAG_15: PassTag = PassTag(0b0100_0000_0000_0000);
    // pub const PASS_TAG_16: PassTag = PassTag(0b1000_0000_0000_0000);
    pub fn new(val: u16) -> Self {
        match val {
            0b0000_0000_0000_0001 => { Self::PASS_TAG_01 }
            0b0000_0000_0000_0010 => { Self::PASS_TAG_02 }
            0b0000_0000_0000_0100 => { Self::PASS_TAG_03 }
            0b0000_0000_0000_1000 => { Self::PASS_TAG_04 }
            0b0000_0000_0001_0000 => { Self::PASS_TAG_05 }
            0b0000_0000_0010_0000 => { Self::PASS_TAG_06 }
            0b0000_0000_0100_0000 => { Self::PASS_TAG_07 }
            _ => { Self::PASS_TAG_08 }
        }
    }

    pub fn index(&self) -> usize {
        match self.0 {
            0b0000_0000_0000_0001 => { 00 },
            0b0000_0000_0000_0010 => { 01 },
            0b0000_0000_0000_0100 => { 02 },
            0b0000_0000_0000_1000 => { 03 },
            0b0000_0000_0001_0000 => { 04 },
            0b0000_0000_0010_0000 => { 05 },
            0b0000_0000_0100_0000 => { 06 },
            0b0000_0000_1000_0000 => { 07 },
            // 0b0000_0001_0000_0000 => { 08 },
            // 0b0000_0010_0000_0000 => { 09 },
            // 0b0000_0100_0000_0000 => { 10 },
            // 0b0000_1000_0000_0000 => { 11 },
            // 0b0001_0000_0000_0000 => { 12 },
            // 0b0010_0000_0000_0000 => { 13 },
            // 0b0100_0000_0000_0000 => { 14 },
            // 0b1000_0000_0000_0000 => { 15 },
            _ => { usize::MAX },
        }
    }
}

// pub const PASS_COUNT: usize = 12;

// /// * 渲染 Pass
// ///   * 每个 Pass 对应一个渲染流程
// ///   * 每个材质 只对应 一个Pass
// ///   * example: ShadowCast, DepthPrePass, Opaque, Skybox, Transparent,

// #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Component)]
// pub enum EPassTag {
//     ShadowCast      = 1,
//     DepthPrePass    ,
//     Opaque          ,
//     OpaqueExtend    ,
//     SkyWater        ,
//     AlphaTest       ,
//     Transparent     ,
//     TransparentExtend,
// }
// impl EPassTag {
//     pub fn index(&self) -> usize {
//         match self {
//             EPassTag::ShadowCast => 0,
//             EPassTag::DepthPrePass => 1,
//             EPassTag::Opaque => 2,
//             EPassTag::SkyWater => 3,
//             EPassTag::AlphaTest => 4,
//             EPassTag::Transparent => 5,
//             EPassTag::OpaqueExtend => 6,
//             EPassTag::TransparentExtend => 7,
//         }
//     }
//     pub fn as_pass(&self) -> PassTag {
//         match self {
//             EPassTag::ShadowCast => Self::PASS_TAG_01,
//             EPassTag::DepthPrePass => Self::PASS_TAG_02,
//             EPassTag::Opaque => Self::PASS_TAG_03,
//             EPassTag::SkyWater => Self::PASS_TAG_04,
//             EPassTag::AlphaTest => Self::PASS_TAG_05,
//             EPassTag::Transparent => Self::PASS_TAG_06,
//             EPassTag::OpaqueExtend => Self::PASS_TAG_07,
//             EPassTag::TransparentExtend => Self::PASS_TAG_08,
//         }
//     }
// }

// /// 每个 Pass 指定一个配置, 配置相同的 Pass 可以合并在一个Renderer中渲染, (需要项目配置时保证)
// #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
// pub struct PassRenderInfo {
//     pub color_format: ColorFormat,
//     pub depth_stencil_format: DepthStencilFormat,
//     pub blendable: bool,
// }
// impl PassRenderInfo {
//     pub fn shadow() -> Self {
//         Self {
//             color_format: ColorFormat::Rgba16Float,
//             depth_stencil_format: DepthStencilFormat::Depth32Float,
//             blendable: false,
//         }
//     }
//     pub fn opaque() -> Self {
//         Self {
//             color_format: ColorFormat::Rgba8Unorm,
//             depth_stencil_format: DepthStencilFormat::Depth24PlusStencil8,
//             blendable: false,
//         }
//     }
//     pub fn transparent() -> Self {
//         Self {
//             color_format: ColorFormat::Rgba8Unorm,
//             depth_stencil_format: DepthStencilFormat::Depth24PlusStencil8,
//             blendable: true,
//         }
//     }
//     pub fn normal() -> Self {
//         Self {
//             color_format: ColorFormat::Rgba8Unorm,
//             depth_stencil_format: DepthStencilFormat::Depth24PlusStencil8,
//             blendable: true,
//         }
//     }
//     pub fn color_format(&self) -> wgpu::TextureFormat {
//         self.color_format.val()
//     }
//     pub fn depth_format(&self) -> Option<wgpu::TextureFormat> {
//         self.depth_stencil_format.val()
//     }

//     pub fn depth_write(&self) -> bool {
//         self.depth_stencil_format != DepthStencilFormat::None
//     }
//     pub fn blend(&self) -> bool {
//         self.blendable
//     }
// }