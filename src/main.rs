use rand::prelude::*;
use image::{RgbImage, Rgb};
use std::time::Instant;

fn main() {
    let width = 200;
    let height = 200;
    let mut rng = rand::thread_rng();
    let mut input = vec![vec![Color { r: 0, g: 0, b: 0 }; width]; height];
    for x in 0 .. width {
        for y in 0 .. height {
            input[y][x] = Color { r: rng.gen_range(0 .. 255), g: rng.gen_range(0 .. 255), b: rng.gen_range(0 .. 255) };
        }
    }
    let start = Instant::now();
    let generated = generate_image(width, height, &input);
    println!("{}", start.elapsed().as_millis());
    let mut img = RgbImage::new(width as u32, height as u32);
    for x in 0 .. width {
        for y in 0 .. height {
            let color = generated[y][x];
            img.put_pixel(x as u32, y as u32, Rgb([color.r as u8, color.g as u8, color.b as u8]));
        }
    }
    img.save("./out.png");
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize
}

#[derive(Debug, Copy, Clone)]
struct Color {
    r: usize,
    g: usize,
    b: usize
}

impl Color {
    fn dist(&self, other: &Color) -> usize {
        usize::pow(self.r - other.r, 2) + usize::pow(self.g - other.g, 2) + usize::pow(self.b - other.b, 2)
    }
}

fn generate_image(width: usize, height: usize, input: &Vec<Vec<Color>>) -> Vec<Vec<Color>> {
    let mut pixels = vec![vec![(Color { r: 0, g: 0, b: 0 }, false); width]; height];
    let mut spots = vec![Point { x: width/2, y: height/2 }];
    let mut x = 0;
    let mut y = 0;

    macro_rules! neighbors {
        ( $spot:expr, $fn:block ) => {
            let x_upper = if $spot.x == width-1 {
                $spot.x
            } else {
                $spot.x + 1
            };
            let x_lower = if $spot.x == 0 {
                $spot.x
            } else {
                $spot.x - 1
            };
            let y_upper = if $spot.y == height-1 {
                $spot.y
            } else {
                $spot.y + 1
            };
            let y_lower = if $spot.y == 0 {
                $spot.y
            } else {
                $spot.y - 1
            };
            for xx in x_lower ..= x_upper {
                for yy in y_lower ..= y_upper {
                    if xx == $spot.x && yy == $spot.y {
                        continue;
                    }

                    x = xx;
                    y = yy;

                    $fn
                }
            }
        }
    }

    let mut rng = rand::thread_rng();
    let mut i = 0;
    while !spots.is_empty() {
        let rand_x = rng.gen_range(0 .. width);
        let rand_y = rng.gen_range(0 .. height);
        let color = input[rand_y][rand_x];
        let best_spot_index = spots.iter().enumerate().min_by_key(|(i, spot)| {
            let mut total_distance = 0;
            neighbors!(spot, {
                total_distance += pixels[y][x].0.dist(&color);
            });
            total_distance
        }).unwrap().0;
        let best_spot = spots.remove(best_spot_index);
        pixels[best_spot.y][best_spot.x] = (color, true);
        neighbors!(best_spot, {
            if !pixels[y][x].1 && !spots.iter().any(|spot| spot.x == x && spot.y == y) {
                spots.push(Point { x, y });
            }
        });
    }

    return pixels.iter().map(|vec| vec.iter().map(|(color, _)| *color).collect()).collect();
}
