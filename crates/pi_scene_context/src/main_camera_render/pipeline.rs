
use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_ecs::{prelude::{Query, ResMut, Res}, query::{Write, Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::GameObject, assets::sync_load::{InterfaceAssetSyncCreate}};
use pi_render::rhi::{device::RenderDevice};
use pi_share::Share;
use render_shader::{shader::{ResShader, KeyShader}, scene_about_code::ERenderTag, skin_code::ESkinCode, instance_code::EInstanceCode};

use crate::{geometry::{AssetResVBLayouts, AssetKeyVBLayouts}, materials::{material::{MaterialID}, shader_effect::{AssetKeyShaderEffect, AssetResShaderEffectMeta}, uniforms::{value_uniform::MaterialValueBind, texture_uniform::MaterialTextureBindGroupID}, bind_group::{RenderBindGroupKey, RenderBindGroupPool}}, renderers::{pipeline::{KeyRenderPipeline, ResRenderPipeline, AssetResRenderPipeline, pipeline_state_key, render_pipeline}, render_blend::RenderBlend, render_depth_and_stencil::RenderDepthAndStencil, render_primitive::PrimitiveState, render_target_state::RenderTargetState, render_object::RenderObjectBindGroup}, main_camera_render::bind_group::IDMainCameraRenderBindGroup, meshes::model::BuildinModelBind};


pub struct AssetResShaderMainCamera{
    pub shader: Handle<ResShader>,
    pub shaderkey: KeyShader,
    pub material_bind_group: RenderBindGroupKey,
    pub material_bind_offet: Option<u32>,
    pub tex_bind_group: Option<RenderBindGroupKey>,
}
impl AssetResShaderMainCamera {
    pub fn renderobj_bind_group(&self, model: &BuildinModelBind, bind_groups: &mut Vec<RenderObjectBindGroup>, pool: &RenderBindGroupPool) -> bool {
        let mut resut = true;


        if let Some(bindgroup) = &self.tex_bind_group {
            if pool.get(bindgroup).unwrap().bind_group.is_some() {
                bind_groups.push(RenderObjectBindGroup {
                    bind_group: bindgroup.clone(),
                    offsets: vec![],
                });
            } else {
                resut = false;
            }
        }

        if resut {
            if let Some(offset) = self.material_bind_offet {
                bind_groups.push(RenderObjectBindGroup {
                    bind_group: self.material_bind_group.clone(),
                    offsets: vec![*model.bind_offset, offset],
                });

            } else {
                bind_groups.push(RenderObjectBindGroup {
                    bind_group: self.material_bind_group.clone(),
                    offsets: vec![*model.bind_offset],
                });
            }
        }

        resut
    }
}

pub struct SysMaterialMainCameraChangeByMesh;
#[setup]
impl SysMaterialMainCameraChangeByMesh {
    #[system]
    pub fn sys(
        mut meshes: Query<
            GameObject,
            (
                Write<AssetResShaderMainCamera>,
                &MaterialID, &AssetKeyVBLayouts, &AssetResVBLayouts, Option<&ESkinCode>, Option<&EInstanceCode>
            ),
            Or<(Changed<MaterialID>, Changed<AssetResVBLayouts>, Changed<ESkinCode>, Changed<EInstanceCode>)>
        >,
        materials: Query<
            GameObject,
            (
                &AssetKeyShaderEffect, &AssetResShaderEffectMeta, &MaterialValueBind, Option<&MaterialTextureBindGroupID>
            ),
        >,
        asset_mgr: Res<Share<AssetMgr<ResShader>>>,
        device: Res<RenderDevice>,
    ) {
        meshes.iter_mut().for_each(|(mut asset, matid, keyvb, vb, skin, instance)| {

            // println!("SysMaterialMainCameraChangeByMesh");

            let skin = if let Some(skin) = skin { skin.clone() } else { ESkinCode::None };
            let instance = if let Some(instance) = instance { instance.clone() } else { EInstanceCode(EInstanceCode::NONE) };
            let vertex_layouts_key = keyvb.0.clone();
            let render_tag = ERenderTag::MainCamera;

            if let Some((matkey, matmeta, valuebind, texbind)) = materials.get(matid.0) {

                let shaderkey = KeyShader { shader: matkey.0.clone(), defines_key: 0, skin_key: skin.clone(), vs_layouts: vertex_layouts_key, render_tag: render_tag.clone() };
    
                let shader = if let Some(shader) = asset_mgr.get(&shaderkey) {
                    shader
                } else {
                    asset_mgr.create_asset(shaderkey.clone(), ResShader::build(&device, &matkey.0, &matmeta.0, &vb.0, &instance, &skin, &render_tag))
                };

                if matmeta.textures.is_some() {
                    if let Some(texbind) = texbind {
                        asset.write(
                            AssetResShaderMainCamera {
                                shader,
                                shaderkey,
                                material_bind_group: RenderBindGroupKey::from(valuebind.bind_group.clone()),
                                material_bind_offet: valuebind.bind_offset(),
                                tex_bind_group: Some(texbind.0.clone()),
                            }
                        );
                    } else {
                        asset.remove();
                    }
                } else {
                    asset.write(
                        AssetResShaderMainCamera {
                            shader,
                            shaderkey,
                            material_bind_group: RenderBindGroupKey::from(valuebind.bind_group.clone()),
                            material_bind_offet: valuebind.bind_offset(),
                            tex_bind_group: None,
                        }
                    );
                }
            }
        });
    }
}

pub struct SysMaterialMainCameraChangeByMat;
#[setup]
impl SysMaterialMainCameraChangeByMat {
    #[system]
    pub fn sys(
        mut meshes: Query<
            GameObject,
            (
                Write<AssetResShaderMainCamera>,
                &MaterialID, &AssetKeyVBLayouts, &AssetResVBLayouts, Option<&ESkinCode>, Option<&EInstanceCode>
            ),
        >,
        materials: Query<
            GameObject,
            (
                &AssetKeyShaderEffect, &AssetResShaderEffectMeta, &MaterialValueBind, Option<&MaterialTextureBindGroupID>
            ),
            Or<(
                Changed<AssetResShaderEffectMeta>, Changed<MaterialValueBind>, Changed<MaterialTextureBindGroupID>
            )>
        >,
        mut asset_mgr: ResMut<Share<AssetMgr<ResShader>>>,
        device: Res<RenderDevice>,
    ) {
        meshes.iter_mut().for_each(|(mut asset, matid, keyvb, vb, skin, instance)| {
            let skin = if let Some(skin) = skin { skin.clone() } else { ESkinCode::None };
            let instance = if let Some(instance) = instance { instance.clone() } else { EInstanceCode(EInstanceCode::NONE) };
            let vertex_layouts_key = keyvb.0.clone();
            let render_tag = ERenderTag::MainCamera;

            if let Some((matkey, matmeta, valuebind, texbind)) = materials.get(matid.0) {
                // println!("SysMaterialMainCameraChangeByMat");

                let shaderkey = KeyShader { shader: matkey.0.clone(), defines_key: 0, skin_key: skin.clone(), vs_layouts: vertex_layouts_key, render_tag: render_tag.clone() };
    
                let shader = if let Some(shader) = asset_mgr.get(&shaderkey) {
                    shader
                } else {
                    asset_mgr.create_asset(shaderkey.clone(), ResShader::build(&device, &matkey.0, &matmeta.0, &vb.0, &instance, &skin, &render_tag))
                };

                if matmeta.textures.is_some() {
                    if let Some(texbind) = texbind {
                        asset.write(
                            AssetResShaderMainCamera {
                                shader,
                                shaderkey,
                                material_bind_group: RenderBindGroupKey::from(valuebind.bind_group.clone()),
                                material_bind_offet: valuebind.bind_offset(),
                                tex_bind_group: Some(texbind.0.clone()),
                            }
                        );
                        // println!("AssetResShaderMainCamera");
                    } else {
                        asset.remove();
                    }
                } else {
                    asset.write(
                        AssetResShaderMainCamera {
                            shader,
                            shaderkey,
                            material_bind_group: RenderBindGroupKey::from(valuebind.bind_group.clone()),
                            material_bind_offet: valuebind.bind_offset(),
                            tex_bind_group: None,
                        }
                    );
                    // println!("AssetResShaderMainCamera");
                }
            }
        });
    }
}


pub struct SysMainCameraPipeline;
#[setup]
impl SysMainCameraPipeline {
    #[system]
    pub fn tick(
        mut items: Query<
            GameObject,
            (
                &AssetResShaderMainCamera,
                &RenderBlend, &RenderDepthAndStencil, &PrimitiveState, &AssetResVBLayouts,
                Write<AssetResRenderPipeline>
            ),
            Or<(
                Changed<AssetResShaderMainCamera>, Changed<RenderBlend>, Changed<RenderDepthAndStencil>, Changed<PrimitiveState>, Changed<AssetResVBLayouts>
            )>
        >,
        device: Res<RenderDevice>,
        layoutpool: Res<RenderBindGroupPool>,
        asset_mgr: Res<Share<AssetMgr<ResRenderPipeline>>>,
    ) {
        items.iter_mut().for_each(|(mat, blend, depth_stencil, primitive, vblayouts, mut pipelinewrite )| {
                // println!("depth_stencil >>> {:?}", depth_stencil);
                let targets = RenderTargetState::color_target(blend);
                let primitive = primitive.state;
                let depth_stencil = depth_stencil.state();

                let key = KeyRenderPipeline { 
                    shader_key: mat.shaderkey.clone(),
                    state_key: pipeline_state_key(
                        targets.as_slice(),
                        &primitive,
                        &depth_stencil,
                        0, 8
                    )
                };

                let pipeline = if let Some(pipeline) = asset_mgr.get(&key) {
                    pipeline
                } else {
                    let mut bind_group_layouts = vec![];
                    let layout = layoutpool.get_layout(&RenderBindGroupKey::from(IDMainCameraRenderBindGroup::LABEL));
                    bind_group_layouts.push(layout.unwrap().value());
                    let layout = layoutpool.get_layout(&mat.material_bind_group);
                    bind_group_layouts.push(layout.unwrap().value());
                    if let Some(texbind) = &mat.tex_bind_group {
                    let layout = layoutpool.get_layout(texbind);
                        bind_group_layouts.push(layout.unwrap().value());
                    }
    
                    let pipeline = render_pipeline::<ResShader>(&mat.shader, &device, targets.as_slice(), depth_stencil, primitive, &vblayouts.0.layouts(), &bind_group_layouts);
                    asset_mgr.create_asset(key, pipeline)
                };

                pipelinewrite.write(AssetResRenderPipeline::from(pipeline));
        });
    }
}
