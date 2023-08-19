#![feature(box_into_inner)]


use pi_3d::{PluginBundleDefault};
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc};
use pi_assets::asset::Handle;
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{prelude::*, frame_time::PluginFrameTime};
use pi_gltf2_load::{GLTF, TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_node_materials::{prelude::*, PluginNodeMaterial, NodeMaterialBlocks};
use pi_particle_system::prelude::{ParticleSystemActionSet, OpsCPUParticleSystem};
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::{cube::*, ball::*, quad::PluginQuadBuilder};
use unlit_material::{PluginUnlitMaterial, command::*, shader::UnlitShader, effects::main_opacity::MainOpacityShader};

use std::sync::Arc;
use pi_async_rt::prelude::AsyncRuntime;
use pi_hal::{init_load_cb, runtime::MULTI_MEDIA_RUNTIME, on_load};


fn setup(
    mut commands: Commands,
    loader: Res<pi_gltf2_load::GLTFResLoader>,
) {
    let id = commands.spawn_empty().id();
    loader.wait.push((id, pi_gltf2_load::KeyGLTF { base_url: Atom::from("E:/Rust/PI/pi_3d/assets/gltf/m_mine_20101_1/m_mine_20101_1.gltf"), dyn_desc: Atom::from("")  }))
}

fn sys_load_check(
    mut loader: ResMut<pi_gltf2_load::GLTFResLoader>,
) {
    let mut item = loader.fails.pop();
    while let Some(param) = item {
        log::error!("Failed: {:?}, Error: {:?}", param, loader.get_fail_reason(param));
        item = loader.fails.pop();
    }
    let mut item = loader.success.pop();
    while let Some(param) = item {
        log::error!("Successed: {:?}, {:?}", param, loader.get_success(param).is_some());
        // log::error!("Successed: {:?}", param.1.errors.len());
        item = loader.success.pop();
    }
}

pub fn create_by_gltf(
    entity: Entity,
    gltf: Handle<GLTF>,
    mut commands: Commands,
    mut scenecmds: ActionSetScene,
    mut cameracmds: ActionSetCamera,
    mut transformcmds: ActionSetTransform,
    mut meshcmds: ActionSetMesh,
    mut geometrycmd: ActionSetGeometry,
    mut matcmds: ActionSetMaterial,
    mut final_render: ResMut<WindowRenderer>,
    mut renderercmds: ActionSetRenderer,
    mut particlesys_cmds: ParticleSystemActionSet,
    mut animegroupcmd: ActionSetAnimationGroup,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    roots: Query<&SceneID>,
) {
    if let Ok(scene) = roots.get(entity) {
        let scene = scene.0;

        let gltfvalue: &pi_gltf::Gltf = gltf.base.as_ref().as_ref();
        gltfvalue.nodes().for_each(|(nodeinfo)| {
            let node = commands.spawn_empty().id();
            if let Some(meshinfo) = nodeinfo.mesh() {
                meshinfo.primitives().for_each(|primitive| {
                    let mut attributes = vec![];
                    primitive.attributes().for_each(|(seg, accessor)| {
                        match seg {
                            pi_gltf::Semantic::Positions => {
                                attributes.push(VertexBufferDesc {
                                    key: KeyVertexBuffer::from(gltf.key_accessor(accessor.index()).as_str()),
                                    attrs: vec![VertexAttribute { kind: EVertexDataKind::Position, format: wgpu::VertexFormat::Float32x3 }],
                                    range: None, step_mode: wgpu::VertexStepMode::Vertex, kind: EInstanceKind::None,
                                });
                            },
                            pi_gltf::Semantic::Normals => {
                                attributes.push(VertexBufferDesc {
                                    key: KeyVertexBuffer::from(gltf.key_accessor(accessor.index()).as_str()),
                                    attrs: vec![VertexAttribute { kind: EVertexDataKind::Normal, format: wgpu::VertexFormat::Float32x3 }],
                                    range: None, step_mode: wgpu::VertexStepMode::Vertex, kind: EInstanceKind::None,
                                });
                            },
                            pi_gltf::Semantic::Tangents => {
                                attributes.push(VertexBufferDesc {
                                    key: KeyVertexBuffer::from(gltf.key_accessor(accessor.index()).as_str()),
                                    attrs: vec![VertexAttribute { kind: EVertexDataKind::Tangent, format: wgpu::VertexFormat::Float32x4 }],
                                    range: None, step_mode: wgpu::VertexStepMode::Vertex, kind: EInstanceKind::None,
                                });
                            },
                            pi_gltf::Semantic::Colors(slot) => {
                                if slot == 0 {
                                    attributes.push(VertexBufferDesc {
                                        key: KeyVertexBuffer::from(gltf.key_accessor(accessor.index()).as_str()),
                                        attrs: vec![VertexAttribute { kind: EVertexDataKind::Color4, format: wgpu::VertexFormat::Float32x4 }],
                                        range: None, step_mode: wgpu::VertexStepMode::Vertex, kind: EInstanceKind::None,
                                    });
                                }
                            },
                            pi_gltf::Semantic::TexCoords(slot) => {
                                let kind = match slot {
                                    0 => Some(EVertexDataKind::UV),
                                    1 => Some(EVertexDataKind::UV2),
                                    2 => Some(EVertexDataKind::UV3),
                                    3 => Some(EVertexDataKind::UV4),
                                    4 => Some(EVertexDataKind::UV5),
                                    5 => Some(EVertexDataKind::UV6),
                                    _ => None
                                };
                                if let Some(kind) = kind {
                                    attributes.push(VertexBufferDesc {
                                        key: KeyVertexBuffer::from(gltf.key_accessor(accessor.index()).as_str()),
                                        attrs: vec![VertexAttribute { kind, format: wgpu::VertexFormat::Float32x2 }],
                                        range: None, step_mode: wgpu::VertexStepMode::Vertex, kind: EInstanceKind::None,
                                    });
                                }
                            },
                            pi_gltf::Semantic::Joints(slot) => {
                                let kind = match slot {
                                    0 => Some(EVertexDataKind::MatricesIndices),
                                    1 => Some(EVertexDataKind::MatricesIndicesExtra),
                                    _ => None
                                };
                                if let Some(kind) = kind {
                                    attributes.push(VertexBufferDesc {
                                        key: KeyVertexBuffer::from(gltf.key_accessor(accessor.index()).as_str()),
                                        attrs: vec![VertexAttribute { kind, format: wgpu::VertexFormat::Float32x4 }],
                                        range: None, step_mode: wgpu::VertexStepMode::Vertex, kind: EInstanceKind::None,
                                    });
                                }
                            },
                            pi_gltf::Semantic::Weights(slot) => {
                                let kind = match slot {
                                    0 => Some(EVertexDataKind::MatricesWeights),
                                    1 => Some(EVertexDataKind::MatricesWeightsExtra),
                                    _ => None
                                };
                                if let Some(kind) = kind {
                                    attributes.push(VertexBufferDesc {
                                        key: KeyVertexBuffer::from(gltf.key_accessor(accessor.index()).as_str()),
                                        attrs: vec![VertexAttribute { kind, format: wgpu::VertexFormat::Float32x4 }],
                                        range: None, step_mode: wgpu::VertexStepMode::Vertex, kind: EInstanceKind::None,
                                    });
                                }
                            },
                            pi_gltf::Semantic::Extras(_) => {
                                
                            },
                        }
                    });

                    let indices = if let Some(accessor) = primitive.indices() {
                        match accessor.data_type() {
                            pi_gltf::accessor::DataType::U16 => Some(IndicesBufferDesc {
                                format: wgpu::IndexFormat::Uint16, buffer_range: None,
                                buffer: KeyVertexBuffer::from(gltf.key_accessor(accessor.index()).as_str()),
                            }),
                            pi_gltf::accessor::DataType::U32 => Some(IndicesBufferDesc {
                                format: wgpu::IndexFormat::Uint16, buffer_range: None,
                                buffer: KeyVertexBuffer::from(gltf.key_accessor(accessor.index()).as_str()),
                            }),
                            _ => None
                        }
                    } else { None };

                    if let Some(extras) = meshinfo.extras() {
                        if extras.get("meshParticle").is_some() {
                            attributes.push(VertexBufferDesc::instance_world_matrix());
                            attributes.push(VertexBufferDesc::instance_color());
                            attributes.push(VertexBufferDesc::instance_tilloff());

                            if let Some(calculator) = particlesys_cmds.calcultors.get(&gltf.key_particle_calculator(nodeinfo.index())) {
                                let trailmesh = commands.spawn_empty().id();
                                let trailgeo = commands.spawn_empty().id();
                                particlesys_cmds.particlesys_cmds.push(OpsCPUParticleSystem::ops(scene, node, trailmesh, trailgeo, calculator));
                            }
                        }
                    }

                    let geo = commands.spawn_empty().id();
                    geometrycmd.create.push(OpsGeomeryCreate::ops(node, geo, attributes, indices));

                    // let matinfo = primitive.material();
                    // matinfo.
                });

                meshcmds.create.push(OpsMeshCreation::ops(scene, node, String::from("")));
            } else {
                transformcmds.create.push(OpsTransformNode::ops(scene, node, String::from("")));
            };

        });
    }
}

pub trait AddEvent {
	// 添加事件， 该实现每帧清理一次
	fn add_frame_event<T: Event>(&mut self) -> &mut Self;
}

impl AddEvent for App {
	fn add_frame_event<T: Event>(&mut self) -> &mut Self {
		if !self.world.contains_resource::<Events<T>>() {
			self.init_resource::<Events<T>>()
				.add_system(Events::<T>::update_system);
		}
		self
	}
}

pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
        app.add_frame_event::<ComponentEvent<Changed<Layer>>>();
    }
}


#[path = "../base.rs"]
mod base;
pub fn main() {
    let mut app = base::test_plugins_with_gltf();
    
    app.add_plugin(PluginTest);
    
    app.add_startup_system(setup);
    app.add_system(sys_load_check);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

}