
    mat4 influence   = boneMatrices[A_JOINT_INC3[0] + PI_SkinBoneOffset.x] * A_JOINT_WEG3[0];
    influence       += boneMatrices[A_JOINT_INC3[0] + PI_SkinBoneOffset.x] * A_JOINT_WEG3[1];
    influence       += boneMatrices[A_JOINT_INC3[0] + PI_SkinBoneOffset.x] * A_JOINT_WEG3[2];
    PI_ObjectToWorld = PI_ObjectToWorld * influence; 
