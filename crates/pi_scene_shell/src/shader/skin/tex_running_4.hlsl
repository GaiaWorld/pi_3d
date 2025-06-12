
    mat4 influence   = readMatrixFromTex(sampler2D(_boneTex, sampler_boneTex), A_JOINT_INC[0], bondTexSize.x, 0., bondTexSize.y)  * A_JOINT_WEG[0];
    influence       += readMatrixFromTex(sampler2D(_boneTex, sampler_boneTex), A_JOINT_INC[1], bondTexSize.x, 0., bondTexSize.y) * A_JOINT_WEG[1];
    influence       += readMatrixFromTex(sampler2D(_boneTex, sampler_boneTex), A_JOINT_INC[2], bondTexSize.x, 0., bondTexSize.y) * A_JOINT_WEG[2];
    influence       += readMatrixFromTex(sampler2D(_boneTex, sampler_boneTex), A_JOINT_INC[3], bondTexSize.x, 0., bondTexSize.y) * A_JOINT_WEG[3];
    PI_ObjectToWorld = PI_ObjectToWorld * influence;
