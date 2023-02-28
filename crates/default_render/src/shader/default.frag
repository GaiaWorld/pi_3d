
layout(set = 1, binding = 1) uniform MatParam {
    @default(0)
    uint debug_normal;
}

layout(location = 0) in vec3 v_normal;
layout(location = 1) in vec3 v_pos;
layout(location = 2) in vec4 v_color;

layout(location = 0) out vec4 gl_FragColor;

const vec3 light = vec3(1., -3., 2.);


void main() {
    vec4 baseColor = v_color;

    float alpha = 1.0;

    vec3 normal = normalize(v_normal);
    baseColor.rgb *= max(0., dot(normal, normalize(-light)));

    // // float level = dot(v_normal, vec3(0., 0., -1.));
    if (debug_normal > 0) {
        baseColor.rgb = mix(baseColor.rgb, v_normal, 0.5);
    }
    // // baseColor.rgb = (v_pos + vec3(1., 1., 1.)) / 2.;

    // baseColor.rgb = v_normal;

    baseColor.rgb = max(vec3(0.008, 0.008, 0.008), baseColor.rgb);

    gl_FragColor = vec4(baseColor.rgb, alpha);
}