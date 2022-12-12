#version 450

#define SHADER_NAME fragment:Default

const float LinearEncodePowerApprox = 2.2;

layout(location = 0) in vec3 v_normal;
layout(location = 1) in vec3 v_pos;
layout(location = 2) in vec2 v_UV;

layout(location = 0) out vec4 gl_FragColor;

layout(set = 1, binding = 1) uniform MatParam0 {
    vec4 emissive;
    vec4 emissive_scaleoffset;
};

layout(set = 2, binding = 0) uniform texture2D _MainTex;
layout(set = 2, binding = 1) uniform sampler sampler_MainTex;

vec3 toLinearSpace(vec3 color)
{
    return pow(color, vec3(LinearEncodePowerApprox));
}

vec3 fromRGBD(vec4 rgbd) {
    // Helps with png quantization.
    rgbd.rgb = toLinearSpace(rgbd.rgb);

    // return rgbd.rgb * ((rgbdMaxRange / 255.0) / rgbd.a);

    return rgbd.rgb / rgbd.a;
}

void main() {
    // vec4 baseColor = vec4(1., 1., 1., 1.);
    float alpha = 1.0;

    vec4 mainTextureColor = texture(sampler2D(_MainTex, sampler_MainTex), v_UV);;

    gl_FragColor = vec4(fromRGBD(mainTextureColor), alpha);
}