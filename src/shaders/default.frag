#version 450

#define SHADER_NAME fragment:Default

layout(location = 0) in vec4 v_color;

layout(location = 1) out vec4 gl_FragColor;

layout(set = 1, binding = 0) uniform MatParam0 {
    vec4 emissive;
};

void main() {
    vec4 baseColor = vec4(1., 1., 1., 1.);

    baseColor.rgba = v_color.rgba;

    baseColor.rgb *= emissive.rgb * emissive.intensity;

    float alpha = 1.0;

    gl_FragColor = vec4(baseColor.rgb, alpha);
}