use std::{hash::{Hash, Hasher}, str::FromStr};

use bevy::prelude::{Resource, ResMut};
use crossbeam::queue::SegQueue;
use pi_assets::{mgr::{LoadResult, Receiver, AssetMgr}, asset::{Garbageer, Handle}};
use pi_engine_shell::prelude::*;
use pi_gltf::Gltf;
use pi_hash::{XHashMap, XHashSet};
use pi_scene_context::prelude::*;
use pi_node_materials::prelude::*;
use pi_render::rhi::RenderQueue;
use pi_share::Share;
use pi_async::prelude::AsyncRuntime;
use pi_hal::{runtime::MULTI_MEDIA_RUNTIME, loader::AsyncLoader};

use crate::{TypeAnimeAssetMgrs, EAnimePropertyType, curve_gltf, p3d_anime_curve_query, interpolation_from_u8};

pub type KeyGLTFBase = Atom;
pub type GLTFJson = String;
pub type GLTFDynamicJson = Atom;

pub struct GLTFBase(Gltf, usize);
impl pi_assets::asset::Asset for GLTFBase {
    type Key = u64;
}
impl pi_assets::asset::Size for GLTFBase {
    fn size(&self) -> usize {
        self.1
    }
}
impl AsRef<Gltf> for GLTFBase {
    fn as_ref(&self) -> &Gltf {
        &self.0
    }
}
impl TAssetCapacity for GLTFBase {
    const ASSET_TYPE: &'static str = "RES_GLTF2_FILE";

    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 10 * 1024 * 1024, max: 20 * 1024 * 1024, timeout: 100 * 1000 }
    }
}

