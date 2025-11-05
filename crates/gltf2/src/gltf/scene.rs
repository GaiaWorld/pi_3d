use miniserde::{Deserialize, Serialize};

pub type IndexNode = u32;
pub type IndexMesh = u32;
pub type IndexCamera = u32;
pub type IndexSkin = u32;

/// A node in the node hierarchy.  When the node contains `skin`, all
/// `mesh.primitives` must contain `JOINTS_0` and `WEIGHTS_0` attributes.
/// A node can have either a `matrix` or any combination of
/// `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted
/// to matrices and postmultiplied in the `T * R * S` order to compose the
/// transformation matrix; first the scale is applied to the vertices, then the
/// rotation, and then the translation. If none are provided, the transform is the
/// identity. When a node is targeted for animation (referenced by an
/// animation.channel.target), only TRS properties may be present; `matrix` will not
/// be present.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node {
    /// The index of the camera referenced by this node.
    pub camera: Option<IndexCamera>,

    /// The indices of this node's children.
    pub children: Option<Vec<IndexNode>>,

    /// Extension specific data.
    pub extensions: Option<extensions::scene::Node>,

    /// Optional application specific data.
    pub extras: Extras,

    /// 4x4 column-major transformation matrix.
    ///
    /// glTF 2.0 specification:
    ///     When a node is targeted for animation (referenced by an
    ///     animation.channel.target), only TRS properties may be present;
    ///     matrix will not be present.
    ///
    /// TODO: Ensure that .matrix is set to None or otherwise skipped during
    ///       serialization, if the node is targeted for animation.
    ///
    pub matrix: Option<[f32; 16]>,

    /// The index of the mesh in this node.
    pub mesh: Option<IndexMesh>,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// The node's unit quaternion rotation in the order (x, y, z, w), where w is
    /// the scalar.
    pub rotation: Option<UnitQuaternion>,

    /// The node's non-uniform scale.
    pub scale: Option<[f32; 3]>,

    /// The node's translation.
    pub translation: Option<[f32; 3]>,

    /// The index of the skin referenced by this node.
    pub skin: Option<IndexSkin>,

    /// The weights of the instantiated Morph Target. Number of elements must match
    /// the number of Morph Targets of used mesh.
    pub weights: Option<Vec<f32>>,
}

/// The root `Node`s of a scene.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scene {
    /// Extension specific data.
    pub extensions: Option<extensions::scene::Scene>,

    /// Optional application specific data.
    pub extras: Extras,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// The indices of each root node.
    pub nodes: Vec<IndexNode>,
}

/// Unit quaternion rotation in the order (x, y, z, w), where w is the scalar.
pub type UnitQuaternion = [f32; 4];
