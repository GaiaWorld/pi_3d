use pi_engine_shell::object::ObjectID;
use render_shader::skin_code::{ESkinBonesPerVertex, ESkinCode};



pub enum ESkinCommand {
    Skin(ObjectID, ESkinCode)
}

pub struct SingleSkinCommands(pub Vec<ESkinCommand>);

// pub 