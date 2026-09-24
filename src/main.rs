use minifb::{Key, Window, WindowOptions};

const W: usize = 640;
const H: usize = 480;

struct Rect { x: f32, y: f32, w: f32, h: f32, vx: f32, vy: f32 }

fn main() {
    let mut window = Window::new("g0", W, H, WindowOptions::default()).unwrap();
    window.set_target_fps(60);

    let mut buf = vec![0u32; W * H];
    let mut player = Rect { x: 320.0, y: 240.0, w: 40.0, h: 30.0, vx: 0.0, vy: 0.0 };

    let mut last = std::time::Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = std::time::Instant::now();
        let dt = (now - last).as_secs_f32();
        last = now;

        let speed = 200.0;
        player.vx = 0.0; player.vy = 0.0;
        if window.is_key_down(Key::Left) { player.vx -= speed; }
        if window.is_key_down(Key::Right) { player.vx += speed; }
        if window.is_key_down(Key::Up) { player.vy -= speed; }
        if window.is_key_down(Key::Down) { player.vy += speed; }

        player.x += player.vx * dt;
        player.y += player.vy * dt;

        player.x = player.x.clamp(0.0, W as f32 -player.w);
        player.y = player.y.clamp(0.0, H as f32 -player.h);

        buf.fill(0x00102030);
        draw_rect(&mut buf, &player, 0x00FF00);

        window.update_with_buffer(&buf, W, H).unwrap();
    }
}

fn draw_rect(buf: &mut [u32], r: &Rect, color: u32) {
    let x0 = r.x.max(0.0) as usize;
    let y0 = r.y.max(0.0) as usize;
    let x1 = ((r.x + r.w) as usize).min(W);
    let y1 = ((r.y + r.h) as usize).min(H);
    for y in y0..y1 {
        for x in x0..x1 {
            buf[y * W + x] = color;
        }
    }
}
