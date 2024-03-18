# PassObj

* 一个 Mesh 的指定Pass 对应一个 PassObj, 管理该Mesh在指定Pass的渲染数据

* [Component](../../../crates/pi_scene_context/src/pass/pass_object.rs)
    * [PassModelID], [PassSceneID], [PassMaterialID]
        * [sys_create_pass_object](../../../crates/pi_scene_context/src/pass/command_sys.rs)
    * [PassRendererID], [PassViewerID]
        * [sys_sets_modify_by_viewer](../../../crates/pi_scene_context/src/renderers/sys_renderer_pre.rs)
        * [sys_passrendererid_pass_reset](../../../crates/pi_scene_context/src/renderers/sys_renderer_pre.rs)
    * [PassGeometryID]
        * [sys_pass_shader_request_by_model](../../../crates/pi_scene_context/src/renderers/sys_bindgroup.rs)
        * [sys_pass_shader_request_by_geometry](../../../crates/pi_scene_context/src/renderers/sys_bindgroup.rs)
    * [PassBindEffectValue], [PassBindEffectTextures]
        * [sys_modify_pass_effect_by_material](../../../crates/pi_scene_context/src/pass/system.rs)
        * [sys_modify_pass_effect_by_material](../../../crates/pi_scene_context/src/pass/system.rs)
    * [PassBindGroupScene]
        * [sys_set0_modify](../../../crates/pi_scene_context/src/renderers/sys_bindgroup_0.rs)
    * [PassBindGroupModel]
        * [sys_set1_modify](../../../crates/pi_scene_context/src/renderers/sys_bindgroup_1.rs)
    * [PassBindGroupTextureSamplers]
        * [sys_set2_modify](../../../crates/pi_scene_context/src/renderers/sys_bindgroup_2.rs)
    * [PassBindGroupLightingShadow]
        * [sys_set3_modify](../../../crates/pi_scene_context/src/renderers/sys_bindgroup_3.rs)
    * [PassBindGroups]
        * [sys_pass_bind_groups](../../../crates/pi_scene_context/src/renderers/sys_bindgroup.rs)
    * [PassShader]
        * [sys_pass_shader](../../../crates/pi_scene_context/src/renderers/sys_bindgroup.rs)
    * [PassPipeline]
        * [sys_pass_pipeline_request_by_model](../../../crates/pi_scene_context/src/renderers/sys_bindgroup.rs)
        * [sys_pass_pipeline_request_by_renderer](../../../crates/pi_scene_context/src/renderers/sys_bindgroup.rs)
        * [sys_pass_pipeline](../../../crates/pi_scene_context/src/renderers/sys_bindgroup.rs)
    * [PassDraw]
        * [sys_pass_draw_modify_by_model](../../../crates/pi_scene_context/src/renderers/sys_bindgroup.rs)
        * [sys_pass_draw_modify_by_pass](../../../crates/pi_scene_context/src/renderers/sys_bindgroup.rs)
* System
    * [OpsPassObject](../../../crates/pi_scene_context/src/pass/command.rs)
    * [sys_modify_pass_effect_by_material](../../../crates/pi_scene_context/src/pass/system.rs)
    * [sys_modify_pass_effect_by_pass](../../../crates/pi_scene_context/src/pass/system.rs)