vec4 emissiveTexture(const vec2 vUV, const vec2 vUVOS, const MatParam matParam ) {return Get_EmissiveTex(vUV, vUVOS, matParam.uEmissiveTilloff);}
vec3 emissiveColor(const MatParam matParam) {return matParam.uEmissiveInfo.xyz;}
float emissiveStrength(const MatParam matParam) {return matParam._EmissiveTexLevel;}
