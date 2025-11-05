
const vec3 LuminanceEncodeApprox = vec3(0.2126, 0.7152, 0.0722);
// BABYLON
float RGB2Gray(const float r, const float g, const float b) {
    return LuminanceEncodeApprox.r * r + LuminanceEncodeApprox.g * g + LuminanceEncodeApprox.b * b;
}
// BABYLON
float RGB2Gray(const vec3 rgb) {
    return LuminanceEncodeApprox.r * rgb.r + LuminanceEncodeApprox.g * rgb.g + LuminanceEncodeApprox.b * rgb.b;
}
// BABYLON
float GetLuminance(const vec3 color) {
    return clamp(dot(color, LuminanceEncodeApprox), 0., 1.);
}
