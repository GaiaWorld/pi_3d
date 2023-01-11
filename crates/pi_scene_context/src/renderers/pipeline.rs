use derive_deref::{Deref, DerefMut};
use pi_assets::asset::{Handle, Asset};
use pi_ecs::{prelude::{ResMut, Query, Setup, Commands}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::{engine_shell::EnginShell, plugin::Plugin, object::{ObjectID, GameObject}, assets::sync_load::{PluginAssetSyncLoad, AssetSyncLoad}, run_stage::{TSystemStageInfo, ERunStageChap}};
use pi_render::rhi::{device::RenderDevice, pipeline::RenderPipeline};
use render_pipeline_key::{pipeline_key::{PipelineStateKeyCalcolator, PipelineStateKey, gen_pipeline_key}, fragment_state::gen_fragment_state_key};
use render_shader::shader::KeyShader;

use crate::shaders::TRenderShader;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyRenderPipeline {
    pub shader_key: KeyShader,
    pub state_key: render_pipeline_key::pipeline_key::PipelineStateKey,
}

/// A [`RenderPipeline`] represents a graphics pipeline and its stages (shaders), bindings and vertex buffers.
///
/// May be converted from and dereferences to a wgpu [`RenderPipeline`](wgpu::RenderPipeline).
/// Can be created via [`RenderDevice::create_render_pipeline`](crate::renderer::RenderDevice::create_render_pipeline).
#[derive(Debug)]
pub struct ResRenderPipeline(pub RenderPipeline);
impl Asset for ResRenderPipeline {
    type Key = KeyRenderPipeline;

    fn size(&self) -> usize {
        256
    }
}

#[derive(Debug)]
enum ECommand {
    Use(ObjectID, KeyRenderPipeline),
}
#[derive(Debug, Default)]
struct CommandListRenderPipeline {
    pub list: Vec<ECommand>,
}
pub struct SysRenderPipelineCommand;
impl TSystemStageInfo for SysRenderPipelineCommand {

}
#[setup]
impl SysRenderPipelineCommand {
    #[system]
    fn cmd(
        mut cmds: ResMut<CommandListRenderPipeline>,
        mut items: Commands<GameObject, AssetkeyRenderPipeline>,
    ) {
        let mut list = std::mem::replace(&mut cmds.list, vec![]);
        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::Use(entity, key) => {
                    items.insert(entity, AssetkeyRenderPipeline(key.clone()));
                },
            }
        });
    }
}

#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetkeyRenderPipeline(pub KeyRenderPipeline);

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResRenderPipeline(Handle<ResRenderPipeline>);
impl From<Handle<ResRenderPipeline>> for AssetResRenderPipeline {
    fn from(value: Handle<ResRenderPipeline>) -> Self {
        Self(value)
    }
}
impl AssetResRenderPipeline {
    pub fn pipeline(&self) -> Handle<ResRenderPipeline> {
        self.0.clone()
    }
}

pub trait InterfaceRenderPipeline {
    fn use_render_pipeline(
        &self,
        entity: ObjectID,
        key: KeyRenderPipeline,
    ) -> &Self;
}

impl InterfaceRenderPipeline for EnginShell {
    fn use_render_pipeline(
        &self,
        entity: ObjectID,
        key: KeyRenderPipeline,
    ) -> &Self  {
        let commands = self.world().get_resource_mut::<CommandListRenderPipeline>().unwrap();
        commands.list.push(ECommand::Use(entity, key));

        self
    }
}

pub type SysRenderPipelineLoad = AssetSyncLoad<KeyRenderPipeline, AssetkeyRenderPipeline, ResRenderPipeline, AssetResRenderPipeline, SysRenderPipelineCommand>;
pub type PluginRenderPipelineLoad = PluginAssetSyncLoad::<KeyRenderPipeline, AssetkeyRenderPipeline, ResRenderPipeline, AssetResRenderPipeline, SysRenderPipelineCommand>;

pub struct PluginRenderPipeline;
impl Plugin for PluginRenderPipeline {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {

        PluginRenderPipelineLoad::new(false, 5 * 1024 * 1024, 60 * 1000).init(engine, stages);

        let world = engine.world_mut();
        world.insert_resource(CommandListRenderPipeline::default());
        SysRenderPipelineCommand::setup(world, stages.query_stage::<SysRenderPipelineCommand>(ERunStageChap::Command));

        Ok(())
    }
}

pub fn pipeline_state_key(
    targets: &[Option<wgpu::ColorTargetState>],
    primitive: &wgpu::PrimitiveState,
    depth_stencil: &Option<wgpu::DepthStencilState>,
    depth_stencil_bias_mode: u8,
    depth_stencil_bias_modes_use_bite: u8,
) -> PipelineStateKey {
    let mut calcolator = PipelineStateKeyCalcolator::new();
    gen_pipeline_key(&mut calcolator, &primitive, &depth_stencil, depth_stencil_bias_mode, depth_stencil_bias_modes_use_bite);
    match targets.get(0) {
        Some(target) => {
            match target {
                Some(target) => {
                    gen_fragment_state_key(&mut calcolator, target);
                },
                None => {},
            }
        },
        None => {},
    }
    calcolator.key
}

pub fn render_pipeline<T: TRenderShader>(
    shader: &T,
    device: &RenderDevice,
    targets: &[Option<wgpu::ColorTargetState>],
    depth_stencil: Option<wgpu::DepthStencilState>,
    primitive: wgpu::PrimitiveState,
    vertex_layouts: &[wgpu::VertexBufferLayout],
    bind_group_layouts: &[&wgpu::BindGroupLayout],
) -> ResRenderPipeline {

    let vs_state = wgpu::VertexState {
        module: shader.vs_module(),
        entry_point: shader.vs_entry_point(),
        buffers: &vertex_layouts,
    };
    let fs_state = wgpu::FragmentState {
        module: shader.fs_module(),
        entry_point: shader.fs_entry_point(),
        targets,
    };

    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts,
            push_constant_ranges: &[],
        }
    );

    let pipeline = device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
            label: None,
            // label: Some(shader.key()),
            layout: Some(&pipeline_layout),
            vertex: vs_state,
            fragment: Some(fs_state),
            primitive,
            depth_stencil,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None,
        }
    );

    ResRenderPipeline(RenderPipeline::from(pipeline))
}