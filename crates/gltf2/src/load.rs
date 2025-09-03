use std::hash::{Hash, Hasher};

use crossbeam::queue::SegQueue;
use pi_scene_shell::prelude::*;
use pi_futures::BoxFuture;
use pi_gltf::{animation::Interpolation, Gltf};
use pi_particle_system::prelude::{IParticleSystemConfig, ParticleSystemCalculatorID, OpsCPUParticleCalculator, KeyParticleSystemCalculator, ActionSetParticleSystem, ResourceParticleSystem};
use pi_scene_context::prelude::*;

use crate::{EAnimePropertyType, curve_gltf, p3d_anime_curve_query, interpolation_from_u8, particle_system::gltf_format_particle_cfg};

pub type KeyGLTFBase = Atom;
pub type GLTFJson = String;
pub type GLTFDynamicJson = Atom;

pub struct GLTFBaseLoader {
    pub fail: Share<SegQueue<(Atom, EError)>>,
    pub success: Share<SegQueue<(Atom, GLTFBase)>>,
    pub loading: XHashSet<Atom>,
    pub loaded: XHashMap<Atom, GLTFBase>,
    pub errors: XHashMap<Atom, EError>,
}
impl GLTFBaseLoader {
    pub fn new() -> Self {
        Self {
            fail: Share::new(SegQueue::default()),
            success: Share::new(SegQueue::default()),
            loading: XHashSet::default(),
            loaded: XHashMap::default(),
            errors: XHashMap::default(),
        }
    }
    pub fn load(&mut self, key: Atom) {
        if self.loading.contains(&key) || self.loaded.contains_key(&key) || self.errors.contains_key(&key) {
            return;
        }
        self.loading.insert(key.clone());
        let fail = self.fail.clone();
        let success = self.success.clone();
        RENDER_RUNTIME
        .spawn(async move {
            match pi_hal::file::load_from_url(&key).await {
                Ok(gltffile) => {
                    // let time0 = pi_time::Instant::now();
                    // let url = key.clone();
                    match Gltf::from_slice(&gltffile) {
                        Ok(gltf) => {
                            let mut buffers = vec![];
                            let mut haserror = false;
                            // log::error!("GLTF Parse: {:?}", (url.as_str(), (pi_time::Instant::now() - time0).as_micros() as u32));
                            for buffer in gltf.buffers() {
                                match buffer.source() {
                                    pi_gltf::buffer::Source::Bin => {
                                        haserror = true;
                                        fail.push((key.clone(), ErrorRecord::ERROR_GLTF_BIN_LOAD_FAIL));
                                    },
                                    pi_gltf::buffer::Source::Uri(bufferpath) => {
                                        if bufferpath.starts_with("data:") {
                                            // if let Some(index) = path.find(',') {
                                            //     let mut path = String::from(base_path.as_str()) + "#";
                                            //     path += buffer.index().to_string().as_str();
                                            //     let path = Atom::from(path);
                                            //     let base64_buffer = path.split_at(index + 1).1;
                                            //     let data = base64::decode(base64_buffer).unwrap();
                                            //     match GLTFBin::load_with_data(&path, &bin_assets, data).await {
                                            //         Ok(val) => {
                                            //             result.push(val);
                                            //         },
                                            //         Err(_e) => {
                                            //             return Err(std::io::Error::new(std::io::ErrorKind::NotFound, ""));
                                            //         },
                                            //     }
                                            // } else {
                                            //     return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Buffer Data Error."));
                                            // }
                                            // Err(std::io::Error::new(std::io::ErrorKind::InvalidData, ""));
                                            haserror = true;
                                            fail.push((key.clone(), ErrorRecord::ERROR_GLTF_BIN_LOAD_FAIL));
                                        } else {
                                            let bufferpath = relative_path(bufferpath, key.as_str());
                                            match pi_hal::file::load_from_url(&Atom::from(bufferpath) ).await {
                                                Ok(bufferfile) => {
                                                    buffers.push(bufferfile);
                                                },
                                                Err(_e) =>  {
                                                    haserror = true;
                                                    // log::warn!("load gltf bin fail: {:?}", desc.as_str());
                                                    fail.push((key.clone(), ErrorRecord::ERROR_GLTF_BIN_LOAD_FAIL));
                                                    // Err(std::io::Error::new(std::io::ErrorKind::NotFound, ""));
                                                },
                                            };
                                        }
                                    },
                                }
                                if haserror { break; }
                            };
                            if haserror == false {
                                let mut size = 0;
                                buffers.iter().for_each(|val| { size += val.len(); });
                                let result = GLTFBase { gltf, size, buffers };
                                success.push((key, result));
                            }
                        },
                        Err(_e) => {
                            fail.push((key, ErrorRecord::ERROR_GLTF_BUFFER));
                        }
                    };
                },
                Err(_e) =>  {
                    fail.push((key, ErrorRecord::ERROR_GLTF_GLTF_LOAD));
                },
            };
        }).unwrap();
    }
    pub fn check(&mut self) {
        while let Some((key, error)) = self.fail.pop() {
            self.loading.remove(&key);
            self.errors.insert(key, error);
        }
        while let Some((key, gltf)) = self.success.pop() {
            self.loading.remove(&key);
            self.loaded.insert(key, gltf);
        }
    }
}

