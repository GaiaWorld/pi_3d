# Node Material

## Varying

* 内置 32 个 varying 数据
* 类型

|枚举ID|枚举值|数据类型|数据名称|
|-|-|-|-|
| PositionWS    | 0b_0000_0000_0000_0000_0000_0000_0000_0001 | vec3 | v_pos |
| PositionOS    | 0b_0000_0000_0000_0000_0000_0000_0000_0010 | vec3 | v_pos_os |
| PositionSS    | 0b_0000_0000_0000_0000_0000_0000_0000_0100 | vec3 | v_pos_ss |
| NormalWS      | 0b_0000_0000_0000_0000_0000_0000_0000_1000 | vec3  | v_normal |
| NormalOS      | 0b_0000_0000_0000_0000_0000_0000_0001_0000 | vec3  | v_normal_os |
| UV            | 0b_0000_0000_0000_0000_0000_0000_0010_0000 | vec2  | v_uv |
| UV2           | 0b_0000_0000_0000_0000_0000_0000_0100_1000 | vec2  | v_uv2 |
| UV3           | 0b_0000_0000_0000_0000_0000_0000_1000_0000 | vec2  | v_uv3 |
| UV4           | 0b_0000_0000_0000_0000_0000_0001_0000_0000 | vec2  | v_uv4 |
| UV5           | 0b_0000_0000_0000_0000_0000_0010_0000_0000 | vec2  | v_uv5 |
| UV6           | 0b_0000_0000_0000_0000_0000_0100_0000_0000 | vec2  | v_uv6 |
| UV7           | 0b_0000_0000_0000_0000_0000_1000_0000_0000 | vec2  | v_uv7 |
| UV8           | 0b_0000_0000_0000_0000_0001_0000_0000_0000 | vec2  | v_uv8 |
| Color         | 0b_0000_0000_0000_0000_0010_0000_0000_0000 | vec2  | v_color |
| Unuse0        | 0b_0000_0000_0000_0000_0100_0000_0000_0000 | 待定  | 待定 |
| Unuse1        | 0b_0000_0000_0000_0000_1000_0000_0000_0000 | 待定  | 待定 |
| Unuse2        | 0b_0000_0000_0000_0001_0000_0000_0000_0000 | 待定  | 待定 |
| Unuse3        | 0b_0000_0000_0000_0010_0000_0000_0000_0000 | 待定  | 待定 |
| Unuse4        | 0b_0000_0000_0000_0100_0000_0000_0000_0000 | 待定  | 待定 |
| Unuse5        | 0b_0000_0000_0000_1000_0000_0000_0000_0000 | 待定  | 待定 |
| Unuse6        | 0b_0000_0000_0001_0000_0000_0000_0000_0000 | 待定  | 待定 |
| Unuse7        | 0b_0000_0000_0010_0000_0000_0000_0000_0000 | 待定  | 待定 |
| Unuse8        | 0b_0000_0000_0100_0000_0000_0000_0000_0000 | 待定  | 待定 |
| Unuse9        | 0b_0000_0000_1000_0000_0000_0000_0000_0000 | 待定  | 待定 |
| V4A           | 0b_0000_0001_0000_0000_0000_0000_0000_0000 | vec4  | v_v4a |
| V4B           | 0b_0000_0010_0000_0000_0000_0000_0000_0000 | vec4  | v_v4b |
| V4C           | 0b_0000_0100_0000_0000_0000_0000_0000_0000 | vec4  | v_v4c |
| V4D           | 0b_0000_1000_0000_0000_0000_0000_0000_0000 | vec4  | v_v4d |
| V4E           | 0b_0001_0000_0000_0000_0000_0000_0000_0000 | vec4  | v_v4e |
| V4F           | 0b_0010_0000_0000_0000_0000_0000_0000_0000 | vec4  | v_v4f |
| V4G           | 0b_0100_0000_0000_0000_0000_0000_0000_0000 | vec4  | v_v4g |
| V4H           | 0b_1000_0000_0000_0000_0000_0000_0000_0000 | vec4  | v_v4h |
