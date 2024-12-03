
float temp          = dot(v_pos, matParam.uDirection.xyz);
temp                = fract(temp / matParam.uStep + matParam.uSpeed * PI_Time.y);
temp                = abs((temp - 0.5) * 2.);
temp                = smoothstep(matParam.uFadeStart, matParam.uFadeEnd, temp);

vec4 finalColor     = mix(matParam.uColor0, matParam.uColor1, temp);

gl_FragColor = finalColor;
