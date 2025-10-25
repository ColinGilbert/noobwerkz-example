use futures::executor::*;
use instant::Duration;
use noobwerkz::app::run;
use noobwerkz::callbacks::*;
use noobwerkz::camera::*;
use noobwerkz::camera_context::CameraContext;
use noobwerkz::graphics_context::*;
use noobwerkz::instance::*;
use noobwerkz::light::LightUniform;
use noobwerkz::light::*;
use noobwerkz::model::*;
use noobwerkz::model_node::*;
use noobwerkz::resource::*;
use noobwerkz::scene::*;
use std::f32::consts::PI;

fn user_setup_implementation(gfx_ctx: &mut GraphicsContext, lights: &mut Vec<LightUniform>) {
    let mut u = noobwerkz::user_context::USER_CONTEXT.lock().unwrap();

    let m = block_on(load_model_from_serialized(
        "res".to_owned(),
        "avocado.bin".to_owned(),
        &mut gfx_ctx.device,
        &mut gfx_ctx.queue,
        &gfx_ctx.texture_bind_group_layout,
    ))
    .unwrap();

    u.models.push(m);

    let projection = Projection::new(
        gfx_ctx.config.height,
        gfx_ctx.config.width,
        degrees_to_radians(45.0),
        0.0001,
        1000.0,
    );

    let mut s = Scene::new();
    let c = Camera::new(
        &glam::Vec3 {
            x: 10.0,
            y: 10.0,
            z: 10.0,
        },
        &glam::Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        &glam::Vec3::Y,
        0.1,
        0.1,
        projection,
    );

    const NUM_INSTANCES_PER_ROW: u32 = 10;
    const SPACE_BETWEEN: f32 = 1.0;
    s.model_nodes.push(ModelNode::new(
        ModelType::NormalMapped,
        u.models.len() - 1,
        (0..NUM_INSTANCES_PER_ROW)
            .flat_map(|z| {
                (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                    let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 10.0);
                    let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 10.0);

                    let position: glam::Vec3A = glam::Vec3 { x, y: 0.0, z }.into();

                    let rotation = if position == glam::Vec3A::ZERO {
                        glam::Quat::from_axis_angle(glam::Vec3::Z, 0.0)
                    } else {
                        let pos: glam::Vec3 = position.into();
                        glam::Quat::from_axis_angle(pos.normalize(), 45.0)
                    };
                    let scale: glam::Vec3A = glam::Vec3 {
                        x: 10.0,
                        y: 10.0,
                        z: 10.0,
                    }
                    .into();
                    Instance {
                        position,
                        rotation,
                        scale,
                    }
                })
            })
            .collect::<Vec<_>>(),
    ));

    lights.push(LightUniform::new(
        glam::Vec3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        },
        glam::Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    ));

    s.cameras.push(c);

    u.scenes.push(s);
}

fn user_update_implementation(
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

fn main() {
    init_user_setup_callback(user_setup_implementation);
    init_user_update_callback(user_update_implementation);
    run().unwrap();
}
