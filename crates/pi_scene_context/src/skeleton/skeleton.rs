use pi_engine_shell::object::ObjectID;
use render_shader::skin_code::ESkinCode;


pub struct Skeleton {
    pub root: ObjectID,
    pub bones: Vec<ObjectID>,
    pub mode: ESkinCode,
}