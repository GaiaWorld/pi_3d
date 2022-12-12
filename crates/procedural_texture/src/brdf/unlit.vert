#version 450

#define SHADER_NAME vertex:Default

// Buildin Vertex >>>>>>>>>>>>>>>>
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec2 a_uv;

layout(set = 0, binding = 0) uniform Camera {
    mat4 PI_MATRIX_V;
    mat4 PI_MATRIX_P;
    mat4 PI_MATRIX_VP;
    vec4 PI_CAMERA_POSITION;
    vec4 PI_VIEW_DIRECTION;
};

layout(set = 0, binding = 1) uniform Time {
    vec4 PI_Time;
    vec4 PI_DeltaTime;
};

layout(set = 1, binding = 0) uniform Model {
    mat4 PI_ObjectToWorld;
    mat4 PI_WorldToObject;
};
// <<<<<<<<<<<<< Buildin Vertex

layout(set = 1, binding = 1) uniform MatParam0 {
    vec4 emissive;
    vec4 emissive_scaleoffset;
};

layout(location = 0) out vec3 v_normal;
layout(location = 1) out vec3 v_pos;
layout(location = 2) out vec2 v_UV;

void main() {
    mat4 finalWorld = PI_ObjectToWorld;

    vec4 position =  vec4(a_position, 1.);
    vec4 worldPos =  finalWorld * position;
    // vec4 worldPos =  position;

    gl_Position = PI_MATRIX_VP * worldPos;
    // gl_Position = position;

    v_pos = worldPos.xyz;

    mat3 normalWorld = mat3(finalWorld);
    v_normal = a_normal; // normalize(vec3(finalWorld * vec4(a_normal, 1.0)));
    
    v_UV = a_uv;
}