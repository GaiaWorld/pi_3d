

use std::{collections::HashSet, i32, mem::transmute, sync::{Arc, Mutex, OnceLock}};
use ahash::HashMap;
use crossbeam::queue::SegQueue;
use pi_async_rt::rt::serial::AsyncRuntimeBuilder;
use pi_bevy_ecs_extend::prelude::{Down, EntityTag, Layer, OrInitSingleRes, OrInitSingleResMut, Up};
use pi_bevy_render_plugin::{node::Node, NodeId, PiRenderDevice, PiRenderGraph, PiScreenTexture, RenderContext};
use pi_hal::font::sdf_gpu::create_indices;
use pi_mesh_builder::cube::CubeBuilder;
use pi_node_materials::prelude::{BlockMainTexture, BlockOpacity, DefaultShader};
use pi_null::Null;
use pi_particle_system::prelude::{ParticleCalculatorBase, ParticleIDs};
use pi_share::ShareRefCell;
use pi_spatial::quad_helper::intersects;
use pi_world::{
    insert::Insert, prelude::{App, Entity, IntoSystemConfigs, Plugin}, 
    world::World, event::ComponentRemoved, filter::{Changed, With}, 
    query::Query, schedule::{End, Last},
    single_res::{SingleRes, SingleResMut}, system_params::{Local, SystemParam}
};
use serde_json::json;
use std::io::Result;
use pi_ws::{connect::WsSocket, server::WebsocketListener, utils::{ChildProtocol, WsFrameType, WsSession}};
use futures::future::{BoxFuture, FutureExt, LocalBoxFuture};
use pi_tcp::{SocketConfig, SocketEvent,
        connect::TcpSocket,
        server::{PortsAdapterFactory, SocketListener}};
use json::JsonValue;
use wgpu::util::DeviceExt;
use wgpu::CommandEncoder;
use pi_scene_context::prelude::*;

use pi_bevy_render_plugin::spector::{send_cmd, sys_parse_cmd, CMDCalls, Cmd, SpectorNode, CMDS, SOCKETS};

pub const CMD_TRANSFORM: &'static str = "request_transform";
pub const CMD_MESHPASSES: &'static str = "request_meshpasses";
pub const CMD_CAMERA: &'static str = "request_camera";
pub const CMD_SELECT3D: &'static str = "request_select3d";

#[derive(Default)]
pub struct DisplayBoxs(pub XHashMap<Entity, Entity>);

pub struct PluginSpector3D;
impl Plugin for PluginSpector3D {
    fn build(&self, app: &mut App) {
        app.insert_resource(DisplayBoxs::default());
        let cmdscalls = app.world.get_single_res_mut::<CMDCalls>().unwrap();
        cmdscalls.cmdcalls.insert(String::from(CMD_TRANSFORM), cmd_request_transform);
        cmdscalls.cmdcalls.insert(String::from(CMD_MESHPASSES), cmd_request_meshpass);
        cmdscalls.cmdcalls.insert(String::from(CMD_CAMERA), cmd_request_camera);
        cmdscalls.cmdcalls.insert(String::from(CMD_SELECT3D), cmd_request_select3d);

        let entity = app.world.get_resource_mut::<SingleIDBaseDefaultMaterial>().unwrap().0;
        app.world.get_resource_mut::<ActionListMaterialCreate>().unwrap().push(OpsMaterialCreate::ops_with_matarray(entity, DefaultShader::KEY));
        app.world.get_resource_mut::<ActionListUniformVal>().unwrap().push(OpsUniformVal::ops(entity, EUniformVal::Float(Atom::from(BlockOpacity::KEY_ALPHA), 0.33)));
        app.world.get_resource_mut::<ActionListUniformVal>().unwrap().push(OpsUniformVal::ops(entity, EUniformVal::Vec3(Atom::from(BlockMainTexture::KEY_COLOR), 0.89, 0., 0.5)));
    }
}

