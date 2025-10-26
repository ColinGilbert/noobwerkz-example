use instant::Duration;
use noobwerkz::camera_context::CameraContext;
use noobwerkz::graphics_context::*;
use noobwerkz::light::*;
use std::f32::consts::PI;

pub fn user_update_implementation(
    gfx_ctx: &mut GraphicsContext,
    cam_ctx: &mut CameraContext,
    light_ctx: &mut LightContext,
    dt: Duration,
) {
    let mut u = noobwerkz::user_context::USER_CONTEXT.lock().unwrap();
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
}