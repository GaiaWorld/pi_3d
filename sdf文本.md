# SDF 文本渲染

* 使用 pi_bevy_render_plugin 下 ShareFontSheet
* ShareFontSheet 初始化化即创建了 SDF 纹理
    * 全局唯一, 包含会用到的所有字体的所有字符
    * 以固定名称命名, 并与普通纹理一起保存
*  一个 FontType 对应一个渲染基础数据(包含 Mesh Material)
    * 每个字符为对应 Mesh 的 InstancedMesh
    * 一个字符串为一个操作整体
        * 内部相对布局
        * 字符更新则对应更新InstancedMesh的局部姿态和UV
