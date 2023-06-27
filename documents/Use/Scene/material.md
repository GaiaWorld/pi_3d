# 材质 Material

## **重要** 材质使用的Shader元数据必须在使用前注册好

* 为了初始化材质实体的原型 不受Shader元数据的异步影响
* 同时也使得可动画的材质属性 作用的 uniform 能在初始化时一次确定
* Shader元数据 作为资源注册到World 这个操作正好是同步的因此可以满足上面的需求
* 顶点Shader中必须将z值转换到 0-1 `gl_Position.z = gl_Position.z * 0.5 + 0.5;`