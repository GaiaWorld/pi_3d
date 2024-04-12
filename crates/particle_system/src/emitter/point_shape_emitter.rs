use pi_scene_math::{Matrix, Vector3};


use crate::tools::Random;

use super::{
    ishape_emitter_type::*,
    SerializationObject,
};

/**
 * 点发射器
 */
pub struct PointShapeEmitter {
    _max_z: f32,
    /**
     * 创建模式
     */
    _direction_mode: EShapeEmitterDirectionMode,
    /**
     * 第一发射方向
     */
    direction1: Vector3,
    /**
     * 第二发射方向
     */
    direction2: Vector3,
    pub(crate)  base: ShapeEmitter,
}

impl PointShapeEmitter {
    pub fn create() -> TypeShapeEmitter {

        let mut result = TypeShapeEmitter::new();

        result.fn_direction = Self::start_direction_function;
        result.fn_position = Self::start_position_function;
        result.fn_orbit_center = Self::orbit_center;
        result
    }

    /**
     * Serializes the particle system to a JSON object.
     * @returns the JSON object
     */
    pub fn serialize(&self) -> SerializationObject {
        SerializationObject {
            _type: Some(PointShapeEmitter::get_class_name()),
            radius: None,
            angle: None,
            direction_randomizer: None,
            radius_range: None,
            height_range: None,
            emit_from_spawn_point_only: None,
            size: None,
            direction1: Some(self.direction1),
            direction2: Some(self.direction2),
            
        }
    }

    /**
     * Parse properties from a JSON object
     * @param serializationObject defines the JSON object
     */
    pub fn parse(&mut self, arg: SerializationObject) {
        self.direction1 = arg.direction1.as_ref().unwrap().clone();
        self.direction2 = arg.direction2.as_ref().unwrap().clone();
    }

    pub fn new() -> Self{
        Self{
            _max_z: 999999999.,
            _direction_mode: EShapeEmitterDirectionMode::Unity,
            direction1: Vector3::new(0.,0.,1.),
            direction2: Vector3::new(0.,0.,1.),
            base: ShapeEmitter::new()
        }
    }
    pub fn start_direction_function<'a>(
        _shape: &'a TypeShapeEmitter,
        direction_to_update: &'a mut Vector3,
        _local_position: &'a Vector3,
        _random: &'a mut Random,
    ) {
        *direction_to_update = Vector3::new(0., 0., 1.);
    }

    pub fn start_position_function<'a>(
        _shape: &'a TypeShapeEmitter,
        position_to_update: &'a mut Vector3,
        _emission_loop: f32,
        _emission_progress: f32,
        _emission_index: f32,
        _emission_total: f32,
        _random: &'a mut Random,
    ) {
        *position_to_update = Vector3::new(0., 0., 0.);
    }
    
    pub fn orbit_center<'a>(_local_position: &'a Vector3, offset: &'a Vector3, result: &'a mut Vector3) {
        result.copy_from(offset);
    }

}

impl IShapeEmitterType for PointShapeEmitter {
    fn start_direction_function(
        &self,
        direction_to_update: &mut Vector3,
        _local_position: &Vector3,
        // is_local: bool,
        _random: &mut Random,
    ) {

        // let rand_x = random.random() * (self.direction2[0] - self.direction1[0]) + self.direction1[0];
        // let rand_z = random.random() * (self.direction2[1] - self.direction1[1]) + self.direction1[0];
        // let rand_y = random.random() * (self.direction2[2] - self.direction1[2]) + self.direction1[0];

        *direction_to_update = Vector3::new(0., 0., 1.);
    }

    fn start_position_function(
        &self,
        position_to_update: &mut Vector3,
        _emission_loop: f32,
        _emission_progress: f32,
        _emission_index: f32,
        _emission_total: f32,
        // is_local: bool,
        _random: &mut Random,
    ) {
            *position_to_update = Vector3::new(0., 0., 0.);
    }

    fn get_class_name() -> String
    where
        Self: Sized {
            return "PointParticleEmitter".to_string();
    }

    fn dispose()
    where
        Self: Sized {
        
    }

    fn set_position(&mut self, position: Vector3) {
        self.base.position = position;
    }

    fn set_rotation(&mut self, rotation: Vector3) {
        self.base.rotation = rotation;
    }

