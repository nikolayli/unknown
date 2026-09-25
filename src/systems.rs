use minifb::{Key, Window};
use crate::constants::{W, H};
use crate::ecs::World;

pub fn input_system(world: &mut World, window: &Window) {
    let speed = 200.0;
    for id in 0..world.positions.len() {
        if world.players[id].is_some() {
            if let Some(vel) = &mut world.velocities[id] {
                vel.x = 0.0;
                vel.y = 0.0;
                if window.is_key_down(Key::Left)  { vel.x -= speed; }
                if window.is_key_down(Key::Right) { vel.x += speed; }
                if window.is_key_down(Key::Up)    { vel.y -= speed; }
                if window.is_key_down(Key::Down)  { vel.y += speed; }
            }
        }
    }
}

pub fn movement_system(world: &mut World, dt: f32) {
    for id in 0..world.positions.len() {
        if let (Some(pos), Some(vel)) = (&mut world.positions[id], &world.velocities[id]) {
            pos.x += vel.x * dt;
            pos.y += vel.y * dt;

            if world.players[id].is_some() {
                if let Some(sprite) = &world.sprites[id] {
                    pos.x = pos.x.clamp(0.0, W as f32 - sprite.w);
                    pos.y = pos.y.clamp(0.0, H as f32 - sprite.h);
                }
            }
        }
    }
}

pub fn render_system(world: &World, buf: &mut [u32]) {
    for id in 0..world.positions.len() {
        if let (Some(pos), Some(sprite)) = (&world.positions[id], &world.sprites[id]) {
            draw_rect(buf, pos.x, pos.y, sprite.w, sprite.h, sprite.color);
        }
    }
}

fn draw_rect(buf: &mut [u32], rx: f32, ry: f32, rw: f32, rh: f32, color: u32) {
    let x0 = rx.max(0.0) as usize;
    let y0 = ry.max(0.0) as usize;
    let x1 = ((rx + rw) as usize).min(W);
    let y1 = ((ry + rh) as usize).min(H);
    for y in y0..y1 {
        for x in x0..x1 {
            buf[y * W + x] = color;
        }
    }
}
