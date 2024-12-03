vec4 emissiveTexture( vec2 vUV, vec2 vUVOS, MatParam matParam ) {return Get_EmissiveTex(vUV, vUVOS, matParam.uEmissiveTilloff);}
vec3 emissiveColor(MatParam matParam) {return matParam.uEmissiveInfo.xyz;}
float emissiveStrength(MatParam matParam) {return matParam._EmissiveTexLevel;}
