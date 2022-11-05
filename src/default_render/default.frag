#version 450

#define SHADER_NAME fragment:Default

layout(location = 0) in vec3 v_normal;
layout(location = 1) in vec3 v_pos;

layout(location = 0) out vec4 gl_FragColor;

layout(set = 1, binding = 1) uniform MatParam0 {
    vec4 emissive;
};

void main() {
    vec4 baseColor = vec4(1., 1., 1., 1.);

    baseColor.rgb *= emissive.rgb * emissive.a;

    float alpha = 1.0;

    // float level = dot(v_normal, vec3(0., 0., -1.));
    baseColor.rgb *= (v_normal + vec3(1., 1., 1.))/ 2.;
    // baseColor.rgb = (v_pos + vec3(1., 1., 1.)) / 2.;

    gl_FragColor = vec4(baseColor.rgb, alpha);
}