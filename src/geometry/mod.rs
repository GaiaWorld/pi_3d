

use render_data_container::{TVertexBufferKindKey, TGeometryBufferID, GeometryBufferPool};
use render_geometry::geometry::{Geometry, GeometryBufferDesc};

pub type VDK = usize;
pub type GBID = usize;

pub type GeometryBufferID = usize;
// impl TGeometryBufferID for GeometryBufferID{}

pub struct GeometryMeta {
    pub data: Geometry<VDK, GeometryBufferID>,
    pub desc: Vec<GeometryBufferDesc<VDK>>,
}

pub struct SingleGeometryBufferPool {
    list: Vec<Option<render_data_container::GeometryBuffer>>,
}

impl GeometryBufferPool<GBID> for SingleGeometryBufferPool {
    fn insert(&mut self, data: render_data_container::GeometryBuffer) -> usize {
        let result = self.list.len();

        self.list.push(Some(data));

        result
    }

    fn remove(&mut self, key: &usize) -> Option<render_data_container::GeometryBuffer> {
        if self.list.len() > *key {
            self.list.push(None);
            self.list.swap_remove(*key)
        } else {
            None
        }
    }

    fn get(&self, key: &usize) -> Option<&render_data_container::GeometryBuffer> {
        match self.list.get(*key) {
            Some(geo) => match geo {
                Some(geo) => Some(geo),
                None => None,
            },
            None => None,
        }
        
    }

    fn get_size(&self, key: &usize) -> usize {
        match self.list.get(*key) {
            Some(geo) => match geo {
                Some(geo) => geo.size(),
                None => 0,
            },
            None => 0,
        }
    }

    fn get_mut(&mut self, key: &usize) -> Option<&mut render_data_container::GeometryBuffer> {
        match self.list.get_mut(*key) {
            Some(geo) => match geo {
                Some(geo) => Some(geo),
                None => None,
            },
            None => None,
        }
    }

    fn get_buffer(&self, key: &usize) -> Option<&wgpu::Buffer> {
        match self.list.get(*key) {
            Some(geo) => match geo {
                Some(geo) => geo.get_buffer(),
                None => None,
            },
            None => None,
        }
    }
}
