///
/// 网格信息单独与 GameObject 绑定

use pi_slotmap::{DefaultKey, SlotMap};
use render_data_container::{TVertexBufferKindKey, TGeometryBufferID, GeometryBufferPool};
use render_geometry::geometry::{VertexAttributeBufferMeta};

use crate::{object::ObjectID, plugin::Plugin};

pub type VDK = usize;
pub type GBID = DefaultKey;

pub struct GeometryID(pub ObjectID);
pub struct PluginBuildinGeometry;
impl Plugin for PluginBuildinGeometry {
    fn init(
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        todo!()
    }
}
