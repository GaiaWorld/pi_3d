
vec4 baseColor = v_color;

float alpha = 1.0;

// vec3 normal = normalize(v_normal);
// baseColor.rgb *= max(0., dot(normal, normalize(-light)));

// // float level = dot(v_normal, vec3(0., 0., -1.));
// if (debug_normal > 0) {
//     baseColor.rgb *= mix(baseColor.rgb, v_normal, 0.5);
// }
// // baseColor.rgb = (v_pos + vec3(1., 1., 1.)) / 2.;

baseColor.rgb *= v_normal;

gl_FragColor = vec4(baseColor.rgb, alpha);