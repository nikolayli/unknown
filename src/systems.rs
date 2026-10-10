use crate::constants::{H, W};
use crate::ecs::World;
use minifb::{Key, Window};

pub fn input_system(world: &mut World, window: &Window) {
    let speed = 200.0;
    for id in 0..world.positions.len() {
        if world.players[id].is_some() {
            if let Some(vel) = &mut world.velocities[id] {
                vel.x = 0.0;
                vel.y = 0.0;
                if window.is_key_down(Key::Left) {
                    vel.x -= speed;
                }
                if window.is_key_down(Key::Right) {
                    vel.x += speed;
                }
                if window.is_key_down(Key::Up) {
                    vel.y -= speed;
                }
                if window.is_key_down(Key::Down) {
                    vel.y += speed;
                }
            }
        }
    }
}

pub fn movement_system(world: &mut World, dt: f32) {
    let count = world.positions.len();

    for id in 0..count {
        if let (Some(vel), Some(sprite)) = (&world.velocities[id], &world.sprites[id]) {
            let w = sprite.w as f32;
            let h = sprite.h as f32;
            let vx = vel.x;
            let vy = vel.y;

            if let Some(pos) = &mut world.positions[id] {
                pos.x += vx * dt;
                pos.x = pos.x.clamp(0.0, W as f32 - w);
            }

            for other_id in 0..count {
                if id == other_id {
                    continue;
                }

                if world.colliders[other_id].is_some() {
                    let mut collision = false;
                    let mut wall_x = 0.0;
                    let mut wall_w = 0.0;

                    if let (Some(pos), Some(wall_pos), Some(wall_sprite)) = (
                        &world.positions[id],
                        &world.positions[other_id],
                        &world.sprites[other_id],
                    ) {
                        let ww = wall_sprite.w as f32;
                        let wh = wall_sprite.h as f32;

                        if check_aabb(pos.x, pos.y, w, h, wall_pos.x, wall_pos.y, ww, wh) {
                            collision = true;
                            wall_x = wall_pos.x;
                            wall_w = ww;
                        }
                    }

                    if collision {
                        if let Some(pos) = &mut world.positions[id] {
                            if vx > 0.0 {
                                pos.x = wall_x - w;
                            } else if vx < 0.0 {
                                pos.x = wall_x + wall_w;
                            }
                        }
                    }
                }
            }

            if let Some(pos) = &mut world.positions[id] {
                pos.y += vy * dt;

                pos.y = pos.y.clamp(0.0, H as f32 - h);
            }

            for other_id in 0..count {
                if id == other_id {
                    continue;
                }

                if world.colliders[other_id].is_some() {
                    let mut collision = false;
                    let mut wall_y = 0.0;
                    let mut wall_h = 0.0;

                    if let (Some(pos), Some(wall_pos), Some(wall_sprite)) = (
                        &world.positions[id],
                        &world.positions[other_id],
                        &world.sprites[other_id],
                    ) {
                        let ww = wall_sprite.w as f32;
                        let wh = wall_sprite.h as f32;

                        if check_aabb(pos.x, pos.y, w, h, wall_pos.x, wall_pos.y, ww, wh) {
                            collision = true;
                            wall_y = wall_pos.y;
                            wall_h = wh;
                        }
                    }

                    if collision {
                        if let Some(pos) = &mut world.positions[id] {
                            if vy > 0.0 {
                                pos.y = wall_y - h;
                            } else if vy < 0.0 {
                                pos.y = wall_y + wall_h;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn render_system(world: &World, buf: &mut [u32]) {
    for id in 0..world.positions.len() {
        if let (Some(pos), Some(sprite)) = (&world.positions[id], &world.sprites[id]) {
            draw_sprite(buf, pos.x, pos.y, sprite);
        }
    }
}

fn draw_sprite(buf: &mut [u32], rx: f32, ry: f32, sprite: &crate::ecs::Sprite) {
    let start_x = rx.round() as i32;
    let start_y = ry.round() as i32;

    for sy in 0..sprite.h {
        let screen_y = start_y + sy as i32;
        if screen_y < 0 || screen_y >= H as i32 {
            continue;
        }

        for sx in 0..sprite.w {
            let screen_x = start_x + sx as i32;
            if screen_x < 0 || screen_x >= W as i32 {
                continue;
            }

            let pixel = sprite.pixels[sy * sprite.w + sx];

            let alpha = (pixel >> 24) & 0xFF;
            if alpha == 0 {
                continue;
            }

            buf[screen_y as usize * W + screen_x as usize] = pixel;
        }
    }
}

fn check_aabb(ax: f32, ay: f32, aw: f32, ah: f32, bx: f32, by: f32, bw: f32, bh: f32) -> bool {
    ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
}
