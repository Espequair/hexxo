mod hex;

#[derive(PartialEq, Debug, Copy, Clone)]
struct Orientation {
    f0: f32,
    f1: f32,
    f2: f32,
    f3: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    b3: f32,
    start_angle: f32,
}

const ORIENTATION_POINTY: Orientation = Orientation{
    f0: f32::sqrt(3.0),
    f1: f32::sqrt(3.0) / 2.0,
    f2: 0.0,
    f3: 1.5,
    b0: f32::sqrt(3.0) / 3.0,
    b1: - 1.0 / 3.0,
    b2: 0.0,
    b3: 2.0 / 3.0,
    start_angle: 0.5
};

const ORIENTATION_FLAT: Orientation = Orientation{
    f0: 1.5,
    f1: 0.0,
    f2: f32::sqrt(3.0) / 2.0,
    f3: 2.0 / 3.0,
    b0: 0.0,
    b1: - 1.0 / 3.0,
    b2: f32::sqrt(3.0),
    b3: f32::sqrt(3.0) / 3.0,
    start_angle: 0.5
};

#[derive(PartialEq, Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Layout {
    orientation: Orientation,
    origin: Point,
    size: Point,
}

impl Layout{

    //hex_to_pixel
    pub fn hex_center(self, hex: Hex){
        let M = self
    }
}
