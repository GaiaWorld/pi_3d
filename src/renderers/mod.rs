use render_data_container::{TGeometryBufferID, GeometryBufferPool, TVertexBufferKindKey, EVertexDataFormat};
use render_geometry::geometry::{Geometry, GeometryBufferDesc};
use render_material::error::EMaterialError;

use crate::{meshes::Mesh, geometry::{VDK, GBID}};

pub mod pipeline;
pub mod render_default;