
const int ALIGNMENT_VIEW = 0;
const int ALIGNMENT_FACING = 3;
const int ALIGNMENT_STRETCHED = 5;
const int ALIGNMENT_VERTICAL = 7;
mat4 rotMatrixFromForward(const mat4 m, const mat4 vr, const vec3 position, const vec3 viewpos) {
    vec3 forward = normalize(position - viewpos);
    vec3 up = normalize(vec3(vr * vec4(0., 1., 0., 1.)));
    vec3 left = cross(up, forward);
    up = cross(forward, left);
    return m * mat4(vec4(left, 0.), vec4(up, 0.), vec4(forward, 0.), vec4(0., 0.,0., 1.));
}
mat4 axisMatrix(const vec3 xAxis, const vec3 yAxis, const vec3 zAxis) {
    return mat4(vec4(xAxis, 0.),vec4(yAxis, 0.),vec4(zAxis, 0.),vec4(0., 0., 0., 1.));
}
mat4 lookat(const vec3 eye, const vec3 target, const vec3 up) {
    vec3 dir = target - eye;
    vec3 zAxis = normalize(dir);
    vec3 xAxis = cross(up, zAxis);
    float xSquareLength = length(xAxis);
    if (xSquareLength < 0.000001) {
        xAxis.x = 1.0;
    } else {
        xAxis = normalize(xAxis);
    }
    vec3 yAxis = normalize(cross(zAxis, xAxis));
    return axisMatrix(xAxis, yAxis, zAxis);
}
mat4 rotMatrixStretched(const mat4 m, const vec3 viewDirection) {
    mat4 invm = inverse(m);
    vec3 xAxis = vec3(1., 0., 0.);
    vec3 zAxis = normalize((invm * vec4(viewDirection, 0.)).xyz);
    vec3 yAxis = normalize(cross(zAxis, xAxis));
    zAxis = normalize(cross(xAxis, yAxis));
    mat4 lm = axisMatrix(xAxis, yAxis, zAxis);
    return m * lm;
}
mat4 matrixVertical(const mat4 m, const vec3 viewDirection) {
    mat4 invm = inverse(m);
    vec3 yAxis = normalize((invm * vec4(vec3(0., 1., 0.), 0.)).xyz);
    vec3 zAxis = normalize((invm * vec4(viewDirection, 0.)).xyz);
    vec3 xAxis = normalize(cross(yAxis, zAxis));
    zAxis = normalize(cross(xAxis, yAxis));
    mat4 lm = axisMatrix(xAxis, yAxis, zAxis);
    return m * lm;
}
