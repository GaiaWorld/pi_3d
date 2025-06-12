
mat4 axisMatrix(const vec3 xAxis, const vec3 yAxis, const vec3 zAxis) {
    return mat4(vec4(xAxis, 0.),vec4(yAxis, 0.),vec4(zAxis, 0.),vec4(0., 0., 0., 1.));
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
