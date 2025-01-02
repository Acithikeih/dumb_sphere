use minifb::{Key, ScaleMode, Window, WindowOptions};
use std::f64::consts::PI;

const WIDTH: usize = 1080;
const HEIGHT: usize = 1080;

fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let buffer2 = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Sphere",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create the window");

    window.set_target_fps(60);

    let mut r = 400;
    let cx: i64 = WIDTH as i64 / 2;
    let cy: i64 = HEIGHT as i64 / 2;
    let dots_count = 400;
    let n = 24;
    let mut lon0 = 0.;
    let mut lat0 = 0.;
    let mut roll = 0.;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for l in -n..n {
            for i in 1..dots_count {
                let mut lat = -PI / 2. + PI * i as f64 / dots_count as f64;
                let mut lon = l as f64 * PI / n as f64;
                let c = f64::sin(lat0) * f64::sin(lat)
                    + f64::cos(lat0) * f64::cos(lat) * f64::cos(lon - lon0);

                if c < 0. {
                    let x1 = r as f64 * f64::cos(lat) * f64::sin(lon - lon0);
                    let y1 = r as f64
                        * (f64::cos(lat0) * f64::sin(lat)
                            - f64::sin(lat0) * f64::cos(lat) * f64::cos(lon - lon0));
                    let x2 = x1 * f64::cos(roll) - y1 * f64::sin(roll);
                    let y2 = x1 * f64::sin(roll) + y1 * f64::cos(roll);
                    let x3 = (cx + x2 as i64) as usize;
                    let y3 = (cy + y2 as i64) as usize;
                    if x3 < WIDTH && y3 < HEIGHT {
                        buffer[x3 + y3 * WIDTH] = 0x00FFFFFF;
                    }
                }

                std::mem::swap(&mut lat, &mut lon);
                let c = f64::sin(lat0) * f64::sin(lat)
                    + f64::cos(lat0) * f64::cos(lat) * f64::cos(lon - lon0);

                if c < 0. {
                    let x1 = r as f64 * f64::cos(lat) * f64::sin(lon - lon0);
                    let y1 = r as f64
                        * (f64::cos(lat0) * f64::sin(lat)
                            - f64::sin(lat0) * f64::cos(lat) * f64::cos(lon - lon0));
                    let x2 = x1 * f64::cos(roll) - y1 * f64::sin(roll);
                    let y2 = x1 * f64::sin(roll) + y1 * f64::cos(roll);
                    let x3 = (cx + x2 as i64) as usize;
                    let y3 = (cy + y2 as i64) as usize;
                    if x3 < WIDTH && y3 < HEIGHT {
                        buffer[x3 + y3 * WIDTH] = 0x00FFFFFF;
                    }
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        buffer = buffer2.clone();

        if window.is_key_down(Key::Up) {
            lat0 += 0.025;
            if lat0 >= PI / 2. {
                lat0 = PI / 2.;
            }
        }

        if window.is_key_down(Key::Down) {
            lat0 -= 0.025;
            if lat0 < -PI / 2. {
                lat0 = -PI / 2.;
            }
        }

        if window.is_key_down(Key::Right) {
            lon0 += 0.025;
            if lon0 >= 2. * PI {
                lon0 -= 2. * PI;
            }
        }

        if window.is_key_down(Key::Left) {
            lon0 -= 0.025;
            if lon0 < 0. {
                lon0 += 2. * PI;
            }
        }

        if window.is_key_down(Key::NumPadPlus) {
            roll += 0.025;
            if roll >= 2. * PI {
                roll -= 2. * PI;
            }
        }

        if window.is_key_down(Key::NumPadMinus) {
            roll -= 0.025;
            if roll < 0. {
                roll += 2. * PI;
            }
        }

        if let Some((_, scroll_y)) = window.get_scroll_wheel() {
            r += 4 * scroll_y as i64;
            r = r.clamp(60, 1200);
        }
    }
}
