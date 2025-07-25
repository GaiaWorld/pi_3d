vec4 opacity2Texture(const vec2 vUV,const vec2 vUVOS, const MatParam matParam) {return Get_Opacity2Tex(vUV, vUVOS, matParam.uOpacity2Tilloff, matParam);}
float opacity2Channel(const vec4 data, const MatParam matParam) {return valueByChannel(data, matParam.uOpacity2Channel) * matParam.uOpacity2Level;}
