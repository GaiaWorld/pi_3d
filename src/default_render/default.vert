#version 450

#define SHADER_NAME vertex:Default

// Buildin Vertex >>>>>>>>>>>>>>>>
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec4 a_color;

layout(set = 0, binding = 0) uniform Camera {
    mat4 PI_MATRIX_V;
    mat4 PI_MATRIX_P;
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

layout(set = 0, binding = 0) uniform Param0 {
    mat4 PI_MATRIX_V;
    mat4 PI_MATRIX_P;
};
// <<<<<<<<<<<<< Buildin Vertex

layout(location = 0) out vec4 v_color;

void main() {
    gl_Position = PI_MATRIX_P * PI_MATRIX_V * vec4(a_position, 1.);
    v_color = a_color;
}