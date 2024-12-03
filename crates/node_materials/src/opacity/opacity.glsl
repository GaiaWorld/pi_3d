vec4 opacityTexture(vec2 vUV,vec2 vUVOS, MatParam matParam) {return Get_OpacityTex(vUV, vUVOS, matParam.uOpacityTilloff);}
float opacityChannel(vec4 data, MatParam matParam) {return valueByChannel(data, matParam.uOpacityChannel) * matParam.uOpacityLevel;}