pub struct GLTF {
    pub textures: Vec<Handle<pi_render::renderer::texture::ImageTexture>>,
    pub vbs: Vec<Handle<pi_render::renderer::vertex_buffer::AssetVertexBuffer>>,
    pub position: Vec<Handle<TypeFrameCurve<LocalPosition>>>,
    pub euler: Vec<Handle<TypeFrameCurve<LocalEulerAngles>>>,
    pub scaling: Vec<Handle<TypeFrameCurve<LocalScaling>>>,
    pub quaternion: Vec<Handle<TypeFrameCurve<LocalRotationQuaternion>>>,
    pub maincolor_curves: Vec<Handle<TypeFrameCurve<MainColor>>>,
    pub alpha: Vec<Handle<TypeFrameCurve<Alpha>>>,
    pub alphacutoff: Vec<Handle<TypeFrameCurve<Cutoff>>>,
    pub maskcutoff_curves: Vec<Handle<TypeFrameCurve<MaskCutoff>>>,
    pub mainuoff_curves: Vec<Handle<TypeFrameCurve<MainTexUOffset>>>,
    pub mainuscl_curves: Vec<Handle<TypeFrameCurve<MainTexUScale>>>,
    pub mainvoff_curves: Vec<Handle<TypeFrameCurve<MainTexVOffset>>>,
    pub mainvscl_curves: Vec<Handle<TypeFrameCurve<MainTexVScale>>>,
    pub maskuoff_curves: Vec<Handle<TypeFrameCurve<MaskTexUOffset>>>,
    pub maskuscl_curves: Vec<Handle<TypeFrameCurve<MaskTexUScale>>>,
    pub maskvoff_curves: Vec<Handle<TypeFrameCurve<MaskTexVOffset>>>,
    pub maskvscl_curves: Vec<Handle<TypeFrameCurve<MaskTexVScale>>>,
    pub opacityuoff_curves: Vec<Handle<TypeFrameCurve<OpacityTexUOffset>>>,
    pub opacityuscl_curves: Vec<Handle<TypeFrameCurve<OpacityTexUScale>>>,
    pub opacityvoff_curves: Vec<Handle<TypeFrameCurve<OpacityTexVOffset>>>,
    pub opacityvscl_curves: Vec<Handle<TypeFrameCurve<OpacityTexVScale>>>,
    pub camerafov: Vec<Handle<TypeFrameCurve<CameraFov>>>,
    pub camerasize: Vec<Handle<TypeFrameCurve<CameraOrthSize>>>,
    pub enable: Vec<Handle<TypeFrameCurve<Enable>>>,
    pub lightdiffuse_curves: Vec<Handle<TypeFrameCurve<LightDiffuse>>>,
    pub boneoff_curves: Vec<Handle<TypeFrameCurve<InstanceBoneoffset>>>,
    pub indicerange_curves: Vec<Handle<TypeFrameCurve<IndiceRenderRange>>>,
    pub output: String,
    pub errors: Vec<ErrorGLTF>,
    pub animecount: usize,
}
impl  GLTF {
    pub fn new(base: Handle<GLTFBase>) -> Self {
        Self {
            textures:               vec![],
            vbs:                    vec![],
            position:        vec![],
            euler:      vec![],
            scaling:    vec![],
            quaternion:       vec![],
            maincolor_curves:       vec![],
            alpha:           vec![],
            alphacutoff:          vec![],
            maskcutoff_curves:      vec![],
            mainuoff_curves:        vec![],
            mainuscl_curves:        vec![],
            mainvoff_curves:        vec![],
            mainvscl_curves:        vec![],
            maskuoff_curves:        vec![],
            maskuscl_curves:        vec![],
            maskvoff_curves:        vec![],
            maskvscl_curves:        vec![],
            opacityuoff_curves:     vec![],
            opacityuscl_curves:     vec![],
            opacityvoff_curves:     vec![],
            opacityvscl_curves:     vec![],
            camerafov:             vec![],
            camerasize:         vec![],
            enable:        vec![],
            lightdiffuse_curves:    vec![],
            boneoff_curves:         vec![],
            indicerange_curves:     vec![],
            output: String::from(""),
            errors: vec![],
            animecount: 0,
        }
    }
}
impl pi_assets::asset::Asset for GLTF {
    type Key = u64;
}
impl pi_assets::asset::Size for GLTF {
    fn size(&self) -> usize {
        100 * 1024
    }
}
impl TAssetCapacity for GLTF {
    const ASSET_TYPE: &'static str = "RES_GLTF2";

    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 10 * 1024 * 1024, max: 20 * 1024 * 1024, timeout: 100 * 1000 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyGLTF {
    pub base_url: KeyGLTFBase,
    pub dyn_desc: GLTFDynamicJson,
}
impl TAssetKeyU64 for KeyGLTF {
    fn asset_u64(&self) -> u64 {
        let mut hasher = pi_hash::DefaultHasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

pub type QueryKey = Entity;
pub type BufferID = usize;
pub type ImageID = usize;
pub type BufferViewID = usize;
pub type AccessorID = usize;

pub struct GLTFBuffer {
    gltfid: KeyGLTF,
    bufferid: BufferID,
    data: Vec<u8>,
}

pub struct GLTFImage {
    gltfid: KeyGLTF,
    imageid: ImageID,
    data: Handle<ImageTexture>,
}

pub struct GLTFAccessorVB {
    gltfid: KeyGLTF,
    bufferid: KeyVertexBuffer,
    data: Vec<u8>,
}

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

pub struct GLTFTempLoaded {
    gltf: Handle<GLTFBase>,
    id: KeyGLTF,
    buffers: XHashMap<BufferID, Vec<u8>>,
    images: XHashMap<ImageID, Handle<ImageTexture>>,
}
impl GLTFTempLoaded {
    pub fn new(id: KeyGLTF, base: Handle<GLTFBase>) -> Self {
        Self { id, gltf: base, buffers: XHashMap::default(), images: XHashMap::default() }
    }
    pub fn is_all_temp_load_ok(&self) -> bool {
        let count = self.gltf.0.buffers().len();
        for idx in 0..count {
            if self.buffers.contains_key(&idx) == false {
                return false;
            }
        }
        
        let count = self.gltf.0.images().len();
        for idx in 0..count {
            if self.images.contains_key(&idx) == false {
                return false;
            }
        }

        return true;
    }
    pub fn load_buffers(&self, bufferqueue: Share<SegQueue<GLTFBuffer>>, errorqueue: Share<SegQueue<(KeyGLTF, ErrorGLTF)>>) {
        for buffer in self.gltf.0.buffers() {
            match buffer.source() {
                pi_gltf::buffer::Source::Bin => {
                    errorqueue.push((self.id.clone(), ErrorGLTF::ErrorBuffer));
                },
                pi_gltf::buffer::Source::Uri(path) => {
                    if path.starts_with("data:") {
                        if let Some(index) = path.find(',') {
                            let base64_buffer = path.split_at(index + 1).1;
                            let data = base64::decode(base64_buffer).unwrap();
                            bufferqueue.push(GLTFBuffer { gltfid: self.id.clone(), bufferid: index, data });
                        }
                    } else {
                        let index = buffer.index();
                        let key = self.id.clone();
                        let buffers = bufferqueue.clone();
                        let errors = errorqueue.clone();
                        let path = relative_path(path, self.id.base_url.as_str());
                        MULTI_MEDIA_RUNTIME
                        .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                            let result = pi_hal::file::load_from_url(&Atom::from(path)).await;
                            match result {
                                Ok(data) => {
                                    buffers.push(GLTFBuffer { gltfid: key, bufferid: index, data });
                                },
                                Err(_) => {
                                    errors.push((key, ErrorGLTF::ErrorBuffer));
                                },
                            }
                        })
                        .unwrap();
                    }
                },
            }
        }
    }
    pub fn load_images(
        &self, imagequeue: Share<SegQueue<GLTFImage>>, errorqueue: Share<SegQueue<(KeyGLTF, ErrorGLTF)>>,
        image_assets_mgr: &ShareAssetMgr<ImageTexture>, device: RenderDevice, queue: RenderQueue
    ) {
        for item in self.gltf.0.images() {
            let imageid = item.index();
            let gltfid = self.id.clone();
            match item.source() {
                pi_gltf::image::Source::View { view, mime_type } => {
                    
                },
                pi_gltf::image::Source::Uri { uri, mime_type } => {
                    let path = relative_path(uri, self.id.base_url.as_str()); 
                    let key = KeyImageTexture::File(Atom::from(path), true);
                    let imageresult = AssetMgr::load(image_assets_mgr, &key);
                    match imageresult {
                        LoadResult::Ok(res) => {
                            imagequeue.push(GLTFImage { gltfid: gltfid, imageid: imageid, data: res })
                        },
                        _ => {
                            let images = imagequeue.clone();
                            let errors = errorqueue.clone();
                            let device = device.clone();
                            let queue = queue.clone();
                            MULTI_MEDIA_RUNTIME
                                .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                                    let desc = ImageTexture2DDesc {
                                        url: key.clone(),
                                        device: device,
                                        queue: queue,
                                    };
            
                                    let result = ImageTexture::async_load(desc, imageresult).await;
                                    match result {
                                        Ok(res) => {
                                            images.push(GLTFImage { gltfid: gltfid, imageid: imageid, data: res })
                                        },
                                        Err(e) => {
                                            // 图片加载失败仍然可渲染, 使用默认图片, 因此不添加Error
                                            // log::error!("load image fail, {:?}", e);
                                            // errorqueue.push(String::from(key.as_str()));
                                            log::error!("load image fail,");
                                            errors.push((gltfid, ErrorGLTF::ErrorImage));
                                        }
                                    }
    
                                })
                                .unwrap();
                        }
                    }
                },
            }
        }
    }
    pub fn analy(
        self,
        vb_assets_mgr: &ShareAssetMgr<AssetVertexBuffer>, 
        vballocator: &mut VertexBufferAllocator3D,
        device: &RenderDevice,
        queue: &RenderQueue,
        anime_assets: &TypeAnimeAssetMgrs,
    ) -> GLTF {
        let mut result = GLTF::new(self.gltf.clone());
        let basekey = self.id.base_url.to_string() + "#";

        // VertexBuffer
        for mesh in self.gltf.0.meshes() {
            mesh.primitives().for_each(|primitive| {
                // Indices Buffer
                if let Some(accessor) = primitive.indices() {
                    let key = basekey.clone() + accessor.index().to_string().as_str();
                    let indice_key = KeyVertexBuffer::from(key.as_str());
                    let indice_key_u64 = indice_key.asset_u64();
                    if let Some(buffer) = vb_assets_mgr.get(&indice_key_u64) {
                        result.vbs.push(buffer);
                    } else {
                        let view = accessor.view().unwrap();
                        let bufferdata = self.buffers.get(&accessor.view().unwrap().buffer().index()).unwrap();
                        let start = view.offset() + accessor.offset();
                        let end = start + accessor.count() * accessor.size();
                        let data = &bufferdata[start..end];
                        if let Some(buffer) = vballocator.create_not_updatable_buffer_for_index(device, queue, data) {
                            if let Ok(buffer) = vb_assets_mgr.insert(indice_key_u64, buffer) {
                                result.vbs.push(buffer);
                            }
                        }
                    };

                }

                // attributes - 未处理稀疏存储情况
                for (semantic, accessor) in primitive.attributes() {
                    let key = basekey.clone() + accessor.index().to_string().as_str();
                    let indice_key = KeyVertexBuffer::from(key.as_str());
                    let indice_key_u64 = indice_key.asset_u64();
                    if let Some(buffer) = vb_assets_mgr.get(&indice_key_u64) {
                        result.vbs.push(buffer);
                    } else {
                        let view = accessor.view().unwrap();
                        let bufferdata = self.buffers.get(&accessor.view().unwrap().buffer().index()).unwrap();
                        let start = view.offset() + accessor.offset();
                        let end = start + accessor.count() * accessor.size();
                        let data = &bufferdata[start..end];
                        if let Some(buffer) = vballocator.create_not_updatable_buffer_for_index(device, queue, data) {
                            if let Ok(buffer) = vb_assets_mgr.insert(indice_key_u64, buffer) {
                                result.vbs.push(buffer);
                            }
                        }
                    };
                }
            });
        }

        // Animation Curve
        for animation in self.gltf.0.animations() {
            let mut group_key = basekey.clone() + animation.index().to_string().as_str();
            group_key += "#";
            let mut index_chanel = 0;
            for channel in animation.channels() {
                let curve_key = KeyVertexBuffer::from((group_key.clone() + index_chanel.to_string().as_str()).as_str());
                index_chanel += 1;
                let curve_key_u64 = curve_key.asset_u64();

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
                // log::warn!("interpolation: {:?}", baseinterpolation);

                if let (Some(property_id), Some(mode)) = (property_id, baseinterpolation) {
                    if p3d_anime_curve_query(&anime_assets, curve_key_u64, property_id) == false {
                        let accessor = channel.sampler().input();
                        let view = accessor.view().unwrap();
                        let bufferdata = self.buffers.get(&accessor.view().unwrap().buffer().index()).unwrap();
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
                        let bufferdata = self.buffers.get(&accessor.view().unwrap().buffer().index()).unwrap();
                        let start = view.offset() + accessor.offset();
                        let end = start + accessor.count() * accessor.size();
                        let values = bytemuck::try_cast_slice(&bufferdata[start..end]);
                        // let values = channel.reader(|buffer| {
                        //     match self.buffers.get(&buffer.index()) {
                        //         Some(val) => Some(val.as_slice()),
                        //         None => None,
                        //     }
                        // }).read_outputs().map(|v| v.collect::<Vec<f32>>());

                        let design_frame_per_second = 100;
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
                                EAnimePropertyType::MainTexUScale => {
                                    let curve = curve_gltf::<1, MainTexUScale>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.mainuscl_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.mainuscl_curves.push(curve);
                                    };
                                },
                                EAnimePropertyType::MainTexVScale => {
                                    let curve = curve_gltf::<1, MainTexVScale>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.mainvscl_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.mainvscl_curves.push(curve);
                                    };
                                },
                                EAnimePropertyType::MainTexUOffset => {
                                    let curve = curve_gltf::<1, MainTexUOffset>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.mainuoff_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.mainuoff_curves.push(curve);
                                    };
                                },
                                EAnimePropertyType::MainTexVOffset => {
                                    let curve = curve_gltf::<1, MainTexVOffset>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.mainvoff_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.mainvoff_curves.push(curve);
                                    };
                                },
                                EAnimePropertyType::Alpha => {
                                    let curve = curve_gltf::<1, Alpha>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.alpha.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.alpha.push(curve);
                                    };
                                },
                                EAnimePropertyType::MainColor => {
                                    let curve = curve_gltf::<3, MainColor>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.maincolor_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.maincolor_curves.push(curve);
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
                                EAnimePropertyType::LightDiffuse => {
                                    // let curve = curve_gltf::<3, Lightdiffuse>(&times, &values, design_frame_per_second, mode);
                                },
                                EAnimePropertyType::AlphaCutoff => {
                                    let curve = curve_gltf::<1, Cutoff>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.alphacutoff.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.alphacutoff.push(curve);
                                    };
                                },
                                EAnimePropertyType::CellId => {
                                    // let curve = curve_gltf::<1, CellId>(&times, &values, design_frame_per_second, mode);
                                },
                                EAnimePropertyType::OpacityTexUScale => {
                                    let curve = curve_gltf::<1, OpacityTexUScale>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.opacityuscl_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.opacityuscl_curves.push(curve);
                                    };
                                },
                                EAnimePropertyType::OpacityTexVScale => {
                                    let curve = curve_gltf::<1, OpacityTexVScale>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.opacityvscl_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.opacityvscl_curves.push(curve);
                                    };
                                },
                                EAnimePropertyType::OpacityTexUOffset => {
                                    let curve = curve_gltf::<1, OpacityTexUOffset>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.opacityuoff_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.opacityuoff_curves.push(curve);
                                    };
                                },
                                EAnimePropertyType::OpacityTexVOffset => {
                                    let curve = curve_gltf::<1, OpacityTexVOffset>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.opacityvoff_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.opacityvoff_curves.push(curve);
                                    };
                                },
                                EAnimePropertyType::MaskCutoff => {
                                    let curve = curve_gltf::<1, MaskCutoff>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.maskcutoff_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.maskcutoff_curves.push(curve);
                                    };
                                },
                                EAnimePropertyType::MaskTexUScale => {
                                    let curve = curve_gltf::<1, MaskTexUScale>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.maskuscl_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.maskuscl_curves.push(curve);
                                    };
                                },
                                EAnimePropertyType::MaskTexVScale => {
                                    let curve = curve_gltf::<1, MaskTexVScale>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.maskvscl_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.maskvscl_curves.push(curve);
                                    };
                                },
                                EAnimePropertyType::MaskTexUOffset => {
                                    let curve = curve_gltf::<1, MaskTexUOffset>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.maskuoff_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.maskuoff_curves.push(curve);
                                    };
                                },
                                EAnimePropertyType::MaskTexVOffset => {
                                    let curve = curve_gltf::<1, MaskTexVOffset>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.maskvoff_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.maskvoff_curves.push(curve);
                                    };
                                },
                                EAnimePropertyType::BoneOffset => {
                                    let curve = curve_gltf::<1, InstanceBoneoffset>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.boneoff_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.boneoff_curves.push(curve);
                                    };
                                },
                                EAnimePropertyType::IndicesRange => {
                                    let curve = curve_gltf::<2, IndiceRenderRange>(&times, &values, design_frame_per_second, mode);
                                    if let Ok(curve) = anime_assets.indicerange_curves.insert(curve_key_u64, TypeFrameCurve(curve)) {
                                        result.indicerange_curves.push(curve);
                                    };
                                },
                            }
                        } else {
                            result.errors.push(ErrorGLTF::ErrorAnimation);
                        }
                    }
                } else {
                    result.errors.push(ErrorGLTF::ErrorAnimation);
                }
            }
        }
        result
    }
}

