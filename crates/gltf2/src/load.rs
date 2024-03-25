use std::hash::{Hash, Hasher};

use crossbeam::queue::SegQueue;
use pi_scene_shell::prelude::*;
use pi_futures::BoxFuture;
use pi_gltf::Gltf;
use pi_particle_system::prelude::{IParticleSystemConfig, ParticleSystemCalculatorID, OpsCPUParticleCalculator, KeyParticleSystemCalculator, ActionSetParticleSystem, ResourceParticleSystem};
use pi_scene_context::prelude::*;

use crate::{EAnimePropertyType, curve_gltf, p3d_anime_curve_query, interpolation_from_u8, particle_system::gltf_format_particle_cfg};

pub type KeyGLTFBase = Atom;
pub type GLTFJson = String;
pub type GLTFDynamicJson = Atom;

pub struct GLTFBin(Vec<u8>);
impl pi_assets::asset::Asset for GLTFBin {
    type Key = u64;
    // const TYPE: &'static str = "GLTFBin";
}
impl pi_assets::asset::Size for GLTFBin {
    fn size(&self) -> usize {
        self.0.len()
    }
}
impl TAssetCapacity for GLTFBin {
    const ASSET_TYPE: &'static str = "RES_GLTF2_BIN";

    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 512 * 1024, max: 2 * 1024 * 1024, timeout: 1000 }
    }
}
impl<'a, G: Garbageer<Self>> AsyncLoader<'a, Self, Atom, G> for GLTFBin  {
	fn async_load(desc: Atom, result: LoadResult<'a, Self, G>) -> BoxFuture<'a, std::io::Result<Handle<Self>>> {
		Box::pin(async move { 
			match result {
				LoadResult::Ok(r) => Ok(r),
				LoadResult::Wait(f) => f.await,
				LoadResult::Receiver(recv) => {
					let file = pi_hal::file::load_from_url(&desc ).await;
					let file = match file {
						Ok(r) => r,
						Err(_e) =>  {
							log::warn!("load gltf bin fail: {:?}", desc.as_str());
							return Err(std::io::Error::new(std::io::ErrorKind::NotFound, ""));
						},
					};

                    recv.receive(desc.asset_u64(), Ok(GLTFBin(file))).await
				}
			}
		})
	}
}
impl GLTFBin {
    pub async fn load(path: &Atom, bin_assets: &ShareAssetMgr<GLTFBin>) -> Result<Handle<GLTFBin>, String> {

        let key = path.asset_u64();
        let result = AssetMgr::load(&bin_assets, &key);
        match result {
            LoadResult::Ok(res) => {
                return Ok(res);
            },
            _ => {
                match GLTFBin::async_load(path.clone(), result).await {
                    Ok(res) => Ok(res),
                    Err(_) => Err(String::from("Load Fail.")),
                }
            }
        }
    }
    pub async fn load_with_data(path: &Atom, bin_assets: &ShareAssetMgr<GLTFBin>, data: Vec<u8>) -> Result<Handle<GLTFBin>, String> {

        let key = path.asset_u64();
        let result = AssetMgr::load(&bin_assets, &key);
        match result {
            LoadResult::Ok(res) => {
                return Ok(res);
            },
            _ => {
                let loading = Box::pin(async move { 
                    match result {
                        LoadResult::Ok(r) => Ok(r),
                        LoadResult::Wait(f) => f.await,
                        LoadResult::Receiver(recv) => {
                            recv.receive(key, Ok(GLTFBin(data))).await
                        }
                    }
                }).await;

                match loading {
                    Ok(res) => Ok(res),
                    Err(_) => Err(String::from("Load Fail.")),
                }
            }
        }
    }
}

pub struct GLTFBase{
    gltf: Gltf,
    size: usize,
    buffers: Vec<Handle<GLTFBin>>,
    textures: Vec<Handle<ImageTexture>>,
}
impl pi_assets::asset::Asset for GLTFBase {
    type Key = u64;
    // const TYPE: &'static str = "GLTFBase";
}
impl pi_assets::asset::Size for GLTFBase {
    fn size(&self) -> usize {
        self.size
    }
}
impl AsRef<Gltf> for GLTFBase {
    fn as_ref(&self) -> &Gltf {
        &self.gltf
    }
}
impl TAssetCapacity for GLTFBase {
    const ASSET_TYPE: &'static str = "RES_GLTF2_FILE";

    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 256 * 1024, max: 512 * 1024, timeout: 1000 }
    }
}
impl GLTFBase {
    async fn load_buffers(gltf: &Gltf, base_path: &Atom, bin_assets: ShareAssetMgr<GLTFBin>) -> std::io::Result<Vec<Handle<GLTFBin>>> {
        let mut result = vec![];
        for buffer in gltf.buffers() {
            match buffer.source() {
                pi_gltf::buffer::Source::Bin => {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, ""));
                },
                pi_gltf::buffer::Source::Uri(path) => {
                    if path.starts_with("data:") {
                        if let Some(index) = path.find(',') {
                            
                            let mut path = String::from(base_path.as_str()) + "#";
                            path += buffer.index().to_string().as_str();
                            let path = Atom::from(path);
                        
                            let base64_buffer = path.split_at(index + 1).1;
                            let data = base64::decode(base64_buffer).unwrap();
                            match GLTFBin::load_with_data(&path, &bin_assets, data).await {
                                Ok(val) => {
                                    result.push(val);
                                },
                                Err(_e) => {
                                    return Err(std::io::Error::new(std::io::ErrorKind::NotFound, ""));
                                },
                            }
                        } else {
                            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Buffer Data Error."));
                        }
                    } else {
                        let path = relative_path(path, base_path.as_str());
                        let path = Atom::from(path);
                        match GLTFBin::load(&path, &bin_assets).await {
                            Ok(val) => {
                                result.push(val);
                            },
                            Err(_e) => {
                                return Err(std::io::Error::new(std::io::ErrorKind::NotFound, ""));
                            },
                        }
                    }
                },
            }
        }
        return Ok(result);
    }
}
pub struct GLTFBaseDesc {
    path: Atom,
    bin_assets: ShareAssetMgr<GLTFBin>
}
impl<'a, G: Garbageer<Self>> AsyncLoader<'a, Self, GLTFBaseDesc, G> for GLTFBase  {
	fn async_load(desc: GLTFBaseDesc, result: LoadResult<'a, Self, G>) -> BoxFuture<'a, std::io::Result<Handle<Self>>> {
		Box::pin(async move {
            let key_u64 = desc.path.asset_u64();
			match result {
				LoadResult::Ok(r) => Ok(r),
				LoadResult::Wait(f) => f.await,
				LoadResult::Receiver(recv) => {
					let file = pi_hal::file::load_from_url(&desc.path).await;
					let file = match file {
						Ok(r) => r,
						Err(_e) =>  {
							// log::debug!("load file fail: {:?}", desc.path.as_str());
							return Err(std::io::Error::new(std::io::ErrorKind::NotFound, ""));
						},
					};
                    
                    let gltf = match Gltf::from_slice(&file) {
                        Ok(gltf) => {
                            let buffers = match GLTFBase::load_buffers(&gltf, &desc.path, desc.bin_assets.clone()).await {
                                Ok(buffers) => buffers,
                                Err(e) => return Err(e),
                            };
                            let textures = vec![] ; // GLTFBase::load_images(&gltf, &desc.path, desc.textures_mgr.clone(), desc.device.clone(), desc.queue.clone()).await;
                            let mut size = 0;
                            buffers.iter().for_each(|val| { size += val.size(); });
                            // textures.iter().for_each(|val| { size += val.size(); });
                            GLTFBase { gltf, buffers, textures, size }
                        },
                        Err(_e) => {
                            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, ""));
                        },
                    };
                    let result = recv.receive(key_u64, Ok(gltf)).await;
					result
				}
			}
		})
	}
}


