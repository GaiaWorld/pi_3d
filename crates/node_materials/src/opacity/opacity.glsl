vec4 opacityTexture(const vec2 vUV) {return Get_OpacityTex(vUV, matParam);}
float opacityChannel(const vec4 data) {return valueByChannel(data, matParam.uOpacityChannel) * matParam.uOpacityLevel;}
