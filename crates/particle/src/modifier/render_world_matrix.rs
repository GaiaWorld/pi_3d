use pi_scene_math::{
    coordiante_system::CoordinateSytem3, vector::TToolMatrix, Isometry3, Matrix, Point3,
    Quaternion, Rotation3, Vector3, Vector4,
};

use crate::{math::direction_to_quaternion, multiply, normalize};

pub type TRenderWorldMatrixCompute = dyn Fn(
    Matrix,
    &mut Matrix,
    &mut Matrix,
    Matrix,
    Vector3,
    &mut Vector3,
    f32,
    Vector3,
    Vector3,
    Vector3,
    f32,
    f32,
);

const TEMP_VECTOR3_TRANSLATION: Vector3 = Vector3::new(0., 0., 0.);

const STRETCHED_BILLBOARD_ROT: Vector3 = Vector3::new(0., 0., -1.5707963267948966);
// (<any>window)._StretchedBillboardRot = _StretchedBillboardRot;

pub struct StretchedBillboard;

impl StretchedBillboard {
    pub fn compute(
        parent_world_matrix: Matrix,
        real_start_matrix: &mut Matrix,
        result_matrix: &mut Matrix,
        camera_rotation_matrix: Matrix,
        _camra_global_pos: Vector3,
        direction: &mut Vector3,
        direction_length: f32,
        position: Vector3,
        scaling: Vector3,
        _rotation: Vector3,
        stretched_length_scale: f32,
        stretched_velocity_scale: f32,
    ) {
        let temp_vector3_translation = parent_world_matrix.transform_vector(&position);

        real_start_matrix[3] = 0.;
        real_start_matrix[7] = 0.;
        real_start_matrix[11] = 0.;

        let mut speed_direction = Vector3::identity();
        let mut x_axis = Vector3::identity();
        let mut view_direction = Vector3::new(0., 0., 1.);

        view_direction = camera_rotation_matrix.transform_vector(&view_direction);

        let mut speed_direction = *direction;
        speed_direction = real_start_matrix.transform_vector(&speed_direction);

        let speed_direction_length = direction_length;

        view_direction = normalize(&view_direction);
        speed_direction = normalize(&speed_direction);
        let mut z_axis = view_direction;
        let mut y_axis = speed_direction;
        let mut x_axis = speed_direction.cross(&z_axis);
        // Vector3.CrossToRef(speedDirection, zAxis, xAxis);
        let x_square_length = x_axis.magnitude_squared();
        if x_square_length == 0. {
            x_axis[0] = 1.0;
        } else {
            x_axis = normalize(&x_axis);
            // x_axis *= x_square_length.sqrt();
        }

        z_axis = x_axis.cross(&y_axis);
        y_axis = normalize(&y_axis);
        CoordinateSytem3::matrix4_from_xyz_axes(&x_axis, &y_axis, &z_axis, real_start_matrix);
        real_start_matrix.m14 = temp_vector3_translation.x;
        real_start_matrix.m24 = temp_vector3_translation.y;
        real_start_matrix.m34 = temp_vector3_translation.z;

        let mut temp_vector3_scaling = scaling;
        temp_vector3_scaling.y *= stretched_length_scale;
        temp_vector3_scaling.y += stretched_velocity_scale * speed_direction_length;
        speed_direction = Vector3::new(0., -(temp_vector3_scaling.y) / 2., 0.);

        let temp_quaternion_0 = Quaternion::identity();
        CoordinateSytem3::matrix4_compose(
            &temp_vector3_scaling,
            &temp_quaternion_0,
            &speed_direction,
            result_matrix,
        );

        *result_matrix = (*real_start_matrix) * (*result_matrix);

        // // Matrix.ComposeToRef(TempVector3One, TempQuaternion_0, TempVector3Zero, TempMatrix_1);
        // let euler_angles = temp_quaternion_0.m();
        let temp_matrix_1 = Matrix::from_euler_angles(
            STRETCHED_BILLBOARD_ROT.x,
            STRETCHED_BILLBOARD_ROT.y,
            STRETCHED_BILLBOARD_ROT.z,
        );

        *result_matrix = temp_matrix_1 * (*result_matrix);
    }
}

