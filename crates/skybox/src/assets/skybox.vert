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
layout(location = 1) out float v_dist;

float applyFog(float dis,  // camera to point distance
               vec3  rayOri,    // camera position
               vec3  rayDir )   // camera to point vector
{
    float a = 0.8;              // 雾的起始浓度。
    float b = 1.0;              // 雾随高度的衰减指数。
    float startDis = 0.1;       // 一定距离内物体保持清晰，不受雾影响。
    float startHeight = 0.0;    // 雾的起始高度

    // vec3 rayOri_pie=rayOri+rayDir*startDis;
    // float c=a/b;

    // vec2 data = vec2(-max(0,rayOri_pie.y-startHeight)*b,-max(0,dis-startDis)*rayDir.y*b);
    // vec2 expData = exp(data);
    // float opticalThickness = c * expData.x * (1.0-expData.y)/rayDir.y;
    // float extinction = exp(-opticalThickness);
    // float fogAmount = 1-extinction;
    float fogAmount = (a/b) * exp(-rayOri.y*b) * (1.0-exp( -dis*rayDir.y*b ))/rayDir.y;
    return fogAmount;
}

void main() {
    mat4 finalWorld = PI_MATRIX_P * PI_MATRIX_V;

    vec4 positionUpdate = vec4(a_position * 2.0, 1.);

    positionUpdate = PI_ObjectToWorld * positionUpdate;
    gl_Position = PI_MATRIX_VP * positionUpdate;
    // gl_Position = vec4(a_position * 0.5, 1.);

    mat3 normalWorld = mat3(PI_ObjectToWorld);
    v_normal = normalize(normalWorld * a_normal);
    v_normal = a_normal;
    v_dist = applyFog(distance(positionUpdate.xyz, PI_CAMERA_POSITION.xyz), PI_CAMERA_POSITION.xyz, PI_CAMERA_POSITION.xyz - positionUpdate.xyz);

    // v_dist = distance(PI_MATRIX_VP * positionUpdate, PI_CAMERA_POSITION) ;

}

