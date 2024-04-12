const float ATLAS_MODE_SCALE = 0.1;
const float ATLAS_MODE_SCALE2 = 10.;
const float ADDRESS_CLAMP = 0.0;
const float ADDRESS_REPEAT = 1.0;
const float ADDRESS_MIRROR_REPEAT = 2.0;
vec2 uvAtlas(vec2 uv, vec4 atlas) {
    vec2 mode = floor(atlas.xy * ATLAS_MODE_SCALE);
    vec4 tilloff = atlas - vec4(mode, 0., 0.) * ATLAS_MODE_SCALE2;

    vec2 h = 0.5 * uv;
    vec2 n = floor(uv);
    vec2 a = 1.0 - floor(h - floor(h) + 0.5);

    vec2 repeat = uv - n;
    vec2 mirrorRepeat = 1.0 - a - repeat + 2.0 * a * repeat;
    vec2 clamp2edge = abs(h) - abs(h - 0.5) + 0.5;

    vec2 temp = mix(
        clamp2edge,
        mix(
            repeat,
            mirrorRepeat,
            vec2(step(ADDRESS_MIRROR_REPEAT, mode.x), step(ADDRESS_MIRROR_REPEAT, mode.y))
        ),
        vec2(step(ADDRESS_REPEAT, mode.x), step(ADDRESS_REPEAT, mode.y))
    );

    return temp * tilloff.xy + tilloff.zw;
}
