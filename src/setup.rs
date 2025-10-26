use futures::executor::*;
use noobwerkz::camera::*;
use noobwerkz::graphics_context::*;
use noobwerkz::instance::*;
use noobwerkz::light::LightUniform;
use noobwerkz::model_node::*;
use noobwerkz::resource::*;
use noobwerkz::scene::*;


pub fn user_setup_implementation(gfx_ctx: &mut GraphicsContext, lights: &mut Vec<LightUniform>) {
    let mut u = noobwerkz::user_context::USER_CONTEXT.lock().unwrap();

    let m = block_on(load_model_from_serialized(
        "res".to_owned(),
        "avocado.bin".to_owned(),
        gfx_ctx.debug_material.clone(),
        &mut gfx_ctx.device,
        &mut gfx_ctx.queue,
        &gfx_ctx.texture_bind_group_layout,
    ))
    .unwrap();

    u.models.push(m);

    let m2 = block_on(load_model_from_serialized(
        "res".to_owned(),
        "cube.bin".to_owned(),
        gfx_ctx.debug_material.clone(),
        &mut gfx_ctx.device,
        &mut gfx_ctx.queue,
        &gfx_ctx.texture_bind_group_layout,
    ))
    .unwrap();
    u.models.push(m2);

   let m3 = block_on(load_model_from_serialized(
        "res".to_owned(),
        "cesium-man.bin".to_owned(),
        gfx_ctx.debug_material.clone(),
        &mut gfx_ctx.device,
        &mut gfx_ctx.queue,
        &gfx_ctx.texture_bind_group_layout,
    ))
    .unwrap();
    u.models.push(m3);

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
        ModelType::Textured,
        0,
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

    s.model_nodes.push(ModelNode::new(
        ModelType::TexturedSkinned,
        1,
        (0..1)
            .flat_map(|z| {
                (0..1).map(move |x| {
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
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
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


        s.model_nodes.push(ModelNode::new(
        ModelType::Textured,
        2,
        (0..1)
            .flat_map(|z| {
                (0..1).map(move |x| {
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
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
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
