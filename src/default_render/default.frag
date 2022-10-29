#version 450

#define SHADER_NAME fragment:Default

layout(location = 0) in vec3 v_normal;

layout(location = 0) out vec4 gl_FragColor;

layout(set = 1, binding = 1) uniform MatParam0 {
    vec4 emissive;
};

void main() {
    vec4 baseColor = vec4(1., 1., 1., 1.);

    baseColor.rgb = normalize(v_normal);
    baseColor.rgb *= emissive.rgb * emissive.a;

    float alpha = 1.0;

    baseColor.rgb *= v_normal;

    gl_FragColor = vec4(baseColor.rgb, alpha);
}