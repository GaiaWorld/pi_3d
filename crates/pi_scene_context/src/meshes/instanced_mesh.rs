use pi_engine_shell::object::ObjectID;
use render_data_container::EVertexDataFormat;
use render_geometry::vertex_data::EVertexDataKind;

pub struct SingleInstanceBufferMgr {
    counter: usize,
}
impl SingleInstanceBufferMgr {
    pub fn gen_key(&mut self, obj: ObjectID, kind: EVertexDataKind) -> String {
        self.counter += 1;
        String::from(self.counter.to_string() + kind.vs_code())
    }
}

pub struct InstancedMesh;

pub struct InstancedList(pub Vec<ObjectID>);

pub struct FlagInstanceModify;


