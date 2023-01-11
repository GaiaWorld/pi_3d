use crate::object::ObjectID;


pub struct SceneID01;
pub struct SceneID02;
pub struct SceneID03;
pub struct SceneID04;

pub struct SceneCameraID01;
pub struct SceneCameraID02;
pub struct SceneCameraID03;
pub struct SceneCameraID04;
pub struct SceneCameraID05;
pub struct SceneCameraID06;

pub struct CullingFlag(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SceneID(pub ObjectID);
#[derive(Debug)]
pub struct CameraID(pub usize);
