vec4 opacityTexture(const vec2 vUV, const MatParam matParam) {return Get_OpacityTex(vUV, matParam);}
float opacityChannel(const vec4 data, const MatParam matParam) {return valueByChannel(data, matParam.uOpacityChannel) * matParam.uOpacityLevel;}
