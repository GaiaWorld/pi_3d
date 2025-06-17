const uint CHANNEL_R = 1;
const uint CHANNEL_G = 2;
const uint CHANNEL_B = 4;
const uint CHANNEL_A = 8;
const uint CHANNEL_GRAY = 0;

float valueByChannel(const vec4 rgba, const uint channel) {
    return CHANNEL_A == channel ? rgba.a : 
           CHANNEL_B == channel ? rgba.b : 
           CHANNEL_G == channel ? rgba.g : 
           CHANNEL_R == channel ? rgba.r : 
           RGB2Gray(rgba.rgb);
    // if (CHANNEL_A == channel) {
    //     return rgba.a;
    // }
    // else if (CHANNEL_B == channel) {
    //     return rgba.b;
    // }
    // else if (CHANNEL_G == channel) {
    //     return rgba.g;
    // }
    // else if (CHANNEL_R == channel) {
    //     return rgba.r;
    // }
    // else {
    //     return RGB2Gray(rgba.rgb);
    // }
    // const float a = step(CHANNEL_A, channel);
    // const float b = step(CHANNEL_B, channel) * (1. - a);
    // const float g = step(CHANNEL_G, channel) * (1. - b) * (1. - a);
    // const float r = step(CHANNEL_R, channel) * (1. - g) * (1. - b) * (1. - a);

    // return rgba.a * a + rgba.b * b + rgba.g * g + rgba.r * r + RGB2Gray(rgba.rgb) * (1. - r) * (1. - g) * (1. - b) * (1. - a);
}
