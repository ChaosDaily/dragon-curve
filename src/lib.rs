mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// State of chaos
#[wasm_bindgen]
struct Universe {
    // Flat array of x, y
    points: [f64; 2],
    // Length of the side of the square
    len: f64,
    // Last x, y and angle
    last_x: f64,
    last_y: f64,
    last_angle: i32,
    last_el: i64,
}

#[wasm_bindgen]
impl Universe {
    /// Create a new universe
    /// with start point x, y
    /// each side of the square is len
    pub fn new(len: f64, x: f64, y: f64) -> Universe {
        Universe {
            points: [x, y],
            len,
            last_x: x,
            last_y: y,
            last_angle: 0,
            last_el: 0,
        }
    }

    /// Generate next point of the dragon curve.
    pub fn tick(&mut self) {
        let (x, y) = self.dragon_curve();
        self.points[0] = x;
        self.points[1] = y;
    }

    /// Return raw data to wasm.
    pub fn points(&self) -> *const f64 {
        self.points.as_ptr()
    }

    /// Return length of the points array.
    pub fn points_len(&self) -> usize {
        self.points.len()
    }
}

impl Universe {
    /// -1 for left and 1 for right
    /// see https://en.wikipedia.org/wiki/Dragon_curve
    fn get_turn(&self, n: i32) -> i32 {
        let turn_flag = (((n + 1) & -(n + 1)) << 1) & (n + 1);
        if turn_flag == 0 {
            1
        } else {
            -1
        }
    }

    /// helper for transforming turns into coordinates
    /// 0 1 0 -1....
    fn sign(&self, x: i32) -> i32 {
        (x % 2) * (2 - (x % 4))
    }

    /// Returns next point of the dragon curve
    pub fn dragon_curve(&mut self) -> (f64, f64) {
        let turn = self.get_turn(self.last_el as i32);
        let angle = self.last_angle + turn;
        let x = self.last_x - self.len * self.sign(angle) as f64;
        let y = self.last_y - self.len * self.sign(angle + 1) as f64;

        self.last_x = x;
        self.last_y = y;
        self.last_angle = angle;
        self.last_el += 1;
        (x, y)
    }
}
