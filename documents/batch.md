# 动态合批处理

* DrawCall 收集的地方使用公共Buffer
* 单个DrawObj的某个顶点数据槽位使用临时Buffer数据结构,在DrawCall排序后拷贝到公共Buffer
* 连续的多个DrawCall已经经 Shader pipeline renderIndex distance 等排序, 相邻DrawCallpipeline与shader相同,动态槽位相同,顶点数据应当可以进行合并
