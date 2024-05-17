vec4 opacityTexture(vec2 vUV,vec2 vUVOS,vec4 atlas) {return Get_OpacityTex(vUV, vUVOS, uOpacityTilloff);}
vec4 opacityTexture( vec2 vUV, vec2 vUVOS ) {return Get_OpacityTex(vUV, vUVOS, uOpacityTilloff);}
float opacityChannel(vec4 data) {return valueByChannel(data, uOpacityChannel) * uOpacityLevel;}
