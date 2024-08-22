use pi_scene_shell::prelude::{TargetAnimatorableIsRunning, ComponentChanged};

use crate::prelude::FlagRenderWorldMatrix;

pub fn runif_targetanime(
    changes: ComponentChanged<TargetAnimatorableIsRunning>,
) -> bool {
    let mut count = 0;
    changes.iter().for_each(|_|{
        count += 1;
    });
    0 < count
}


pub fn runif_rendermatrix(
    changes: ComponentChanged<FlagRenderWorldMatrix>,
) -> bool {
    let mut count = 0;
    changes.iter().for_each(|_|{
        count += 1;
    });
    0 < count
}
