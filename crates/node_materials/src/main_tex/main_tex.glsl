vec4 mainTexture(const vec2 vUV) {return Get_MainTex(vUV, matParam);}
vec3 mainColor() {return matParam.uMainInfo.xyz;}
float mainStrength() {return matParam._MainTexLevel;}
