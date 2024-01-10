
use pi_scene_math::{*, coordiante_system::CoordinateSytem3, vector::TToolVector3};

use crate::interpolation::{Color4Gradient, FloatInterpolation, BaseRandom, IInterpolation};

#[derive(Debug, Clone)]
pub struct PathPoint {
    pub pos: Vector3,
    pub xaxis: Vector3,
    pub zaxis: Vector3,
    pub time: u32,
    pub distance: Number,
    pub distance_percent: Number,
}

pub struct PathPoints {
    pub pos: Vec<Vector3>,
    pub xaxis: Vec<Vector3>,
    pub zaxis: Vec<Vector3>,
    pub color: Vec<Vector4>,
    pub distances: Vec<Number>,
    pub total_distance: Number,
}

impl PathPoints {
    pub fn path_update_point(
        newpos: Vector3,
        newaxisx: Vector3,
        newaxisz: Vector3,
        newtime: u32,
        list: &mut Vec<PathPoint>,
        limit_time: u32,
        limit_distance: Number,
        limit_between_distance: Number,
    ) -> (Vec<PathPoint>, Number) {
        let lastpos = if let Some(point) = list.pop() {
            point
        } else {
            PathPoint {
                pos: newpos.clone(),
                xaxis: newaxisx.clone(),
                zaxis: newaxisz.clone(),
                time: newtime,
                distance: 0.,
                distance_percent: 0.,
            }
        };
        let newkey = PathPoint {
            pos: newpos,
            xaxis: newaxisx,
            zaxis: newaxisz,
            time: newtime,
            distance: 0.,
            distance_percent: 0.,
        };

        if let Some(lastkey) = list.last() {
            let distance = CoordinateSytem3::distance(&lastkey.pos, &newkey.pos);
            if limit_between_distance <= distance {
                let mut newlast = newkey.clone();
                newlast.distance = distance;
                list.push(newlast);
            } else {
            list.push(lastpos);
            }
            list.push(newkey);
        } else {
            list.push(lastpos);
            list.push(newkey);
        }

        let count = list.len();
        let mut total_distance = 0.;

        let mut index = 0;
        let mut flag = true;
        for idx in 1..count {
            if flag {
                index = count - idx - 1;
                let point = &mut list[index];
                total_distance += point.distance;
                point.distance_percent = total_distance;

                if point.time <= limit_time || limit_distance <= total_distance {
                    flag = false;
                }
            }
        }

        total_distance = f32::min(total_distance, limit_distance);

        let mut result = list.split_off(index);
        result.iter_mut().for_each(|point| {
            // log::warn!("Point: {:?}  {:?}", point.time, point.distance_percent);
            if total_distance < point.distance_percent {
                let zlen = CoordinateSytem3::length(&point.zaxis);
                if 0.0 < zlen {
                    point.pos += point.zaxis.scale(1. / zlen * (point.distance_percent - total_distance));
                }
            }
            point.distance_percent = 1.0 - f32::min(1., point.distance_percent / total_distance);
        });

        (result, total_distance)
    }
    pub fn path_color(
        list: &Vec<PathPoint>,
        randoms: &BaseRandom,
        colorcontrol: &Vector4,
        colorinterpolator: &Color4Gradient,
    ) -> Vec<Vector4> {
        let mut result = vec![];

        list.iter().for_each(|point| {
            let mut color = [0., 0., 0., 0.];
            colorinterpolator.interpolate(point.distance_percent, &mut color, randoms);
            let color = Vector4::new(color[0] * colorcontrol.x, color[1] * colorcontrol.y, color[2] * colorcontrol.z, color[3] * colorcontrol.w);
            result.push(color);
        });

        result
    }
    pub fn path_width(
        list: &Vec<PathPoint>,
        randoms: &BaseRandom,
        width_control: Number,
        width_interpolator: &FloatInterpolation,
    ) -> Vec<Number> {
        let mut result = vec![];

        list.iter().for_each(|point| {
            let width = width_control * width_interpolator.interpolate(point.distance_percent, randoms.x) * CoordinateSytem3::length(&point.xaxis);
            result.push(width * 0.5);
        });

        result
    }
}