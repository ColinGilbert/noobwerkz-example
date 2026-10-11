use noobwerkz::camera::*;
use noobwerkz::graphics::*;
use noobwerkz::index_types::*;
use noobwerkz::instance::*;
use noobwerkz::light::LightUniform;
use noobwerkz::model3d_node::*;
use noobwerkz::scene3d::*;
use noobwerkz::simple_animgraph::animgraph_definition::AnimGraphDefinition;
use noobwerkz::skeletal_context::SkeletalContext;
use noobwerkz::user_context::UserContext;
// use noobwerkz::simple_animgraph::edge_definitions::TransitionDefinition;
use crate::user_data::*;
use noobwerkz::simple_animgraph::node_definitions::GenericNodeDefinition;
use noobwerkz::simple_animgraph::node_definitions::SamplerNodeDefinition;

// MODIFY ME!
pub fn user_setup(
    gfx_ctx: &mut GraphicsContext,
    user_ctx: &mut UserContext,
    lights: &mut Vec<LightUniform>,
) {
    let u = user_ctx;
    let _user_data = USER_DATA
        .lock()
        .expect("UserData poisoned during user_update()");

    let projection = Projection::new(
        gfx_ctx.config.height,
        gfx_ctx.config.width,
        degrees_to_radians(45.0),
        0.0001,
        1000.0,
    );

    let mut s = Scene3D::new(&noobwerkz::glam::Vec3 {
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

    let skeletal_ctx_idx = SkeletalContextIndex::new(u.skeletals.len() - 1);
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
        &u.skeletals[skeletal_ctx_idx],
    );

    //const SPACE_BETWEEN: f32 = 1.6;
    let mut skeletal_anim_instances = Vec::<Instance>::new();
    //let mut outer = 0;
    // while outer < 2 {
    //     let mut inner = 0;
    //     while inner < 2 {
    skeletal_anim_instances.push(Instance {
        position: noobwerkz::glam::Vec3A::from_array([
            0.0, //SPACE_BETWEEN * outer as f32,
            0.0, 0.0, //SPACE_BETWEEN * inner as f32,
        ]),
        orientation: noobwerkz::glam::Quat::IDENTITY,
        scale: noobwerkz::glam::Vec3A::splat(1.0),
    });
    //     inner += 1;
    // }
    // outer += 1;
    // }

    let mut animgraph_definition = AnimGraphDefinition::new();
    let root_node_def = GenericNodeDefinition::Sampler(SamplerNodeDefinition {
        speed: 1.0,
        animation_name: "animation_0".to_owned(),
        looping: true,
        name: "walk".to_owned(),
    });

    let root = animgraph_definition.graph.add_node(root_node_def);

    animgraph_definition.root = Some(root);

    match cesium_man {
        Ok(val) => {
            s.add_characters(
                &mut gfx_ctx.device,
                &gfx_ctx.bone_matrices_bind_group_layout,
                val,
                &skeletal_anim_instances,
                &animgraph_definition,
                u.skeletals[skeletal_ctx_idx].skeleton.clone(),
                &u.skeletals[skeletal_ctx_idx].get_anim_name_map(),
                "cesium-man".to_owned(),
            );
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
            let mut instances = Instances::<Instance>::new();
            instances.push(Instance {
                position: noobwerkz::glam::Vec3A::from_array([0.0, 0.0, 0.0]),
                orientation: noobwerkz::glam::Quat::IDENTITY,
                scale: noobwerkz::glam::Vec3A::splat(10.0),
            });
            s.model_nodes.push(Model3DNode::new(val, instances));
        }
        Err(err) => {
            println!("Could not load model from file. Error {}", err)
        }
    }

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
