# Mesh

* 可渲染节点

## 网格数据

* Component
  * GeometryDesc
    * 网格数据的描述
  * RenderGeometry
    * 渲染用网格数据

## 材质数据

* Component
  * MaterialID
    * 渲染使用的效果材质的 实体ID

## 实例化相关

* Component
  * EInstanceCode
    * 实例化模式
  * InstancedList
    * 记录关联的实例的ID列表
  * InstancedBufferWorldMatrix
    * 存储实例化的变换矩阵数据
  * InstancedBufferColor
    * 存储实例化的Color数据
  * InstancedBufferTilloff
    * 存储实例化的Tilloff数据

## 骨骼相关

* Component
  * ESkinCode
    * 骨骼模式