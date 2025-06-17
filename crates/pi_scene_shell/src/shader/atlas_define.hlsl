
const float ATLAS_MODE_SCALE = 0.1;
const float ATLAS_MODE_SCALE2 = 10.;
const float ADDRESS_CLAMP = 0.0;
const float ADDRESS_REPEAT = 1.0;
const float ADDRESS_MIRROR_REPEAT = 2.0;
const vec2 M_ONE = vec2(1.);
const vec2 M_ZERO = vec2(0.);

vec2 uvAtlas(const vec2 uv, const vec4 atlas, const vec4 mode) {
    // vec2 mirror_mask = step(1.5, mode.xy); // 1.0 for MIRROR mode
    // vec2 clamp_mask = step(0.5, 1.0 - mode.xy); // 1.0 for CLAMP mode
    
    // vec2 base = fract(uv); // REPEAT模式
    // vec2 mirror_val = 1.0 - abs(mod(uv, 2.0) - 1.0); // MIRROR模式
    
    // return mix(mix(base, mirror_val, mirror_mask), 
    //            clamp(uv, 0.0, 1.0), 
    //            clamp_mask) * atlas.xy + atlas.zw;

    // return mix(mix(fract(uv), 1.0 - abs(mod(uv, 2.0) - 1.0), step(1.5, mode.xy)), 
    //            clamp(uv, 0.0, 1.0), 
    //            step(0.5, 1.0 - mode.xy)) * atlas.xy + atlas.zw;
    return mode.x > ADDRESS_MIRROR_REPEAT ? uv :
            mix(mix(fract(uv), 1.0 - abs(mod(uv, 2.0) - 1.0), step(1.5, mode.xy)), 
                clamp(uv, 0.0, 1.0), 
                step(0.5, 1.0 - mode.xy)
            ) * atlas.xy + atlas.zw;
}
