use minifb::{Key, Window, WindowOptions};
use std::path::Path;

mod constants;
mod ecs;
mod systems;

use constants::{W, H};
use ecs::{World, Position, Velocity, Sprite, PlayerControlled, Collider};

fn load_sprite_from_png<P: AsRef<Path>>(path: P) -> Sprite {
    let img = image::open(path)
        .expect("Failed to load the image! Check the file path.")
        .to_rgba8();

    let (width, height) = img.dimensions();
    let mut pixels = Vec::with_capacity((width * height) as usize);

    for pixel in img.pixels() {
        let r = pixel.0[0] as u32;
        let g = pixel.0[1] as u32;
        let b = pixel.0[2] as u32;
        let a = pixel.0[3] as u32;

        let argb_pixel = (a << 24) | (r << 16) | (g << 8) | b;
        pixels.push(argb_pixel);
    }

    Sprite {
        w: width as usize,
        h: height as usize,
        pixels,
    }
}

fn main() {
    let mut window = Window::new("g0", W, H, WindowOptions::default()).unwrap();
    window.set_target_fps(60);

    let mut buf = vec![0u32; W * H];
    let mut last = std::time::Instant::now();

    let mut world = World::new();

    let player_sprite = load_sprite_from_png("assets/player.png");

    let player_id = world.create_entity();
    world.positions[player_id] = Some(Position { x: 320.0, y: 240.0 });
    world.velocities[player_id] = Some(Velocity { x: 0.0, y: 0.0 });
    world.sprites[player_id] = Some(player_sprite);
    world.players[player_id] = Some(PlayerControlled);

    let box_id = world.create_entity();
    world.positions[box_id] = Some(Position { x: 100.0, y: 100.0 });
    world.sprites[box_id] = Some(Sprite {
        w: 50,
        h: 50,
        pixels: vec![0xFFFF0000; 50 * 50],
    });
    world.colliders[box_id] = Some(Collider);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = std::time::Instant::now();
        let dt = (now - last).as_secs_f32();
        last = now;

        buf.fill(0x00102030);

        systems::input_system(&mut world, &window);
        systems::movement_system(&mut world, dt);
        systems::render_system(&world, &mut buf);

        window.update_with_buffer(&buf, W, H).unwrap();
    }
}
