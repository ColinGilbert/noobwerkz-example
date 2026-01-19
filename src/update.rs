use noobwerkz::camera::CameraContext;
use noobwerkz::graphics::*;
use noobwerkz::light::*;
use noobwerkz::user_context::UserContext;
use noobwerkz::web_time::Duration;
use std::f32::consts::PI;

use crate::user_data::*;

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
    s.cameras[cam_idx].update();
    cam_ctx
        .uniform
        .update_view_proj(&s.cameras[cam_idx], &s.cameras[cam_idx].projection);
    gfx_ctx.queue.write_buffer(
        &cam_ctx.buffer,
        0,
        noobwerkz::bytemuck::cast_slice(&[cam_ctx.uniform]),
    );

    // Update the light
    let old_position: noobwerkz::glam::Vec3 = light_ctx.light_uniforms[0].position.into();
    light_ctx.light_uniforms[0].position =
        (noobwerkz::glam::Quat::from_axis_angle(noobwerkz::glam::Vec3::Y, PI * dt.as_secs_f32())
            * old_position)
            .into();
    gfx_ctx.queue.write_buffer(
        &light_ctx.light_buffer,
        0,
        noobwerkz::bytemuck::cast_slice(&[light_ctx.light_uniforms[0]]),
    );

    let fps = Duration::from_micros((1_000_000.0 / 30.0) as u64);
    u.time_elapsed += dt.as_micros();
    if u.time_elapsed > fps.as_micros() {
       s.update_characters(dt, &u.asset_mgr.skinned_models, &gfx_ctx.queue);
        u.time_elapsed -= fps.as_micros();
    }

    let _user_data = USER_DATA.lock().expect("UserData poisoned during user_update()");
    // user_data.

}