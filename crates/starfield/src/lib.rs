use egui::{Color32, Pos2};
use std::f32::consts::TAU;
use std::ops::AddAssign;

pub mod ui;
use pcg::Pcg;

pub const NUM_STARS: usize = 2048;

#[derive(Clone)]
pub struct SfPixel {
    pub pos2: Pos2,
    pub c: Color32,
}

impl Default for SfPixel {
    fn default() -> Self {
        Self {
            pos2: Pos2::ZERO,
            c: Color32::BLACK,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x: x, y: y, z: z }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Clone, Copy)]
struct Matrix {
    x: Vec3,
    y: Vec3,
    z: Vec3,
}

impl Default for Matrix {
    fn default() -> Self {
        Self {
            x: Vec3::default(),
            y: Vec3::default(),
            z: Vec3::default(),
        }
    }
}

pub struct StarField {
    pub position: Vec<SfPixel>,  // x, y, rgba
    pub screen_size: (u32, u32), // screen width x height
    pub rotation_frame: Vec3,
    star_points: Vec<Vec3>, // fixed stars
    rotation: Vec3,
    matrix: Matrix,
    rng: Pcg,
}

impl Default for StarField {
    fn default() -> Self {
        let mut s = Self {
            position: vec![],
            screen_size: (400, 400),
            rotation_frame: Vec3 {
                x: -0.00011,
                y: 0.00006,
                z: 0.0,
            },
            star_points: vec![],
            rotation: Vec3::default(),
            matrix: Matrix::default(),
            rng: Pcg::default(),
        };
        for _ in 0..NUM_STARS {
            s.star_points.push(Vec3::new(
                s.rng.genf32() - 0.5,
                s.rng.genf32() - 0.5,
                s.rng.genf32() - 0.5,
            ));

        }
        s
    }
}

impl StarField {
    pub fn frame(&mut self) {
        self.rotate_matrix();
        self.starfield();
        self.rotation += self.rotation_frame;
    }

    fn rotate_matrix(&mut self) {
        let xx = self.rotation.x * TAU;
        let sa = xx.sin();
        let ca = xx.cos();

        let yy = self.rotation.y * TAU;
        let sb = yy.sin();
        let cb = yy.cos();

        let zz = self.rotation.z * TAU;
        let sc = zz.sin();
        let cc = zz.cos();

        // x
        self.matrix.x.x = (ca * cc) + (sc * sa * sb);
        self.matrix.y.x = sa * cb;
        self.matrix.z.x = (cc * sa * sb) - (ca * sc);

        // y
        self.matrix.x.y = (sc * ca * sb) - (sa * cc);
        self.matrix.y.y = ca * cb;
        self.matrix.z.y = (sa * sc) + (cc * ca * sb);

        // z
        self.matrix.x.z = cb * sc;
        self.matrix.y.z = 0.0 - sb;
        self.matrix.z.z = cb * cc;
    }

    fn starfield(&mut self) {
        self.position = vec![];

        for i in 0..NUM_STARS {
            // Z

            self.star_points[i].z += 0.00015;
            if self.star_points[i].z >= 0.5 {
                self.star_points[i].z -= 1.0;
            }

            let z = (self.matrix.x.z * self.star_points[i].x)
                + (self.matrix.y.z * self.star_points[i].y)
                + (self.matrix.z.z * self.star_points[i].z);
            if z < 0.0 {
                continue;
            }

            // Y

            let y = ((self.matrix.x.y * self.star_points[i].x)
                + (self.matrix.y.y * self.star_points[i].y)
                + (self.matrix.z.y * self.star_points[i].z))
                / z;

            let y = (self.screen_size.1) as f32 * (y / 2.0);
            if y >= self.screen_size.1 as f32 || y <= 0.0 {
                continue;
            }

            // X

            let x = ((self.matrix.x.x * self.star_points[i].x)
                + (self.matrix.y.x * self.star_points[i].y)
                + (self.matrix.z.x * self.star_points[i].z))
                / z;

            let x = (self.screen_size.0) as f32 * (x / 2.0);
            if x >= self.screen_size.0 as f32 || x <= 0.0 {
                continue;
            }

            let mut d: i32 = (z * 255.0) as i32;
            if d > 255 {
                d = 255;
            }

            d ^= 255;

            let mut r: i32 = d - 75;
            if r < 0 {
                r = 0;
            }

            let mut g: i32 = d - 38;
            if g < 0 {
                g = 0;
            }

            let mut b: i32 = d + 64;
            if b > 255 {
                b = 255;
            }

            self.position.push(SfPixel {
                pos2: Pos2::new(x.ceil(), y.ceil()),
                c: Color32::from_rgb(r as u8, g as u8, b as u8),
            });
        }
    }
}
