
const float ATLAS_MODE_SCALE = 0.1;
const float ATLAS_MODE_SCALE2 = 10.;
const float ADDRESS_CLAMP = 0.0;
const float ADDRESS_REPEAT = 1.0;
const float ADDRESS_MIRROR_REPEAT = 2.0;
const vec2 M_ONE = vec2(1.);
const vec2 M_ZERO = vec2(0.);
vec2 uvAtlas(const vec2 uv, const vec4 atlas, const vec4 mode) {
    vec2 f = floor(uv);
    vec2 temp = max(M_ZERO, M_ONE - mode.xy) * min(M_ONE, max(M_ZERO, uv)) 
              + min(M_ONE,          mode.xy) * abs(
                    uv - f 
              + max(M_ZERO, mode.xy - M_ONE) * (3. * f - 2. * (uv + floor(0.5 * uv)))
            );
    return temp * atlas.xy + atlas.zw;
}
