vec4 emissiveTexture(const vec2 vUV) {return Get_EmissiveTex(vUV, matParam);}
vec3 emissiveColor() {return matParam.uEmissiveInfo.xyz;}
float emissiveStrength() {return matParam._EmissiveTexLevel;}
