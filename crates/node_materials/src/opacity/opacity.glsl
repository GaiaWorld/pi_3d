vec4 opacityTexture(const vec2 vUV,const vec2 vUVOS, const MatParam matParam) {return Get_OpacityTex(vUV, vUVOS, matParam.uOpacityTilloff, matParam);}
float opacityChannel(const vec4 data, const MatParam matParam) {return valueByChannel(data, matParam.uOpacityChannel) * matParam.uOpacityLevel;}