#[derive(Clone)]
pub struct GLTFBase{
    pub gltf: Gltf,
    pub size: usize,
    pub buffers: Vec<Share<Vec<u8>>>,
}

pub struct GLTF {
    pub textures: Vec<Handle<ResImageTexture>>,
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
    pub errors: Vec<EError>,
    pub animecount: usize,
    pub path: String,
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

        key
    }
    pub fn new(path: String) -> Self {
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
        }
    }
}
impl pi_assets::asset::Asset for GLTF {
    type Key = u64;
    // const TYPE: &'static str = "GLTF";
}
impl pi_assets::asset::Size for GLTF {
    fn size(&self) -> usize {
        self.textures               .capacity() * 24 +
        self.vbs                    .capacity() * 24 +
        self.position               .capacity() * 24 +
        self.euler                  .capacity() * 24 +
        self.scaling                .capacity() * 24 +
        self.quaternion             .capacity() * 24 +
        self.camerafov              .capacity() * 24 +
        self.camerasize             .capacity() * 24 +
        self.enable                 .capacity() * 24 +
        self.indicerange_curves     .capacity() * 24 +
        self.float                  .capacity() * 24 +
        self.vec2s                  .capacity() * 24 +
        self.vec3s                  .capacity() * 24 +
        self.vec4s                  .capacity() * 24 +
        self.uints                  .capacity() * 24 +
        self._ints                  .capacity() * 24 +
        0
    }
}
impl TAssetCapacity for GLTF {
    const ASSET_TYPE: &'static str = "RES_GLTF2";

    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 64 * 1024, max: 1, timeout: 100 }
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct KeyGLTF {
    pub base_url: KeyGLTFBase,
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

pub struct GLTFTempLoaded;
impl GLTFTempLoaded {
    pub fn analy(
        gltf: GLTFBase,
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
        // let time0 = pi_time::Instant::now();
        let mut result = GLTF::new(base_url.to_string());
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
                            let bufferdata = &bufferdata;
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
                            let bufferdata = &bufferdata;
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
                let accessor = channel.sampler().input();
                let offset = accessor.offset();
                let count = accessor.count();

                if let (Some(property_id), Some(mode)) = (property_id, baseinterpolation) {
                    if p3d_anime_curve_query(&anime_assets, curve_key_u64, property_id) == false {
                        let view = accessor.view().unwrap();
                        if let Some(time_bufferdata) = gltf.buffers.get(accessor.view().unwrap().buffer().index()) {
                            let start = view.offset() + offset;
                            let end = start + count * accessor.size();
                            let time_range = (start, end);
            
                            let accessor = channel.sampler().output();
                            let view = accessor.view().unwrap();
                            let offset = accessor.offset();
                            let count = accessor.count();
                            if let Some(value_bufferdata) = gltf.buffers.get(accessor.view().unwrap().buffer().index()) {
                                let start = view.offset() + offset;
                                let end = start + count * accessor.size();
                                let value_range = (start, end);

                                let design_frame_per_second = 120;
                                Self::animation_curve(&mut result, time_bufferdata, value_bufferdata, time_range, value_range, property_id, mode, anime_assets, design_frame_per_second, curve_key_u64);
                            }
                        };
                    }
                } else {
                    result.errors.push(ErrorRecord::ERROR_GLTF_ANIMATION);
                }
            }
        
            // log::debug!("channels: {:?}, ", index_chanel);
        }

        // for animation in gltf.gltf.pianimations() {
        //     index_group += 1;
        //     let mut index_chanel = 0;
        //     if let Some(channels) = animation.channels() {
        //         for channel in channels {
        //             index_chanel += 1;
        //             let curve_key_u64 = result.key_anime_curve(index_group, index_chanel);
        //             let property_id = EAnimePropertyType::from_u8(channel.0 as u8);
        //             let baseinterpolation = interpolation_from_u8(channel.10 as u8);
        //             if let (Some(property_id), Some(mode)) = (property_id, baseinterpolation) {
        //                 if p3d_anime_curve_query(&anime_assets, curve_key_u64, property_id) == false {
        //                     if let Some(bufferdata) = gltf.buffers.get(animation.buffer()) {
        //                         let time_bufferdata = bufferdata;
        //                         let value_bufferdata = bufferdata;

        //                         let offset = channel.2 as usize;
        //                         let count = channel.3 as usize;
        //                         let start = channel.4 as usize + offset;
        //                         let end = start + count * 4;
        //                         let time_range = (start, end);

        //                         let offset = channel.6 as usize;
        //                         let count = channel.7 as usize;
        //                         let start = channel.8 as usize + offset;
        //                         let end = start + count * property_id.size();
        //                         let value_range = (start, end);

        //                         let design_frame_per_second = 120;
        //                         Self::animation_curve(&mut result, time_bufferdata, value_bufferdata, time_range, value_range, property_id, mode, anime_assets, design_frame_per_second, curve_key_u64);
        //                     }
        //                 }
        //             } else {
        //                 log::error!("Not Prop {:?}", (channel, baseinterpolation.is_some()));
        //             }
        //         }
        //     }
        // }

        // ParticleSystemCalculator
        for node in gltf.gltf.nodes() {
            if let Some(extras) = node.extras() {
                if let Some(cfg) = extras.get("meshParticle") {
                    let index = node.index();
                    let cfg: IParticleSystemConfig = gltf_format_particle_cfg(cfg);
                    let key_u64 = result.key_particle_calculator(index);
                    let id = commands.spawn_empty_id();
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

        // result.textures = gltf.textures.clone();

        // log::error!("GLTF Analy: {:?}", (base_url.to_string(), (pi_time::Instant::now() - time0).as_micros() as u32));

        result
    }
    fn animation_curve(
        result: &mut GLTF,
        time_bufferdata: &Share<Vec<u8>>,
        value_bufferdata: &Share<Vec<u8>>,
        time_range: (usize, usize),
        value_range: (usize, usize),
        property_id: EAnimePropertyType,
        mode: Interpolation,
        anime_assets: &TypeAnimeAssetMgrs,
        design_frame_per_second: FramePerSecond,
        curve_key_u64: u64,
    ) {
        let times = bytemuck::try_cast_slice(&time_bufferdata[time_range.0..time_range.1]);
        let values = bytemuck::try_cast_slice(&value_bufferdata[value_range.0..value_range.1]);
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
            result.errors.push(ErrorRecord::ERROR_GLTF_ANIMATION);
        }
    }
}

#[derive(Resource)]
pub struct GLTFResLoader {
    pub waiting: SegQueue<(QueryKey, Atom)>,
    pub querys: XHashMap<Atom, Vec<QueryKey>>,
    pub loaded: XHashMap<Atom, Handle<GLTF>>,
    pub errors: XHashMap<Atom, EError>,
    pub successed: XHashMap<QueryKey, Handle<GLTF>>,
    pub failed: XHashMap<QueryKey, EError>,
    pub baseloader: GLTFBaseLoader,
    pub successquerys: SegQueue<QueryKey>,
    pub failquerys: SegQueue<QueryKey>,
}
impl MemSize for GLTFResLoader {
    fn memsize(&self) -> usize {
        self.waiting.len() * 16 + 256
        + self.querys.capacity() * 24
        + self.loaded.capacity() * 16
        + self.errors.capacity() * 12
        + self.successed.capacity() * 16
        + self.failed.capacity() * 16
        + self.successquerys.len() * 16 + 256
        + self.failquerys.len() * 16 + 256
    }
}
impl GLTFResLoader {
    pub fn new() -> Self {
        Self {
            // query_counter: 0,
            waiting: SegQueue::default(),
            querys: XHashMap::default(),
            loaded: XHashMap::default(),
            errors: XHashMap::default(),
            successed: XHashMap::default(),
            failed: XHashMap::default(),
            baseloader: GLTFBaseLoader::new(),
            successquerys: SegQueue::default(),
            failquerys: SegQueue::default(),
        }
    }
    pub fn create_load(&self, key: QueryKey, param: Atom) {
        self.waiting.push((key, param));
    }
    pub fn load(
        &mut self,
        gltfassets: &ShareAssetMgr<GLTF>,
    ) {
        while let Some((query, key)) = self.waiting.pop() {
            let key_u64 = key.asset_u64();
            if let Some(gltf) = gltfassets.get(&key_u64) {
                self.successed.insert(query, gltf);
                self.successquerys.push(query);
            } else {
                self.baseloader.load(key.clone());
                if self.querys.contains_key(&key) == false {
                    self.querys.insert(key.clone(), vec![]);
                }
                self.querys.get_mut(&key).unwrap().push(query);
            }
        }
    }
    pub fn check(
        &mut self,
        commands: &mut Commands,
        vb_assets_mgr: &ShareAssetMgr<AssetVertexBuffer>, 
        vballocator: &mut VertexBufferAllocator3D,
        device: &RenderDevice,
        queue: &RenderQueue,
        anime_assets: &TypeAnimeAssetMgrs,
        particlesys_cmds: &mut ActionSetParticleSystem,
        particlesys_res: &mut ResourceParticleSystem,
        gltfassets: &ShareAssetMgr<GLTF>,
    ) {
        self.baseloader.check();
        self.baseloader.loaded.drain().for_each(|(key, gltfbase)|{
            let gltf = GLTFTempLoaded::analy(gltfbase, key.clone(), commands, vb_assets_mgr, vballocator, device, queue, anime_assets, particlesys_cmds, particlesys_res);
            let key_u64 = key.asset_u64();
            if let Ok(gltf) = gltfassets.insert(key_u64, gltf) {
                self.loaded.insert(key, gltf);
            }
        });
        self.baseloader.errors.drain().for_each(|(key, error)| {
            self.errors.insert(key, error);
        });

        self.loaded.drain().for_each(|(key, gltf)| {
            if let Some(mut querys) = self.querys.remove(&key) {
                querys.drain(..).for_each(|query| {
                    self.successed.insert(query, gltf.clone());
                    self.successquerys.push(query);
                });
            }
        });
        self.errors.drain().for_each(|(key, error)| {
            if let Some(mut querys) = self.querys.remove(&key) {
                querys.drain(..).for_each(|query| {
                    self.failed.insert(query, error);
                    self.failquerys.push(query);
                });
            }
        });
    }
    pub fn get_success(&mut self, key: QueryKey) -> Option<Handle<GLTF>> {
        self.successed.remove(&key)
    }
    pub fn get_fail_reason(&mut self, key: QueryKey) -> Option<String> {
        if let Some(err) = self.failed.remove(&key) {
            Some(err.to_string())
        } else {
            None
        }
    }
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
    mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_gltf_analy"));
    if performance.debug { performance.t_gltfanaly = pi_time::Instant::now(); }

    loader.load(&assets_mgr);
    loader.check(&mut commands, &vb_assets_mgr, &mut vballocator, &device, &queue, &anime_assets, &mut particlesys, &mut particlesys_res, &assets_mgr);

    if performance.debug { performance.gltfanaly = (pi_time::Instant::now() - performance.t_gltfanaly).as_micros() as u32; }
}


pub fn relative_path(mut file_path: &str, mut dir: &str) -> String {
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
