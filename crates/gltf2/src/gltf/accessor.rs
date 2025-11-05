use miniserde::{Deserialize, Serialize, json::Value};

/// The component data type.
// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
// pub enum ComponentType {
//     /// Corresponds to `GL_BYTE`.
//     I8 = 1,
//     /// Corresponds to `GL_UNSIGNED_BYTE`.
//     U8,
//     /// Corresponds to `GL_SHORT`.
//     I16,
//     /// Corresponds to `GL_UNSIGNED_SHORT`.
//     U16,
//     /// Corresponds to `GL_UNSIGNED_INT`.
//     U32,
//     /// Corresponds to `GL_FLOAT`.
//     F32,
// }
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

/// Specifies whether an attribute, vector, or matrix.
// #[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
// pub enum Type {
//     /// Scalar quantity.
//     Scalar = 1,
//     /// 2D vector.
//     Vec2,
//     /// 3D vector.
//     Vec3,
//     /// 4D vector.
//     Vec4,
//     /// 2x2 matrix.
//     Mat2,
//     /// 3x3 matrix.
//     Mat3,
//     /// 4x4 matrix.
//     Mat4,
// }
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

pub type IndexBufferView    = u32;
pub type IndexBuffer        = u32;

/// Contains data structures for sparse storage.
pub mod sparse {
    use super::*;
    use crate::extensions;

    /// Indices of those attributes that deviate from their initialization value.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Indices {
        /// The parent buffer view containing the sparse indices.
        ///
        /// The referenced buffer view must not have `ARRAY_BUFFER` nor
        /// `ELEMENT_ARRAY_BUFFER` as its target.
        #[serde(rename = "bufferView")]
        pub buffer_view: IndexBufferView,

        /// The offset relative to the start of the parent `BufferView` in bytes.
        #[serde(rename = "byteOffset")]
        pub byte_offset: u32,

        /// The data type of each index.
        #[serde(rename = "componentType")]
        pub component_type: TComponentType,

        /// Extension specific data.
        pub extensions: Option<extensions::accessor::sparse::Indices>,

        /// Optional application specific data.
        pub extras: Extras,
    }

    /// Sparse storage of attributes that deviate from their initialization value.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Sparse {
        /// The number of attributes encoded in this sparse accessor.
        pub count: u32,

        /// Index array of size `count` that points to those accessor attributes
        /// that deviate from their initialization value.
        ///
        /// Indices must strictly increase.
        pub indices: Indices,

        /// Array of size `count * number_of_components` storing the displaced
        /// accessor attributes pointed by `indices`.
        ///
        /// Substituted values must have the same `component_type` and number of
        /// components as the base `Accessor`.
        pub values: Values,

        /// Extension specific data.
        pub extensions: Option<extensions::accessor::sparse::Sparse>,

        /// Optional application specific data.
        pub extras: Extras,
    }

    /// Array of size `count * number_of_components` storing the displaced
    /// accessor attributes pointed by `accessor::sparse::Indices`.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Values {
        /// The parent buffer view containing the sparse indices.
        ///
        /// The referenced buffer view must not have `ARRAY_BUFFER` nor
        /// `ELEMENT_ARRAY_BUFFER` as its target.
        #[serde(rename = "bufferView")]
        pub buffer_view: IndexBufferView,

        /// The offset relative to the start of the parent buffer view in bytes.
        #[serde(rename = "byteOffset")]
        pub byte_offset: u32,

        /// Extension specific data.
        pub extensions: Option<extensions::accessor::sparse::Values>,

        /// Optional application specific data.
        pub extras: Extras,
    }
}

/// A typed view into a buffer view.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Accessor {
    /// The parent buffer view this accessor reads from.
    ///
    /// This field can be omitted in sparse accessors.
    #[serde(rename = "bufferView")]
    pub buffer_view: Option<IndexBufferView>,

    /// The offset relative to the start of the parent `BufferView` in bytes.
    #[serde(rename = "byteOffset")]
    pub byte_offset: u32,

    /// The number of components within the buffer view - not to be confused
    /// with the number of bytes in the buffer view.
    pub count: u32,

    /// The data type of components in the attribute.
    #[serde(rename = "componentType")]
    pub component_type: TComponentType,

    /// Extension specific data.
    pub extensions: Option<extensions::accessor::Accessor>,

    /// Optional application specific data.
    pub extras: Extras,

    /// Specifies if the attribute is a scalar, vector, or matrix.
    #[serde(rename = "type")]
    pub type_: TType,

    /// Minimum value of each component in this attribute.
    pub min: Option<Value>,

    /// Maximum value of each component in this attribute.
    pub max: Option<Value>,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// Specifies whether integer data values should be normalized.
    pub normalized: bool,

    /// Sparse storage of attributes that deviate from their initialization
    /// value.
    pub sparse: Option<sparse::Sparse>,
}

impl ComponentType {
    /// Returns the number of bytes this value represents.
    pub fn size(val: TComponentType) -> usize {
        match val {
            ComponentType::BYTE | ComponentType::UNSIGNED_BYTE      => 1,
            ComponentType::SHORT | ComponentType::UNSIGNED_SHORT    => 2,
            ComponentType::FLOAT | ComponentType::UNSIGNED_INT      => 4,
            _ => 1,
        }
    }
}

impl Type {
    /// Returns the equivalent number of scalar quantities this type represents.
    pub fn multiplicity(val: TType) -> usize {
        match val {
            Type::SCALAR => 1,
            Type::VEC2 => 2,
            Type::VEC3 => 3,
            Type::VEC4 | Type::MAT2 => 4,
            Type::MAT3 => 9,
            Type::MAT4 => 16,
            _ => 1,
        }
    }
}
