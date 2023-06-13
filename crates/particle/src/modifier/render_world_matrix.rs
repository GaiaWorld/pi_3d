use nalgebra::{AbstractRotation, Point};
use pi_scene_math::{
    coordiante_system::CoordinateSytem3,
    vector::{TToolMatrix, TToolRotation},
    Isometry3, Matrix, Point3, Quaternion, Rotation3, Vector3, Vector4,
};

use crate::{math::direction_to_quaternion, multiply, normalize};

use super::base::TempVector3A;

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

const TempVector3Translation: Vector3 = Vector3::new(0., 0., 0.);
const TempVector3Scaling: Vector3 = Vector3::new(0., 0., 0.);
const TempVector3Rotation: Vector3 = Vector3::new(0., 0., 0.);
// const TempQuaternion_0 = new Quaternion();
// const TempQuaternion_1 = new Quaternion();
// const TempQuaternion_Y = Quaternion.RotationYawPitchRoll(Math.PI / 2, 0, 0);
// const TempMatrix_View = new Matrix();
// const TempMatrix_1 = new Matrix();
const TempVector3Zero: Vector3 = Vector3::new(0., 0., 0.);
const TempVector3One: Vector3 = Vector3::new(0., 0., 0.);

const _StretchedBillboardRot: Vector3 = Vector3::new(0., 0., -1.5707963267948966);
// (<any>window)._StretchedBillboardRot = _StretchedBillboardRot;

pub struct StretchedBillboard;

impl StretchedBillboard {
    pub fn compute(
        parentWorldMatrix: Matrix,
        realStartMatrix: &mut Matrix,
        resultMatrix: &mut Matrix,
        cameraRotationMatrix: Matrix,
        camraGlobalPos: Vector3,
        direction: &mut Vector3,
        directionLength: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        stretchedLengthScale: f32,
        stretchedVelocityScale: f32,
    ) {
        let empVector3Translation = parentWorldMatrix.transform_vector(&position);

        realStartMatrix[3] = 0.;
        realStartMatrix[7] = 0.;
        realStartMatrix[11] = 0.;

        let mut tempVector3Scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            realStartMatrix,
            Some(&mut tempVector3Scaling),
            None,
            None,
        );

        let mut viewDirection = Vector3::new(0., 0., 1.);
        viewDirection = cameraRotationMatrix.transform_vector(&viewDirection);

        let mut speedDirection = *direction;
        speedDirection = realStartMatrix.transform_vector(&speedDirection);

        let speedDirectionLength = directionLength;

        viewDirection = normalize(&viewDirection);
        speedDirection = normalize(&speedDirection);
        let mut zAxis = viewDirection;
        let mut yAxis = speedDirection;
        let mut xAxis = multiply(&speedDirection, &zAxis);
        // Vector3.CrossToRef(speedDirection, zAxis, xAxis);
        let xSquareLength = xAxis.magnitude_squared();
        if xSquareLength == 0. {
            xAxis[0] = 1.0;
        } else {
            normalize(&xAxis);
            xAxis *= xSquareLength.sqrt();
        }

        zAxis = xAxis.cross(&yAxis);
        yAxis = normalize(&yAxis);

        *realStartMatrix = Matrix::new(
            xAxis[0],
            yAxis[0],
            zAxis[0],
            tempVector3Scaling[0],
            xAxis[1],
            yAxis[1],
            zAxis[1],
            tempVector3Scaling[1],
            xAxis[2],
            yAxis[2],
            zAxis[2],
            tempVector3Scaling[2],
            0.,
            0.,
            0.,
            1.,
        );

        tempVector3Scaling = scaling;
        tempVector3Scaling[1] *= stretchedLengthScale;
        tempVector3Scaling[1] += stretchedVelocityScale * speedDirectionLength;
        speedDirection = Vector3::new(0., -(tempVector3Scaling[1]) / 2., 0.);

        let mut TempQuaternion_0 =
            Quaternion::from_quaternion(nalgebra::Quaternion::new(0., 0., 0., 1.));
        let euler_angles = TempQuaternion_0.euler_angles();
        *resultMatrix = Matrix::new_nonuniform_scaling(&tempVector3Scaling)
            * Matrix::from_euler_angles(euler_angles.0, euler_angles.1, euler_angles.2)
            * Matrix::new_translation(&speedDirection);

        *resultMatrix = resultMatrix.cross(&realStartMatrix);

