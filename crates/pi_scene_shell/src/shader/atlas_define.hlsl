
const float ATLAS_MODE_SCALE = 0.1;
const float ATLAS_MODE_SCALE2 = 10.;
const float ADDRESS_CLAMP = 0.0;
const float ADDRESS_REPEAT = 1.0;
const float ADDRESS_MIRROR_REPEAT = 2.0;
const vec2 M_ONE = vec2(1.);
const vec2 M_ZERO = vec2(0.);

vec2 uvAtlas(const vec2 uv, const vec4 atlas, const vec4 mode) {
    vec2 result = uv;
    if (mode.x <= ADDRESS_MIRROR_REPEAT) {
        vec2 mirror_mask = step(1.5, mode.xy); // 1.0 for MIRROR mode
        vec2 clamp_mask = step(0.5, 1.0 - mode.xy); // 1.0 for CLAMP mode

        result = clamp(uv, 0.0, 1.0) * clamp_mask + fract(uv) * (1. - clamp_mask) * (1. - mirror_mask) + (1. - abs(uv - floor(0.5*uv) * 2. - 1.)) * mirror_mask;
        result = result * atlas.xy + atlas.zw;
    }
    return result;
}
