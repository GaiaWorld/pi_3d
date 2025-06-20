// use pi_particle_system::prelude::ActionSetParticleSystem;
// use pi_scene_shell::prelude::*;
// use pi_scene_context::prelude::*;

// use crate::GLTF;

// pub fn factory(
//     gltf:           &Handle<GLTF>,
//     commands:       &mut Commands,
//     camera:         &mut ActionSetCamera,
//     light:          &mut ActionSetLighting,
//     shadow:         &mut ActionSetShadow,
//     transform:      &mut ActionSetTransform,
//     mesh:           &mut ActionSetMesh,
//     skin:           &mut ActionSetSkeleton,
//     instance:       &mut ActionSetInstanceMesh,
//     geometry:       &mut ActionSetGeometry,
//     material:       &mut ActionSetMaterial,
//     anime:          &mut ActionSetAnimationGroup,
//     anime_instance: &mut ResMut<ActionListTargetAnimationAttribute>,
//     renderer:       &mut ActionSetRenderer,
//     parsys:         &mut ActionSetParticleSystem,
//     prop_anim:      &mut ResMut<ActionListPropertyTargetAnimation>,
//     spritecreate:   &mut ResMut<ActionListSpriteCreate>,
//     spritemodify:   &mut ResMut<ActionListSpriteModify>,
// ) -> Vec<Entity> {
//     let mut result = vec![];

//     // gltf.gltf.gltf.nodes().for_each(|node| {
//     //     if let Some(mesh) = node.mesh() {
//     //         mesh.primitives().for_each(f);
//     //     }
//     //     node.children()
//     // });

//     result
// }