// use serde::{Deserialize, Serialize};

// use crate::gltf::{Extras, IndexAccessor, IndexBufferView, TComponentType, TInterpolation};




// #[derive(Clone, Debug, Default, Deserialize, Serialize)]
// pub struct Accessor {
//     #[serde(rename = "bufferView")]
//     pub buffer_view: Option<IndexBufferView>,

//     #[serde(rename = "byteOffset")]
//     pub byte_offset: u32,

//     pub count: u32,
    
//     #[serde(rename = "componentType")]
//     pub component_type: TComponentType,

//     // pub extensions: Option<extensions::accessor::sparse::Indices>,
    
//     pub extras: Extras,
// }

// /// Image data used to create a texture.
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Image {
//     // /// The index of the buffer view that contains the image. Use this instead of
//     // /// the image's uri property.
//     // #[serde(rename = "bufferView")]
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     // pub buffer_view: Option<Index<buffer::View>>,

//     // /// The image's MIME type.
//     // #[serde(rename = "mimeType")]
//     // pub mime_type: Option<MimeType>,

//     /// Optional user-defined name for this object.
//     pub name: Option<String>,

//     /// The uri of the image.  Relative paths are relative to the .gltf file.
//     /// Instead of referencing an external file, the uri can also be a data-uri.
//     /// The image format must be jpg or png.
//     pub uri: Option<String>,

//     // /// Extension specific data.
//     // pub extensions: Option<extensions::image::Image>,

//     /// Optional application specific data.
//     pub extras: Extras,
// }

// /// A buffer points to binary data representing geometry, animations, or skins.
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Buffer {
//     /// The length of the buffer in bytes.
//     #[serde(rename = "byteLength")]
//     pub byte_length: u32,

//     /// Optional user-defined name for this object.
//     pub name: Option<String>,

//     /// The uri of the buffer.  Relative paths are relative to the .gltf file.
//     /// Instead of referencing an external file, the uri can also be a data-uri.
//     pub uri: Option<String>,

//     // /// Extension specific data.
//     // pub extensions: Option<extensions::buffer::Buffer>,

//     /// Optional application specific data.
//     pub extras: Extras,
// }

// /// A keyframe animation.
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Animation {
//     // /// Extension specific data.
//     // pub extensions: Option<extensions::animation::Animation>,

//     /// Optional application specific data.
//     pub extras: Extras,

//     /// An array of channels, each of which targets an animation's sampler at a
//     /// node's property.
//     ///
//     /// Different channels of the same animation must not have equal targets.
//     pub channels: Vec<Channel>,

//     /// Optional user-defined name for this object.
//     pub name: Option<String>,

//     /// An array of samplers that combine input and output accessors with an
//     /// interpolation algorithm to define a keyframe graph (but not its target).
//     pub samplers: Vec<Sampler>,
// }

// /// Targets an animation's sampler at a node's property.
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Channel {
//     /// The index of a sampler in this animation used to compute the value for the
//     /// target.
//     pub sampler: Index<Sampler>,

//     /// The index of the node and TRS property to target.
//     pub target: Target,

//     // /// Extension specific data.
//     // pub extensions: Option<extensions::animation::Channel>,

//     /// Optional application specific data.
//     pub extras: Extras,
// }

// /// The index of the node and TRS property that an animation channel targets.
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Target {
//     // /// Extension specific data.
//     // pub extensions: Option<extensions::animation::Target>,

//     /// Optional application specific data.
//     pub extras: Extras,

//     /// The index of the node to target.
//     pub node: Index<scene::Node>,

//     /// The name of the node's property to modify or the 'weights' of the
//     /// morph targets it instantiates.
//     pub path: Checked<Property>,
// }

// /// Defines a keyframe graph but not its target.
// #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
// pub struct Sampler {
//     // /// Extension specific data.
//     // pub extensions: Option<extensions::animation::Sampler>,

//     /// Optional application specific data.
//     pub extras: Extras,

//     /// The index of an accessor containing keyframe input values, e.g., time.
//     pub input: IndexAccessor,

//     /// The interpolation algorithm.
//     pub interpolation: TInterpolation,

//     /// The index of an accessor containing keyframe output values.
//     pub output: IndexAccessor,
// }

// pub struct Root {

// }

// pub struct GLTFJSON {
//     json: serde_json::Value,
// }
// impl GLTFJSON {
//     pub fn from_slice(val: &[u8]) -> Option<Self> {
//         serde_json::de::from_slice(v)
//     }
// }