#version 450

#define SHADER_NAME vertex:Default

// Buildin Vertex >>>>>>>>>>>>>>>>
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec4 a_joint;
layout(location = 3) in vec4 a_weight;

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
    mat4 u_jointMat[2];
};



void main() {
   
    
    mat4 skinMat =
        a_weight.x * u_jointMat[int(a_joint.x)] +
        a_weight.y * u_jointMat[int(a_joint.y)] +
        a_weight.z * u_jointMat[int(a_joint.z)] +
        a_weight.w * u_jointMat[int(a_joint.w)];

    vec4 worldPosition = skinMat * vec4(a_position,1.0);

    mat4 finalWorld = PI_MATRIX_P * PI_MATRIX_V;

    vec4 positionUpdate = vec4(worldPosition.xyz * 2.0, 1.);

    positionUpdate = PI_ObjectToWorld * positionUpdate;
    gl_Position = PI_MATRIX_VP * positionUpdate;

    // vec4 cameraPosition = u_viewMatrix * worldPosition;
    // gl_Position = u_projectionMatrix * cameraPosition;

}
