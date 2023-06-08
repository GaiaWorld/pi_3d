use pi_scene_math::{Vector3, Vector4};

const RADIANS_ITER_COUNT: usize = 360;
const RADIANS_ITER_E: f32 = std::f32::consts::PI * 2. / 360.;

lazy_static! {
    static ref SIN_TABLE: Vec<f32> = {
        let mut res = Vec::with_capacity(RADIANS_ITER_COUNT);
        for i in 0..RADIANS_ITER_COUNT {
            res.push(f32::sin(i as f32 / 360. * std::f32::consts::PI * 2.));
        }
        res
    };
    static ref COS_TABLE: Vec<f32> = {
        let mut res = Vec::with_capacity(RADIANS_ITER_COUNT);
        for i in 0..RADIANS_ITER_COUNT {
            res.push(f32::cos(i as f32 / 360. * std::f32::consts::PI * 2.));
        }
        res
    };
}

pub fn rotation_yaw_pitch_roll_to_ref(yaw: f32, pitch: f32, roll: f32, result: &mut Vector4) {
    // Produces a quaternion from Euler angles in the z-y-x orientation (Tait-Bryan angles)
    let half_roll = roll * 0.5;
    let half_pitch = pitch * 0.5;
    let half_yaw = yaw * 0.5;

    let sin_roll = f32::sin(half_roll);
    let cos_roll = f32::cos(half_roll);
    let sin_pitch = f32::sin(half_pitch);
    let cos_pitch = f32::cos(half_pitch);
    let sin_yaw = f32::sin(half_yaw);
    let cos_yaw = f32::cos(half_yaw);

    result[0] = (cos_yaw * sin_pitch * cos_roll) + (sin_yaw * cos_pitch * sin_roll);
    result[1] = (sin_yaw * cos_pitch * cos_roll) - (cos_yaw * sin_pitch * sin_roll);
    result[2] = (cos_yaw * cos_pitch * sin_roll) - (sin_yaw * sin_pitch * cos_roll);
    result[3] = (cos_yaw * cos_pitch * cos_roll) + (sin_yaw * sin_pitch * sin_roll);
}

pub fn sin(radians: f32) -> f32 {
    let mut index = f32::round(radians / RADIANS_ITER_E) as i32 % RADIANS_ITER_COUNT as i32;
    index = if index < 0 {
        RADIANS_ITER_COUNT as i32 + index
    } else {
        index
    };
    return SIN_TABLE[index as usize];
}

pub fn cos(radians: f32) -> f32 {
    let mut index = f32::round(radians / RADIANS_ITER_E) as i32 % RADIANS_ITER_COUNT as i32;
    index = if index < 0 {
        RADIANS_ITER_COUNT as i32 + index
    } else {
        index
    };
    return COS_TABLE[index as usize];
}

pub fn _sin(index: usize) -> f32 {
    return SIN_TABLE[index];
}

pub fn _cos(index: usize) -> f32 {
    return COS_TABLE[index];
}

pub fn direction_to_quaternion(direction: Vector3, result: &mut Vector4) {
    let x_axis = direction[0];
    let y_axis = direction[1];
    let z_axis = direction[2];

    let yaw = -f32::atan2(z_axis, x_axis) + std::f32::consts::PI / 2.;
    let len = f32::sqrt(x_axis * x_axis + z_axis * z_axis);
    let pitch = -f32::atan2(y_axis, len);

    rotation_yaw_pitch_roll_to_ref(yaw, pitch, 0., result);
}
