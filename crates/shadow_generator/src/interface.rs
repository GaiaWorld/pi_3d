use pi_engine_shell::object::ObjectID;


pub trait TShadowGenerator {
    fn create_materail(
        &self,
        id_light: ObjectID,
    );
}
