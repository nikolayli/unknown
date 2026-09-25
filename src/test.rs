use minifb::{Key, Window, WindowOptions};

const W: usize = 640;
const H: usize = 480;

// ==========================================
// 1. КОМПОНЕНТЫ (Просто структуры данных)
// ==========================================
struct Position { x: f32, y: f32 }
struct Velocity { x: f32, y: f32 }
struct Sprite { w: f32, h: f32, color: u32 }
// Маркерный компонент, чтобы система ввода знала, кем управлять
struct PlayerControlled;

// ==========================================
// МИР (Хранилище всех сущностей и компонентов)
// ==========================================
struct World {
    // Индексы в массивах — это и есть ID наших сущностей (Entity ID)
    // Option нужен, потому что не у каждой сущности есть все компоненты
    positions: Vec<Option<Position>>,
    velocities: Vec<Option<Velocity>>,
    sprites: Vec<Option<Sprite>>,
    players: Vec<Option<PlayerControlled>>,
}

impl World {
    fn new() -> Self {
        Self {
            positions: Vec::new(),
            velocities: Vec::new(),
            sprites: Vec::new(),
            players: Vec::new(),
        }
    }

    // Метод для создания «пустой» сущности и получения её ID
    fn create_entity(&mut self) -> usize {
        let id = self.positions.len();
        self.positions.push(None);
        self.velocities.push(None);
        self.sprites.push(None);
        self.players.push(None);
        id
    }
}

fn main() {
    let mut window = Window::new("g0 - Steps into ECS", W, H, WindowOptions::default()).unwrap();
    window.set_target_fps(60);

    let mut buf = vec![0u32; W * H];
    let mut last = std::time::Instant::now();

    // Создаем наш игровой мир
    let mut world = World::new();

    // Создаем сущность Игрока и навешиваем на неё компоненты
    let player_id = world.create_entity();
    world.positions[player_id] = Some(Position { x: 320.0, y: 240.0 });
    world.velocities[player_id] = Some(Velocity { x: 0.0, y: 0.0 });
    world.sprites[player_id] = Some(Sprite { w: 40.0, h: 30.0, color: 0x00FF00 }); // Зеленый
    world.players[player_id] = Some(PlayerControlled);

    // ВАЖНОСТЬ ECS: Мы можем легко создать статичную коробку (декорацию) за 3 строки!
    let box_id = world.create_entity();
    world.positions[box_id] = Some(Position { x: 100.0, y: 100.0 });
    // У коробки НЕТ компонента Velocity, поэтому никакие системы движения её не тронут
    world.sprites[box_id] = Some(Sprite { w: 50.0, h: 50.0, color: 0xFF0000 }); // Красный

    // Игровой цикл
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = std::time::Instant::now();
        let dt = (now - last).as_secs_f32();
        last = now;

        // Очищаем экран подложкой темно-синего цвета
        buf.fill(0x00102030);

        // ==========================================
        // 3. СИСТЕМЫ (Логика движка)
        // ==========================================

        // Система 1: Обработка ввода (Управляет только сущностями с PlayerControlled и Velocity)
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

        // Система 2: Движение (Обновляет позиции всех объектов, у которых есть скорость)
        for id in 0..world.positions.len() {
            if let (Some(pos), Some(vel)) = (&mut world.positions[id], &world.velocities[id]) {
                pos.x += vel.x * dt;
                pos.y += vel.y * dt;

                // Если это игрок, зажмем его в рамках экрана (временно, пока нет коллизий)
                if world.players[id].is_some() {
                    if let Some(sprite) = &world.sprites[id] {
                        pos.x = pos.x.clamp(0.0, W as f32 - sprite.w);
                        pos.y = pos.y.clamp(0.0, H as f32 - sprite.h);
                    }
                }
            }
        }

        // Система 3: Отрисовка (Рендерит всё, у чего есть Позиция и Спрайт)
        for id in 0..world.positions.len() {
            if let (Some(pos), Some(sprite)) = (&world.positions[id], &world.sprites[id]) {
                draw_rect(&mut buf, pos.x, pos.y, sprite.w, sprite.h, sprite.color);
            }
        }

        window.update_with_buffer(&buf, W, H).unwrap();
    }
}

// Немного переписанная функция отрисовки, принимающая чистые параметры компонентов
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