    fn set_scaling(&mut self, scaling: Vector3) {
        self.base.scaling = scaling;
    }

    fn get_postion(&self) -> Vector3 {
        self.base.position.clone()
    }

    fn get_rotation(&self) -> Vector3 {
        self.base.rotation.clone()
    }

    fn get_scaling(&self) -> Vector3 {
        self.base.scaling.clone()
    }

    fn set_local_matrix(&mut self, local_matrix: Matrix) {
        self.base.local_matrix = local_matrix;
    }

    fn set_align_direction(&mut self, align_direction: bool) {
        self.base.align_direction = align_direction;
    }
    

    fn set_randomize_direction(&mut self, randomize_direction: f32) {
        self.base.randomize_direction = randomize_direction;
    }

    fn set_spherize_direction(&mut self, spherize_direction: f32) {
        self.base.spherize_direction = spherize_direction;
    }

    fn set_randomize_position(&mut self, randomize_position: f32) {
        self.base.randomize_position = randomize_position;
    }

    fn get_local_matrix(& self) -> Matrix {
        self.base.local_matrix.clone()
    }

    fn get_align_direction(& self) -> bool {
        self.base.align_direction.clone()
    }

    fn get_randomize_direction(& self, ) -> f32 {
        self.base.randomize_direction.clone()
    }

    fn get_spherize_direction(& self) -> f32 {
        self.base.spherize_direction.clone()
    }

    fn get_randomize_position(& self) -> f32 {
        self.base.randomize_position.clone()
    }
}

// export class PointShapeEmitter implements IShapeEmitterType {

//     public rotation: BABYLON.Vector3 = new BABYLON.Vector3(0, 0, 0);
//     public position: BABYLON.Vector3 = new BABYLON.Vector3(0, 0, 0);
//     public scaling: BABYLON.Vector3 = new BABYLON.Vector3(1, 1, 1);
//     public localMatrix: BABYLON.Matrix = BABYLON.Matrix.Identity();

//     alignDirection: boolean = false;
//     randomizeDirection: number = 0;
//     spherizeDirection: number = 0;
//     randomizePosition: number = 0;
//     /**
//      *
//      */
//     public startDirectionFunction(worldMatrix: BABYLON.Matrix, directionToUpdate: BABYLON.Vector3, position: BABYLON.Vector3, localPosition: BABYLON.Vector3, isLocal: boolean): void {
//         let randX = rng.gen_range(self.direction1[0], self.direction2[0]);
//         let randY = rng.gen_range(self.direction1[1], self.direction2[1]);
//         let randZ = rng.gen_range(self.direction1[2], self.direction2[2]);

//         if (isLocal) {
//             directionToUpdate.copyFromFloats(randX, randY, randZ);
//         }
//         else {
//             BABYLON.Vector3.TransformNormalFromFloatsToRef(randX, randY, randZ, worldMatrix, directionToUpdate);
//         }
//     }

//     /**
//      * Called by the particle System when the position is computed for the created particle.
//      * @param worldMatrix is the world matrix of the particle system
//      * @param positionToUpdate is the position vector to update with the result
//      * @param particle is the particle we are computed the position for
//      * @param isLocal defines if the position should be set in local space
//      */
//     startPositionFunction(worldMatrix: BABYLON.Matrix, positionToUpdate: BABYLON.Vector3, emissionLoop: number, emissionProgress: number, emissionIndex: number, emissionTotal: number, isLocal: boolean): void {
//         if (isLocal) {
//             positionToUpdate.copyFromFloats(0, 0, 0);
//         }
//         else {
//             BABYLON.Vector3.TransformCoordinatesFromFloatsToRef(0, 0, 0, worldMatrix, positionToUpdate);
//         }
//     }

//     /**
//      * Clones the current emitter and returns a copy of it
//      * @returns the new emitter
//      */
//     public clone(): PointShapeEmitter {
//         let newOne = new PointShapeEmitter();

//         BABYLON.DeepCopier.DeepCopy(self, newOne);

//         return newOne;
//     }

//     /**
//      * @returns a string containing the class name
//      */
//     public getClassName(): string {
//         return "PointParticleEmitter";
//     }

//     public dispose() {
//         // self.direction1 = undefined;
//         // self.direction2 = undefined;
//     }
// }
