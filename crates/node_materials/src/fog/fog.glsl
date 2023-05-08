
#define FOGMODE_NONE 0.
#define FOGMODE_EXP 1.
#define FOGMODE_EXP2 2.
#define FOGMODE_LINEAR 3.
#define FOGMODE_HIGHT_BASE 4.

// 普通雾
vec3 applyFog(
    in vec3 rgb,      // 像素颜色
    in vec3 fogColor, // 雾颜色
    in float distance,  // 相机坐标到像素点坐标距离
    float b             // b 控制雾气强度随距离增强的速度
) {
    float fogAmount = 1.0 - exp( -distance * b );
    return mix(rgb, fogColor, fogAmount);
}

// 受太阳光照影响的雾
vec3 applyFog(
    in vec3 rgb,
    in vec3 fogColor,
    in float distance,
    in vec3 rayDir,   // 相机坐标到像素点坐标的向量
    in vec3 sunDir,   // 太阳光照方向向量
    in vec3 sunColor, // 太阳光颜色
    float b
) {
    
    float fogAmount = 1.0 - exp( -distance * b );
    float sunAmount = max( dot( rayDir, sunDir ), 0.0 );
    vec3 color = mix( fogColor, sunColor, pow( sunAmount, 8.0 ) );
    return mix(rgb, fogColor, fogAmount);
}

// 基于高度积分的雾
vec3 applyFog(
    in vec3 rgb,
    in vec3 fogColor,
    in float distance,
    in vec3 rayOri,       // Camera Porition
    in vec3 rayDir,
    float baseH,            // 最大浓度时的高度(伪)
    float a,                //
    float b                 // 衰减系数 - Fallof
) {
    // baseFunction : d(y) = a * exp(-b * y) - https://www.iquilezles.org/www/articles/fog/fog.htm
    float fogAmount = (a / b) * exp(-b * (rayOri.y - baseH)) * (1.0 - exp(-b * distance * (rayDir.y)) ) / (rayDir.y);
    return mix(rgb, fogColor, clamp(fogAmount, 0.0, 1.0));
}

float CalcFogFactor(vec3 vFogDistance) {
    vec4 vFogInfos    = PI_FogParam;

    float fogCoeff      = 1.0;
    float fogStart      = vFogInfos.y;
    float fogEnd        = vFogInfos.z;
    float fogDensity    = vFogInfos.w;
    float fogDistance   = abs(length(vFogDistance));
    if (FOGMODE_LINEAR == vFogInfos.x) {
        fogCoeff = (fogEnd-fogDistance)/(fogEnd-fogStart);
    }
    else if (FOGMODE_HIGHT_BASE == vFogInfos.x) {
        fogCoeff = 1.0/pow(E, fogDistance*fogDistance*fogDensity*fogDensity);
    }
    else if (FOGMODE_EXP == vFogInfos.x) {
        fogCoeff = 1.0/pow(E, fogDistance*fogDensity);
    }
    else if (FOGMODE_EXP2 == vFogInfos.x) {
        fogCoeff = 1.0/pow(E, fogDistance*fogDistance*fogDensity*fogDensity);
    }

    return clamp(fogCoeff, 0.0, 1.0);
}

float CalcFogFactor(float fogDistance) {
    vec4 vFogInfos    = _FogInfo;

    float fogCoeff      = 1.0;
    float fogStart      = vFogInfos.y;
    float fogEnd        = vFogInfos.z;
    float fogDensity    = vFogInfos.w;
    if (FOGMODE_LINEAR == vFogInfos.x) {
        fogCoeff = (fogEnd-fogDistance)/(fogEnd-fogStart);
    }
    else if (FOGMODE_EXP == vFogInfos.x) {
        fogCoeff = 1.0/pow(E, fogDistance*fogDensity);
    }
    else if (FOGMODE_EXP2 == vFogInfos.x) {
        fogCoeff = 1.0/pow(E, fogDistance*fogDistance*fogDensity*fogDensity);
    }

    return clamp(fogCoeff, 0.0, 1.0);
}

vec3 applyFog(
    vec3 vFogDistance,
    vec3 finalColor,
    vec3 vFogColor,
    vec3 rayOri,
    vec3 rayDir
) {
    float fogDistance   = length(vFogDistance);
    if (FOGMODE_HIGHT_BASE == PI_FogParam.x) {
        return applyFog(finalColor, PI_FogInfo.rgb, fogDistance, rayOri, rayDir, PI_FogParam.y, PI_FogParam.z, PI_FogParam.w);
    }
    else {
        float fog 					= CalcFogFactor(abs(fogDistance));
        return finalColor.rgb 		= mix(PI_FogInfo.rgb, finalColor, fog);
    }
}
