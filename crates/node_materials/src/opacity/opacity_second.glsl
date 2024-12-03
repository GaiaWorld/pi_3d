vec4 opacity2Texture(vec2 vUV,vec2 vUVOS, MatParam matParam) {return Get_Opacity2Tex(vUV, vUVOS, matParam.uOpacity2Tilloff);}
float opacity2Channel(vec4 data, MatParam matParam) {return valueByChannel(data, matParam.uOpacity2Channel) * matParam.uOpacity2Level;}
