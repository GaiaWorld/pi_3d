# Particle

* 每种属性为一类Component
  * 保存的是 Max_Count 大小的属性数据数组

## Emitter

* 各种类型的 Emitter 可以归并一种 Component
  * 避免修改类型时 原型变动

## Age

* Component

## Start XXX 

* 各种 Start 属性 各自为一种 Component
  * 计算过程 可互相并行

## Particle Vertices

* position
* scaling
* rotation
* direction
* color
* tilloff

## Render Mode 

* StretchedBillboard
  * Scaling 保持
  * position 保持
  * rotation 
* HorizontalBillboard
* VerticalBilldoard
* AlignmentView
* AlignmentWorld
* AlignmentLocal
* AlignmentFacing
* AlignmentVelocity