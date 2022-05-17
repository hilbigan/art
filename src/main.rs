use itertools::iproduct;
use rand::prelude::*;
use std::time::Instant;

fn main() {
    let width = 200;
    let height = 200;
    let mut rng = rand::thread_rng();
    let input = vec![vec![Color { r: rng.gen_range(0 .. 255), g: rng.gen_range(0 .. 255), b: rng.gen_range(0 .. 255) }; width]; height];
    let start = Instant::now();
    println!("{:?}", generate_image(width, height, &input)[0][0]);
    println!("{}", start.elapsed().as_millis());
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize
}

impl<'a> Point {
    fn iter_neighbors(&'a self, width: usize, height: usize) -> Box<dyn Iterator<Item = Point> + 'a> {
        Box::new(iproduct!((self.x-1..=self.x+1).into_iter(), (self.y-1..=self.y+1).into_iter()).filter(move |(x, y)| {
            (*x != self.x || *y != self.y) && (*x >= 0 && *x < width && *y >= 0 && *y < height)
        }).map(|(x, y)| Point { x, y }))
    }
}


#[derive(Debug, Copy, Clone)]
struct Color {
    r: usize,
    g: usize,
    b: usize
}

impl Color {
    fn dist(&self, other: &Color) -> usize {
        (usize::pow(self.r - other.r, 2) + usize::pow(self.g - other.b, 2) + usize::pow(self.b - other.b, 2))
    }
}

fn generate_image(width: usize, height: usize, input: &Vec<Vec<Color>>) -> Vec<Vec<Color>> {
    let mut pixels = vec![vec![(Color { r: 0, g: 0, b: 0 }, false); width]; height];
    let mut spots = vec![Point { x: width/2, y: height/2 }];

    let mut rng = rand::thread_rng();
    while !spots.is_empty() {
        let rand_x = rng.gen_range(0 .. width);
        let rand_y = rng.gen_range(0 .. height);
        let color = input[rand_y][rand_x];
        let best_spot_index = spots.iter().enumerate().min_by_key(|(i, spot)| {
            let mut totalDistance = 0;
            spot.iter_neighbors(width, height).for_each(|Point { x, y }| {
                totalDistance += pixels[y][x].0.dist(&color);
            });
            totalDistance
        }).unwrap().0;
        let best_spot = spots.remove(best_spot_index);
        pixels[best_spot.y][best_spot.x] = (color, true);
        best_spot.iter_neighbors(width, height).for_each(|Point { x, y }| {
            if !pixels[y][x].1 && !spots.iter().any(|spot| spot.x == x && spot.y == y) {
                spots.push(Point { x, y });
            }
        });
    }

    return pixels.iter().map(|vec| vec.iter().map(|(color, _)| *color).collect()).collect();
}
