const int CHANNEL_R = 1;
const int CHANNEL_G = 2;
const int CHANNEL_B = 4;
const int CHANNEL_A = 8;
const int CHANNEL_GRAY = 0;

float valueByChannel(vec4 rgba, int channel) {
    if (channel == CHANNEL_R) {
        return rgba.r;
    }
    if (channel == CHANNEL_G) {
        return rgba.g;
    }
    if (channel == CHANNEL_B) {
        return rgba.b;
    }
    if (channel == CHANNEL_GRAY) {
        return RGB2Gray(rgba.rgb);
    }
    return rgba.a;
}
