use web_time::Duration;
use noobwerkz::instance::*;
use noobwerkz::camera_context::CameraContext;
use noobwerkz::graphics_context::*;
use noobwerkz::light::*;
use noobwerkz::user_context::UserContext;
use std::f32::consts::PI;

pub fn user_update(
    gfx_ctx: &mut GraphicsContext,
    cam_ctx: &mut CameraContext,
    light_ctx: &mut LightContext,
    user_ctx: &mut UserContext,
    dt: Duration,
) {
    let u = user_ctx;
    let scene_idx = u.active_scene;
    let s = &mut u.scenes[scene_idx];
    let cam_idx = s.active_camera;
    s.cameras[s.active_camera].update();
    cam_ctx
        .uniform
        .update_view_proj(&s.cameras[cam_idx], &s.cameras[cam_idx].projection);
    gfx_ctx
        .queue
        .write_buffer(&cam_ctx.buffer, 0, bytemuck::cast_slice(&[cam_ctx.uniform]));

    // Update the light
    let old_position: glam::Vec3 = light_ctx.light_uniforms[0].position.into();
    light_ctx.light_uniforms[0].position =
        (glam::Quat::from_axis_angle(glam::Vec3::Y, PI * dt.as_secs_f32()) * old_position).into();
    gfx_ctx.queue.write_buffer(
        &light_ctx.light_buffer,
        0,
        bytemuck::cast_slice(&[light_ctx.light_uniforms[0]]),
    );

    s.skinned_model_nodes[0].update(&mut gfx_ctx.device, &mut gfx_ctx.queue, &u.skinned_models[0], &gfx_ctx.bone_matrices_bind_group_layout, dt);

    // let mut instances = Vec::<Instance>::new();
    // for bm in &s.skinned_model_nodes[0].untransformed_bone_matrices {

    //     let (scale, rot, trans) = glam::Mat4::to_scale_rotation_translation(&glam::Mat4::from_cols_array_2d(&bm.data));
    //     let i = Instance { scale: scale.into(), rotation: rot, position: trans.into()};
    //     instances.push(i);
    // }
    // s.model_nodes[0].instances.clear();
    // s.model_nodes[0].instances = instances;

}