pub struct HorizontalBillboard;
impl HorizontalBillboard {
    pub fn compute(
        parent_world_matrix: Matrix,
        real_start_matrix: &mut Matrix,
        result_matrix: &mut Matrix,
        _camera_rotation_matrix: Matrix,
        _camra_global_pos: Vector3,
        _direction: &mut Vector3,
        _direction_length: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        _stretched_length_scale: f32,
        _stretched_velocity_scale: f32,
    ) {
        let temp_vector3_translation = parent_world_matrix.transform_vector(&position);

        real_start_matrix[3] = 0.;
        real_start_matrix[7] = 0.;
        real_start_matrix[11] = 0.;

        let mut temp_matrix_1 = Matrix::identity();
        CoordinateSytem3::matrix4_compose_euler_angle(
            &scaling,
            &rotation,
            &Vector3::zeros(),
            &mut temp_matrix_1,
        );
        *real_start_matrix = temp_matrix_1 * (*real_start_matrix);

        let mut temp_vector3_scaling = Vector3::zeros();
        CoordinateSytem3::matrix4_decompose_rotation(
            real_start_matrix,
            Some(&mut temp_vector3_scaling),
            None,
            None,
        );

        CoordinateSytem3::matrix4_compose_euler_angle(
            &temp_vector3_scaling,
            &Vector3::new(std::f32::consts::PI / 2., 0., 0.),
            &temp_vector3_translation,
            result_matrix,
        );
    }
}

pub struct VerticalBillboard;
impl VerticalBillboard {
    pub fn compute(
        parent_world_matrix: Matrix,
        real_start_matrix: &mut Matrix,
        result_matrix: &mut Matrix,
        _camera_rotation_matrix: Matrix,
        camra_global_pos: Vector3,
        _direction: &mut Vector3,
        _direction_length: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        _stretched_length_scale: f32,
        _stretched_velocity_scale: f32,
    ) {
        let temp_vector3_translation = parent_world_matrix.transform_vector(&position);

        real_start_matrix[3] = 0.;
        real_start_matrix[7] = 0.;
        real_start_matrix[11] = 0.;

        let mut temp_matrix_1 = Matrix::identity();
        CoordinateSytem3::matrix4_compose_euler_angle(
            &scaling,
            &rotation,
            &Vector3::zeros(),
            &mut temp_matrix_1,
        );
        *real_start_matrix = temp_matrix_1 * (*real_start_matrix);

        let mut temp_vector3_scaling = Vector3::zeros();
        CoordinateSytem3::matrix4_decompose_rotation(
            real_start_matrix,
            Some(&mut temp_vector3_scaling),
            None,
            None,
        );

        let mut temp_vector3_a = temp_vector3_translation - camra_global_pos;
        temp_vector3_a.y = 0.;
        temp_vector3_a.z = 0.;
        if temp_vector3_a.x == 0. && temp_vector3_a.z == 0. {
            temp_vector3_a.y = 1.;
        }
        temp_vector3_a = normalize(&temp_vector3_a);
        // Vector3.NormalizeToRef(TempVector3A, TempVector3A);
        let mut temp_quaternion_0 = Vector4::identity();
        direction_to_quaternion(temp_vector3_a, &mut temp_quaternion_0);
        let rotation = Quaternion::from_quaternion(nalgebra::Quaternion::new(
            temp_quaternion_0[0],
            temp_quaternion_0[1],
            temp_quaternion_0[2],
            temp_quaternion_0[2],
        ));

        CoordinateSytem3::matrix4_compose(
            &temp_vector3_scaling,
            &rotation,
            &temp_vector3_translation,
            result_matrix,
        );
    }
}

pub struct RenderAlignmentView;

impl RenderAlignmentView {
    pub fn compute(
        parent_world_matrix: Matrix,
        real_start_matrix: &mut Matrix,
        result_matrix: &mut Matrix,
        camera_rotation_matrix: Matrix,
        _camra_global_pos: Vector3,
        _direction: &mut Vector3,
        _direction_length: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        _stretched_length_scale: f32,
        _stretched_velocity_scale: f32,
    ) {
        let temp_vector3_translation = parent_world_matrix.transform_vector(&position);

        real_start_matrix[3] = 0.;
        real_start_matrix[7] = 0.;
        real_start_matrix[11] = 0.;

        // 自身旋转
        let mut temp_matrix_1 = Matrix::identity();
        CoordinateSytem3::matrix4_compose_euler_angle(
            &scaling,
            &rotation,
            &Vector3::zeros(),
            &mut temp_matrix_1,
        );
        *real_start_matrix = temp_matrix_1 * (*real_start_matrix);

        let mut temp_vector3_scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            real_start_matrix,
            Some(&mut temp_vector3_scaling),
            None,
            None,
        );

