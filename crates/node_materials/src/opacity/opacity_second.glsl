vec4 opacity2Texture(vec2 vUV,vec2 vUVOS,vec4 atlas) {return Get_Opacity2Tex(vUV, vUVOS, uOpacity2Tilloff);}
vec4 opacity2Texture(vec2 vUV, vec2 vUVOS) {return Get_Opacity2Tex(vUV, vUVOS, uOpacity2Tilloff);}
float opacity2Channel(vec4 data) {return valueByChannel(data, uOpacity2Channel) * uOpacity2Level;}