#[derive(Resource)]
pub struct GLTFResLoader {
    // pub query_counter: QueryKey,
    pub wait: Share<SegQueue<(QueryKey, KeyGLTF)>>,
    /// 需要加载 GLTF 基础文件
    pub waitbase: Share<SegQueue<KeyGLTF>>,
    pub success: Share<SegQueue<(QueryKey, Handle<GLTF>)>>,
    pub basesuccess: Share<SegQueue<GLTFTempLoaded>>,
    pub basefail: Share<SegQueue<KeyGLTF>>,
    pub queue: Share<SegQueue<(KeyGLTF, Handle<GLTF>)>>,
    pub bufferqueue: Share<SegQueue<GLTFBuffer>>,
    pub imagequeue: Share<SegQueue<GLTFImage>>,
    pub failqueue: Share<SegQueue<KeyGLTF>>,
    pub errorqueue: Share<SegQueue<(KeyGLTF, ErrorGLTF)>>,
    pub fail: XHashSet<KeyGLTF>,
    pub temp: XHashMap<KeyGLTF, GLTFTempLoaded>,
}
impl GLTFResLoader {
    pub fn new() -> Self {
        Self {
            // query_counter: 0,
            wait: Share::new(SegQueue::default()),
            waitbase: Share::new(SegQueue::default()),
            success: Share::new(SegQueue::default()),
            basesuccess: Share::new(SegQueue::default()),
            basefail: Share::new(SegQueue::default()),
            queue: Share::new(SegQueue::default()),
            bufferqueue: Share::new(SegQueue::default()),
            imagequeue: Share::new(SegQueue::default()),
            failqueue: Share::new(SegQueue::default()),
            errorqueue: Share::new(SegQueue::default()),
            fail: XHashSet::default(),
            temp: XHashMap::default(),
        }
    }
}

