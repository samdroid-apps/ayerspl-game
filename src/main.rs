extern crate three;
extern crate mint;
extern crate rand;

use three::Object;
use three::controls::Key;
use rand::prelude::*;

mod color;
mod roadlane;


const CAMERA_MOVEMENT_SPEED: f32 = 5.;

fn new_building<R: rand::Rng>(
    factory: &mut three::Factory,
    rng: &mut R,
    x_width: f32,
    y_width: f32) -> three::Mesh {

    let stories = rng.gen_range(1, 9);

    let height = (stories as f32) * 1.0;
    let geometry = three::Geometry::cuboid(x_width, y_width, height);
    let material = three::material::Basic {
        color: color::get_building_color(rng),
        map: None,
    };
    let mesh = factory.mesh(geometry, material);
    mesh
}


fn main() {
    let mut win = three::Window::new("Game");
    let mut rng = rand::prng::XorShiftRng::from_seed([
        0xFF, 0x04, 0x20, 0xFF, 0xFF, 0x04, 0x20, 0xFF,
        0xFF, 0x04, 0x20, 0xFF, 0xFF, 0x04, 0x20, 0xFF]);

    let mut dir_light = win.factory.directional_light(0xffffff, 0.9);
    let shadow_map = win.factory.shadow_map(1024*2, 1024*2);
    /*
    let _debug_shadow = win.renderer
        .debug_shadow_quad(&shadow_map, 1, [10, 10], [256, 256]);
    */
	dir_light.set_shadow(shadow_map, 60.0, 1.0 .. 200.0);
	win.scene.add(&dir_light);

    let ground_geometry = three::Geometry::plane(300., 300.);
    let ground_material = three::material::Lambert {
        color: 0xFFFFFF,
        flat: false,
    };
    let ground_mesh = win.factory.mesh(ground_geometry, ground_material);
    ground_mesh.set_position([0., 0., roadlane::ROAD_LEVEL_Z-0.0001]);
	win.scene.add(&ground_mesh);

    /*
    for x in -5..6 {
        for y in -5..6 {
            let bld = new_building(&mut win.factory, &mut rng, 2.0, 2.0);
            bld.set_position([3.0*(x as f32), 3.0*(y as f32), 0.]);
            win.scene.add(&bld);
        }
    }
    */

    let lane = roadlane::Lane::new([0., 0.].into(), [1., 2.].into());
    let road_object = lane.get_object(&mut win.factory);
    win.scene.add(&road_object);


    win.scene.background = three::Background::Color(0xC6F0FF);

    let camera = win.factory.perspective_camera(60.0, 0.1..100.);
    let mut camera_pos: mint::Point3<f32> = [0., 0., 15.].into();

    while win.update() {
        if win.input.hit(three::KEY_ESCAPE) {
            return;
        }

        if win.input.hit(Key::Right) || win.input.hit(Key::D) {
            camera_pos.x += CAMERA_MOVEMENT_SPEED * win.input.delta_time();
        }
        if win.input.hit(Key::Left) || win.input.hit(Key::A) {
            camera_pos.x -= CAMERA_MOVEMENT_SPEED * win.input.delta_time();
        }
        if win.input.hit(Key::Up) || win.input.hit(Key::W) {
            camera_pos.y += CAMERA_MOVEMENT_SPEED * win.input.delta_time();
        }
        if win.input.hit(Key::Down) || win.input.hit(Key::S) {
            camera_pos.y -= CAMERA_MOVEMENT_SPEED * win.input.delta_time();
        }
        if win.input.hit(Key::Q) {
            camera_pos.z -= CAMERA_MOVEMENT_SPEED * win.input.delta_time();
        }
        if win.input.hit(Key::E) {
            camera_pos.z += CAMERA_MOVEMENT_SPEED * win.input.delta_time();
        }
        camera.set_position(camera_pos);
        dir_light.set_position(camera_pos);
        dir_light.look_at(camera_pos, [10.0, 10.0, 0.0], None);

        win.render(&camera);
    }
}

