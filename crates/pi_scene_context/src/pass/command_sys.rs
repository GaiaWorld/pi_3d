use pi_scene_shell::prelude::*;

use crate::renderers::prelude::*;

use super::{command::*, pass_object::*};


pub fn sys_create_pass_object(
    mut cmds: ResMut<ActionListPassObject>,
    mut commands: Commands,
    models: Query<& PassIDs>,
    mut passes: Query<(&mut PassReset, &mut PassDrawDirty, &mut PassModelID, &mut PassMaterialID, &mut PassPipelineStateDirty, &mut PassBindGroupsDirty)>,
    // mut alter: Alter<(), (), PassObjBundle, ()>,
) {
    cmds.drain().for_each(|OpsPassObject(idmodel, idmaterial, pass)| {
        if let Ok(passid) = models.get(idmodel) {
            let id_pass = passid.0[pass.index()];

            if let Ok(mut comps) = passes.get_mut(id_pass) {
                *comps.0 = PassReset;
                *comps.1 = PassDrawDirty;
                *comps.2 = PassModelID(idmodel);
                *comps.3 = PassMaterialID(idmaterial);
                *comps.4 = PassPipelineStateDirty;
                *comps.5 = PassBindGroupsDirty;
            }

            // if let Some(mut cmd) = commands.get_entity(id_pass) {
                let bundle = ActionPassObject::reset();
                // log::warn!("Pass {:?}", (idmodel, pass, idmaterial, id_pass));
                commands.entity(id_pass).insert(bundle);
                // alter.alter(id_pass, bundle);
            // }
        }
    });
}

pub fn sys_act_pass_object(
    models: Query<&PassIDs>,
    mut items: Query<&mut RenderState>,
    mut primivite_cmds: ResMut<ActionListPrimitiveState>,
    mut depth_cmds: ResMut<ActionListDepthState>,
    mut blend_cmds: ResMut<ActionListBlend>,
    mut stencil_cmds: ResMut<ActionListStencilState>,
) {
    primivite_cmds.drain().for_each(|OpsPrimitiveState(entity, tag, cmd)| {
        if let Ok(passids) = models.get(entity) {
            let passid = passids.0[tag.index()];

            if let Ok(mut item) = items.get_mut(passid) {
                match cmd {
                    EPrimitiveState::CCullMode      (val) => item.primitive.cull = val ,
                    EPrimitiveState::CPolygonMode   (val) => item.primitive.polygon = val ,
                    EPrimitiveState::CFrontFace     (val) => item.primitive.frontface = val ,
                    EPrimitiveState::CUnClipDepth   (val) => item.primitive.unclip_depth = val ,
                    EPrimitiveState::Topology       (val) => item.primitive.topology = val ,
                }
            // } else {
            //     log::error!("Not Found RenderState {:?}", passid);
            }
        // } else {
        //     log::error!("Not Found Mesh {:?}", entity);
        }
    });

    depth_cmds.drain().for_each(|OpsDepthState(entity, tag, cmd)| {
        if let Ok(passids) = models.get(entity) {
            let passid = passids.0[tag.index()];

            if let Ok(mut item) = items.get_mut(passid) {
                match cmd {
                    EDepthState::Write(val)         => item.depth.depth_write = val,
                    EDepthState::Compare(val)   => item.depth.compare = val,
                    EDepthState::Bias(val)      => item.depth.bias = val,
                }
            }
        }
    });
    blend_cmds.drain().for_each(|cmd| {
        match cmd {
            OpsRenderBlend::Disable(_) => todo!(),
            OpsRenderBlend::Blend(entity, tag, value) => {
                if let Ok(passids) = models.get(entity) {
                    let passid = passids.0[tag.index()];
        
                    if let Ok(mut item) = items.get_mut(passid) {
                        item.blend = value;
                    }
                }
            },
        }
    });
    stencil_cmds.drain().for_each(|OpsStencilState(entity, tag, cmd)| {
        if let Ok(passids) = models.get(entity) {
            let passid = passids.0[tag.index()];

            if let Ok(mut item) = items.get_mut(passid) {
                match cmd {
                    EStencilState::Front(val)   => item.stencil.stencil_front = val,
                    EStencilState::Back(val)    => item.stencil.stencil_back = val,
                    EStencilState::Read(val)    => item.stencil.stencil_read = val,
                    EStencilState::Write(val)   => item.stencil.stencil_write = val,
                }
            }
        }
    });
}

pub type PassObjInitBundle = (
    PassModelID,
    PassMaterialID,
    PassGeometryID,
    PassRendererID,
    PassPipelineStateDirty,
    PassDrawDirty,
    RenderState,
    PassReset,
    PassBindGroupsDirty,
    PassFlagShader,
);

pub type PassObjBundle = (
        PassBindGroups,
        PassShader,
        PassPipeline,
        PassDraw,
);

pub struct ActionPassObject;
impl ActionPassObject {
    pub fn init(
        empty: Entity,
        idmodel: Entity,
        _idscene: Entity,
    ) -> PassObjInitBundle {
        (
            PassModelID(idmodel),
            PassMaterialID(empty),
            PassGeometryID(empty),
            PassRendererID(empty),
            PassPipelineStateDirty,
            PassDrawDirty,
            RenderState::default(),
            PassReset,
            PassBindGroupsDirty,
            PassFlagShader,
        )
    }
    pub fn reset(
    ) -> PassObjBundle {
            (
                PassBindGroups::default(),
                PassShader(None),
                PassPipeline(None),
                PassDraw::default(),
            )
    }
}
