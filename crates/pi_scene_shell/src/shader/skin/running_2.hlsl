
    mat4 influence   = boneMatrices[A_JOINT_INC2[0] + PI_SkinBoneOffset.x] * A_JOINT_WEG2[0];
    influence       += boneMatrices[A_JOINT_INC2[1] + PI_SkinBoneOffset.x] * A_JOINT_WEG2[1];
    PI_ObjectToWorld = PI_ObjectToWorld * influence;
