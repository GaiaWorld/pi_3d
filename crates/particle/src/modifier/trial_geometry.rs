/**
 * 一个关键点 增加 两个绘制点 两个UV点 2个三角形
 */
pub struct TrailGeometryModifier;
impl TrailGeometryModifier {
    const triangleCount: f32 = 2.;
    pub fn modifyPosition(
        ax: f32,
        ay: f32,
        az: f32,
        extendX: f32,
        extendY: f32,
        extendZ: f32,
        positiondata: &mut Vec<f32>,
        pointIndex: usize,
    ) {
        let offset = pointIndex * 2 * 3; // 2 个 vec3
        positiondata[offset + 0] = ax - extendX;
        positiondata[offset + 1] = ay - extendY;
        positiondata[offset + 2] = az - extendZ;

        positiondata[offset + 3] = ax + extendX;
        positiondata[offset + 4] = ay + extendY;
        positiondata[offset + 5] = az + extendZ;
    }
    pub fn modifyUV(au: f32, uvdata: &mut Vec<f32>, pointIndex: usize) {
        let offset = pointIndex * 2 * 2;
        uvdata[offset + 0] = au;
        uvdata[offset + 1] = 0.;

        uvdata[offset + 2] = au;
        uvdata[offset + 3] = 1.;
    }
    pub fn modifyColor(r: f32, g: f32, b: f32, a: f32, colordata: &mut Vec<f32>, pointIndex: usize) {
        let offset = pointIndex * 2 * 4;
        colordata[offset + 0] = r;
        colordata[offset + 1] = g;
        colordata[offset + 2] = b;
        colordata[offset + 3] = a;

        colordata[offset + 4] = r;
        colordata[offset + 5] = g;
        colordata[offset + 6] = b;
        colordata[offset + 7] = a;
    }
    pub fn modifyIndices(indicesdata: &mut Vec<f32>, pointIndex: usize) {
        let offset = (pointIndex - 1) * 2 * 3; // 2 个 三角形
        let indicesOffset = (pointIndex - 1) * 2;

        indicesdata[offset + 0] = indicesOffset as f32 + 0.;
        indicesdata[offset + 1] = indicesOffset as f32 + 1.;
        indicesdata[offset + 2] = indicesOffset as f32 + 2.;

        indicesdata[offset + 3] = indicesOffset as f32 + 1.;
        indicesdata[offset + 4] = indicesOffset as f32 + 3.;
        indicesdata[offset + 5] = indicesOffset as f32 + 2.;
    }
}
