use serde_json::Value;


pub type TComponentType = u32;
pub struct ComponentType{}
impl ComponentType {
    /// Corresponds to `GL_BYTE`.
    pub const BYTE: TComponentType = 5120;

    /// Corresponds to `GL_UNSIGNED_BYTE`.
    pub const UNSIGNED_BYTE: TComponentType = 5121;

    /// Corresponds to `GL_SHORT`.
    pub const SHORT: TComponentType = 5122;

    /// Corresponds to `GL_UNSIGNED_SHORT`.
    pub const UNSIGNED_SHORT: TComponentType = 5123;

    /// Corresponds to `GL_UNSIGNED_INT`.
    pub const UNSIGNED_INT: TComponentType = 5125;

    /// Corresponds to `GL_FLOAT`.
    pub const FLOAT: TComponentType = 5126;
}

pub type TType = String;
pub struct Type{}
impl Type {
pub const SCALAR: &'static str = "SCALAR";
pub const VEC2  : &'static str = "VEC2";
pub const VEC3  : &'static str = "VEC3";
pub const VEC4  : &'static str = "VEC4";
pub const MAT2  : &'static str = "MAT2";
pub const MAT3  : &'static str = "MAT3";
pub const MAT4  : &'static str = "MAT4";
}

pub type TInterpolation = String;
pub struct Interpolation {}
impl Interpolation{
    pub const LINEAR: &'static str = "LINEAR";
    pub const STEP: &'static str = "STEP";
    pub const CUBICSPLINE: &'static str = "CUBICSPLINE";
}

pub type IndexAccessor      = u32;
pub type IndexBufferView    = u32;
pub type IndexBuffer        = u32;

pub type Extras = Option<Value>;