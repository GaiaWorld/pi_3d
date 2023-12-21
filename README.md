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

* 泛化的固定大小Buffer分配器
  * FixedSizeBufferAllocator
    * 分配器
    * 分配器中 每个 FixedSize 一个Pool
  * FixedSizeBufferPool
    * 每个 Pool 有 1..n 个 FixedSizeBufferBlock
  * FixedSizeBufferBlock
    * 大内存块
    * 块大小 64K、128K、......
  * FixedSizeBufferRange
    * 小内存区间
    * 分配出来的数据区间


* 导航
  * RecastDetour 集成
    * 寻路
    * 动态障碍
    * 跳跃
* 包围盒检测


  var count = 16;
  window.record = [];
  window.recordRoot = document.createElement("div");
  window.recordRoot.style.left = "40%";
  window.recordRoot.style.bottom = "0%";
  window.recordRoot.style.width = "400px"; 
  window.recordRoot.style.height = "400px";
  window.recordRoot.style.backgroundColor = "#55555588";
  window.recordRoot.style.position = "absolute";
  document.body.appendChild(window.recordRoot); 
  window.recordNodes = [];
  for (var i = 0; i < count; i++) {
    var ddd1 = document.createElement("div");
    ddd1.style.left = i / count * 100 + "%";
    ddd1.style.bottom = "0%";
    ddd1.style.width = 1 / count * 100 + "%"; 
    ddd1.style.height = "0%";
    ddd1.style.backgroundColor = "#ff000088";
    ddd1.style.position = "absolute";
    window.recordRoot.appendChild(ddd1); 
    window.record.push(0);
    window.recordNodes.push(ddd1);
  }
  setInterval(() => {
    if (tttt && tttt.impl._activeMeshes) {
      var temp = tttt.impl._activeMeshes.length;
      window.record.shift();
      window.record.push(temp);
    }

    for (let i = 0; i < count; i++) {
      window.recordNodes[i].style.height = window.record[i] / 1000 * 100 + "%";
    }
  }, 25) ;

var scene = tttt.impl;
scene.onBeforeRenderObservable.add(() => {
    if (window.displayActiveMesh == undefined) {
      var div = document.createElement("div");
      div.style.left = "40%";
      div.style.top = "0%";
      div.style.width = "200px"; 
      div.style.height = "100px";
      div.style.backgroundColor = "#55555588";
      div.style.position = "absolute";
      document.body.appendChild(div); 
      window.displayActiveMesh = div;
    }
    if (window.displaylastTime == undefined) {  window.displaylastTime = Date.now(); }
    var now = Date.now();
    var textContent = "ActiveMesh: " + (scene?._activeMeshes?.length || 0);
    textContent += "</br>"
    if (scene) {
      var syscount = 0;
      var particlecount = 0;
      scene.meshes.forEach((mesh) => {
        if (mesh.psTool && mesh.isEnabled()) {
          syscount += 1;
          particlecount += mesh.psTool.activeParticleList.length;
        }
      })
    }
    var frameTime = (now - window.displaylastTime);
    textContent += "ActiveParticleSystem: " + syscount + (syscount > 100 * 10 ? " !! ": "");
    textContent += "</br>";
    textContent += "ActiveParticles: " + particlecount;
    textContent += "</br>";
    textContent += "FrameTime: " + frameTime + (frameTime > 22 ? " !! ": "");
    textContent += "</br>";
    window.displaylastTime = now;
    window.displayActiveMesh.innerHTML = textContent;
  });