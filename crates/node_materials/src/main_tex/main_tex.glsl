vec4 mainTexture(const vec2 vUV, const MatParam matParam) {return Get_MainTex(vUV, matParam);}
vec3 mainColor(const MatParam matParam) {return matParam.uMainInfo.xyz;}
float mainStrength(const MatParam matParam) {return matParam._MainTexLevel;}