/// cmd `request_transform`
fn cmd_request_transform(world: &mut World, connect: WsSocket<TcpSocket>, obj: json::object::Object) {
    match obj.get("payload") {
        Some(JsonValue::Number(select_node_id)) => {
            let entity: Entity = unsafe{ transmute::<_, Entity>(f64::from(select_node_id.clone())) };

            let mut style = serde_json::from_str::<serde_json::Value>("{}").unwrap();
            if let Ok(r) =  world.get_component::<LocalPosition>(entity) {
                    style["position"] = format!("[{:?},{:?},{:?}]", r.0.x, r.0.y, r.0.z).into();
            };
            
            if let Ok(r) =  world.get_component::<LocalScaling>(entity) {
                    style["scaling"] = format!("[{:?},{:?},{:?}]", r.0.x, r.0.y, r.0.z).into();
            };
            
            if let Ok(r) =  world.get_component::<LocalEulerAngles>(entity) {
                    style["rotation"] = format!("[{:?},{:?},{:?}]", r.0.x, r.0.y, r.0.z).into();
            };

            if let Ok(r) = world.get_component::<Enable>(entity) {
                    style["enable"] = r.bool().into();
            };

            if let Ok(r) = world.get_component::<GlobalEnable>(entity) {
                style["isEnabled"] = r.0.into();
            };
            if let Ok(r) = world.get_component::<Collider>(entity) {
                style["ColliderMin"] = r.minimum.to_string().into();
                style["ColliderMax"] = r.maximum.to_string().into();
                style["ColliderTreshold"] = r.intersection_treshold.into();
            };

            if let Ok(r) = world.get_component::<RenderQueueSortParam>(entity) {
                style["RenderGroup"] = r.group.to_string().into();
                style["RenderIndex"] = r.index.to_string().into();
            }

            if let Ok(gm) = world.get_component::<GlobalMatrix>(entity) {
                let vec = gm.xyz();
                style["globalPosition"] = format!("[{:?},{:?},{:?}]", vec.0, vec.1, vec.2).into();
                let gm = gm.matrix().clone();
                if let Ok(mut transform) = world.get_component_mut::<AbsoluteTransform>(entity) {
                    let mut tmpscl = Vector3::zeros();
                    let mut tmprot = Rotation3::identity();
                    let vec = transform.rotation_quaternion(&gm, &mut tmpscl, &mut tmprot);
                    style["globalRotation"] = format!("[{:?},{:?},{:?},{:?}]", vec.i, vec.j, vec.k, vec.w).into();
                    let vec = transform.scaling(&gm, &mut tmpscl, &mut tmprot);
                    style["globalScaling"] = format!("[{:?},{:?},{:?}]", vec.x, vec.y, vec.z).into();
                }
            }

            let msg = format!("{{\"cmd\": \"transform\", \"payload\": {} }}", style.to_string());
            if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
                log::error!("send error: {}", e);
            }
        },
        r => log::error!("cmd invalid: {:?}", r),
    };
}

/// cmd `request_meshpasses`
fn cmd_request_meshpass(world: &mut World, connect: WsSocket<TcpSocket>, obj: json::object::Object) {
    match obj.get("payload") {
        Some(JsonValue::Number(select_node_id)) => {
            let entity: Entity = unsafe{ transmute::<_, Entity>(f64::from(select_node_id.clone())) };

            let mut style = serde_json::from_str::<serde_json::Value>("{}").unwrap();
            match world.get_component::<PassIDs>(entity) {
                Ok(r) => {
                    let mut passinfos = serde_json::from_str::<serde_json::Value>("{}").unwrap();
                    let mut idx = 0;
                    r.clone().0.iter().for_each(|pass| {
                        let mut passinfo = serde_json::from_str::<serde_json::Value>("{}").unwrap();
                        let entity = *pass;
                        if let Ok(val) = world.get_component::<PassDraw>(entity) {
                            passinfo["Draw"] = val.0.into();
                        }
                        if let Ok(val) = world.get_component::<PassBindGroups>(entity) {
                            passinfo["BindGroups"] = val.val().is_some().into();
                        }
                        if let Ok(val) = world.get_component::<PassPipeline>(entity) {
                            passinfo["Pipeline"] = val.val().is_some().into();
                        }
                        if let Ok(val) = world.get_component::<PassShader>(entity) {
                            if let Some(val) = &val.0 {
                                passinfo["Shader"] = val.key().key_meta.as_str().into();
                            } else {
                                passinfo["Shader"] = "None".into();
                            }
                        }
                        
                        if let Ok(r) = world.get_component::<RenderState>(entity) {
                            style["Blend"] = r.blend.to_string().into();
                            style["DepthWrite"] = r.depth.depth_write.into();
                            style["DepthTest"] = format!("{:?}", r.depth.compare).into();
                            style["DepthBias"] = format!("{:?}", r.depth.bias).into();
                            style["Cull"] = format!("{:?}", r.primitive.cull).into();
                            style["FrontFace"] = format!("{:?}", r.primitive.frontface).into();
                            style["Polygon"] = format!("{:?}", r.primitive.polygon).into();
                            style["Topology"] = format!("{:?}", r.primitive.topology).into();
                            style["Stencil"] = format!("{:?}", r.stencil).into();
                        }
                        passinfos[idx.to_string()] = passinfo.into();
                        idx += 1;
                    });
                    style["PassInfos"] = passinfos.into();
                },
                Err(_) => {},
            };

            let msg = format!("{{\"cmd\": \"meshpasses\", \"payload\": {} }}", style.to_string());
            if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
                log::error!("send error: {}", e);
            }
        },
        r => log::error!("cmd invalid: {:?}", r),
    };
}


