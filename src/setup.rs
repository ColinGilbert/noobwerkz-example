use noobwerkz::camera::*;
use noobwerkz::graphics_context::*;
use noobwerkz::instance::*;
use noobwerkz::light::LightUniform;
use noobwerkz::model_node::*;
use noobwerkz::resource::*;
use noobwerkz::scene::*;
use noobwerkz::skeletal_context::SkeletalContext;
use noobwerkz::skinned_model_node::*;
use noobwerkz::user_context::UserContext;

pub fn user_setup(
    gfx_ctx: &mut GraphicsContext,
    user_ctx: &mut UserContext,
    lights: &mut Vec<LightUniform>,
) {
    let u = user_ctx;

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
            x: 1.5,
            y: 1.0,
            z: 3.0,
        },
        &glam::Vec3 {
            x: 0.0,
            y: 1.0,
            z: -0.0,
        },
        &glam::Vec3::Y,
        0.1,
        0.1,
        projection,
    );

    let mut anims = Vec::new();
    anims.push("animation_0.ozz".to_owned());
    u.skeletals.push(SkeletalContext::new(
        "res".to_owned(),
        "skeleton.ozz".to_owned(),
        &anims,
    ));

    let mut data = load_serialized_model("res".to_owned(), "model.bin".to_owned());

    let m = load_skinned_model_from_serialized(
        &mut data,
        gfx_ctx.debug_material.clone(),
        "res".to_owned(),
        &mut gfx_ctx.device,
        &mut gfx_ctx.queue,
        &gfx_ctx.texture_bind_group_layout,
        &u.skeletals[0],
    )
    .expect("Model should load");

    u.skinned_models.push(m);

    let m2 = load_model_from_serialized(
        &mut data,
        gfx_ctx.debug_material.clone(),
        "res".to_owned(),
        &mut gfx_ctx.device,
        &mut gfx_ctx.queue,
        &gfx_ctx.texture_bind_group_layout,
    )
    .expect("Model should load");

    const NUM_INSTANCES_PER_ROW: u32 = 1;
    const SPACE_BETWEEN: f32 = 1.0;

    u.models.push(m2);

    s.model_nodes.push(ModelNode::new(
        0,
        (0..NUM_INSTANCES_PER_ROW)
            .flat_map(|z| {
                (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                    let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 10.0);
                    let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 10.0);

                    let position: glam::Vec3A = glam::Vec3 { x, y: 0.0, z }.into();
                    let rotation = glam::Quat::IDENTITY;
                    let scale: glam::Vec3A = glam::Vec3::splat(1.0).into();
                    
                    Instance {
                        position,
                        rotation,
                        scale,
                    }
                })
            })
            .collect::<Vec<_>>(),
    ));

    s.skinned_model_nodes.push(SkinnedModelNode::new(
        &mut gfx_ctx.device,
        &gfx_ctx.bone_matrices_bind_group_layout,
        &u.skinned_models[0],
        0,
        (0..1)
            .flat_map(|z| {
                (0..1).map(move |x| {
                    let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 10.0);
                    let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 10.0);

                    let position: glam::Vec3A = glam::Vec3 { x, y: 0.0, z}.into();
                    let rotation = glam::Quat::IDENTITY;
                    let scale: glam::Vec3A = glam::Vec3::splat(1.0).into();

                    Instance {
                        position,
                        rotation,
                        scale,
                    }
                })
            })
            .collect::<Vec<_>>(),
    
        &u.skeletals[0],

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
