# pi_3d
3D 渲染相关


# MainCamera - DefaultMaterial

* 主相机 默认材质渲染
    * Component:
        * MainCameraBind 

# 渲染组织

* 不透明渲染阶段
  * 材质一 渲染
  * 材质二 渲染
* 半透明渲染阶段
  * 层级 1
    * 材质一 渲染
    * 材质二 渲染
    * ...
  * 层级 2
    * 材质一 渲染
    * 材质二 渲染
    * ...
  * ...
  * 层级 4000
    * 材质一 渲染
    * 材质二 渲染
    * ...

## 问题

* 不透明渲染阶段 与 半透明渲染阶段 可以分别为一个图节点
* 材质X 的渲染 可以为一个system
  * 如何 在图节点中正确执行 各个渲染 system ?
  * 如何 保证通过 拓展 材质component 和 system 就能在图节点渲染中进行所有材质的渲染system ？