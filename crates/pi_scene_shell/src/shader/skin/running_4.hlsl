
    mat4 influence   = boneMatrices[A_JOINT_INC[0] + PI_SkinBoneOffset.x] * A_JOINT_WEG[0];
    influence       += boneMatrices[A_JOINT_INC[1] + PI_SkinBoneOffset.x] * A_JOINT_WEG[1];
    influence       += boneMatrices[A_JOINT_INC[2] + PI_SkinBoneOffset.x] * A_JOINT_WEG[2];
    influence       += boneMatrices[A_JOINT_INC[3] + PI_SkinBoneOffset.x] * A_JOINT_WEG[3];
    PI_ObjectToWorld = PI_ObjectToWorld * influence;