pub fn sys_load_gltf_launch(
    mut loader: ResMut<GLTFResLoader>,
    assets_mgr: Res<ShareAssetMgr<GLTF>>,
    base_assets_mgr: Res<ShareAssetMgr<GLTFBase>>,
) {
    let mut waitagain = vec![];
    let mut item = loader.wait.pop();
    while let Some((id, param)) = item {
        item = loader.wait.pop();

        let key_u64 = param.asset_u64();
        if let Some(res) = assets_mgr.get(&key_u64) {
            loader.success.push((id, res));
        } else if loader.fail.contains(&param) {
            log::error!("Failed: {:?}", id);
        } else {
            waitagain.push((id, param.clone()));
            // 是否正在等待 buffer文件 、 图片
            if loader.temp.contains_key(&param) == false {
                let base_key = param.base_url.clone();
                let base_key_u64 = base_key.asset_u64();
                if let Some(base) = base_assets_mgr.get(&base_key_u64) {
                    loader.temp.insert(param.clone(), GLTFTempLoaded::new(param, base));
                } else {
                    loader.waitbase.push(param);
                }
            }
        }
    }
    waitagain.drain(..).for_each(|item| {
        loader.wait.push(item);
    });
}

pub fn sys_gltf_base_loaded_launch(
    mut loader: ResMut<GLTFResLoader>,
    base_assets_mgr: Res<ShareAssetMgr<GLTFBase>>,
) {
    let mut item = loader.waitbase.pop();
    while let Some(param) = item {
        let base_key = param.base_url.clone();
        let base_key_u64 = base_key.asset_u64();
        if let Some(base) = base_assets_mgr.get(&base_key_u64) {
            loader.temp.insert(param.clone(), GLTFTempLoaded::new(param, base));
        } else {
            let base_key = param.base_url.clone();
            let base_key_u64 = base_key.asset_u64();
            let basesuccess = loader.basesuccess.clone();
            let errorqueue = loader.errorqueue.clone();
            let assets_mgr = base_assets_mgr.clone();
            MULTI_MEDIA_RUNTIME
            .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                let result = pi_hal::file::load_from_url(&base_key).await;
                match result {
                    Ok(data) => {
                        match Gltf::from_slice(&data) {
                            Ok(gltf) => {
                                match assets_mgr.insert(base_key_u64, GLTFBase(gltf, data.len())) {
                                    Ok(base) => {
                                        basesuccess.push(GLTFTempLoaded::new(param, base));
                                    },
                                    Err(e) => {
                                        errorqueue.push((param.clone(), ErrorGLTF::ErrorGLTFCache));
                                    },
                                }
                            }
                            Err(e) => {
                                log::error!("{:?}", e);
                                errorqueue.push((param.clone(), ErrorGLTF::ErrorGLTFParse));
                            },
                        }
                        // basesuccess.push(GLTFBuffer { gltfid: desc.key, bufferid: index, data });
                    },
                    Err(_) => {
                        errorqueue.push((param.clone(), ErrorGLTF::ErrorGLTFLoad));
                    },
                }
            })
            .unwrap();
        }

        item = loader.waitbase.pop();
    }
}