/// cmd `request_camera`
fn cmd_request_camera(world: &mut World, connect: WsSocket<TcpSocket>, obj: json::object::Object) {
    match obj.get("payload") {
        Some(JsonValue::Number(select_node_id)) => {
            let entity: Entity = unsafe{ transmute::<_, Entity>(f64::from(select_node_id.clone())) };

            let mut style = serde_json::from_str::<serde_json::Value>("{}").unwrap();
            if let Ok(val) = world.get_component::<CameraParam>(entity) {
                let mut info = serde_json::from_str::<serde_json::Value>("{}").unwrap();
                info["Fov"]     = val.fov.0.into();
                info["Near"]    = val.nearfar.0.into();
                info["Far"]     = val.nearfar.1.into();
                info["Size"]    = val.orth.0.into();

                style["Camera"] = info.into();
            };

            let msg = format!("{{\"cmd\": \"camera\", \"payload\": {} }}", style.to_string());
            if let Err(e) = connect.send(WsFrameType::Text, msg.as_bytes().to_vec()) {
                log::error!("send error: {}", e);
            }
        },
        r => log::error!("cmd invalid: {:?}", r),
    };
}

/// cmd `request-showbox`
fn cmd_request_select3d(world: &mut World, connect: WsSocket<TcpSocket>, obj: json::object::Object) {
    match obj.get("payload") {
        Some(JsonValue::Number(select_node_id)) => {
            let entity: Entity = unsafe{ transmute::<_, Entity>(f64::from(select_node_id.clone())) };
            if let Ok(sceneid) = world.get_component::<SceneID>(entity) {
                let sceneid = sceneid.0.clone();
                if let Ok(gm) = world.get_component::<GlobalMatrix>(entity) {
                    let pose = gm.matrix.clone();
                    let boxs = world.get_resource::<DisplayBoxs>().unwrap();
                    if boxs.0.get(&sceneid).is_none() {
                        let (vertices, indices) = (CubeBuilder::attrs_meta(), CubeBuilder::indices_meta());
                        let id_geo: Entity = world.spawn_empty_id();
                        let mesh = world.spawn_empty_id(); 
                        let state: MeshInstanceState = MeshInstanceState::default();
                        world.get_resource_mut::<ActionListTransformNodeParent>().unwrap().push(OpsTransformNodeParent::ops(mesh, sceneid));
                        world.get_resource_mut::<ActionListMeshCreate>().unwrap().push(OpsMeshCreation::ops(sceneid, mesh, state));
                        world.get_resource_mut::<ActionListGeometryCreate>().unwrap().push(OpsGeomeryCreate::ops(mesh, id_geo, vertices, indices));

                        // actions.mesh.depth_compare.push(OpsDepthCompare::ops(mesh, CompareFunction::LessEqual));
                        world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_01, EDepthState::Compare(CompareFunction::Always)));
                        world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_02, EDepthState::Compare(CompareFunction::Always)));
                        world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_03, EDepthState::Compare(CompareFunction::Always)));
                        world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_04, EDepthState::Compare(CompareFunction::Always)));
                        world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_05, EDepthState::Compare(CompareFunction::Always)));
                        world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_06, EDepthState::Compare(CompareFunction::Always)));
                        world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_07, EDepthState::Compare(CompareFunction::Always)));
                        world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_08, EDepthState::Compare(CompareFunction::Always)));

                        let defaultmat = world.get_resource::<SingleIDBaseDefaultMaterial>().unwrap().0;
                        world.get_resource_mut::<ActionListMaterialUse>().unwrap().push(OpsMaterialUse::ops(mesh, defaultmat, PassTag::PASS_TAG_06));
                        world.get_resource_mut::<ActionListMeshStateModify>().unwrap().push(OpsMeshStateModify::ops(mesh, EMeshStateModify::BoundingCullingMode(ECullingStrategy::None)));
                        world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_06, EDepthState::Write(false)));
                        world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::primitive_state(mesh, PassTag::PASS_TAG_06, EPrimitiveState::CPolygonMode(PolygonMode::Line)));
                        world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::render_queue(mesh, i32::MAX, i32::MAX));
                        let  mut blend = ModelBlend::one_one();
                        blend.combine();
                        world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::blend(mesh, PassTag::PASS_TAG_06, blend));

                        let boxs = world.get_resource_mut::<DisplayBoxs>().unwrap();
                        boxs.0.insert(sceneid, mesh);
                    }
                    let mesh = world.get_resource_mut::<DisplayBoxs>().unwrap().0.get(&sceneid).unwrap().clone();
                    world.get_resource_mut::<ActionListAbstractMeshPose>().unwrap().push(OpsAbstractMeshPose::ops(mesh, pose));
                    world.get_resource_mut::<ActionListNodeEnable>().unwrap().push(OpsNodeEnable::ops(mesh, true));
                    return;
                }

            };
        },
        r => log::error!("cmd invalid: {:?}", r),
    };

    // world.get_resource::<DisplayBoxs>().unwrap().0.clone().iter().for_each(|(k, source)| {
    //     world.get_resource_mut::<ActionListNodeEnable>().unwrap().push(OpsNodeEnable::ops(*source, false));
    // });
}
