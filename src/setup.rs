use anim_graph_rs::animgraph_definitions::AnimGraphDefinition;
use anim_graph_rs::edge_definitions::AnimEdgeDefinition;
use anim_graph_rs::node_definitions::AnimNodeDefinition;
use anim_graph_rs::node_definitions::SamplerNodeDefinition;
use anim_graph_rs::node_definitions::StateMachineNodeDefinition;
use noobwerkz::camera::*;
use noobwerkz::graphics::*;
use noobwerkz::instance::*;
use noobwerkz::light::LightUniform;
use noobwerkz::model_node::*;
use noobwerkz::scene::*;
use noobwerkz::skeletal_context::SkeletalContext;
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

    //const SPACE_BETWEEN: f32 = 1.6;
    let mut skeletal_anim_instances = Vec::<Instance>::new();
    //let mut outer = 0;
    // while outer < 2 {
    //     let mut inner = 0;
    //     while inner < 2 {
    skeletal_anim_instances.push(Instance {
        position: noobwerkz::glam::Vec3A::from_array([
            0.0, //SPACE_BETWEEN * outer as f32,
            0.0,
            0.0,//SPACE_BETWEEN * inner as f32,
        ]),
        rotation: noobwerkz::glam::Quat::IDENTITY,
        scale: noobwerkz::glam::Vec3A::splat(1.0),
    });
    //     inner += 1;
    // }
    // outer += 1;
    // }

    let mut animgraph_definition = AnimGraphDefinition::new();
    let mut state_machine_definition = StateMachineNodeDefinition::new();
    let node = state_machine_definition
        .graph
        .add_node(AnimNodeDefinition::Sampler(SamplerNodeDefinition::new(
            "animation_0".to_owned(),
        )));
    let _ = state_machine_definition.graph.add_edge(
        AnimEdgeDefinition::Simple,
        state_machine_definition.start,
        node,
    );
    let _ = state_machine_definition.graph.add_edge(
        AnimEdgeDefinition::Simple,
        node,
        state_machine_definition.end,
    );
    let root = animgraph_definition
        .graph
        .add_node(state_machine_definition);
    animgraph_definition.root = Some(root);

    match cesium_man {
        Ok(val) => {
            s.add_characters(
                &mut gfx_ctx.device,
                &gfx_ctx.bone_matrices_bind_group_layout,
                val,
                &skeletal_anim_instances,
                &animgraph_definition,
                u.skeletals[0].skeleton.clone(),
                &u.skeletals[0].get_anim_name_map(),
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