        TempQuaternion_0 = Quaternion::from_euler_angles(
            _StretchedBillboardRot[0],
            _StretchedBillboardRot[1],
            _StretchedBillboardRot[2],
        );
        // // Matrix.ComposeToRef(TempVector3One, TempQuaternion_0, TempVector3Zero, TempMatrix_1);
        let euler_angles = TempQuaternion_0.euler_angles();
        let TempMatrix_1 =
            Matrix::from_euler_angles(euler_angles.0, euler_angles.1, euler_angles.2);

        *resultMatrix = TempMatrix_1.cross(&resultMatrix);
    }
}

pub struct HorizontalBillboard;
impl HorizontalBillboard {
    pub fn compute(
        parentWorldMatrix: Matrix,
        realStartMatrix: &mut Matrix,
        resultMatrix: &mut Matrix,
        cameraRotationMatrix: Matrix,
        camraGlobalPos: Vector3,
        direction: &mut Vector3,
        directionLength: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        stretchedLengthScale: f32,
        stretchedVelocityScale: f32,
    ) {
        let tempVector3Translation = parentWorldMatrix.transform_vector(&position);

        realStartMatrix[3] = 0.;
        realStartMatrix[7] = 0.;
        realStartMatrix[11] = 0.;

        *realStartMatrix = Matrix::new_nonuniform_scaling(&scaling)
            * Matrix::from_euler_angles(rotation[2], rotation[1], rotation[0])
            * (*realStartMatrix);

        let mut tempVector3Scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            realStartMatrix,
            Some(&mut tempVector3Scaling),
            None,
            None,
        );

        *resultMatrix = Matrix::new_nonuniform_scaling(&tempVector3Scaling)
            * Matrix::from_euler_angles(std::f32::consts::PI / 2., 0., 0.)
            * Matrix::new_translation(&tempVector3Translation);
    }
}

pub struct VerticalBillboard;
impl VerticalBillboard {
    pub fn compute(
        parentWorldMatrix: Matrix,
        realStartMatrix: &mut Matrix,
        resultMatrix: &mut Matrix,
        cameraRotationMatrix: Matrix,
        camraGlobalPos: Vector3,
        direction: &mut Vector3,
        directionLength: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        stretchedLengthScale: f32,
        stretchedVelocityScale: f32,
    ) {
        let tempVector3Translation = parentWorldMatrix.transform_vector(&position);

        realStartMatrix[3] = 0.;
        realStartMatrix[7] = 0.;
        realStartMatrix[11] = 0.;

        *realStartMatrix = Matrix::new_nonuniform_scaling(&scaling)
            * Matrix::from_euler_angles(rotation[2], rotation[1], rotation[0])
            * (*realStartMatrix);

        let mut tempVector3Scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            realStartMatrix,
            Some(&mut tempVector3Scaling),
            None,
            None,
        );

        let mut tempVector3A = TempVector3Translation - camraGlobalPos;
        tempVector3A[1] = 0.;
        tempVector3A[2] = 0.;
        if (tempVector3A[0] == 0. && tempVector3A[2] == 0.) {
            tempVector3A[1] = 1.;
        }
        tempVector3A = normalize(&tempVector3A);
        // Vector3.NormalizeToRef(TempVector3A, TempVector3A);
        let mut TempQuaternion_0 = Vector4::identity();
        direction_to_quaternion(tempVector3A, &mut TempQuaternion_0);

        let mut rotation = Quaternion::from_quaternion(nalgebra::Quaternion::new(
            TempQuaternion_0[0],
            TempQuaternion_0[1],
            TempQuaternion_0[2],
            TempQuaternion_0[2],
        ))
        .euler_angles();

        *resultMatrix = Matrix::new_nonuniform_scaling(&scaling)
            * Matrix::from_euler_angles(rotation.0, rotation.1, rotation.2)
            * Matrix::new_translation(&tempVector3Translation);
    }
}

pub struct RenderAlignmentView;

impl RenderAlignmentView {
    pub fn compute(
        parentWorldMatrix: Matrix,
        realStartMatrix: &mut Matrix,
        resultMatrix: &mut Matrix,
        cameraRotationMatrix: Matrix,
        camraGlobalPos: Vector3,
        direction: &mut Vector3,
        directionLength: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        stretchedLengthScale: f32,
        stretchedVelocityScale: f32,
    ) {
        let tempVector3Translation = parentWorldMatrix.transform_vector(&position);

        realStartMatrix[3] = 0.;
        realStartMatrix[7] = 0.;
        realStartMatrix[11] = 0.;

        // 自身旋转
        *realStartMatrix = Matrix::new_nonuniform_scaling(&scaling)
            * Matrix::from_euler_angles(rotation[2], rotation[1], rotation[0])
            * (*realStartMatrix);

        let mut tempVector3Scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            realStartMatrix,
            Some(&mut tempVector3Scaling),
            None,
            None,
        );

