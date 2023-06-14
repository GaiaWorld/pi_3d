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
        let _temp_vector3_translation = parent_world_matrix.transform_vector(&position);

        real_start_matrix[3] = 0.;
        real_start_matrix[7] = 0.;
        real_start_matrix[11] = 0.;

        let mut temp_vector3_scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            real_start_matrix,
            Some(&mut temp_vector3_scaling),
            None,
            None,
        );

        let mut view_direction = Vector3::new(0., 0., 1.);
        view_direction = camera_rotation_matrix.transform_vector(&view_direction);

        let mut speed_direction = *direction;
        speed_direction = real_start_matrix.transform_vector(&speed_direction);

        let speed_direction_length = direction_length;

        view_direction = normalize(&view_direction);
        speed_direction = normalize(&speed_direction);
        let mut z_axis = view_direction;
        let mut y_axis = speed_direction;
        let mut x_axis = multiply(&speed_direction, &z_axis);
        // Vector3.CrossToRef(speedDirection, zAxis, xAxis);
        let x_square_length = x_axis.magnitude_squared();
        if x_square_length == 0. {
            x_axis[0] = 1.0;
        } else {
            normalize(&x_axis);
            x_axis *= x_square_length.sqrt();
        }

        z_axis = x_axis.cross(&y_axis);
        y_axis = normalize(&y_axis);

        *real_start_matrix = Matrix::new(
            x_axis[0],
            y_axis[0],
            z_axis[0],
            temp_vector3_scaling[0],
            x_axis[1],
            y_axis[1],
            z_axis[1],
            temp_vector3_scaling[1],
            x_axis[2],
            y_axis[2],
            z_axis[2],
            temp_vector3_scaling[2],
            0.,
            0.,
            0.,
            1.,
        );

        temp_vector3_scaling = scaling;
        temp_vector3_scaling[1] *= stretched_length_scale;
        temp_vector3_scaling[1] += stretched_velocity_scale * speed_direction_length;
        speed_direction = Vector3::new(0., -(temp_vector3_scaling[1]) / 2., 0.);

        let mut temp_quaternion_0 =
            Quaternion::from_quaternion(nalgebra::Quaternion::new(0., 0., 0., 1.));
        let euler_angles = temp_quaternion_0.euler_angles();
        *result_matrix = Matrix::new_nonuniform_scaling(&temp_vector3_scaling)
            * Matrix::from_euler_angles(euler_angles.0, euler_angles.1, euler_angles.2)
            * Matrix::new_translation(&speed_direction);

        *result_matrix = result_matrix.cross(&real_start_matrix);

        temp_quaternion_0 = Quaternion::from_euler_angles(
            STRETCHED_BILLBOARD_ROT[0],
            STRETCHED_BILLBOARD_ROT[1],
            STRETCHED_BILLBOARD_ROT[2],
        );
        // // Matrix.ComposeToRef(TempVector3One, TempQuaternion_0, TempVector3Zero, TempMatrix_1);
        let euler_angles = temp_quaternion_0.euler_angles();
        let temp_matrix_1 =
            Matrix::from_euler_angles(euler_angles.0, euler_angles.1, euler_angles.2);

        *result_matrix = temp_matrix_1.cross(&result_matrix);
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

        *real_start_matrix = Matrix::new_nonuniform_scaling(&scaling)
            * Matrix::from_euler_angles(rotation[2], rotation[1], rotation[0])
            * (*real_start_matrix);

        let mut temp_vector3_scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            real_start_matrix,
            Some(&mut temp_vector3_scaling),
            None,
            None,
        );

        *result_matrix = Matrix::new_nonuniform_scaling(&temp_vector3_scaling)
            * Matrix::from_euler_angles(std::f32::consts::PI / 2., 0., 0.)
            * Matrix::new_translation(&temp_vector3_translation);
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

        *real_start_matrix = Matrix::new_nonuniform_scaling(&scaling)
            * Matrix::from_euler_angles(rotation[2], rotation[1], rotation[0])
            * (*real_start_matrix);

        let mut temp_vector3_scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            real_start_matrix,
            Some(&mut temp_vector3_scaling),
            None,
            None,
        );

        let mut temp_vector3_a = TEMP_VECTOR3_TRANSLATION - camra_global_pos;
        temp_vector3_a[1] = 0.;
        temp_vector3_a[2] = 0.;
        if temp_vector3_a[0] == 0. && temp_vector3_a[2] == 0. {
            temp_vector3_a[1] = 1.;
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
        ))
        .euler_angles();

        *result_matrix = Matrix::new_nonuniform_scaling(&scaling)
            * Matrix::from_euler_angles(rotation.0, rotation.1, rotation.2)
            * Matrix::new_translation(&temp_vector3_translation);
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
        *real_start_matrix = Matrix::new_nonuniform_scaling(&scaling)
            * Matrix::from_euler_angles(rotation[2], rotation[1], rotation[0])
            * (*real_start_matrix);

        let mut temp_vector3_scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            real_start_matrix,
            Some(&mut temp_vector3_scaling),
            None,
            None,
        );

        // 相机方向
        let mut temp_quaternion_0 = Rotation3::identity();
        CoordinateSytem3::matrix4_decompose_rotation(
            &camera_rotation_matrix,
            None,
            Some(&mut temp_quaternion_0),
            None,
        );
        // 叠加自身旋转
        let temp_quaternion_0 = temp_quaternion_0.rotation_to(&temp_quaternion_0);
        // let mut tempQuaternion_0 =
        //     Vector3::new(tempQuaternion_0.0, tempQuaternion_0.1, tempQuaternion_0.2)
        //         .cross(&rotation);
        // let tempQuaternion_0 = tempQuaternion_0.matrix();
        let temp_quaternion_0 = temp_quaternion_0.euler_angles();
        *result_matrix = Matrix::new_nonuniform_scaling(&temp_vector3_scaling)
            * Matrix::from_euler_angles(temp_quaternion_0.0, temp_quaternion_0.1, temp_quaternion_0.2)
            * Matrix::new_translation(&temp_vector3_translation);
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

        let mut temp_vector3_scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            real_start_matrix,
            Some(&mut temp_vector3_scaling),
            None,
            None,
        );
        let temp_quaternion_0 =
            Quaternion::from_quaternion(nalgebra::Quaternion::new(0., 0., 0., 1.)).euler_angles();

        *result_matrix = Matrix::new_nonuniform_scaling(&temp_vector3_scaling)
            * Matrix::from_euler_angles(temp_quaternion_0.0, temp_quaternion_0.1, temp_quaternion_0.2)
            * Matrix::new_translation(&temp_vector3_translation);
    }
}

