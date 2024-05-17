vec4 mainTexture(vec2 vUV,vec2 vUVOS,vec4 atlas) {return Get_MainTex(vUV, vUVOS, uMainTilloff);}
vec4 mainTexture( vec2 vUV, vec2 vUVOS ) {return Get_MainTex(vUV, vUVOS, uMainTilloff);}
vec3 mainColor() {return uMainInfo.xyz;}
float mainStrength() {return _MainTexLevel;}