        // 相机方向
        let mut tempQuaternion_0 = Rotation3::identity();
        CoordinateSytem3::matrix4_decompose_rotation(
            &cameraRotationMatrix,
            None,
            Some(&mut tempQuaternion_0),
            None,
        );
        // 叠加自身旋转
        let tempQuaternion_0 = tempQuaternion_0.rotation_to(&tempQuaternion_0);
        // let mut tempQuaternion_0 =
        //     Vector3::new(tempQuaternion_0.0, tempQuaternion_0.1, tempQuaternion_0.2)
        //         .cross(&rotation);
        // let tempQuaternion_0 = tempQuaternion_0.matrix();
        let tempQuaternion_0 = tempQuaternion_0.euler_angles();
        *resultMatrix = Matrix::new_nonuniform_scaling(&tempVector3Scaling)
            * Matrix::from_euler_angles(tempQuaternion_0.0, tempQuaternion_0.1, tempQuaternion_0.2)
            * Matrix::new_translation(&tempVector3Translation);
    }
}

pub struct RenderAlignmentWorld;

impl RenderAlignmentWorld {
    pub fn compute(
        parentWorldMatrix: Matrix,
        realStartMatrix: &mut Matrix,
        resultMatrix: &mut Matrix,
        cameraRotationMatrix: Matrix,
        camraGlobalPos: Vector3,
        direction: &mut Vector3,
        directionLength: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        stretchedLengthScale: f32,
        stretchedVelocityScale: f32,
    ) {
        let tempVector3Translation = parentWorldMatrix.transform_vector(&position);

        realStartMatrix[3] = 0.;
        realStartMatrix[7] = 0.;
        realStartMatrix[11] = 0.;

        // Local - Scaling - Rotation
        // 自身旋转
        *realStartMatrix = Matrix::new_nonuniform_scaling(&scaling)
            * Matrix::from_euler_angles(rotation[0], rotation[1], rotation[2])
            * (*realStartMatrix);

        let mut tempVector3Scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            realStartMatrix,
            Some(&mut tempVector3Scaling),
            None,
            None,
        );
        let tempQuaternion_0 =
            Quaternion::from_quaternion(nalgebra::Quaternion::new(0., 0., 0., 1.)).euler_angles();

        *resultMatrix = Matrix::new_nonuniform_scaling(&tempVector3Scaling)
            * Matrix::from_euler_angles(tempQuaternion_0.0, tempQuaternion_0.1, tempQuaternion_0.2)
            * Matrix::new_translation(&tempVector3Translation);
    }
}

pub struct RenderAlignmentFacing {
    useRightHandedSystem: bool,
}

impl RenderAlignmentFacing {
    pub fn compute(
        parentWorldMatrix: Matrix,
        realStartMatrix: &mut Matrix,
        resultMatrix: &mut Matrix,
        cameraRotationMatrix: Matrix,
        camraGlobalPos: Vector3,
        direction: &mut Vector3,
        directionLength: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        stretchedLengthScale: f32,
        stretchedVelocityScale: f32,
    ) {
        let tempVector3Translation = parentWorldMatrix.transform_vector(&position);

        realStartMatrix[3] = 0.;
        realStartMatrix[7] = 0.;
        realStartMatrix[11] = 0.;

        // Local - Scaling - Rotation
        // 自身旋转
        *realStartMatrix = Matrix::new_nonuniform_scaling(&scaling)
            * Matrix::from_euler_angles(rotation[0], rotation[1], rotation[2])
            * (*realStartMatrix);

        let mut tempVector3Scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            realStartMatrix,
            Some(&mut tempVector3Scaling),
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
        let tempQuaternion_0 =
            cameraRotationMatrix * Matrix::from_euler_angles(rotation[0], rotation[1], rotation[2]);
        let tempQuaternion_0 = tempQuaternion_0.normalize();

        *resultMatrix = Matrix::new_nonuniform_scaling(&tempVector3Scaling)
            * Matrix::from_euler_angles(
                tempQuaternion_0[0],
                tempQuaternion_0[1],
                tempQuaternion_0[2],
            )
            * Matrix::new_translation(&tempVector3Translation);
    }
}

