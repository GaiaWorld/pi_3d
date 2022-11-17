use std::mem::replace;

use pi_ecs::{sys::system, prelude::{ResMut, Query, Setup, Res}, query::{Write, Changed}, entity};
use pi_ecs_macros::setup;
use pi_render::rhi::device::RenderDevice;
use render_resource::sampler::{EAnisotropyClamp, SamplerDesc, SamplerKey, SamplerPool};

use crate::object::{ObjectID, GameObject};

#[derive(Debug, Default)]
pub struct TextureSamplerDesc(pub SamplerDesc);

pub struct TextureSamplerID(pub SamplerKey);

#[derive(Debug)]
pub enum ETextureSamplerCommand {
    AddressU(ObjectID, wgpu::AddressMode),
    AddressV(ObjectID, wgpu::AddressMode),
    AddressW(ObjectID, wgpu::AddressMode),
    MagFilter(ObjectID, wgpu::FilterMode),
    MinFilter(ObjectID, wgpu::FilterMode),
    MipmapFilter(ObjectID, wgpu::FilterMode),
    Compare(ObjectID, Option<wgpu::CompareFunction>),
    Anisotropy(ObjectID, EAnisotropyClamp),
    Border(ObjectID, Option<wgpu::SamplerBorderColor>),
}

#[derive(Debug, Default)]
struct SingleCommandList {
    pub list: Vec<ETextureSamplerCommand>,
}

struct SysCommand;
#[setup]
impl SysCommand {
    #[system]
    fn cmd(
        mut cmds: ResMut<SingleCommandList>,
        mut textures: Query<GameObject, Write<TextureSamplerDesc>>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ETextureSamplerCommand::AddressU(entity, value) => {
                    if let Some(mut target) = textures.get_mut(entity) {
                        match target.get_mut() {
                            Some(target) => {
                                target.0.address_mode_u = value;
                            },
                            None => {
                                let mut result = TextureSamplerDesc::default();
                                result.0.address_mode_u = value;
                                target.write(result);
                            },
                        };
                        target.notify_modify();
                    }
                },
                ETextureSamplerCommand::AddressV(entity, value) => {
                    if let Some(mut target) = textures.get_mut(entity) {
                        match target.get_mut() {
                            Some(target) => {
                                target.0.address_mode_v = value;
                            },
                            None => {
                                let mut result = TextureSamplerDesc::default();
                                result.0.address_mode_v = value;
                                target.write(result);
                            },
                        };
                        target.notify_modify();
                    }
                },
                ETextureSamplerCommand::AddressW(entity, value) => {
                    if let Some(mut target) = textures.get_mut(entity) {
                        match target.get_mut() {
                            Some(target) => {
                                target.0.address_mode_w = value;
                            },
                            None =>  {
                                let mut result = TextureSamplerDesc::default();
                                result.0.address_mode_w = value;
                                target.write(result);
                            },
                        };
                        target.notify_modify();
                    }
                },
                ETextureSamplerCommand::MagFilter(entity, value) => {
                    if let Some(mut target) = textures.get_mut(entity) {
                        target.notify_modify();
                        match target.get_mut() {
                            Some(target) => {
                                target.0.mag_filter = value;
                            },
                            None =>  {
                                let mut result = TextureSamplerDesc::default();
                                result.0.mag_filter = value;
                                target.write(result);
                            },
                        };
                        target.notify_modify();
                    }
                },
                ETextureSamplerCommand::MinFilter(entity, value) => {
                    if let Some(mut target) = textures.get_mut(entity) {
                        match target.get_mut() {
                            Some(target) => {
                                target.0.min_filter = value;
                            },
                            None => {
                                let mut result = TextureSamplerDesc::default();
                                result.0.min_filter = value;
                                target.write(result);
                            },
                        };
                        target.notify_modify();
                    }
                },
                ETextureSamplerCommand::MipmapFilter(entity, value) => {
                    if let Some(mut target) = textures.get_mut(entity) {
                        match target.get_mut() {
                            Some(target) => {
                                target.0.mipmap_filter = value;
                            },
                            None => {
                                let mut result = TextureSamplerDesc::default();
                                result.0.mipmap_filter = value;
                                target.write(result);
                            },
                        };
                        target.notify_modify();
                    }
                },
                ETextureSamplerCommand::Compare(entity, value) => {
                    if let Some(mut target) = textures.get_mut(entity) {
                        match target.get_mut() {
                            Some(target) => {
                                target.0.compare = value;
                            },
                            None =>  {
                                let mut result = TextureSamplerDesc::default();
                                result.0.compare = value;
                                target.write(result);
                            },
                        };
                        target.notify_modify();
                    }
                },
                ETextureSamplerCommand::Anisotropy(entity, value) => {
                    if let Some(mut target) = textures.get_mut(entity) {
                        match target.get_mut() {
                            Some(target) => {
                                target.0.anisotropy_clamp = value;
                            },
                            None => {
                                let mut result = TextureSamplerDesc::default();
                                result.0.anisotropy_clamp = value;
                                target.write(result);
                            },
                        };
                        target.notify_modify();
                    }
                },
                ETextureSamplerCommand::Border(entity, value) => {
                    if let Some(mut target) = textures.get_mut(entity) {
                        match target.get_mut() {
                            Some(target) => {
                                target.0.border_color = value;
                            },
                            None => {
                                let mut result = TextureSamplerDesc::default();
                                result.0.border_color = value;
                                target.write(result);
                            },
                        };
                        target.notify_modify();
                    }
                },
            }
        });
    }
}

