vec4 emissiveTexture(vec2 vUV,vec2 vUVOS,vec4 atlas) {return Get_EmissiveTex(vUV, vUVOS, uEmissiveTilloff);}
vec4 emissiveTexture( vec2 vUV, vec2 vUVOS ) {return Get_EmissiveTex(vUV, vUVOS, uEmissiveTilloff);}
vec3 emissiveColor() {return uEmissiveInfo.xyz;}
float emissiveStrength() {return _EmissiveTexLevel;}
