
if (uAlignment == ALIGNMENT_VIEW) {
    PI_ObjectToWorld = PI_ObjectToWorld * PI_MATRIX_V_R_INV;
}
if (uAlignment == ALIGNMENT_FACING) {
    PI_ObjectToWorld = rotMatrixFromForward(PI_ObjectToWorld, PI_MATRIX_V_R_INV, (PI_ObjectToWorld * vec4(0., 0., 0., 1.)).xyz, PI_CAMERA_POSITION.xyz);
}
if (uAlignment == ALIGNMENT_STRETCHED) {
    PI_ObjectToWorld = rotMatrixStretched( PI_ObjectToWorld, PI_VIEW_DIRECTION.xyz );
}
if (uAlignment == ALIGNMENT_VERTICAL) {
    PI_ObjectToWorld = matrixVertical( PI_ObjectToWorld, PI_VIEW_DIRECTION.xyz );
}
