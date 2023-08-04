/**
 * 一个关键点 增加 两个绘制点 两个UV点 2个三角形
 */
pub struct TrailGeometryModifier;
impl TrailGeometryModifier {
    pub fn modify_position(
        ax: f32,
        ay: f32,
        az: f32,
        extend_x: f32,
        extend_y: f32,
        extend_z: f32,
        positiondata: &mut Vec<f32>,
        point_index: usize,
    ) {
        let offset = point_index * 2 * 3; // 2 个 vec3
        positiondata[offset + 0] = ax - extend_x;
        positiondata[offset + 1] = ay - extend_y;
        positiondata[offset + 2] = az - extend_z;

        positiondata[offset + 3] = ax + extend_x;
        positiondata[offset + 4] = ay + extend_y;
        positiondata[offset + 5] = az + extend_z;
    }
    pub fn modify_uv(au: f32, uvdata: &mut Vec<f32>, point_index: usize) {
        let offset = point_index * 2 * 2;
        uvdata[offset + 0] = au;
        uvdata[offset + 1] = 0.;

        uvdata[offset + 2] = au;
        uvdata[offset + 3] = 1.;
    }
    pub fn modify_color(r: f32, g: f32, b: f32, a: f32, colordata: &mut Vec<f32>, point_index: usize) {
        let offset = point_index * 2 * 4;
        colordata[offset + 0] = r;
        colordata[offset + 1] = g;
        colordata[offset + 2] = b;
        colordata[offset + 3] = a;

        colordata[offset + 4] = r;
        colordata[offset + 5] = g;
        colordata[offset + 6] = b;
        colordata[offset + 7] = a;
    }
    pub fn modify_indices(indicesdata: &mut Vec<f32>, point_index: usize) {
        let offset = (point_index - 1) * 2 * 3; // 2 个 三角形
        let indices_offset = (point_index - 1) * 2;

        indicesdata[offset + 0] = indices_offset as f32 + 0.;
        indicesdata[offset + 1] = indices_offset as f32 + 1.;
        indicesdata[offset + 2] = indices_offset as f32 + 2.;

        indicesdata[offset + 3] = indices_offset as f32 + 1.;
        indicesdata[offset + 4] = indices_offset as f32 + 3.;
        indicesdata[offset + 5] = indices_offset as f32 + 2.;
    }
}
