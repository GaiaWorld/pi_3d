
PI_ObjectToWorld = rotMatrixFromForward(PI_ObjectToWorld, PI_MATRIX_V_R_INV, (PI_ObjectToWorld * vec4(0., 0., 0., 1.)).xyz, PI_CAMERA_POSITION.xyz);
