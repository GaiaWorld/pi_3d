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

void main() {
    mat4 finalWorld = PI_MATRIX_P * PI_MATRIX_V;

    vec4 positionUpdate = vec4(a_position * 2.0, 1.);

    positionUpdate = PI_ObjectToWorld * positionUpdate;
    gl_Position = PI_MATRIX_VP * positionUpdate;

}