pub struct RenderAlignmentVelocity;
impl RenderAlignmentVelocity {
    pub fn compute(
        parentWorldMatrix: Matrix,
        realStartMatrix: &mut Matrix,
        resultMatrix: &mut Matrix,
        cameraRotationMatrix: Matrix,
        camraGlobalPos: Vector3,
        direction: &mut Vector3,
        directionLength: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        stretchedLengthScale: f32,
        stretchedVelocityScale: f32,
    ) {
        let tempVector3Translation = parentWorldMatrix.transform_vector(&position);

        realStartMatrix[3] = 0.;
        realStartMatrix[7] = 0.;
        realStartMatrix[11] = 0.;

        let rotation = Rotation3::from_euler_angles(rotation[0], rotation[1], rotation[2]);
        // Local - Scaling - Rotation
        // 自身旋转
        let tempMatrix_1 = Matrix::new_nonuniform_scaling(&scaling)
            * rotation.to_homogeneous();

        // let direction = direction * 1.;
        let tempVector3A = realStartMatrix.transform_vector(direction);
        // Vector3.TransformCoordinatesToRef(TempVector3A, realStartMatrix, TempVector3A);
        let tempVector3A = normalize(&tempVector3A);
        // Vector3.NormalizeToRef(TempVector3A, TempVector3A);
        // 如何旋转以`看向`速度方向
        let TempMatrix_View = Isometry3::look_at_lh(
            &Point3::new(0., 0., 0.),
            &Point3::new(tempVector3A[0], tempVector3A[1], tempVector3A[2]),
            &Vector3::new(0., 1., 0.),
        );
        let tempQuaternion_0 = TempMatrix_View.rotation.euler_angles();
        let tempQuaternion_0 = Rotation3::from_euler_angles(tempQuaternion_0.0, tempQuaternion_0.1, tempQuaternion_0.2);
        // let tempQuaternion_0 =
        //     Vector3::new(tempQuaternion_0.0, tempQuaternion_0.1, tempQuaternion_0.2);
        // TempMatrix_View.getRotationMatrixToRef(TempMatrix_View);
        // TempQuaternion_0.fromRotationMatrix(TempMatrix_View);
        // 叠加自身旋转
        let tempQuaternion_0 = tempQuaternion_0.rotation_to(&rotation);
        let tempQuaternion_0 = tempQuaternion_0;

        *realStartMatrix = tempMatrix_1 * (*realStartMatrix);
        // realStartMatrix.decompose(TempVector3Scaling, undefined, undefined);
        let mut tempVector3Scaling = Vector3::new(1., 1., 1.);
        CoordinateSytem3::matrix4_decompose_rotation(
            realStartMatrix,
            Some(&mut tempVector3Scaling),
            None,
            None,
        );

        *resultMatrix = Matrix::new_nonuniform_scaling(&tempVector3Scaling)
            * tempQuaternion_0.to_homogeneous()
            * Matrix::new_translation(&tempVector3Translation);
    }
}

pub struct RenderAlignmentLocal;
impl RenderAlignmentLocal {
    pub fn compute(
        parentWorldMatrix: Matrix,
        realStartMatrix: &mut Matrix,
        resultMatrix: &mut Matrix,
        cameraRotationMatrix: Matrix,
        camraGlobalPos: Vector3,
        direction: &mut Vector3,
        directionLength: f32,
        position: Vector3,
        scaling: Vector3,
        rotation: Vector3,
        stretchedLengthScale: f32,
        stretchedVelocityScale: f32,
    ) {
        let tempVector3Translation = parentWorldMatrix.transform_vector(&position);
        // println!("tempVector3Translation: {:?}", tempVector3Translation);
        // println!("parentWorldMatrix: {:?}", parentWorldMatrix);
        // println!("position: {:?}", position);

        realStartMatrix[3] = 0.;
        realStartMatrix[7] = 0.;
        realStartMatrix[11] = 0.;

        let tempMatrix_1 = Matrix::new_nonuniform_scaling(&scaling)
            * Matrix::from_euler_angles(rotation[0], rotation[1], rotation[2])
            * (*realStartMatrix);
        *resultMatrix = tempMatrix_1 * (*realStartMatrix);

        // resultMatrix(TempVector3Translation);
        resultMatrix[3] = tempVector3Translation[0];
        resultMatrix[7] = tempVector3Translation[1];
        resultMatrix[11] = tempVector3Translation[2];
    }
}
