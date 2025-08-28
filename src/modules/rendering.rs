use crate::modules::math::Vec2;
use crate::modules::math::Vec2f;
pub struct Renderer<'a> {
    pub frame: &'a mut [u8],
    pub width: usize,
    pub height: usize,
}


pub struct Camera {
    pub position: Vec2f,
    pub fov: f32,
}

impl Camera {
    pub fn world_to_screen(&self, world: Vec2f, screen: Vec2f) -> Vec2f {
        Vec2f::new(
            (world.x - self.position.x) * self.fov + screen.x / 2.0,
            (world.y - self.position.y) * self.fov + screen.y / 2.0,
        )
    }
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

    fn edge(&self, a: &Vec2<i32>, b: &Vec2<i32>, c: &Vec2<i32>) -> i32 {
        (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
    }

    fn edge_float(&self, a: &Vec2<i32>, b: &Vec2<i32>, cx: f32, cy: f32) -> f32 {
        (cx - a.x as f32) * (b.y as f32 - a.y as f32)
            - (cy - a.y as f32) * (b.x as f32 - a.x as f32)
    }

    pub fn draw_triangle(
        &mut self,
        pos1: Vec2<i32>,
        pos2: Vec2<i32>,
        pos3: Vec2<i32>,
        color: [u8; 4],
        thickness: f32,
        filled: bool,
    ) {
        if !filled {
            self.draw_line(pos1.clone(), pos2.clone(), color, thickness);
            self.draw_line(pos2.clone(), pos3.clone(), color, thickness);
            self.draw_line(pos3.clone(), pos1.clone(), color, thickness);
        } else {
            let v1 = pos1;
            let v2 = pos2;
            let v3 = pos3;

            // Bounding box with some padding for antialiasing
            let min_x = (v1.x.min(v2.x).min(v3.x) as f32 - 1.0) as i32;
            let max_x = (v1.x.max(v2.x).max(v3.x) as f32 + 1.0) as i32;
            let min_y = (v1.y.min(v2.y).min(v3.y) as f32 - 1.0) as i32;
            let max_y = (v1.y.max(v2.y).max(v3.y) as f32 + 1.0) as i32;

            let area = self.edge(&v1, &v2, &v3) as f32;
            if area.abs() < 1e-6 {
                return; // degenerate triangle
            }

            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    // Sample multiple points within the pixel for antialiasing
                    let mut coverage = 0.0;
                    let samples = 4; // 4x4 supersampling
                    let step = 1.0 / samples as f32;

                    for sy in 0..samples {
                        for sx in 0..samples {
                            let px = x as f32 + (sx as f32 + 0.5) * step;
                            let py = y as f32 + (sy as f32 + 0.5) * step;

                            let w0 = self.edge_float(&v2, &v3, px, py);
                            let w1 = self.edge_float(&v3, &v1, px, py);
                            let w2 = self.edge_float(&v1, &v2, px, py);

                            // Check if sample point is inside triangle
                            let inside = if area > 0.0 {
                                w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
                            } else {
                                w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
                            };

                            if inside {
                                coverage += 1.0;
                            }
                        }
                    }

                    coverage /= (samples * samples) as f32;

                    if coverage > 0.0 {
                        Self::blend_pixel_direct(self.frame, self.width, x, y, color, coverage);
                    }
                }
            }
        }
    }

    pub fn draw_line(&mut self, start: Vec2<i32>, end: Vec2<i32>, color: [u8; 4], thickness: f32) {
        let dx = end.x as f32 - start.x as f32;
        let dy = end.y as f32 - start.y as f32;
        let length = (dx * dx + dy * dy).sqrt();

        if length < 1e-6 {
            return;
        }

        let half_thickness = thickness / 2.0;

        // For each pixel in the bounding rectangle, check if it's within the thick line
        let min_x = ((start.x.min(end.x)) as f32 - half_thickness - 1.0) as i32;
        let max_x = ((start.x.max(end.x)) as f32 + half_thickness + 1.0) as i32;
        let min_y = ((start.y.min(end.y)) as f32 - half_thickness - 1.0) as i32;
        let max_y = ((start.y.max(end.y)) as f32 + half_thickness + 1.0) as i32;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let px = x as f32 + 0.5; // Pixel center
                let py = y as f32 + 0.5;

                // Calculate distance from pixel to line
                let distance = self.point_to_line_distance(px, py, &start, &end);

                if distance <= half_thickness + 0.5 {
                    // Calculate antialiasing alpha based on distance
                    let alpha = if distance < half_thickness - 0.5 {
                        1.0
                    } else {
                        // Smooth falloff at edges
                        (half_thickness + 0.5 - distance).max(0.0)
                    };

                    if alpha > 0.0 {
                        Self::blend_pixel_direct(self.frame, self.width, x, y, color, alpha);
                    }
                }
            }
        }
    }

    fn point_to_line_distance(&self, px: f32, py: f32, start: &Vec2<i32>, end: &Vec2<i32>) -> f32 {
        let dx = end.x as f32 - start.x as f32;
        let dy = end.y as f32 - start.y as f32;
        let length_sq = dx * dx + dy * dy;

        if length_sq < 1e-12 {
            // Line is a point
            return ((px - start.x as f32) * (px - start.x as f32)
                + (py - start.y as f32) * (py - start.y as f32))
                .sqrt();
        }

        // Project point onto line
        let t = ((px - start.x as f32) * dx + (py - start.y as f32) * dy) / length_sq;
        let t_clamped = t.clamp(0.0, 1.0);

        let closest_x = start.x as f32 + t_clamped * dx;
        let closest_y = start.y as f32 + t_clamped * dy;

        ((px - closest_x) * (px - closest_x) + (py - closest_y) * (py - closest_y)).sqrt()
    }

    // Helper function for pixel blending
    fn blend_pixel_direct(
        frame: &mut [u8],
        width: usize,
        x: i32,
        y: i32,
        color: [u8; 4],
        alpha: f32,
    ) {
        if x < 0 || y < 0 || x >= width as i32 {
            return;
        }

        let height = frame.len() / 4 / width;
        if y >= height as i32 {
            return;
        }

        let idx = ((y as usize * width) + x as usize) * 4;

        // Alpha blending with proper bounds checking
        if idx + 3 < frame.len() {
            for i in 0..4 {
                let bg = frame[idx + i] as f32 / 255.0;
                let fg = color[i] as f32 / 255.0;
                let blended = bg * (1.0 - alpha) + fg * alpha;
                frame[idx + i] = (blended * 255.0).clamp(0.0, 255.0) as u8;
            }
        }
    }

    pub fn clear(&mut self, color: [u8; 4]) {
        for pixel in self.frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
        }
    }
}