pub struct GLTF {
    pub textures: Vec<Handle<ImageTexture>>,
    pub vbs: Vec<Handle<AssetVertexBuffer>>,
    pub position: Vec<Handle<TypeFrameCurve<LocalPosition>>>,
    pub euler: Vec<Handle<TypeFrameCurve<LocalEulerAngles>>>,
    pub scaling: Vec<Handle<TypeFrameCurve<LocalScaling>>>,
    pub quaternion: Vec<Handle<TypeFrameCurve<LocalRotationQuaternion>>>,
    pub camerafov: Vec<Handle<TypeFrameCurve<CameraFov>>>,
    pub camerasize: Vec<Handle<TypeFrameCurve<CameraOrthSize>>>,
    pub enable: Vec<Handle<TypeFrameCurve<Enable>>>,
    pub indicerange_curves: Vec<Handle<TypeFrameCurve<IndiceRenderRange>>>,
    
    pub float: Vec<Handle<TypeFrameCurve<AnimatorableFloat>>>,
    pub vec2s: Vec<Handle<TypeFrameCurve<AnimatorableVec2>>>,
    pub vec3s: Vec<Handle<TypeFrameCurve<AnimatorableVec3>>>,
    pub vec4s: Vec<Handle<TypeFrameCurve<AnimatorableVec4>>>,
    pub uints: Vec<Handle<TypeFrameCurve<AnimatorableUint>>>,
    pub _ints: Vec<Handle<TypeFrameCurve<AnimatorableSint>>>,

