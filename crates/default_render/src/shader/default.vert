
layout(location = 0) out vec3 v_normal;
layout(location = 1) out vec3 v_pos;
layout(location = 2) out vec4 v_color;

void main() {
    vec3 position = A_POSITION;
    vec3 normal = A_NORMAL;
    mat4 finalWorld = PI_ObjectToWorld;

    vec4 positionUpdate =  vec4(position, 1.);
    vec4 worldPos =  finalWorld * positionUpdate;
    // vec4 worldPos =  positionUpdate;

    gl_Position = PI_MATRIX_VP * worldPos;
    // gl_Position = positionUpdate;

    v_pos = worldPos.xyz;

    mat3 normalWorld = mat3(finalWorld);
    v_normal = normal; //normalize(vec3(finalWorld * vec4(normal, 1.0)));

    v_color = A_COLOR4;
}
