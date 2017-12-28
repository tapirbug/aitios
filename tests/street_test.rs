extern crate aitios;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate chrono;

mod common;

use std::fs::create_dir;

#[cfg_attr(not(feature = "expensive_tests"), ignore)]
#[test]
fn street_test() {
    let directory = common::prepare_test_directory("street_test");

    let mut obj_file = directory.clone();
    obj_file.push("street_test");
    obj_file.set_extension("obj");
    let obj_file = obj_file.to_str().unwrap();

    let mut mtl_file = directory.clone();
    mtl_file.push("street_test");
    mtl_file.set_extension("mtl");
    let mtl_file = mtl_file.to_str().unwrap();

    let mut density_map_output_directory = directory.clone();
    density_map_output_directory.push("substance-density-maps");
    create_dir(&density_map_output_directory).unwrap();

    let density_map_output_directory = density_map_output_directory.to_str().unwrap();

    let blent_map_output_directory = directory.to_str().unwrap();

    let input_path = "test-scenes/street-lowpoly";
    let model_obj_path = format!("{}/street-lowpoly.obj", input_path);

    let mut hit_map_path = directory.clone();
    hit_map_path.push("interacted_surfels");
    hit_map_path.set_extension("obj");

    let mut surfels_path = directory.clone();
    surfels_path.push("surfels");
    surfels_path.set_extension("obj");

    aitios::SimulationBuilder::new()
        .ton_to_surface_interaction_weight(0.2)
        .surfel_obj_path(surfels_path)
        .scene(
            &model_obj_path,
            |s| {
                s.min_sample_distance(0.1)
                    .delta_straight(1.0)
                    .substances(&vec![0.0])
            }
        )
        .add_source(|s| {
            s.p_straight(1.0)
                .substances(&vec![1.0])
                .point_shaped(0.0, 2.0, 0.0)
                .emission_count(20000)
        })
        .add_effect_blend(
            0, // Index of substance that drives the blend
            "Metal", // material that gets changed
            "map_Kd", // map of the material that gets changed
            "test-scenes/buddha-scene/weathered_bronze.png",
            blent_map_output_directory
        )
        .add_effect_blend(
            0,
            "street",
            "map_Kd",
            "test-scenes/buddha-scene/moss.png",
            blent_map_output_directory
        )
        //.add_effect_density_map(256, 256, density_map_output_directory)
        //.add_effect_density_map(512, 512, density_map_output_directory)
        //.add_effect_density_map(1024, 1024, density_map_output_directory)
        .add_effect_density_map(4096, 4096, density_map_output_directory)
        .add_scene_sink_obj_mtl(obj_file, mtl_file)
        .hit_map_path(hit_map_path)
        .iterations(1)
        .build()
        .run();
}