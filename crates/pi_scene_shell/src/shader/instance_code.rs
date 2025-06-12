
pub const INVERSE: &'static str = "
mat4 inverse(mat4 m) {
    vec4 m0 = m[0];
    vec4 m1 = m[1];
    vec4 m2 = m[2];
    vec4 m3 = m[3];
    float det_22_33 = m2[2] * m3[3] - m3[2] * m2[3];
    float det_21_33 = m2[1] * m3[3] - m3[1] * m2[3];
    float det_21_32 = m2[1] * m3[2] - m3[1] * m2[2];
    float det_20_33 = m2[0] * m3[3] - m3[0] * m2[3];
    float det_20_32 = m2[0] * m3[2] - m2[2] * m3[0];
    float det_20_31 = m2[0] * m3[1] - m3[0] * m2[1];
    
    float cofact_00 = +(m1[1] * det_22_33 - m1[2] * det_21_33 + m1[3] * det_21_32);
    float cofact_01 = -(m1[0] * det_22_33 - m1[2] * det_20_33 + m1[3] * det_20_32);
    float cofact_02 = +(m1[0] * det_21_33 - m1[1] * det_20_33 + m1[3] * det_20_31);
    float cofact_03 = -(m1[0] * det_21_32 - m1[1] * det_20_32 + m1[2] * det_20_31);

    float det = m0[0] * cofact_00 + m0[1] * cofact_01 + m0[2] * cofact_02 + m0[3] * cofact_03;

    if (abs(det) < 0.000000001) {
        return mat4(vec4(1., 0., 0., 0.), vec4(0., 1., 0., 0.), vec4(0., 0., 1., 0.), vec4(0., 0., 0., 1.))
    }
    
    float detInv = 1 / det;
    float det_12_33 = m1[2] * m3[3] - m3[2] * m1[3];
    float det_11_33 = m1[1] * m3[3] - m3[1] * m1[3];
    float det_11_32 = m1[1] * m3[2] - m3[1] * m1[2];
    float det_10_33 = m1[0] * m3[3] - m3[0] * m1[3];
    float det_10_32 = m1[0] * m3[2] - m3[0] * m1[2];
    float det_10_31 = m1[0] * m3[1] - m3[0] * m1[1];
    float det_12_23 = m1[2] * m2[3] - m2[2] * m1[3];
    float det_11_23 = m1[1] * m2[3] - m2[1] * m1[3];
    float det_11_22 = m1[1] * m2[2] - m2[1] * m1[2];
    float det_10_23 = m1[0] * m2[3] - m2[0] * m1[3];
    float det_10_22 = m1[0] * m2[2] - m2[0] * m1[2];
    float det_10_21 = m1[0] * m2[1] - m2[0] * m1[1];

    float cofact_10 = -(m0[1] * det_22_33 - m0[2] * det_21_33 + m0[3] * det_21_32);
    float cofact_11 = +(m0[0] * det_22_33 - m0[2] * det_20_33 + m0[3] * det_20_32);
    float cofact_12 = -(m0[0] * det_21_33 - m0[1] * det_20_33 + m0[3] * det_20_31);
    float cofact_13 = +(m0[0] * det_21_32 - m0[1] * det_20_32 + m0[2] * det_20_31);

    float cofact_20 = +(m0[1] * det_12_33 - m0[2] * det_11_33 + m0[3] * det_11_32);
    float cofact_21 = -(m0[0] * det_12_33 - m0[2] * det_10_33 + m0[3] * det_10_32);
    float cofact_22 = +(m0[0] * det_11_33 - m0[1] * det_10_33 + m0[3] * det_10_31);
    float cofact_23 = -(m0[0] * det_11_32 - m0[1] * det_10_32 + m0[2] * det_10_31);

    float cofact_30 = -(m0[1] * det_12_23 - m0[2] * det_11_23 + m0[3] * det_11_22);
    float cofact_31 = +(m0[0] * det_12_23 - m0[2] * det_10_23 + m0[3] * det_10_22);
    float cofact_32 = -(m0[0] * det_11_23 - m0[1] * det_10_23 + m0[3] * det_10_21);
    float cofact_33 = +(m0[0] * det_11_22 - m0[1] * det_10_22 + m0[2] * det_10_21);
    return mat4(
        vec4(cofact_00, cofact_10, cofact_20, cofact_30), 
        vec4(cofact_01, cofact_11, cofact_21, cofact_31), 
        vec4(cofact_02, cofact_12, cofact_22, cofact_32), 
        vec4(cofact_03, cofact_13, cofact_23, cofact_33)
    ) * detInv;
}
";