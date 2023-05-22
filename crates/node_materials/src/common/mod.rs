use crate::base::TNodeMaterialBlock;


pub struct BlockColorGray;
impl TNodeMaterialBlock for BlockColorGray {
    const KEY: &'static str = "COLOR_GRAY";

    const FS_DEFINED: &'static str = include_str!("./color_gray.glsl");

    const VS_DEFINED: &'static str = "";
}


pub struct BlockColorSpace;
impl TNodeMaterialBlock for BlockColorSpace {
    const KEY: &'static str = "COLOR_SPACE";

    const FS_DEFINED: &'static str = include_str!("./color_space.glsl");

    const VS_DEFINED: &'static str = "";
}

pub struct BlockColorHSV;
impl TNodeMaterialBlock for BlockColorHSV {
    const KEY: &'static str = "COLOR_HSV";

    const FS_DEFINED: &'static str = include_str!("./color_hsv.glsl");

    const VS_DEFINED: &'static str = "";
}

pub struct BlockTextureChannel;
impl BlockTextureChannel {
    pub const CHANNEL_R: i32 = 1;
    pub const CHANNEL_G: i32 = 2;
    pub const CHANNEL_B: i32 = 4;
    pub const CHANNEL_A: i32 = 8;
    pub const CHANNEL_GRAY: i32 = 0;
}
impl TNodeMaterialBlock for BlockTextureChannel {
    const KEY: &'static str = "TEXTURE_CHANNEL";

    const FS_DEFINED: &'static str = include_str!("./texture_channel.glsl");

    const VS_DEFINED: &'static str = "";

    fn depends() -> Vec<&'static str> {
        vec![
            BlockColorGray::KEY
        ]
    }
}

pub struct BlockViewDirection;
impl BlockViewDirection {
}
impl TNodeMaterialBlock for BlockViewDirection {
    const KEY: &'static str = "VIEW_DIRECTION";

    const FS_DEFINED: &'static str = include_str!("./view_direction.glsl");

    const VS_DEFINED: &'static str = "";
}

pub struct BlockFloat;
impl BlockFloat {
}
impl TNodeMaterialBlock for BlockFloat {
    const KEY: &'static str = "FLOAT";

    const FS_DEFINED: &'static str = include_str!("./float.glsl");

    const VS_DEFINED: &'static str = "";
}

pub struct BlockUVOffsetSpeed;
impl BlockUVOffsetSpeed {
}
impl TNodeMaterialBlock for BlockUVOffsetSpeed {
    const KEY: &'static str = "UVOFFSETSPEED";

    const FS_DEFINED: &'static str = include_str!("./uv_offset_speed.glsl");

    const VS_DEFINED: &'static str = "";
}