pub struct RenderAlignmentFacing {
    _use_right_handed_system: bool,
}

impl RenderAlignmentFacing {
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

        // Local - Scaling - Rotation
        // 自身旋转
        *real_start_matrix = Matrix::new_nonuniform_scaling(&scaling)
            * Matrix::from_euler_angles(rotation[0], rotation[1], rotation[2])
            * (*real_start_matrix);

        let mut temp_vector3_scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            real_start_matrix,
            Some(&mut temp_vector3_scaling),
            None,
            None,
        );

        // 相机方向
        // TempVector3Translation.subtractFromFloatsToRef(camraGlobalPos.x, camraGlobalPos.y, camraGlobalPos.z, TempVector3A);
        // if (RenderAlignmentFacing.useRightHandedSystem) {
        //     Matrix.LookAtRHToRef(Vector3.ZeroReadOnly, TempVector3A, Vector3.UpReadOnly, TempMatrix_View);
        // } else {
        //     Matrix.LookAtLHToRef(Vector3.ZeroReadOnly, TempVector3A, Vector3.UpReadOnly, TempMatrix_View);
        // }

        // let TempMatrix_View = Isometry3::look_at_lh(
        //     &Point3::new(camraGlobalPos[0], camraGlobalPos[1], camraGlobalPos[2]),
        //     &Point3::new(TempVector3Translation[0], TempVector3Translation[1], TempVector3Translation[2]),
        //     &Vector3::new(0., 1., 0.),
        // );
        // Matrix.LookAtRHToRef(camraGlobalPos, TempVector3Translation, Vector3.UpReadOnly, TempMatrix_View);

        // let TempMatrix_View = TempMatrix_View.rotation;

        // TempQuaternion_0.fromRotationMatrix(cameraRotationMatrix);

        // 叠加自身旋转
        let temp_quaternion_0 =
            camera_rotation_matrix * Matrix::from_euler_angles(rotation[0], rotation[1], rotation[2]);
        let temp_quaternion_0 = temp_quaternion_0.normalize();

        *result_matrix = Matrix::new_nonuniform_scaling(&temp_vector3_scaling)
            * Matrix::from_euler_angles(
                temp_quaternion_0[0],
                temp_quaternion_0[1],
                temp_quaternion_0[2],
            )
            * Matrix::new_translation(&temp_vector3_translation);
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

        let rotation = Rotation3::from_euler_angles(rotation[0], rotation[1], rotation[2]);
        // Local - Scaling - Rotation
        // 自身旋转
        let temp_matrix_1 = Matrix::new_nonuniform_scaling(&scaling) * rotation.to_homogeneous();

        // let direction = direction * 1.;
        let temp_vector3_a = real_start_matrix.transform_vector(direction);
        // Vector3.TransformCoordinatesToRef(TempVector3A, realStartMatrix, TempVector3A);
        let temp_vector3_a = normalize(&temp_vector3_a);
        // Vector3.NormalizeToRef(TempVector3A, TempVector3A);
        // 如何旋转以`看向`速度方向
        let temp_matrix_view = Isometry3::look_at_lh(
            &Point3::new(0., 0., 0.),
            &Point3::new(temp_vector3_a[0], temp_vector3_a[1], temp_vector3_a[2]),
            &Vector3::new(0., 1., 0.),
        );
        let temp_quaternion_0 = temp_matrix_view.rotation.euler_angles();
        let temp_quaternion_0 = Rotation3::from_euler_angles(
            temp_quaternion_0.0,
            temp_quaternion_0.1,
            temp_quaternion_0.2,
        );
        // let tempQuaternion_0 =
        //     Vector3::new(tempQuaternion_0.0, tempQuaternion_0.1, tempQuaternion_0.2);
        // TempMatrix_View.getRotationMatrixToRef(TempMatrix_View);
        // TempQuaternion_0.fromRotationMatrix(TempMatrix_View);
        // 叠加自身旋转
        let temp_quaternion_0 = temp_quaternion_0.rotation_to(&rotation);
        let temp_quaternion_0 = temp_quaternion_0;

        *real_start_matrix = temp_matrix_1 * (*real_start_matrix);
        // realStartMatrix.decompose(TempVector3Scaling, undefined, undefined);
        let mut temp_vector3_scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            real_start_matrix,
            Some(&mut temp_vector3_scaling),
            None,
            None,
        );

        *result_matrix = Matrix::new_nonuniform_scaling(&temp_vector3_scaling)
            * temp_quaternion_0.to_homogeneous()
            * Matrix::new_translation(&temp_vector3_translation);
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

        let temp_matrix_1 = Matrix::new_nonuniform_scaling(&scaling)
            * Matrix::from_euler_angles(rotation[0], rotation[1], rotation[2])
            * (*real_start_matrix);
        *result_matrix = temp_matrix_1 * (*real_start_matrix);

        // resultMatrix(TempVector3Translation);
        result_matrix[3] = temp_vector3_translation[0];
        result_matrix[7] = temp_vector3_translation[1];
        result_matrix[11] = temp_vector3_translation[2];
    }
}
