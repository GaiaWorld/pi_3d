#version 450

#define SHADER_NAME vertex:Default

// Buildin Vertex >>>>>>>>>>>>>>>>
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;

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

layout(location = 0) out vec3 v_normal;
layout(location = 1) out vec3 v_pos;

void main() {
    mat4 finalWorld = PI_ObjectToWorld;

    mat4 w = mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    mat4 v = mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 10.0,
        0.0, 0.0, 0.0, 1.0
    );

    mat4 p = mat4(
        0.25, 0.0, 0.0, 0.0,
        0.0, 0.25, 0.0, 0.0,
        0.0, 0.0, 0.002002002,  -1.002002,
        0.0, 0.0, 0.0, 1.0
    );

    mat4 vp = mat4(
         0.25, 0.0, 0.0, 0.0,
        0.0, 0.25, 0.0, 0.0,
        0.0, 0.0, 0.0020020019728690386,  -0.9819819927215576,
        0.0, 0.0, 0.0, 1.0
    );

    vec4 worldPos =  PI_MATRIX_VP * PI_ObjectToWorld * vec4(a_position * 0.5, 1.);

worldPos.z = (worldPos.z + 1.) * 0.5;
    gl_Position = worldPos; //  * PI_MATRIX_VP;
    v_pos = gl_Position.xyz;
    // gl_Position = vec4(a_position * 0.5, 1.);

    mat3 normalWorld = mat3(PI_ObjectToWorld);
    v_normal = normalize(normalWorld * a_normal);
    v_normal = a_normal;
}