# 动态合批处理

* DrawCall 收集的地方使用公共Buffer
* 单个DrawObj的某个顶点数据槽位使用临时Buffer数据结构,在DrawCall排序后拷贝到公共Buffer
* 连续的多个DrawCall已经经 Shader pipeline renderIndex distance 等排序, 相邻DrawCallpipeline与shader相同,动态槽位相同,顶点数据应当可以进行合并

# 材质

* 材质数据
  * Shader
  * Uniform
    * Uniform 通过Uniform数组生成器,返回Bind信息,后续可更新Uniform数据
    * 每Shader对应一个生成器, 生成器可能构造新的uniform数组, 返回Bind引用及材质idx
    
  * Texture
    * 纹理通过Combine, 返回运行时纹理, 可能为原数据,可能为运行时的合并纹理(纹理数组)

