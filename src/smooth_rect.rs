use std::time::Duration;

use nenobi::array::TimeBaseEasingValueN;
use windows::Win32::Foundation::RECT;

pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn to_f32(&self) -> [f32; 4] {
        [
            self.left as f32,
            self.top as f32,
            self.right as f32,
            self.bottom as f32,
        ]
    }

    pub fn from_f32(values: [f32; 4]) -> Self {
        Self {
            left: values[0] as i32,
            top: values[1] as i32,
            right: values[2] as i32,
            bottom: values[3] as i32,
        }
    }
}

impl From<RECT> for Rect {
    fn from(rect: RECT) -> Self {
        Self {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

pub struct SmoothRect {
    // The easing values for the left, top, right, and bottom sides of the rectangle.
    values: TimeBaseEasingValueN<f32, 4>,
}

impl SmoothRect {
    pub fn new(rect: Rect) -> Self {
        Self {
            values: TimeBaseEasingValueN::new(rect.to_f32()),
        }
    }

    pub fn add(&mut self, rect: Rect) {
        self.values.add(
            rect.to_f32(),
            Duration::from_millis(500),
            nenobi::functions::quad_in_out,
        );
    }

    pub fn current_rect(&self) -> Rect {
        Rect::from_f32(self.values.current_value())
    }

    pub fn last_rect(&self) -> Rect {
        Rect::from_f32(self.values.last_value())
    }
}
