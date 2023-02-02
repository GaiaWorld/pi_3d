use std::sync::Arc;

use pi_engine_shell::object::ObjectID;
use render_shader::{skin_code::ESkinCode, shader_bind::ShaderBindModelAboutSkin};

pub struct Skeleton {
    pub root: ObjectID,
    pub bones: Vec<ObjectID>,
    pub mode: ESkinCode,
    pub meshes: Vec<ObjectID>,
    pub bind: Arc<ShaderBindModelAboutSkin>,
}