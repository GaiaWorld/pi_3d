
# 防御性

## 语义化语义化

* 命名语义正确不重复
* Stage: StageCamera, StageRender, StageAnimation ...
* 函数逻辑语义正确, 禁止冗杂混乱
* 禁止复杂代码片段拷贝使用
* 常量语义明确, 具备兼容性 - WebGLConstant

## 类型

#### 简单类型

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

#### 是资源 还是实例

#### 值类型的边界


## 接口

#### 接口设计

* 为后续升级留好冗余
* 不稳定的接口、内部逻辑可能与高层关联的接口 设计为高层可重载

#### 接口变动

* 外部不明确的接口变动预留一个版本过渡
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

## 数据交换

#### 数据打包
* wasm 与 js 存在数据交换, 数据交换存在成本
  * 多个同类型数据可打包尽量打包交换, 而不是一个一个传输
* 与 GPU 存在数据传输, 数据传输存在成本
  * 数据尽量打包到大Buffer, 而不是单个创建传输

#### 位标识数据
* JS 使用 << 运算 最大值 1 << 30
* 1 << 32 = 1
* 1 << 31 = -2147483648

## 优化

#### 遍历逻辑
* 在遍历逻辑中 取数据的操作, 检查该操作是否可以在遍历循环外进行
* [EParticleAttributeType](../../crates/particle_system/src/system.rs)

## Example/Test

* 每个版本的测试能正确执行、通过
* 保证错误/异常不会在多个版本间扩散
