
# 防御性

## 类型

* 某个数据类型不确定时 声明一个语义明确的纯类型
* 使用时传参时 使用 as 语句, 以后值类型修改时引用代码不需要修改
* Example
```rust
type FrameIndex = u16; // 支持更长的动画时长 u32,
type KeyFrameCurveValue = f32;
```
```rust
let curve = FrameCurve::<LocalScaling>::curve_easing(
    LocalScaling(Vector3::new(1., 1., 1.)),
    LocalScaling(Vector3::new(0., 2., 0.)),
    (60. * (1.1 + ((i * j) as f32).cos())) as FrameIndex,
    30,
    EEasingMode::None
);
```

## 接口

* 接口变动预留一个版本过渡
* 变动时不修改原接口参数, 新增一个接口, 原接口逻辑转换为新实现
* Example
```rust
#[deprecated(
    since = "0.2.1",
    note = "Please use the bar function instead"
)]
pub fn foo() {

}

pub fn bar(a: u32) {

}
```

## 引用

* 用一个引入导出模块,引入项目会使用的外部库和模块, 并导出,项目则引用该模块来使用
* 后期外部依赖更新或替换时, 只修改该模块
* [pi_scene_shell](../../crates/pi_scene_shell/src/prelude.rs)
