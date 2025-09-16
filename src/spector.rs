

use std::{i32, mem::transmute};
use pi_mesh_builder::cube::CubeBuilder;
use pi_node_materials::prelude::{BlockMainTexture, BlockOpacity, DefaultShader};

use pi_world::{
    prelude::{App, Entity, Plugin}, 
    world::World
};
use pi_ws::{connect::WsSocket, utils::{WsFrameType}};
use pi_tcp::{
        connect::TcpSocket,
    };
use json::JsonValue;
use pi_scene_context::prelude::*;

use pi_bevy_render_plugin::spector::{send_cmd, sys_parse_cmd, CMDCalls, CMDS, SOCKETS};
use crate::{DisplayBoxs, _request_camera, _request_meshpass, _request_select3d, _request_transform};

pub const CMD_TRANSFORM: &'static str = "request_transform";
pub const CMD_MESHPASSES: &'static str = "request_meshpasses";
pub const CMD_CAMERA: &'static str = "request_camera";
pub const CMD_SELECT3D: &'static str = "request_select3d";

pub struct PluginSpector3D;
impl Plugin for PluginSpector3D {
    fn build(&self, app: &mut App) {
        app.insert_resource(DisplayBoxs::default());
        let cmdscalls = app.world.get_single_res_mut::<CMDCalls>().unwrap();
        cmdscalls.cmdcalls.insert(String::from(CMD_TRANSFORM), cmd_request_transform);
        cmdscalls.cmdcalls.insert(String::from(CMD_MESHPASSES), cmd_request_meshpass);
        cmdscalls.cmdcalls.insert(String::from(CMD_CAMERA), cmd_request_camera);
        cmdscalls.cmdcalls.insert(String::from(CMD_SELECT3D), cmd_request_select3d);
    }
}

/// cmd `request_transform`
fn cmd_request_transform(world: &mut World, connect: WsSocket<TcpSocket>, obj: json::object::Object) {
    match obj.get("payload") {
        Some(JsonValue::Number(select_node_id)) => {
            let entity: Entity = unsafe{ transmute::<_, Entity>(f64::from(select_node_id.clone())) };

            let msg = _request_transform(world, entity);
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

            let msg = _request_meshpass(world, entity);
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

            let msg = _request_camera(world, entity);
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
            _request_select3d(world, entity);
        },
        r => log::error!("cmd invalid: {:?}", r),
    };

    // world.get_resource::<DisplayBoxs>().unwrap().0.clone().iter().for_each(|(k, source)| {
    //     world.get_resource_mut::<ActionListNodeEnable>().unwrap().push(OpsNodeEnable::ops(*source, false));
    // });
}