struct SysSamplerUpdate;
#[setup]
impl SysSamplerUpdate {
    #[system]
    pub fn sys(
        device: Res<RenderDevice>,
        mut pool: ResMut<SamplerPool>,
        mut textures: Query<GameObject, (&TextureSamplerDesc, Write<TextureSamplerID>), Changed<TextureSamplerDesc>>,
    ) {
        textures.iter_mut().for_each(|(texture, mut samplerid)| {
            let key = SamplerPool::cacl_key(&texture.0);
            pool.create(&texture.0, &device);
            samplerid.write(TextureSamplerID(key));
        });
    }
}

pub trait InterfaceTextureAddressMode {
    fn with_texture_sampler(
        &self,
        entity: ObjectID,
    ) -> &Self;
    fn texture_address(
        &self,
        entity: ObjectID,
        mode: wgpu::AddressMode,
    ) -> &Self;
    fn texture_address_u(
        &self,
        entity: ObjectID,
        mode: wgpu::AddressMode,
    ) -> &Self;
    fn texture_address_v(
        &self,
        entity: ObjectID,
        mode: wgpu::AddressMode,
    ) -> &Self;
    fn texture_address_w(
        &self,
        entity: ObjectID,
        mode: wgpu::AddressMode,
    ) -> &Self;
    fn texture_filter(
        &self,
        entity: ObjectID,
        mode: wgpu::FilterMode,
    ) -> &Self;
    fn texture_mag_filter(
        &self,
        entity: ObjectID,
        mode: wgpu::FilterMode,
    ) -> &Self;
    fn texture_min_filter(
        &self,
        entity: ObjectID,
        mode: wgpu::FilterMode,
    ) -> &Self;
    fn texture_mipmap_filter(
        &self,
        entity: ObjectID,
        mode: wgpu::FilterMode,
    ) -> &Self;
    fn texture_compare(
        &self,
        entity: ObjectID,
        mode: Option<wgpu::CompareFunction>,
    ) -> &Self;
    fn texture_anisotropy(
        &self,
        entity: ObjectID,
        mode: EAnisotropyClamp,
    ) -> &Self;
    fn texture_border_color(
        &self,
        entity: ObjectID,
        mode: Option<wgpu::SamplerBorderColor>,
    ) -> &Self;
}

impl InterfaceTextureAddressMode for crate::engine::Engine {
    fn with_texture_sampler(
        &self,
        entity: ObjectID,
    ) -> &Self {
        self.texture_address(entity, wgpu::AddressMode::ClampToEdge);
        self
    }
    fn texture_address(
        &self,
        entity: ObjectID,
        mode: wgpu::AddressMode,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleCommandList>().unwrap();
        commands.list.push(ETextureSamplerCommand::AddressU(entity, mode));
        commands.list.push(ETextureSamplerCommand::AddressV(entity, mode));
        commands.list.push(ETextureSamplerCommand::AddressW(entity, mode));

        self
    }

    fn texture_address_u(
        &self,
        entity: ObjectID,
        mode: wgpu::AddressMode,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleCommandList>().unwrap();
        commands.list.push(ETextureSamplerCommand::AddressU(entity, mode));

        self
    }

    fn texture_address_v(
        &self,
        entity: ObjectID,
        mode: wgpu::AddressMode,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleCommandList>().unwrap();
        commands.list.push(ETextureSamplerCommand::AddressV(entity, mode));


        self
    }

    fn texture_address_w(
        &self,
        entity: ObjectID,
        mode: wgpu::AddressMode,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleCommandList>().unwrap();
        commands.list.push(ETextureSamplerCommand::AddressW(entity, mode));

        self
    }

    fn texture_filter(
        &self,
        entity: ObjectID,
        mode: wgpu::FilterMode,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleCommandList>().unwrap();
        commands.list.push(ETextureSamplerCommand::MinFilter(entity, mode));
        commands.list.push(ETextureSamplerCommand::MagFilter(entity, mode));
        commands.list.push(ETextureSamplerCommand::MipmapFilter(entity, mode));

        self
    }

    fn texture_mag_filter(
        &self,
        entity: ObjectID,
        mode: wgpu::FilterMode,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleCommandList>().unwrap();
        commands.list.push(ETextureSamplerCommand::MagFilter(entity, mode));

        self
    }

    fn texture_min_filter(
        &self,
        entity: ObjectID,
        mode: wgpu::FilterMode,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleCommandList>().unwrap();
        commands.list.push(ETextureSamplerCommand::MinFilter(entity, mode));

        self
    }

    fn texture_mipmap_filter(
        &self,
        entity: ObjectID,
        mode: wgpu::FilterMode,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleCommandList>().unwrap();
        commands.list.push(ETextureSamplerCommand::MipmapFilter(entity, mode));

        self
    }

    fn texture_compare(
        &self,
        entity: ObjectID,
        mode: Option<wgpu::CompareFunction>,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleCommandList>().unwrap();
        commands.list.push(ETextureSamplerCommand::Compare(entity, mode));


        self
    }

    fn texture_anisotropy(
        &self,
        entity: ObjectID,
        mode: EAnisotropyClamp,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleCommandList>().unwrap();
        commands.list.push(ETextureSamplerCommand::Anisotropy(entity, mode));

        self
    }

    fn texture_border_color(
        &self,
        entity: ObjectID,
        mode: Option<wgpu::SamplerBorderColor>,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleCommandList>().unwrap();
        commands.list.push(ETextureSamplerCommand::Border(entity, mode));

        self
    }
}

pub struct PluginTextureSampler;
impl crate::Plugin for PluginTextureSampler {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleCommandList::default());
        world.insert_resource(SamplerPool::default());

        SysCommand::setup(world, stages.command_stage());
        SysSamplerUpdate::setup(world, stages.command_stage());

        Ok(())
    }
}