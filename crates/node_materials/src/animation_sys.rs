use std::marker::PhantomData;

use pi_animation::type_animation_context::TypeAnimationContext;
use pi_curves::curve::frame::FrameDataValue;
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;

use crate::animation::{MaterialAnimeSlots, TMaterialAnimeValue};


pub fn sys_material_anime_init(
    materials: Query<(Entity, &BindEffect), Changed<BindEffect>>,
    mut commands: Commands,
) {
    materials.iter().for_each(|(entity, bindeffect)| {
        let info = MaterialAnimeSlots::new(&bindeffect.0);
        commands.entity(entity)
            .insert(info);
    });
}


pub fn sys_calc_type_anime<D: FrameDataValue + Component + TMaterialAnimeValue>(
    type_ctx: Res<TypeAnimeContext<D>>,
    runinfos: Res<GlobalAnimeAbout>,
    mut materials: Query<(&MaterialAnimeSlots, &BindEffect, &mut BindEffectValueDirty)>,
) {
    // let time0 = pi_time::Instant::now();

    let ty = type_ctx.ctx.ty();
    let curves = type_ctx.ctx.curves();
    if let Some(list) = runinfos.runtimeinfos.list.get(ty) {
        for info in list {
            if let Some(Some(curve)) = curves.get(info.curve_id) {
                // println!(">>>>>>>>>>>>>>>>>{}", info.amount_in_second);
                let value = curve.as_ref().interple(info.amount_in_second);
                if let Ok((slots, bindeffect, mut dirty)) = materials.get_mut(info.target) {
                    value.apply(slots, &bindeffect.0);
                    *dirty = BindEffectValueDirty(true);
                }
            }
        }
    } else {
        log::trace!("Not Found Anime Type: {}", ty);
    }

    // let time1 = pi_time::Instant::now();
    // log::debug!("sys_calc_type_anime : {:?}", time1 - time0);
}

pub struct PluginMaterialAnime<D: FrameDataValue + Component + TMaterialAnimeValue>(bool, usize, usize, PhantomData<D>);
impl<D: FrameDataValue + Component + TMaterialAnimeValue> PluginMaterialAnime<D> {
    pub fn new(ref_garbage: bool, capacity: usize, timeout: usize) -> Self {
        Self(ref_garbage, capacity, timeout, PhantomData::default())
    }
}
impl<D: FrameDataValue + Component + TMaterialAnimeValue> Plugin for PluginMaterialAnime<D> {

    fn build(&self, app: &mut bevy::prelude::App) {
        
        let ty = app.world.get_resource_mut::<GlobalAnimeAbout>().unwrap().ty_alloc.alloc().expect("");
        
        // 创建 动画曲线 资产表
        app.world.insert_resource(ShareAssetMgr::<TypeFrameCurve<D>>::new(GarbageEmpty(), self.0, self.1, self.2));

        let mut runtime_info_map = &mut app.world.get_resource_mut::<GlobalAnimeAbout>().unwrap().runtimeinfos;

        let type_ctx = TypeAnimeContext::<D> {
            ctx: TypeAnimationContext::<D, AssetTypeFrameCurve<D>>::new(ty, &mut runtime_info_map),
        };

        app.world.insert_resource(type_ctx);

        // app.add_system(
        //     sys_listen_type_anime_ctx::<D>.in_set(ERunStageChap::Command)
        // );
        app.add_system(sys_calc_type_anime::<D>.in_set(ERunStageChap::Anime));
    }
}
