pub struct Renderer<'a> {
    pub frame: &'a mut [u8],
    pub width: usize,
    pub height: usize,
}

impl<'a> Renderer<'a> {
    pub fn new(frame: &'a mut [u8], width: usize, height: usize) -> Self {
        Self {
            frame,
            width,
            height,
        }
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, rgba: [u8; 4]) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        let idx = ((y as usize * self.width) + x as usize) * 4;
        self.frame[idx..idx + 4].copy_from_slice(&rgba);
    }
    pub fn clear(&mut self, color: [u8; 4]) {
        for pixel in self.frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
        }
    }
}
