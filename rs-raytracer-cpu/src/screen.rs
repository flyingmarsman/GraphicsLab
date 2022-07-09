pub struct ScreenPos {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl ScreenPos {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x,
            y,
            w,
            h,
        }
    }

    pub fn x_pos(&self) -> f32 {
        self.x as f32 / self.w as f32 * 2.0 - 1.0
    }

    pub fn y_pos(&self) -> f32 {
        - self.y as f32 / self.h as f32 * 2.0 + 1.0
    }
}