        // 相机方向
        let mut temp_quaternion_0 = Quaternion::identity();
        CoordinateSytem3::matrix4_decompose(
            &camera_rotation_matrix,
            None,
            Some(&mut temp_quaternion_0),
            None,
        );
        // 叠加自身旋转
        let temp_quaternion_0 = temp_quaternion_0.rotation_to(&temp_quaternion_0);
        let temp_quaternion_0 = Quaternion::from_quaternion(temp_quaternion_0.normalize());
        // let mut tempQuaternion_0 =
        //     Vector3::new(tempQuaternion_0.0, tempQuaternion_0.1, tempQuaternion_0.2)
        //         .cross(&rotation);
        // let tempQuaternion_0 = tempQuaternion_0.matrix();
        CoordinateSytem3::matrix4_compose(
            &temp_vector3_scaling,
            &temp_quaternion_0,
            &temp_vector3_translation,
            result_matrix,
        );
    }
}

pub struct RenderAlignmentWorld;

impl RenderAlignmentWorld {
    pub fn compute(
        parent_world_matrix: Matrix,
        real_start_matrix: &mut Matrix,
        result_matrix: &mut Matrix,
        _camera_rotation_matrix: Matrix,
        _camra_global_pos: Vector3,
        _direction: &mut Vector3,
        _direction_length: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        _stretched_length_scale: f32,
        _stretched_velocity_scale: f32,
    ) {
        let temp_vector3_translation = parent_world_matrix.transform_vector(&position);

        real_start_matrix[3] = 0.;
        real_start_matrix[7] = 0.;
        real_start_matrix[11] = 0.;

        // Local - Scaling - Rotation
        // 自身旋转
        *real_start_matrix = Matrix::new_nonuniform_scaling(&scaling)
            * Matrix::from_euler_angles(rotation[0], rotation[1], rotation[2])
            * (*real_start_matrix);

        let mut temp_matrix_1 = Matrix::identity();
        CoordinateSytem3::matrix4_compose_euler_angle(
            &scaling,
            &rotation,
            &Vector3::zeros(),
            &mut temp_matrix_1,
        );

        *real_start_matrix = temp_matrix_1 * (*real_start_matrix);

        let mut temp_vector3_scaling = Vector3::zeros();
        CoordinateSytem3::matrix4_decompose_rotation(
            real_start_matrix,
            Some(&mut temp_vector3_scaling),
            None,
            None,
        );

        CoordinateSytem3::matrix4_compose(
            &temp_vector3_scaling,
            &Quaternion::identity(),
            &temp_vector3_translation,
            result_matrix,
        );
    }
}

pub struct RenderAlignmentFacing {
    _use_right_handed_system: bool,
}

impl RenderAlignmentFacing {
    pub const use_right_handed_system: bool = false;

    pub fn compute(
        parent_world_matrix: Matrix,
        real_start_matrix: &mut Matrix,
        result_matrix: &mut Matrix,
        camera_rotation_matrix: Matrix,
        _camra_global_pos: Vector3,
        _direction: &mut Vector3,
        _direction_length: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        _stretched_length_scale: f32,
        _stretched_velocity_scale: f32,
    ) {
        let temp_vector3_translation = parent_world_matrix.transform_vector(&position);

        real_start_matrix[3] = 0.;
        real_start_matrix[7] = 0.;
        real_start_matrix[11] = 0.;

        let mut temp_matrix_1 = Matrix::identity();
        CoordinateSytem3::matrix4_compose_euler_angle(
            &scaling,
            &rotation,
            &Vector3::zeros(),
            &mut temp_matrix_1,
        );
        *real_start_matrix = temp_matrix_1 * (*real_start_matrix);

        let mut temp_vector3_scaling = Vector3::zeros();
        CoordinateSytem3::matrix4_decompose_rotation(
            real_start_matrix,
            Some(&mut temp_vector3_scaling),
            None,
            None,
        );

        let mut temp_matrix_view = if Self::use_right_handed_system {
            Matrix::look_at_rh(
                &Point3::new(
                    _camra_global_pos.x,
                    _camra_global_pos.y,
                    _camra_global_pos.z,
                ),
                &Point3::new(
                    temp_vector3_translation.x,
                    temp_vector3_translation.y,
                    temp_vector3_translation.z,
                ),
                &Vector3::new(0., 1., 0.),
            )
        } else {
            Matrix::look_at_lh(
                &Point3::new(
                    _camra_global_pos.x,
                    _camra_global_pos.y,
                    _camra_global_pos.z,
                ),
                &Point3::new(
                    temp_vector3_translation.x,
                    temp_vector3_translation.y,
                    temp_vector3_translation.z,
                ),
                &Vector3::new(0., 1., 0.),
            )
        };

        let mut quaternion = Quaternion::identity();
        CoordinateSytem3::matrix4_decompose(&temp_matrix_view, None, Some(&mut quaternion), None);
        CoordinateSytem3::matrix4_compose(
            &Vector3::new(1., 1., 1.),
            &quaternion,
            &Vector3::zeros(),
            &mut temp_matrix_view,
        );

        let mut temp_quaternion_0 = Quaternion::identity();
        CoordinateSytem3::matrix4_decompose(
            &camera_rotation_matrix,
            None,
            Some(&mut temp_quaternion_0),
            None,
        );
        temp_quaternion_0 = temp_quaternion_0.rotation_to(&Quaternion::from_euler_angles(rotation.x, rotation.y, rotation.z));
        temp_quaternion_0 = Quaternion::from_quaternion(temp_quaternion_0.normalize());

        CoordinateSytem3::matrix4_compose(
            &temp_vector3_scaling,
            &temp_quaternion_0,
            &temp_vector3_translation,
            result_matrix,
        )
    }
}

