
pub enum EParticleScalingMode {
    /// * 粒子： 节点树上的缩放信息被保留应用
    /// * 发射器: 节点树上的缩放信息被保留应用
    Hierarchy,
    /// * 粒子： 节点树上的缩放信息 只保留了LocalScaling
    /// * 发射器: 节点树上的缩放信息 只保留了LocalScaling
    Local,
    /// * 粒子： 节点树上的缩放信息被忽略
    /// * 发射器: 节点树上的缩放信息被保留应用
    Shape,
}

pub enum EParticleSimulationSpace {
    Local,
    World,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ERenderAlignmentForShader {
    None(bool),
    View,
    Facing,
    StretchedBillboard,
    VerticalBillboard,
}
impl ERenderAlignmentForShader {
    pub fn running_code(&self) -> String {
        match self {
            ERenderAlignmentForShader::None(flag) => {
                if *flag {
                    return Self::_running_code();
                } else {
                    return String::from("");
                }
            },
            ERenderAlignmentForShader::View => Self::view_running_code(),
            ERenderAlignmentForShader::Facing => Self::facing_running_code(),
            ERenderAlignmentForShader::StretchedBillboard => Self::stretched_running_code(),
            ERenderAlignmentForShader::VerticalBillboard => Self::vertical_running_code(),
        }
    }
    pub fn define_code(&self) -> String {
        match self {
            ERenderAlignmentForShader::None(flag) => {
                if *flag {
                    return Self::_define_code();
                } else {
                    return String::from("");
                }
            },
            ERenderAlignmentForShader::View => Self::view_define_code(),
            ERenderAlignmentForShader::Facing => Self::facing_define_code(),
            ERenderAlignmentForShader::StretchedBillboard => Self::stretched_define_code(),
            ERenderAlignmentForShader::VerticalBillboard => Self::vertical_define_code(),
        }
    }
    fn _running_code() -> String {
        return String::from("
        if (uAlignment == ALIGNMENT_VIEW) {
            PI_ObjectToWorld = PI_ObjectToWorld * PI_MATRIX_V_R_INV;
        }
        if (uAlignment == ALIGNMENT_FACING) {
            PI_ObjectToWorld = rotMatrixFromForward(PI_ObjectToWorld, PI_MATRIX_V_R_INV, (PI_ObjectToWorld * vec4(0., 0., 0., 1.)).xyz, PI_CAMERA_POSITION.xyz);
        }
        if (uAlignment == ALIGNMENT_STRETCHED) {
            PI_ObjectToWorld = rotMatrixStretched( PI_ObjectToWorld, PI_VIEW_DIRECTION.xyz );
        }
        if (uAlignment == ALIGNMENT_VERTICAL) {
            PI_ObjectToWorld = matrixVertical( PI_ObjectToWorld, PI_VIEW_DIRECTION.xyz );
        }
        ");
    }
    fn _define_code() -> String {
        return  String::from("
const int ALIGNMENT_VIEW = 0;
const int ALIGNMENT_FACING = 3;
const int ALIGNMENT_STRETCHED = 5;
const int ALIGNMENT_VERTICAL = 7;
mat4 rotMatrixFromForward(mat4 m, mat4 vr, vec3 position, vec3 viewpos) {
    vec3 forward = normalize(position - viewpos);
    vec3 up = normalize(vec3(vr * vec4(0., 1., 0., 1.)));
    vec3 left = cross(up, forward);
    up = cross(forward, left);
    return m * mat4(vec4(left, 0.), vec4(up, 0.), vec4(forward, 0.), vec4(0., 0.,0., 1.));
}
mat4 axisMatrix(vec3 xAxis, vec3 yAxis, vec3 zAxis) {
    return mat4(vec4(xAxis, 0.),vec4(yAxis, 0.),vec4(zAxis, 0.),vec4(0., 0., 0., 1.));
}
mat4 lookat(vec3 eye, vec3 target, vec3 up) {
    target = target - eye;
    vec3 zAxis = normalize(target);
    vec3 xAxis = cross(up, zAxis);
    float xSquareLength = length(xAxis);
    if (xSquareLength < 0.000001) {
        xAxis.x = 1.0;
    } else {
        xAxis = normalize(xAxis);
    }
    vec3 yAxis = normalize(cross(zAxis, xAxis));
    return axisMatrix(xAxis, yAxis, zAxis);
}
mat4 rotMatrixStretched(mat4 m, vec3 viewDirection) {
    mat4 invm = inverse(m);
    vec3 xAxis = vec3(1., 0., 0.);
    vec3 zAxis = normalize((invm * vec4(viewDirection, 0.)).xyz);
    vec3 yAxis = normalize(cross(zAxis, xAxis));
    zAxis = normalize(cross(xAxis, yAxis));
    mat4 lm = axisMatrix(xAxis, yAxis, zAxis);
    return m * lm;
}
mat4 matrixVertical(mat4 m, vec3 viewDirection) {
    mat4 invm = inverse(m);
    vec3 yAxis = normalize((invm * vec4(vec3(0., 1., 0.), 0.)).xyz);
    vec3 zAxis = normalize((invm * vec4(viewDirection, 0.)).xyz);
    vec3 xAxis = normalize(cross(yAxis, zAxis));
    zAxis = normalize(cross(xAxis, yAxis));
    mat4 lm = axisMatrix(xAxis, yAxis, zAxis);
    return m * lm;
}
        ");
    }
    /// Mesh 自身CPU逻辑中移除节点树上旋转信息, shader 中应用相机的节点旋转(视口旋转的逆)
    fn view_running_code() -> String {
        String::from(
"
PI_ObjectToWorld = PI_ObjectToWorld * PI_MATRIX_V_R_INV;
"            
        )
    }
    fn view_define_code() -> String {
        String::from(
"
"            
        )
    }
//     ///
//     /// Mesh 自身CPU逻辑中移除节点树上旋转信息, shader 中无特殊处理
//     fn world_running_code() -> String {
//         String::from(
// "
// "            
//         )
//     }
//     fn world_define_code() -> String {
//         String::from(
// "
// "            
//         )
//     }
//     /// 无特殊处理
//     fn local_running_code() -> String {
//         String::from(
// "
// "            
//         )
//     }
//     fn local_define_code() -> String {
//         String::from(
// "
// "            
//         )
//     }
    /// Mesh 自身CPU逻辑中移除节点树上旋转信息, shader 应用粒子指向相机的方向 (与直接使用相机旋转不同)
    fn facing_running_code() -> String {
        String::from(
"
PI_ObjectToWorld = rotMatrixFromForward(PI_ObjectToWorld, PI_MATRIX_V_R_INV, (PI_ObjectToWorld * vec4(0., 0., 0., 1.)).xyz, PI_CAMERA_POSITION.xyz);
"            
        )
    }
    fn facing_define_code() -> String {
        String::from(
"
mat4 rotMatrixFromForward(mat4 m, mat4 vr, vec3 position, vec3 viewpos) {
    vec3 forward = normalize(position - viewpos);
    vec3 up = normalize(vec3(vr * vec4(0., 1., 0., 1.)));
    vec3 left = cross(up, forward);
    up = cross(forward, left);
    return m * mat4(vec4(left, 0.), vec4(up, 0.), vec4(forward, 0.), vec4(0., 0.,0., 1.));
}
"            
        )
    }
//     /// 
//     fn velocity_running_code() -> String {
//         String::from(
// "
// "            
//         )
//     }
//     fn velocity_define_code() -> String {
//         String::from(
// "
// "            
//         )
//     }
    /// 
    fn stretched_running_code() -> String {
        String::from(
"
PI_ObjectToWorld = rotMatrixStretched(
    PI_ObjectToWorld, 
    PI_VIEW_DIRECTION.xyz
);
"            
        )
    }
    /// 全局X方向与看向相机方向 求出全局Y方向, 转换到局部空间, 求出局部旋转
    fn stretched_define_code() -> String {
        String::from(
"
mat4 axisMatrix(vec3 xAxis, vec3 yAxis, vec3 zAxis) {
    return mat4(vec4(xAxis, 0.),vec4(yAxis, 0.),vec4(zAxis, 0.),vec4(0., 0., 0., 1.));
}
mat4 lookat(vec3 eye, vec3 target, vec3 up) {
    target = target - eye;
    vec3 zAxis = normalize(target);
    vec3 xAxis = cross(up, zAxis);
    float xSquareLength = length(xAxis);
    if (xSquareLength < 0.000001) {
        xAxis.x = 1.0;
    } else {
        xAxis = normalize(xAxis);
    }
    vec3 yAxis = normalize(cross(zAxis, xAxis));
    return axisMatrix(xAxis, yAxis, zAxis);
}
mat4 rotMatrixStretched(mat4 m, vec3 viewDirection) {
    mat4 invm = inverse(m);
    vec3 xAxis = vec3(1., 0., 0.);
    vec3 zAxis = normalize((invm * vec4(viewDirection, 0.)).xyz);
    vec3 yAxis = normalize(cross(zAxis, xAxis));
    zAxis = normalize(cross(xAxis, yAxis));
    mat4 lm = axisMatrix(xAxis, yAxis, zAxis);
    return m * lm;
}
"            
        )
    }
    fn vertical_running_code() -> String {
        String::from(
"
PI_ObjectToWorld = matrixVertical(
    PI_ObjectToWorld,
    PI_VIEW_DIRECTION.xyz
);
"            
        )
    }
    fn vertical_define_code() -> String {
        String::from(
"
mat4 axisMatrix(vec3 xAxis, vec3 yAxis, vec3 zAxis) {
    return mat4(vec4(xAxis, 0.),vec4(yAxis, 0.),vec4(zAxis, 0.),vec4(0., 0., 0., 1.));
}
mat4 matrixVertical(mat4 m, vec3 viewDirection) {
    mat4 invm = inverse(m);
    vec3 yAxis = normalize((invm * vec4(vec3(0., 1., 0.), 0.)).xyz);
    vec3 zAxis = normalize((invm * vec4(viewDirection, 0.)).xyz);
    vec3 xAxis = normalize(cross(yAxis, zAxis));
    zAxis = normalize(cross(xAxis, yAxis));
    mat4 lm = axisMatrix(xAxis, yAxis, zAxis);
    return m * lm;
}
"            
        )
    }
//     fn horizontal_running_code() -> String {
//         String::from(
// "
// "            
//         )
//     }
//     fn horizontal_define_code() -> String {
//         String::from(
// "
// "            
//         )
//     }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ERenderAlignment {
    /// * 只保留了粒子的旋转信息, 节点树上的旋转信息被忽略
    /// * 先应用相机的旋转矩阵的逆矩阵, 这样正方向与相机 中轴线 上视线方向刚好相反
    /// * 再应用粒子旋转
    /// * 即获得最终世界矩阵 
    View,
    /// * 节点树上的旋转信息被忽略
    /// * 应用粒子旋转
    /// * 即获得最终世界矩阵
    World,
    /// * 节点树上的旋转信息保留并应用
    /// * 再应用粒子旋转
    /// * 即获得最终世界矩阵
    Local,
    /// * 只保留了粒子的旋转信息, 节点树上的旋转信息被忽略
    /// * 先应用粒子指向相机的方向的旋转信息, 这样正方向与相机 相机观察目标 的视线方向刚好相反
    /// * 再应用粒子旋转
    /// * 即获得最终世界矩阵 
    Facing,
    /// * 只保留了粒子的旋转信息, 节点树上的旋转信息被忽略
    /// * 先应用粒子速度方向的旋转信息
    /// * 再应用粒子旋转
    /// * 即获得最终世界矩阵 
    Velocity,
    /// * 所有旋转信息被忽略,
    /// * 发射时的 缩放 偏移 应用 粒子的缩放、局部坐标 获得粒子 全局 缩放 偏移，传入shader
    StretchedBillboard,
    /// * 所有旋转信息被忽略, 仅应用 粒子 Z 轴旋转信息
    /// * 发射时的 缩放 偏移 应用 粒子的缩放、局部坐标 获得粒子 全局 缩放 偏移, 加上 粒子 z 旋转 和 固定 x 轴 90 度旋转, 即 粒子的世界矩阵, ，传入shader
    HorizontalBillboard,
    /// * 所有旋转信息被忽略, 粒子 Z 轴强制为指向相机的方向, 并应用 粒子 z 轴旋转信息
    /// * 发射时的 缩放 偏移 应用 粒子的缩放、局部坐标 获得粒子 全局 缩放 偏移, 加上 粒子 z 旋转, 即 粒子的世界矩阵，传入shader
    /// * 由 粒子 全局坐标 和 相机全局坐标 的 X-Z 轴投影获得渲染阶段的矩阵, 被粒子世界矩阵作用
    VerticalBillboard,
}
impl ERenderAlignment {
    pub fn shader_tag(&self, align_by_uniform: bool) -> ERenderAlignmentForShader {
        if align_by_uniform == true {
            ERenderAlignmentForShader::None(align_by_uniform)
        } else {
            match self {
                ERenderAlignment::View => ERenderAlignmentForShader::View,
                ERenderAlignment::Facing => ERenderAlignmentForShader::Facing,
                ERenderAlignment::StretchedBillboard => ERenderAlignmentForShader::StretchedBillboard,
                ERenderAlignment::VerticalBillboard => ERenderAlignmentForShader::VerticalBillboard,
                _ => ERenderAlignmentForShader::None(align_by_uniform)
            }
        }
    }
}


// pub enum EParticleRenderMode {
//     Base,
//     /// * 所有旋转信息被忽略,
//     /// * 发射时的 缩放 偏移 应用 粒子的缩放、局部坐标 获得粒子 全局 缩放 偏移，传入shader
//     StretchedBillboard,
//     /// * 所有旋转信息被忽略, 仅应用 粒子 Z 轴旋转信息
//     /// * 发射时的 缩放 偏移 应用 粒子的缩放、局部坐标 获得粒子 全局 缩放 偏移, 加上 粒子 z 旋转 和 固定 x 轴 90 度旋转, 即 粒子的世界矩阵, ，传入shader
//     HorizontalBillboard,
//     /// * 所有旋转信息被忽略, 粒子 Z 轴强制为指向相机的方向, 并应用 粒子 z 轴旋转信息
//     /// * 发射时的 缩放 偏移 应用 粒子的缩放、局部坐标 获得粒子 全局 缩放 偏移, 加上 粒子 z 旋转, 即 粒子的世界矩阵，传入shader
//     /// * 由 粒子 全局坐标 和 相机全局坐标 的 X-Z 轴投影获得渲染阶段的矩阵, 被粒子世界矩阵作用
//     VerticalBillboard,
// }
// impl EParticleRenderMode {
//     pub fn running_code(&self) -> String {
//         match self {
//             EParticleRenderMode::Base => todo!(),
//             EParticleRenderMode::StretchedBillboard => todo!(),
//             EParticleRenderMode::HorizontalBillboard => todo!(),
//             EParticleRenderMode::VerticalBillboard => todo!(),
//         }
//     }
// }
