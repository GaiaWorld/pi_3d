
    A_UV = vec2(TRAIL_INFO.y, step(0., TRAIL_INFO.x));

    vec3 zaxis = normalize(TRAIL_AXIS_Z);
    vec3 xaxis = normalize(TRAIL_AXIS_X);
    A_POSITION += xaxis * TRAIL_INFO.x;

    A_NORMAL = normalize(cross(zaxis, xaxis));
