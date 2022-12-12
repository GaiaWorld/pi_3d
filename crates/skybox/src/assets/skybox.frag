#version 450

#define SHADER_NAME fragment:Default

layout(location = 0) in vec3 v_normal;

layout(location = 1) in float v_dist;

layout(location = 0) out vec4 gl_FragColor;

layout(set = 1, binding = 1) uniform MatParam0 {
    vec4 emissive;
};

float noise( vec2 n )
{
	return fract(sin(dot(n.xy, vec2(12.9898, 78.233)))* 43758.5453);
}


float n1noise( vec2 n )
{
	float t = fract(100.0);
	float nrnd0 = noise( n + 0.07*t );
	return nrnd0;
}

void main() {
    vec2 uv = gl_FragCoord.xy / vec2(800, 600).xy;
    float o = n1noise(200.0 * uv);

    float alpha = 1.0;

    vec3 pixel_color = vec3(emissive.rgb * o);

    vec3 fog_color = vec3(1.0, 1.0, 1.0); // 雾颜色。

    pixel_color = mix(fog_color, pixel_color, v_dist);

    gl_FragColor = vec4(pixel_color, alpha);
}