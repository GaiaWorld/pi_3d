
mat4 rotMatrixFromForward(const mat4 m, const mat4 vr, const vec3 position, const vec3 viewpos) {
    vec3 forward = normalize(position - viewpos);
    vec3 up = normalize(vec3(vr * vec4(0., 1., 0., 1.)));
    vec3 left = cross(up, forward);
    up = cross(forward, left);
    return m * mat4(vec4(left, 0.), vec4(up, 0.), vec4(forward, 0.), vec4(0., 0.,0., 1.));
}
