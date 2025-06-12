
A_UV = vec2(TRAIL_INFO.y, step(0., TRAIL_INFO.x));

vec3 zaxis = normalize(TRAIL_AXIS_Z);
vec3 yaxis = normalize(PI_CAMERA_POSITION.xyz - A_POSITION.xyz);
vec3 xaxis = normalize(cross(yaxis, zaxis)) * TRAIL_INFO.x;
A_POSITION += xaxis;

A_NORMAL = yaxis;
