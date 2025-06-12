
    mat4 influence = boneMatrices[A_JOINT_INC1 + PI_SkinBoneOffset.x];
    PI_ObjectToWorld = PI_ObjectToWorld * influence; 