    pub particlesys_calculators: XHashMap<usize, Handle<ParticleSystemCalculatorID>>,
    pub output: String,
    pub errors: Vec<ErrorGLTF>,
    pub animecount: usize,
    pub path: String,
    // pub base: Handle<GLTFBase>,
}
impl  GLTF {
    pub fn key_accessor(&self, index: usize) -> String {
        let path = self.path.clone() + "#";
        path + index.to_string().as_str()
    }
    pub fn key_particle_calculator(&self, index: usize) -> KeyParticleSystemCalculator {
        let path = self.path.clone() + "#";
        let key = Atom::from(path + index.to_string().as_str());

        key.asset_u64()
    }
    pub fn key_anime_curve(&self, group_index: usize, channel_index: usize) -> u64 {
        let mut path = self.path.clone() + "#";
        path += group_index.to_string().as_str();
        path += "#";
        path += channel_index.to_string().as_str();

        let key = Atom::from(path.as_str());

        let key = key.asset_u64();
        
        // log::debug!("Curve: id: {:?}, {:?}-{:?}", key, group_index, channel_index);

        key
    }
    pub fn new(_base: Handle<GLTFBase>, path: String) -> Self {
        Self {
            textures:               vec![],
            vbs:                    vec![],
            position:               vec![],
            euler:                  vec![],
            scaling:                vec![],
            quaternion:             vec![],
            camerafov:              vec![],
            camerasize:             vec![],
            enable:                 vec![],
            // boneoff_curves:         vec![],
            indicerange_curves:     vec![],
            
            float:                  vec![],
            vec2s:                  vec![],
            vec3s:                  vec![],
            vec4s:                  vec![],
            uints:                  vec![],
            _ints:                  vec![],
            particlesys_calculators: XHashMap::default(),
            output: String::from(""),
            errors: vec![],
            animecount: 0,
            path,
            // base
        }
    }
}
impl pi_assets::asset::Asset for GLTF {
    type Key = u64;
    // const TYPE: &'static str = "GLTF";
}
impl pi_assets::asset::Size for GLTF {
    fn size(&self) -> usize {
        100 * 1024
    }
}
impl TAssetCapacity for GLTF {
    const ASSET_TYPE: &'static str = "RES_GLTF2";

    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 64 * 1024, max: 128 * 1024, timeout: 100 }
    }
}
impl<'a, G: Garbageer<Self>> AsyncLoader<'a, Self, (GLTF, u64), G> for GLTF  {
	fn async_load(desc: (GLTF, u64), result: LoadResult<'a, Self, G>) -> BoxFuture<'a, std::io::Result<Handle<Self>>> {
		Box::pin(async move {
            let key_u64 = desc.1;
			match result {
				LoadResult::Ok(r) => Ok(r),
				LoadResult::Wait(f) => f.await,
				LoadResult::Receiver(recv) => {
                    let result = recv.receive(key_u64, Ok(desc.0)).await;
					result
				}
			}
		})
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyGLTF {
    pub base_url: KeyGLTFBase,
    pub dyn_desc: GLTFDynamicJson,
}
impl TAssetKeyU64 for KeyGLTF {
    fn asset_u64(&self) -> u64 {
        let mut hasher = DefaultHasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

pub type QueryKey = Entity;
pub type BufferID = usize;
pub type ImageID = usize;
pub type BufferViewID = usize;
pub type AccessorID = usize;

#[derive(Debug, Clone, Copy)]
pub enum ErrorGLTF {
    ErrorBuffer,
    ErrorAccessor,
    ErrorImage,
    ErrorGLTFLoad,
    ErrorGLTFParse,
    ErrorGLTFCache,
    ErrorVertexBuffer,
    ErrorAnimation,
}
impl ToString for ErrorGLTF {
    fn to_string(&self) -> String {
        match self {
            Self::ErrorBuffer => String::from("ErrorBuffer, "),
            Self::ErrorAccessor => String::from("ErrorAccessor, "),
            Self::ErrorImage => String::from("ErrorImage, "),
            Self::ErrorGLTFLoad => String::from("ErrorGLTFLoad, "),
            Self::ErrorGLTFParse => String::from("ErrorGLTFParse, "),
            Self::ErrorGLTFCache => String::from("ErrorGLTFCache, "),
            Self::ErrorVertexBuffer => String::from("ErrorVertexBuffer, "),
            Self::ErrorAnimation => String::from("ErrorAnimation, "),
        }
    }
}

pub struct GLTFTempLoaded {
    gltf: Handle<GLTFBase>,
    id: KeyGLTF,
    entity: QueryKey,
}
impl GLTFTempLoaded {
    pub fn new(id: KeyGLTF, base: Handle<GLTFBase>, entity: QueryKey) -> Self {
        Self { id, gltf: base, entity }
    }
    pub fn analy(
        gltf: Handle<GLTFBase>,
        base_url: Atom,
        commands: &mut Commands,
        vb_assets_mgr: &ShareAssetMgr<AssetVertexBuffer>, 
        vballocator: &mut VertexBufferAllocator3D,
        device: &RenderDevice,
        queue: &RenderQueue,
        anime_assets: &TypeAnimeAssetMgrs,
        particlesys_cmds: &mut ActionSetParticleSystem,
        particlesys_res: &mut ResourceParticleSystem,
    ) -> GLTF {
        let mut result = GLTF::new(gltf.clone(), base_url.to_string());
        // let basekey = self.id.base_url.to_string() + "#";

        // VertexBuffer
        for mesh in gltf.gltf.meshes() {
            mesh.primitives().for_each(|primitive| {
                // Indices Buffer
                if let Some(accessor) = primitive.indices() {
                    let key = result.key_accessor(accessor.index());
                    let indice_key = KeyVertexBuffer::from(key.as_str());
                    let indice_key_u64 = indice_key.asset_u64();
                    if let Some(buffer) = vb_assets_mgr.get(&indice_key_u64) {
                        result.vbs.push(buffer);
                    } else {
                        let view = accessor.view().unwrap();
                        if let Some(bufferdata) = gltf.buffers.get(accessor.view().unwrap().buffer().index()) {
                            let bufferdata = &bufferdata.0;
                            let start = view.offset() + accessor.offset();
                            let end = start + accessor.count() * accessor.size();
                            let data = &bufferdata[start..end];
                            if let Some(buffer) = vballocator.create_not_updatable_buffer_for_index(device, queue, data) {
                                if let Ok(buffer) = vb_assets_mgr.insert(indice_key_u64, buffer) {
                                    result.vbs.push(buffer);
                                }
                            }
                        }
                    };
                }

                // attributes - 未处理稀疏存储情况
                for (_semantic, accessor) in primitive.attributes() {
                    let key = result.key_accessor(accessor.index());
                    // log::error!("VB {:?}", key);
                    let indice_key = KeyVertexBuffer::from(key.as_str());
                    let indice_key_u64 = indice_key.asset_u64();
                    if let Some(buffer) = vb_assets_mgr.get(&indice_key_u64) {
                        result.vbs.push(buffer);
                    } else {
                        let view = accessor.view().unwrap();
                        if let Some(bufferdata) = gltf.buffers.get(accessor.view().unwrap().buffer().index()) {
                            let bufferdata = &bufferdata.0;
                            let start = view.offset() + accessor.offset();
                            let end = start + accessor.count() * accessor.size();
                            let data = &bufferdata[start..end];
                            if let Some(buffer) = vballocator.create_not_updatable_buffer(device, queue, data, None) {
                                if let Ok(buffer) = vb_assets_mgr.insert(indice_key_u64, buffer) {
                                    result.vbs.push(buffer);
                                }
                            }
                        }
                    };
                }
            });
        }

        // Animation Curve
        let mut index_group = 0;
        for animation in gltf.gltf.animations() {
            index_group += 1;
            let mut index_chanel = 0;
            for channel in animation.channels() {
                index_chanel += 1;
                let curve_key_u64 = result.key_anime_curve(index_group, index_chanel);

                let mut baseinterpolation = Some(channel.sampler().interpolation());

                let mut property_id: Option<EAnimePropertyType> = match channel.target().property() {
                    pi_gltf::animation::Property::Translation => Some(EAnimePropertyType::LocalPosition),
                    pi_gltf::animation::Property::Rotation => Some(EAnimePropertyType::LocalRotation),
                    pi_gltf::animation::Property::Scale => Some(EAnimePropertyType::LocalScaling),
                    pi_gltf::animation::Property::MorphTargetWeights => None,
                    pi_gltf::animation::Property::Pointer => None,
                };

                if let Some(extras) = channel.sampler().extras() {
                    if let Some(val) = extras.get("interpolation") {
                        baseinterpolation = interpolation_from_u8(val.as_u64().unwrap() as u8);
                    }
                }
                if let Some(extras) = channel.extras() {
                    if let Some(val) = extras.get("property") {
                        property_id = EAnimePropertyType::from_u8(val.as_u64().unwrap() as u8);
                    }
                }

                if let (Some(property_id), Some(mode)) = (property_id, baseinterpolation) {
                    if p3d_anime_curve_query(&anime_assets, curve_key_u64, property_id) == false {
                        let accessor = channel.sampler().input();
                        let view = accessor.view().unwrap();
                        if let Some(bufferdata) = gltf.buffers.get(accessor.view().unwrap().buffer().index()) {
                            let bufferdata = &bufferdata.0;
                            let start = view.offset() + accessor.offset();
                            let end = start + accessor.count() * accessor.size();
                            let times = bytemuck::try_cast_slice(&bufferdata[start..end]);

                            // let times = channel.reader(|buffer| {
                            //     match self.buffers.get(&buffer.index()) {
                            //         Some(val) => Some(val.as_slice()),
                            //         None => None,
                            //     }
                            // }).read_inputs().map(|v| v.collect::<Vec<f32>>());
            
                            let accessor = channel.sampler().output();
                            let view = accessor.view().unwrap();
                            if let Some(bufferdata) = gltf.buffers.get(accessor.view().unwrap().buffer().index()) {
                                let bufferdata = &bufferdata.0;
                                let start = view.offset() + accessor.offset();
                                let end = start + accessor.count() * accessor.size();
                                let values = bytemuck::try_cast_slice(&bufferdata[start..end]);
                                // let values = channel.reader(|buffer| {
                                //     match self.buffers.get(&buffer.index()) {
                                //         Some(val) => Some(val.as_slice()),
                                //         None => None,
                                //     }
                                // }).read_outputs().map(|v| v.collect::<Vec<f32>>());

                                // log::debug!("Curve: {:?}, {:?}, {:?}", curve_key_u64, property_id, mode);

                                let design_frame_per_second = 120;
                                if let (Ok(times), Ok(values)) = (times, values) {
                                    match property_id {
                                        EAnimePropertyType::LocalPosition => {
                                            let curve = curve_gltf::<3, LocalPosition>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.position.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.position.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::LocalRotation => {
                                            let curve = curve_gltf::<4, LocalRotationQuaternion>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.quaternion.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.quaternion.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::LocalScaling => {
                                            let curve = curve_gltf::<3, LocalScaling>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.scaling.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.scaling.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::CameraOrthSize => {
                                            let curve = curve_gltf::<1, CameraOrthSize>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.camerasize.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.camerasize.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::CameraFov => {
                                            let curve = curve_gltf::<1, CameraFov>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.camerafov.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.camerafov.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::Enable => {
                                            let curve = curve_gltf::<1, Enable>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.enable.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.enable.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::LocalEulerAngles => {
                                            let curve = curve_gltf::<3, LocalEulerAngles>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.euler.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.euler.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::Intensity => {
                                            // let curve = curve_gltf::<1, Intensity>(&times, &values, design_frame_per_second, mode);
                                        },
                                        EAnimePropertyType::CellId => {
                                            // let curve = curve_gltf::<1, CellId>(&times, &values, design_frame_per_second, mode);
                                        },
                                        EAnimePropertyType::IndicesRange => {
                                            let curve = curve_gltf::<2, IndiceRenderRange>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.indicerange_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.indicerange_curves.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::MainTexUScale => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::MainTexVScale => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::MainTexUOffset => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::MainTexVOffset => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::Alpha => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::MainColor => {
                                            let curve = curve_gltf::<3, AnimatorableVec3>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.vec3s.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.vec3s.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::LightDiffuse => {
                                            // let curve = curve_gltf::<3, Lightdiffuse>(&times, &values, design_frame_per_second, mode);
                                        },
                                        EAnimePropertyType::AlphaCutoff => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::OpacityTexUScale => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::OpacityTexVScale => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::OpacityTexUOffset => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::OpacityTexVOffset => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::MaskCutoff => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::MaskTexUScale => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::MaskTexVScale => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::MaskTexUOffset => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::MaskTexVOffset => {
                                            let curve = curve_gltf::<1, AnimatorableFloat>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.float.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.float.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::MainTexTilloff => {
                                            let curve = curve_gltf::<4, AnimatorableVec4>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.vec4s.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.vec4s.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::MaskTexTilloff => {
                                            let curve = curve_gltf::<4, AnimatorableVec4>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.vec4s.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.vec4s.push(curve);
                                            };
                                        },
                                        EAnimePropertyType::OpacityTexTilloff => {
                                            let curve = curve_gltf::<4, AnimatorableVec4>(&times, &values, design_frame_per_second, mode);
                                            if let Ok(curve) = anime_assets.vec4s.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                                result.vec4s.push(curve);
                                            };
                                        },
                                    }
                                } else {
                                    result.errors.push(ErrorGLTF::ErrorAnimation);
                                }
                            }
                        };
                    }
                } else {
                    result.errors.push(ErrorGLTF::ErrorAnimation);
                }
            }
        
            // log::debug!("channels: {:?}, ", index_chanel);
        }

        // ParticleSystemCalculator
        for node in gltf.gltf.nodes() {
            if let Some(extras) = node.extras() {
                if let Some(cfg) = extras.get("meshParticle") {
                    let index = node.index();
                    let cfg: IParticleSystemConfig = gltf_format_particle_cfg(cfg);
                    let key_u64 = result.key_particle_calculator(index);
                    let id = commands.spawn_empty().id();
                    particlesys_cmds.calculator.push(OpsCPUParticleCalculator::ops(id, cfg));
                    let res = ParticleSystemCalculatorID(id, 1024, particlesys_res.calculator_queue.queue());
                    if let Ok(res) = particlesys_res.calcultors.insert(key_u64, res) {
                        result.particlesys_calculators.insert(index, res);
                    } else {
                        particlesys_res.calculator_queue.queue().push(id);
                    }
                }
            }
        }

        result.textures = gltf.textures.clone();

        result
    }
}

#[derive(Resource)]
pub struct GLTFResLoader {
    // pub query_counter: QueryKey,
    pub wait: Share<SegQueue<(QueryKey, KeyGLTF)>>,
    pub waitmap: XHashSet<(QueryKey, KeyGLTF)>,
    /// 需要加载 GLTF 基础文件
    pub waitbase: Share<SegQueue<(QueryKey, KeyGLTF)>>,
    pub success: Share<SegQueue<QueryKey>>,
    pub fails: Share<SegQueue<QueryKey>>,
    pub basesuccess: Share<SegQueue<GLTFTempLoaded>>,
    pub basefail: Share<SegQueue<KeyGLTF>>,
    // pub bufferqueue: Share<SegQueue<GLTFBuffer>>,
    // pub imagequeue: Share<SegQueue<GLTFImage>>,
    pub errorqueue: Share<SegQueue<(KeyGLTF, ErrorGLTF)>>,
    pub fail_reason: XHashMap<KeyGLTF, Vec<ErrorGLTF>>,
    pub successed: XHashMap<QueryKey, Handle<GLTF>>,
    pub successed_temp: Share<SegQueue<Handle<GLTF>>>,
    pub failed: XHashMap<QueryKey, KeyGLTF>,
    // pub temp: XHashMap<KeyGLTF, GLTFTempLoaded>,
}
impl GLTFResLoader {
    pub fn new() -> Self {
        Self {
            // query_counter: 0,
            wait: Share::new(SegQueue::default()),
            waitmap: XHashSet::default(),
            waitbase: Share::new(SegQueue::default()),
            success: Share::new(SegQueue::default()),
            fails: Share::new(SegQueue::default()),
            basesuccess: Share::new(SegQueue::default()),
            basefail: Share::new(SegQueue::default()),
            // bufferqueue: Share::new(SegQueue::default()),
            // imagequeue: Share::new(SegQueue::default()),
            errorqueue: Share::new(SegQueue::default()),
            fail_reason: XHashMap::default(),
            // temp: XHashMap::default(),
            successed: XHashMap::default(),
            successed_temp: Share::new(SegQueue::default()),
            failed: XHashMap::default(),
        }
    }
    pub fn create_load(&self, key: QueryKey, param: KeyGLTF) {
        self.wait.push((key, param));
    }
    pub fn get_success(&mut self, key: QueryKey) -> Option<Handle<GLTF>> {
        self.successed.remove(&key)
    }
    pub fn get_fail_reason(&mut self, key: QueryKey) -> Option<String> {
        if let Some(key) = self.failed.remove(&key) {
            if let Some(errors) = self.fail_reason.get(&key) {
                let mut str = String::from("");
                errors.iter().for_each(|err| {
                    str += err.to_string().as_str();
                });
                Some(str)
            } else {
                Some(String::from("Unkown."))
            }
        } else {
            None
        }
    }
}

pub fn sys_load_gltf_launch(
    mut loader: ResMut<GLTFResLoader>,
    assets_mgr: Res<ShareAssetMgr<GLTF>>,
    _base_assets_mgr: Res<ShareAssetMgr<GLTFBase>>,
) {
    let mut waitagain = vec![];
    // log::warn!("Len: {:?}", loader.wait.len());
    let mut item = loader.wait.pop();
    while let Some((id, param)) = item {
        item = loader.wait.pop();

        let key_u64 = param.asset_u64();
        if let Some(res) = assets_mgr.get(&key_u64) {
            loader.success.push(id);
            loader.successed.insert(id, res);
            loader.waitmap.remove(&(id, param));
        } else if loader.fail_reason.contains_key(&param) {
            // log::error!("Failed: {:?}", id);
            loader.fails.push(id);
            loader.failed.insert(id, param.clone());
            loader.waitmap.remove(&(id, param));
        } else {
            let param = (id, param.clone());
            if loader.waitmap.contains(&param) == false {
                loader.waitmap.insert(param.clone());
                loader.waitbase.push(param.clone());
            }
            waitagain.push(param);

            // // 是否正在等待 buffer文件 、 图片
            // if loader.temp.contains_key(&param) == false {
            //     let base_key = param.base_url.clone();
            //     let base_key_u64 = base_key.asset_u64();
                // if let Some(base) = base_assets_mgr.get(&base_key_u64) {
                //     loader.temp.insert(param.clone(), GLTFTempLoaded::new(param, base));
                // } else {
                //     loader.waitbase.push(param);
                // }
            // }
        }
    }
    waitagain.drain(..).for_each(|item| {
        loader.wait.push(item);
    });

    let mut temp = loader.successed_temp.pop();
    while let Some(_) = temp {
        temp = loader.successed_temp.pop();
    }
}

pub fn sys_gltf_base_loaded_launch(
    loader: Res<GLTFResLoader>,
    base_assets_mgr: Res<ShareAssetMgr<GLTFBase>>,
    bin_assets_mgr: Res<ShareAssetMgr<GLTFBin>>,
) {
    let mut item = loader.waitbase.pop();
    while let Some((id, param)) = item {
        let base_key = param.base_url.clone();
        let base_key_u64 = base_key.asset_u64();
        if let Some(base) = base_assets_mgr.get(&base_key_u64) {
            // loader.temp.insert(param.clone(), GLTFTempLoaded::new(param, base));
            loader.basesuccess.push(GLTFTempLoaded::new(param, base, id));
        } else {
            let base_key = param.base_url.clone();
            let base_key_u64 = base_key.asset_u64();
            let basesuccess = loader.basesuccess.clone();
            let errorqueue = loader.errorqueue.clone();
            let result = AssetMgr::load(&base_assets_mgr, &base_key_u64);
            match result {
                LoadResult::Ok(data) => {
                    // log::warn!("Base: 1");
                    basesuccess.push(GLTFTempLoaded::new(param, data, id));
                },
                _ => {
                    let desc = GLTFBaseDesc{
                        path: param.base_url.clone(),
                        bin_assets: bin_assets_mgr.clone()
                    };
                    RENDER_RUNTIME
                    .spawn(async move {
                        match GLTFBase::async_load(desc, result).await {
                            Ok(data) => {
                                // log::warn!("Base: 2");
                                basesuccess.push(GLTFTempLoaded::new(param, data, id));
                            },
                            Err(_e) => {
                                // log::warn!("Base: 3");
                                errorqueue.push((param.clone(), ErrorGLTF::ErrorGLTFParse));
                            },
                        }
                    }).unwrap();
                },
            }
        }

        item = loader.waitbase.pop();
    }
}

pub fn sys_gltf_base_loaded_check(
    // mut loader: ResMut<GLTFResLoader>,
    // image_assets_mgr: Res<ShareAssetMgr<ImageTexture>>,
    // device: Res<PiRenderDevice>,
    // queue: Res<PiRenderQueue>,
) {
    // let mut item = loader.basesuccess.pop();
    // while let Some(param) = item {
    //     param.load_buffers(loader.bufferqueue.clone(), loader.errorqueue.clone());
    //     // param.load_images(loader.imagequeue.clone(), loader.errorqueue.clone(), &image_assets_mgr, device.clone(), queue.clone());

    //     loader.temp.insert(param.id.clone(), param);
    //     item = loader.basesuccess.pop();
    // }
}

pub fn sys_gltf_analy(
    mut commands: Commands,
    mut loader: ResMut<GLTFResLoader>,
    anime_assets: TypeAnimeAssetMgrs,
    mut vballocator: ResMut<VertexBufferAllocator3D>,
    mut particlesys: ActionSetParticleSystem,
    mut particlesys_res: ResourceParticleSystem,
    vb_assets_mgr: Res<ShareAssetMgr<AssetVertexBuffer>>,
    assets_mgr: Res<ShareAssetMgr<GLTF>>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
    mut _performance: ResMut<Performance>,
) {
    let mut base = loader.basesuccess.pop();
    while let Some(temp) = &base {
        let key_u64 = temp.id.asset_u64();
        let result = AssetMgr::load(&assets_mgr, &key_u64);
        // log::warn!("OK -1");
        match result {
            LoadResult::Ok(data) => {
                loader.successed_temp.push(data);
                // log::warn!("OK 0");
            },
            _ => {
                let fails = loader.fails.clone();
                let successed_temp = loader.successed_temp.clone();
                let res = GLTFTempLoaded::analy(temp.gltf.clone(), temp.id.base_url.clone(), &mut commands, &vb_assets_mgr, &mut vballocator, &device, &queue, &anime_assets, &mut particlesys, &mut particlesys_res);
                let result = GLTF::async_load((res, key_u64), result);
                let id = temp.entity;
                RENDER_RUNTIME
                .spawn(async move {
                    match result.await {
                        Ok(data) => {
                            // log::warn!("OK 1");
                            successed_temp.push(data);
                        },
                        Err(_) => {
                            // log::warn!("Fail 0");
                            fails.push(id);
                        },
                    }
                }).unwrap();
            },
        }

        base = loader.basesuccess.pop();
    }
    
    let mut item = loader.errorqueue.pop();
    while let Some(temp) = item {
        // log::error!("Error: {:?}", temp.1);

        if loader.fail_reason.get_mut(&temp.0).is_none() {
            loader.fail_reason.insert(temp.0.clone(), vec![]);
        }
        let record = loader.fail_reason.get_mut(&temp.0).unwrap();
        record.push(temp.1);

        item = loader.errorqueue.pop();
    }

    // performance.gltfanaly = (pi_time::Instant::now() - time0).as_micros() as u32;
}


fn relative_path(mut file_path: &str, mut dir: &str) -> String {
    let (file_path_len, dir_len) = (file_path.len(), dir.len());
    if file_path_len == 0 {
        return "".to_string();
    }
    // 不以 . 开头，就是绝对路径，直接返回
    // 目录为空字符串，直接返回
    // if &file_path[0..1] != "." || dir.len() == 0 {
    if dir.len() == 0 {
        return file_path.to_string();
    }

    let (mut i, mut j) = (0, dir_len as isize - 1);

    // 最后一个字符不是/，就代表dir不是目录，需要定位到目录
    if j >= 0 && &dir[j as usize..dir_len] != "/" {
        j = dir.rfind("/").map_or(-1, |r| r as isize);
    }

    while i < file_path_len {
        if &file_path[i..i + 1] != "." {
            break;
        }
        if let Some(r) = file_path.get(i + 1..i + 2) {
            // ./的情况
            if r == "/" {
                i += 2;
                break;
            }
        }

        if let Some(r) = file_path.get(i + 1..i + 3) {
            // ./的情况
            if r != "./" {
                break;
            }
        }
        // ../的情况
        i += 3;

        if j > 0 {
            j = dir[0..j as usize].rfind("/").map_or(-1, |r| r as isize);
        } else {
            j = -1;
        }
    }

    if i > 0 {
        file_path = &file_path[i..file_path_len];
    };

    if j < 0 {
        return file_path.to_string();
    }

    if j < dir_len as isize - 1 {
        dir = &dir[0..(j + 1) as usize];
    }

    return dir.to_string() + file_path;
}

#[test]
fn test() {
    let res = relative_path("a.png", "ac/ress/models/eff_01/eff_01.gltf");
    println!("{:?}", res);
}
