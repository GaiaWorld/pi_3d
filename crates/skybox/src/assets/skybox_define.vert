

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
