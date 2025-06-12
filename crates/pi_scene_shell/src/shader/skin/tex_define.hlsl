
#define inline
mat4 readMatrixFromTex(texture2D tex, sampler samp, const float index, const float texWidth, const float row, const float texHeight) {
    float offset = index * 4.0;
    float dx = 1. / texWidth;
    float dy = row * 1. / texHeight;
    vec4 m0 = texture(sampler2D(tex, samp), vec2(dx * (offset + 0.5), dy));
    vec4 m1 = texture(sampler2D(tex, samp), vec2(dx * (offset + 1.5), dy));
    vec4 m2 = texture(sampler2D(tex, samp), vec2(dx * (offset + 2.5), dy));
    vec4 m3 = texture(sampler2D(tex, samp), vec2(dx * (offset + 3.5), dy));
    return mat4(m0, m1, m2, m3);
}
