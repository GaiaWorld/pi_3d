vec4 opacity2Texture(const vec2 vUV) {return Get_Opacity2Tex(vUV, matParam);}
float opacity2Channel(const vec4 data) {return valueByChannel(data, matParam.uOpacity2Channel) * matParam.uOpacity2Level;}
