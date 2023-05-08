
float temp          = dot(v_pos, uDirection.xyz);
temp                = fract(temp / uStep + uSpeed * PI_Time.y);
temp                = abs((temp - 0.5) * 2.);
temp                = smoothstep(uFadeStart, uFadeEnd, temp);

vec4 finalColor     = mix(uColor0, uColor1, temp);

gl_FragColor = finalColor;