pub struct RenderAlignmentVelocity;
impl RenderAlignmentVelocity {
    pub fn compute(
        parent_world_matrix: Matrix,
        real_start_matrix: &mut Matrix,
        result_matrix: &mut Matrix,
        _camera_rotation_matrix: Matrix,
        _camra_global_pos: Vector3,
        direction: &mut Vector3,
        _direction_length: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        _stretched_length_scale: f32,
        _stretched_velocity_scale: f32,
    ) {
        let temp_vector3_translation = parent_world_matrix.transform_vector(&position);

        real_start_matrix[3] = 0.;
        real_start_matrix[7] = 0.;
        real_start_matrix[11] = 0.;

        let mut temp_matrix_1 = Matrix::identity();
        CoordinateSytem3::matrix4_compose_euler_angle(
            &scaling,
            &rotation,
            &Vector3::zeros(),
            &mut temp_matrix_1,
        );

        let mut temp_vector3_a = *direction;
        temp_vector3_a = real_start_matrix.transform_vector(&temp_vector3_a);
        temp_vector3_a = temp_vector3_a.normalize();

        let mut temp_matrix_view = Matrix::look_at_lh(
            &Point3::new(0., 0., 0.),
            &Point3::new(temp_vector3_a.x, temp_vector3_a.y, temp_vector3_a.z),
            &Vector3::new(0., 1., 0.),
        );

        let mut temp_quaternion_0 = Quaternion::identity();
        CoordinateSytem3::matrix4_decompose(&temp_matrix_view, None, Some(&mut temp_quaternion_0), None);

        temp_quaternion_0 = temp_quaternion_0.rotation_to(&Quaternion::from_euler_angles(rotation.x, rotation.y, rotation.z));
        temp_quaternion_0 = Quaternion::from_quaternion(temp_quaternion_0.normalize());

        *real_start_matrix = temp_matrix_1 * (*real_start_matrix);
        // realStartMatrix.decompose(TempVector3Scaling, undefined, undefined);
        let mut temp_vector3_scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            real_start_matrix,
            Some(&mut temp_vector3_scaling),
            None,
            None,
        );

        CoordinateSytem3::matrix4_compose(
            &temp_vector3_scaling,
            &temp_quaternion_0,
            &temp_vector3_translation,
            result_matrix,
        )
    }
}

pub struct RenderAlignmentLocal;
impl RenderAlignmentLocal {
    pub fn compute(
        parent_world_matrix: Matrix,
        real_start_matrix: &mut Matrix,
        result_matrix: &mut Matrix,
        _camera_rotation_matrix: Matrix,
        _camra_global_pos: Vector3,
        _direction: &mut Vector3,
        _direction_length: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        _stretched_length_scale: f32,
        _stretched_velocity_scale: f32,
    ) {
        let temp_vector3_translation = parent_world_matrix.transform_vector(&position);
        // println!("tempVector3Translation: {:?}", tempVector3Translation);
        // println!("parentWorldMatrix: {:?}", parentWorldMatrix);
        // println!("position: {:?}", position);

        real_start_matrix[3] = 0.;
        real_start_matrix[7] = 0.;
        real_start_matrix[11] = 0.;

        let mut temp_matrix_1 = Matrix::identity();
        CoordinateSytem3::matrix4_compose_euler_angle(
            &scaling,
            &rotation,
            &Vector3::zeros(),
            &mut temp_matrix_1,
        );

        *result_matrix = temp_matrix_1 * (*real_start_matrix);

        // resultMatrix(TempVector3Translation);
        result_matrix[3] = temp_vector3_translation[0];
        result_matrix[7] = temp_vector3_translation[1];
        result_matrix[11] = temp_vector3_translation[2];
    }
}
