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
    let count = world.positions.len();

    for id in 0..count {
        if let (Some(vel), Some(sprite)) = (&world.velocities[id], &world.sprites[id]) {
            let w = sprite.w;
            let h = sprite.h;
            let vx = vel.x;
            let vy = vel.y;

            if let Some(pos) = &mut world.positions[id] {
                pos.x += vx * dt;

                pos.x = pos.x.clamp(0.0, W as f32 - w);
            }

            for other_id in 0..count {
                if id == other_id { continue; }

                if world.colliders[other_id].is_some() {
                    let mut collision = false;
                    let mut wall_x = 0.0;
                    let mut wall_w = 0.0;

                    if let (Some(pos), Some(wall_pos), Some(wall_sprite)) =
                        (&world.positions[id], &world.positions[other_id], &world.sprites[other_id])
                    {
                        if check_aabb(pos.x, pos.y, w, h, wall_pos.x, wall_pos.y, wall_sprite.w, wall_sprite.h) {
                            collision = true;
                            wall_x = wall_pos.x;
                            wall_w = wall_sprite.w;
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
                if id == other_id { continue; }

                if world.colliders[other_id].is_some() {
                    let mut collision = false;
                    let mut wall_y = 0.0;
                    let mut wall_h = 0.0;

                    if let (Some(pos), Some(wall_pos), Some(wall_sprite)) =
                        (&world.positions[id], &world.positions[other_id], &world.sprites[other_id])
                    {
                        if check_aabb(pos.x, pos.y, w, h, wall_pos.x, wall_pos.y, wall_sprite.w, wall_sprite.h) {
                            collision = true;
                            wall_y = wall_pos.y;
                            wall_h = wall_sprite.h;
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

fn check_aabb( ax: f32, ay: f32, aw: f32, ah: f32, bx: f32, by: f32, bw: f32, bh: f32) -> bool {
    ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
}
