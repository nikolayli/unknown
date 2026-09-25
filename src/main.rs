use minifb::{Key, Window, WindowOptions};

mod constants;
mod ecs;
mod systems;

use constants::{W, H};
use ecs::{World, Position, Velocity, Sprite, PlayerControlled};

fn main() {
    let mut window = Window::new("g0", W, H, WindowOptions::default()).unwrap();
    window.set_target_fps(60);

    let mut buf = vec![0u32; W * H];
    let mut last = std::time::Instant::now();

    let mut world = World::new();

    let player_id = world.create_entity();
    world.positions[player_id] = Some(Position { x: 320.0, y: 240.0 });
    world.velocities[player_id] = Some(Velocity { x: 0.0, y: 0.0 });
    world.sprites[player_id] = Some(Sprite { w: 40.0, h: 30.0, color: 0x00FF00 });
    world.players[player_id] = Some(PlayerControlled);

    let box_id = world.create_entity();
    world.positions[box_id] = Some(Position { x: 100.0, y: 100.0 });
    world.sprites[box_id] = Some(Sprite { w: 50.0, h: 50.0, color: 0xFF0000 });

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