pub fn sys_gltf_base_loaded_check(
    mut loader: ResMut<GLTFResLoader>,
    base_assets_mgr: Res<ShareAssetMgr<GLTFBase>>,
    image_assets_mgr: Res<ShareAssetMgr<ImageTexture>>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
) {
    let mut item = loader.basesuccess.pop();
    while let Some(param) = item {
        param.load_buffers(loader.bufferqueue.clone(), loader.errorqueue.clone());
        param.load_images(loader.imagequeue.clone(), loader.errorqueue.clone(), &image_assets_mgr, device.clone(), queue.clone());

        loader.temp.insert(param.id.clone(), param);
        item = loader.basesuccess.pop();
    }
}

pub fn sys_gltf_analy(
    mut loader: ResMut<GLTFResLoader>,
    anime_assets: TypeAnimeAssetMgrs,
    mut vballocator: ResMut<VertexBufferAllocator3D>,
    vb_assets_mgr: Res<ShareAssetMgr<AssetVertexBuffer>>,
    assets_mgr: Res<ShareAssetMgr<GLTF>>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
) {
    let mut dirtyids = vec![];
    let mut item = loader.bufferqueue.pop();
    while let Some(buffer) = item {
        if let Some(temp) = loader.temp.get_mut(&buffer.gltfid) {
            temp.buffers.insert(buffer.bufferid, buffer.data);
            dirtyids.push(buffer.gltfid);
        }
        item = loader.bufferqueue.pop();
    }
    let mut item = loader.imagequeue.pop();
    while let Some(image) = item {
        if let Some(temp) = loader.temp.get_mut(&image.gltfid) {
            temp.images.insert(image.imageid, image.data);
            dirtyids.push(image.gltfid);
        }
        item = loader.imagequeue.pop();
    }
    dirtyids.drain(..).for_each(|id| {
        let isok = if let Some(temp) = loader.temp.get(&id) {
            temp.is_all_temp_load_ok()
        } else {
            false
        };

        let key_u64 = id.asset_u64();
        if isok && assets_mgr.contains_key(&key_u64) == false {
            if let Some(temp) = loader.temp.remove(&id) {
                // analy(temp, loader.queue.clone(), &mut allocator, &vb_asset_mgr);
                let res = temp.analy(&vb_assets_mgr, &mut vballocator, &device, &queue, &anime_assets);
                assets_mgr.insert(key_u64, res);
            }
        }
    });
    
    let mut item = loader.errorqueue.pop();
    while let Some(temp) = item {
        log::error!("Error: {:?}", temp.1);
        loader.fail.insert(temp.0);
        item = loader.errorqueue.pop();
    }
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
