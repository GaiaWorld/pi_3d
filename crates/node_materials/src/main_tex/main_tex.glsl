vec4 mainTexture( vec2 vUV, vec2 vUVOS, MatParam matParam) {return Get_MainTex(vUV, vUVOS, matParam.uMainTilloff);}
vec3 mainColor(MatParam matParam) {return matParam.uMainInfo.xyz;}
float mainStrength(MatParam matParam) {return matParam._MainTexLevel;}
