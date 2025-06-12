
    mat4 influence   = readMatrixFromTex(sampler2D(_boneTex, sampler_boneTex), A_JOINT_INC3[0], bondTexSize.x, 0., bondTexSize.y)  * A_JOINT_WEG3[0];
    influence       += readMatrixFromTex(sampler2D(_boneTex, sampler_boneTex), A_JOINT_INC3[1], bondTexSize.x, 0., bondTexSize.y) * A_JOINT_WEG3[1];
    influence       += readMatrixFromTex(sampler2D(_boneTex, sampler_boneTex), A_JOINT_INC3[2], bondTexSize.x, 0., bondTexSize.y) * A_JOINT_WEG3[2];
    PI_ObjectToWorld = PI_ObjectToWorld * influence; 
