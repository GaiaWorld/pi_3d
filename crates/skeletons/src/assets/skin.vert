
    mat4 skinMat =
        A_JOINT_WEG.x * jointMat(int(A_JOINT_INC.x)) +
        A_JOINT_WEG.y * jointMat(int(A_JOINT_INC.y)) +
        A_JOINT_WEG.z * jointMat(int(A_JOINT_INC.z)) +
        A_JOINT_WEG.w * jointMat(int(A_JOINT_INC.w));


    vec4 worldPosition = skinMat * vec4(A_POSITION, 1.);

    gl_Position =  worldPosition;

