use noobwerkz::camera::*;
use noobwerkz::graphics::*;
use noobwerkz::instance::*;
use noobwerkz::light::LightUniform;
use noobwerkz::model_node::*;
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

    let mut s = Scene::new(&noobwerkz::glam::Vec3 {
        x: 0.0,
        y: -9.81,
        z: 0.0,
    });

    let c = Camera::new(
        &noobwerkz::glam::Vec3 {
            x: 0.0,
            y: 1.0,
            z: 3.0,
        },
        &noobwerkz::glam::Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        &noobwerkz::glam::Vec3::Y,
        0.1,
        0.1,
        projection,
    );

    let mut anims = Vec::new();

    anims.push("animation_0.ozz");

    u.skeletals.push(SkeletalContext::new(
        &vec!["res"],
        "cesium-man-skeleton.ozz",
        &anims,
    ));


    let mut path = std::path::PathBuf::new();
    path.push("res");
    let mut cesium_man_path = path.clone();
    cesium_man_path.push("cesium-man-model.bin");

    let cesium_man = u.asset_mgr.load_skinned_model_from_file(
        &cesium_man_path,
        "Cesium Man",
        &mut gfx_ctx.device,
        &mut gfx_ctx.queue,
        &gfx_ctx.debug_material,
        &gfx_ctx.texture_bind_group_layout_3d,
        &u.skeletals[0],
    );

    const SPACE_BETWEEN: f32 = 1.6;
    let mut skeletal_anim_instances = Vec::<Instance>::new();
    let mut i = 0;
    while i < 1 {
        let mut j = 0;
        while j < 1 {
            skeletal_anim_instances.push(Instance {
                position: noobwerkz::glam::Vec3A::from_array([
                    SPACE_BETWEEN * i as f32,
                    0.0,
                    SPACE_BETWEEN * j as f32,
                ]),
                rotation: noobwerkz::glam::Quat::IDENTITY,
                scale: noobwerkz::glam::Vec3A::splat(1.0),
            });
            j += 1;
        }
        i += 1;
    }
    match cesium_man {
        Ok(val) => {
            s.skinned_model_nodes.push(SkinnedModelNode::new(
                &mut gfx_ctx.device,
                &gfx_ctx.bone_matrices_bind_group_layout,
                val,
                skeletal_anim_instances,
                &u.skeletals[0],
            ));
        }
        Err(err) => {
            println!("Could not load skinned model. Error: {}", err);
        }
    }

    let mut avocado_path = path.clone();
    avocado_path.push("avocado-model.bin");

    let avocado = u.asset_mgr.load_model_from_file(
        &avocado_path,
        "Avocado",
        &mut gfx_ctx.device,
        &mut gfx_ctx.queue,
        &gfx_ctx.debug_material,
        &gfx_ctx.texture_bind_group_layout_3d,
    );

    match avocado {
        Ok(val) => {
            s.model_nodes.push(ModelNode::new(
                val,
                vec![Instance {
                    position: noobwerkz::glam::Vec3A::from_array([0.0, 0.0, 0.0]),
                    rotation: noobwerkz::glam::Quat::IDENTITY,
                    scale: noobwerkz::glam::Vec3A::splat(10.0),
                }],
            ));
        }
        Err(err) => {
            println!("Could not load model from file. Error {}", err)
        }
    }

    //     let mut cone_data = cone(1.0, 1.0);

    //     let cone_model = load_model_from_serialized(
    //         &mut cone_data,
    //         gfx_ctx.debug_material.clone(),
    //         "res".to_owned(),
    //         &mut gfx_ctx.device,
    //         &mut gfx_ctx.queue,
    //         &gfx_ctx.texture_bind_group_layout,
    //     )
    //     .expect("Cone should load");

    // //    let center_point = noobwerkz::glam::Vec3::from_array(cone_data.meshes[0].dimensions) / -2.0;
    //     u.models.push(cone_model);
    //     s.model_nodes.push(ModelNode::new(
    //         1,
    //         vec![Instance {
    //             position: noobwerkz::glam::Vec3::ZERO.into(),
    //             rotation: noobwerkz::glam::Quat::IDENTITY,//from_axis_angle(noobwerkz::glam::Vec3::X, -180.0),
    //             scale: noobwerkz::glam::Vec3A::splat(1.0),
    //         }],
    //     ));

    //      let mut cuboid_data = cuboid(3.0, 1.0, 5.0);

    //     let cuboid_model = load_model_from_serialized(
    //         &mut cuboid_data,
    //         gfx_ctx.debug_material.clone(),
    //         "res".to_owned(),
    //         &mut gfx_ctx.device,
    //         &mut gfx_ctx.queue,
    //         &gfx_ctx.texture_bind_group_layout,
    //     )
    //     .expect("Cone should load");

    //     u.models.push(cuboid_model);
    //     s.model_nodes.push(ModelNode::new(
    //         2,
    //         vec![Instance {
    //             position: noobwerkz::glam::Vec3::ZERO.into(),
    //             rotation: noobwerkz::glam::Quat::from_axis_angle(noobwerkz::glam::Vec3::X, -90.0),
    //             scale: noobwerkz::glam::Vec3A::splat(1.0),
    //         }],
    //     ));

    // let mut capsule_data = capsule(2.5, 0.3);

    // let capsule_model = load_model_from_serialized(
    //     &mut capsule_data,
    //     gfx_ctx.debug_material.clone(),
    //     "res".to_owned(),
    //     &mut gfx_ctx.device,
    //     &mut gfx_ctx.queue,
    //     &gfx_ctx.texture_bind_group_layout,
    // )
    // .expect("Capsule should load");

    // let mut capsule_instances =  Vec::<Instance>::new();
    // let mut i = 0;
    // while i < 2 {
    //     let mut j = 0;
    //     while j < 2 {
    //         capsule_instances.push(Instance {position: noobwerkz::glam::Vec3A::from_array([SPACE_BETWEEN * i as f32, 0.0, SPACE_BETWEEN * j as f32]), rotation: noobwerkz::glam::Quat::IDENTITY, scale: noobwerkz::glam::Vec3A::splat(1.0)});
    //         j +=1;
    //     }
    //     i += 1;
    // }

    // u.models.push(capsule_model);
    // s.model_nodes.push(ModelNode::new(
    //     1,
    //     capsule_instances
    // ));

    lights.push(LightUniform::new(
        noobwerkz::glam::Vec3 {
            x: 3.0,
            y: 2.0,
            z: 3.0,
        },
        noobwerkz::glam::Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    ));

    s.cameras.push(c);

    u.scenes.push(s);
}
