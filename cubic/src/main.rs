use std::{thread, time};

struct Cube {
    width: usize,
    height: usize,
    distance_from_camera: f32,
    cube_width: f32,
    background_ascii_code: char,
    speed: f32,
    k1: f32,
    z_buffer: Vec<f32>,
    buffer: Vec<char>,
    a: f32,
    b: f32,
    c: f32,
}

impl Cube {
    fn new(w: usize, h: usize, d: f32, cw: f32) -> Cube {
        let area = w * h;
        Cube {
            width: w,
            height: h,
            distance_from_camera: d,
            cube_width: cw,
            background_ascii_code: '-',
            speed: 0.57,
            k1: 40.0,
            z_buffer: vec![0.0; area],
            buffer: vec!['-'; area],
            a: 0.0,
            b: 0.0,
            c: 0.0,
        }
    }

    fn calculate_x(&self, i: f32, j: f32, k: f32) -> f32 {
        j * (self.a).sin() * (self.b).sin() * (self.c).cos() -
        k * (self.a).cos() * (self.b).sin() * (self.c).cos() +
        j * (self.a).cos() * (self.c).sin() + 
        k * (self.a).sin() * (self.c).sin() + 
        i * (self.b).cos() * (self.c).cos()
    }

    fn calculate_y(&self, i: f32, j: f32, k: f32) -> f32 {
        j * (self.a).cos() * (self.c).cos() + 
        k * (self.a).sin() * (self.c).cos() -
        j * (self.a).sin() * (self.b).sin() * (self.c).sin() + 
        k * (self.a).cos() * (self.b).sin() * (self.c).sin() -
        i * (self.b).cos() * (self.c).sin()
    }

    fn calculate_z(&self, i: f32, j: f32, k: f32) -> f32 {
        k * (self.a).cos() * (self.b).cos() - 
        j * (self.a).sin() * (self.b).cos() + 
        i * (self.b).sin()
    }

    fn calculate_surf(&mut self, cube_x: f32, cube_y: f32, cube_z: f32, ch: char) {
        let x = self.calculate_x(cube_x, cube_y, cube_z);
        let y = self.calculate_y(cube_x, cube_y, cube_z);
        let z = self.calculate_z(cube_x, cube_y, cube_z) + self.distance_from_camera;

        let ooz = 1.0 / z;

        let xp = (self.width as f32 / 2.0 + self.k1 * ooz * x * 2.0) as usize;
        let yp = (self.height as f32 / 2.0 + self.k1 * ooz * y) as usize;

        let idx = xp + yp * self.width;
        if idx < self.buffer.len() && ooz > self.z_buffer[idx] {
            self.z_buffer[idx] = ooz;
            self.buffer[idx] = ch;
        }
    }

    fn process(&mut self) {
        let sleep_duration = time::Duration::from_millis(16);
        loop {
            self.buffer.fill(self.background_ascii_code);
            self.z_buffer.fill(0.0);

            let mut cube_x: f32 = -self.cube_width;
            while cube_x <= self.cube_width {
                let mut cube_y: f32 = -self.cube_width;
                while cube_y <= self.cube_width {
                    self.calculate_surf(          cube_x,           cube_y, -self.cube_width, '@');
                    self.calculate_surf( self.cube_width,           cube_y,           cube_x, '$');
                    self.calculate_surf(-self.cube_width,           cube_y,          -cube_x, '~');
                    self.calculate_surf(         -cube_x,           cube_y,  self.cube_width, '#');
                    self.calculate_surf(          cube_x, -self.cube_width,          -cube_y, ';');
                    self.calculate_surf(          cube_x,  self.cube_width,           cube_y, '+');
                    
                    cube_y += self.speed;
                }
                cube_x += self.speed;
            }
            
            print!("\x1b[H");
            for k in 0..self.buffer.len() {
                if k % self.width == 0 { println!() };
                print!("{}", self.buffer[k]);
            }

            self.a += 0.05;
            self.b += 0.05;
            self.c += 0.01;
            thread::sleep(sleep_duration);
        }
    }
}

fn main() {
    let mut cube = Cube::new(110, 44, 100.0, 20.0);
    cube.process();
}
