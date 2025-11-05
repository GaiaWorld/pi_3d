
/// The root object of a glTF 2.0 asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Root {
    /// An array of accessors.
    pub accessors: Vec<Accessor>,

    /// An array of keyframe animations.
    pub animations: Vec<Animation>,

    /// Metadata about the glTF asset.
    pub asset: Asset,

    /// An array of buffers.
    pub buffers: Vec<Buffer>,

    /// An array of buffer views.
    pub buffer_views: Vec<buffer::View>,

    /// The default scene.
    pub scene: Option<Index<Scene>>,

    /// Extension specific data.
    pub extensions: Option<extensions::root::Root>,

    /// Optional application specific data.
    pub extras: Extras,

    /// Names of glTF extensions used somewhere in this asset.
    pub extensions_used: Vec<String>,

    /// Names of glTF extensions required to properly load this asset.
    pub extensions_required: Vec<String>,

    /// An array of cameras.
    pub cameras: Vec<Camera>,

    /// An array of images.
    pub images: Vec<Image>,

    /// An array of materials.
    pub materials: Vec<Material>,

    /// An array of meshes.
    pub meshes: Vec<Mesh>,

    /// An array of nodes.
    pub nodes: Vec<Node>,

    /// An array of samplers.
    pub samplers: Vec<texture::Sampler>,

    /// An array of scenes.
    pub scenes: Vec<Scene>,

    /// An array of skins.
    pub skins: Vec<Skin>,

    /// An array of textures.
    pub textures: Vec<Texture>,
}
