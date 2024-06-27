use std::collections::HashMap;

use nalgebra::{Matrix4, Vector3, Vector4};
use crate::triangle::Triangle;

#[allow(dead_code)]
pub enum Buffer {
    Color,
    Depth,
    Both,
}

#[allow(dead_code)]
pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Default, Clone)]
pub struct Rasterizer {
    model: Matrix4<f64>,
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    pos_buf: HashMap<usize, Vec<Vector3<f64>>>,
    ind_buf: HashMap<usize, Vec<Vector3<usize>>>,
    col_buf: HashMap<usize, Vec<Vector3<f64>>>,

    frame_buf: Vec<Vector3<f64>>,
    depth_buf: Vec<f64>,
    /*  You may need to uncomment here to implement the MSAA method  */
    // frame_sample: Vec<Vector3<f64>>,
    // depth_sample: Vec<f64>,
    width: u64,
    height: u64,
    next_id: usize,
}

#[derive(Clone, Copy)]
pub struct PosBufId(usize);

#[derive(Clone, Copy)]
pub struct IndBufId(usize);

#[derive(Clone, Copy)]
pub struct ColBufId(usize);

impl Rasterizer {
    pub fn new(w: u64, h: u64) -> Self {
        let mut r = Rasterizer::default();
        r.width = w;
        r.height = h;
        r.frame_buf.resize((4 * w * h) as usize, Vector3::zeros());
        r.depth_buf.resize((16 * w * h) as usize, 0.0);
        r
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y as u64) * self.width + x as u64) as usize
    }

    fn set_pixel(&mut self, point: &Vector3<f64>, color: &Vector3<f64>) {
        let ind = (self.height as f64 - 1.0 - point.y) * self.width as f64 + point.x;
        self.frame_buf[ind as usize] = *color;
    }

    pub fn clear(&mut self, buff: Buffer) {
        match buff {
            Buffer::Color => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
            }
            Buffer::Depth => {
                self.depth_buf.fill(f64::MAX);
            }
            Buffer::Both => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                self.depth_buf.fill(f64::MAX);
            }
        }
    }

    pub fn set_model(&mut self, model: Matrix4<f64>) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f64>) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Matrix4<f64>) {
        self.projection = projection;
    }

    fn get_next_id(&mut self) -> usize {
        let res = self.next_id;
        self.next_id += 1;
        res
    }

    pub fn load_position(&mut self, positions: &Vec<Vector3<f64>>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.clone());
        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: &Vec<Vector3<usize>>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices.clone());
        IndBufId(id)
    }

    pub fn load_colors(&mut self, colors: &Vec<Vector3<f64>>) -> ColBufId {
        let id = self.get_next_id();
        self.col_buf.insert(id, colors.clone());
        ColBufId(id)
    }

    pub fn draw(&mut self, pos_buffer: PosBufId, ind_buffer: IndBufId, col_buffer: ColBufId, _typ: Primitive) {
        let buf = &self.clone().pos_buf[&pos_buffer.0];
        let ind: &Vec<Vector3<usize>> = &self.clone().ind_buf[&ind_buffer.0];
        let col = &self.clone().col_buf[&col_buffer.0];

        let f1 = (50.0 - 0.1) / 2.0;
        let f2 = (50.0 + 0.1) / 2.0;

        let mvp = self.projection * self.view * self.model;

        for i in ind {
            let mut t = Triangle::new();
            let mut v =
                vec![mvp * to_vec4(buf[i[0]], Some(1.0)), // homogeneous coordinates
                     mvp * to_vec4(buf[i[1]], Some(1.0)), 
                     mvp * to_vec4(buf[i[2]], Some(1.0))];
    
            for vec in v.iter_mut() {
                *vec = *vec / vec.w;
            }
            for vert in v.iter_mut() {
                vert.x = 0.5 * self.width as f64 * (vert.x + 1.0);
                vert.y = 0.5 * self.height as f64 * (vert.y + 1.0);
                vert.z = vert.z * f1 + f2;
            }
            for j in 0..3 {
                // t.set_vertex(j, Vector3::new(v[j].x, v[j].y, v[j].z));
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
            }
            let col_x = col[i[0]];
            let col_y = col[i[1]];
            let col_z = col[i[2]];
            t.set_color(0, col_x[0], col_x[1], col_x[2]);
            t.set_color(1, col_y[0], col_y[1], col_y[2]);
            t.set_color(2, col_z[0], col_z[1], col_z[2]);

            self.rasterize_triangle(&t);
        }
    }

    pub fn rasterize_triangle(&mut self, t: &Triangle) {
        /*  implement your code here  */
        let v: [Vector4<f64>; 3] = t.to_vector4();
        let min_x = v.iter().map(|x| x.x).fold(f64::INFINITY, f64::min).max(0.0);
        let max_x = v.iter().map(|x| x.x).fold(f64::NEG_INFINITY, f64::max).min(self.width as f64 - 1.0);
        let min_y = v.iter().map(|x| x.y).fold(f64::INFINITY, f64::min).max(0.0);
        let max_y = v.iter().map(|x| x.y).fold(f64::NEG_INFINITY, f64::max).min(self.height as f64 - 1.0);

        for x in min_x as usize..=max_x as usize {
            for y in min_y as usize..=max_y as usize {
                // let mut cnt = 0;
                // for t in 0..16 {
                //     let x1 = x as f64 + 0.125+0.25*(t/4)as f64;
                //     let y1 = y as f64 + 0.125+0.25*(t%4)as f64;
                //     if inside_triangle(x1, y1, &[v[0].xyz(), v[1].xyz(), v[2].xyz()]) {
                //         let z = compute_barycentric2d(x1, y1, &[v[0].xyz(), v[1].xyz(), v[2].xyz()]);
                //         let z = z.0 * v[0].z + z.1 * v[1].z + z.2 * v[2].z;
                //         let ind = self.get_index(x, y) * 16 + t;
                //         if z < self.depth_buf[ind] {
                //             self.depth_buf[ind] = z;
                //             cnt += 1;
                //         }
                //     }
                // }
                // if (cnt > 0) {
                //     let ind = self.get_index(x, y);
                //     let mut color = t.get_color();
                //     color.x = color.x / 16.0 * cnt as f64 + self.frame_buf[ind].x * (1.0 - cnt as f64 / 16.0);
                //     color.y = color.y / 16.0 * cnt as f64 + self.frame_buf[ind].y * (1.0 - cnt as f64 / 16.0);
                //     color.z = color.z / 16.0 * cnt as f64 + self.frame_buf[ind].z * (1.0 - cnt as f64 / 16.0);
                //     self.set_pixel(&Vector3::new(x as f64, y as f64, 0.0), &color);
                // }
                if inside_triangle(x as f64, y as f64, &[v[0].xyz(), v[1].xyz(), v[2].xyz()]) {
                    let z = compute_barycentric2d(x as f64, y as f64, &[v[0].xyz(), v[1].xyz(), v[2].xyz()]);
                    let z = z.0 * v[0].z + z.1 * v[1].z + z.2 * v[2].z;
                    let ind = self.get_index(x, y);
                    if z < self.depth_buf[ind] {
                        self.depth_buf[ind] = z;
                        self.set_pixel(&Vector3::new(x as f64, y as f64, 0.0), &t.get_color());
                    }
                }
            }
        }
        self.apply_fxaa();
    }

    pub fn frame_buffer(&self) -> &Vec<Vector3<f64>> {
        &self.frame_buf
    }
    pub fn apply_fxaa(&mut self) {
        let mut new_frame = self.frame_buf.clone();
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let idx = self.get_index(x, y);
                let color = self.luma_edge_detection(x, y);
                new_frame[idx] = color;
            }
        }
        self.frame_buf = new_frame;
    }

    fn luma_edge_detection(&self, x: usize, y: usize) -> Vector3<f64> {
        let luma = |x: usize, y: usize| {
            let idx = self.get_index(x, y);
            let pixel = self.frame_buf[idx];
            0.299 * pixel.x + 0.587 * pixel.y + 0.114 * pixel.z
        };

        let current = luma(x, y);
        let left = if x > 0 { luma(x - 1, y) } else { current };
        let right = if x < self.width as usize - 1 { luma(x + 1, y) } else { current };
        let up = if y > 0 { luma(x, y - 1) } else { current };
        let down = if y < self.height as usize - 1 { luma(x, y + 1) } else { current };

        let min_luma = left.min(right).min(up).min(down).min(current);
        let max_luma = left.max(right).max(up).max(down).max(current);

        if max_luma - min_luma > 0.1 {
            let blend = |x: usize, y: usize, factor: f64| {
                let idx = self.get_index(x, y);
                self.frame_buf[idx] * factor
            };

            let count = [x > 0, x < self.width as usize - 1, y > 0, y < self.height as usize - 1];
            let mut total = 0.0;
            let mut color = Vector3::new(0.0, 0.0, 0.0);

            if count[0] { color += blend(x - 1, y, 0.25); total += 0.25; }
            if count[1] { color += blend(x + 1, y, 0.25); total += 0.25; }
            if count[2] { color += blend(x, y - 1, 0.25); total += 0.25; }
            if count[3] { color += blend(x, y + 1, 0.25); total += 0.25; }

            color / total
        } else {
            let idx = self.get_index(x, y);
            self.frame_buf[idx]
        }
    }

}

fn to_vec4(v3: Vector3<f64>, w: Option<f64>) -> Vector4<f64> {
    Vector4::new(v3.x, v3.y, v3.z, w.unwrap_or(1.0))
}

fn inside_triangle(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> bool {
    /*  implement your code here  */
    let mut c = compute_barycentric2d(x, y, v);
    if c.0 >= 0.0 && c.1 >= 0.0 && c.2 >= 0.0 {
        return true;
    }
    false
}

fn compute_barycentric2d(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> (f64, f64, f64) {
    let c1 = (x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * y + v[1].x * v[2].y - v[2].x * v[1].y)
        / (v[0].x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * v[0].y + v[1].x * v[2].y - v[2].x * v[1].y);
    let c2 = (x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * y + v[2].x * v[0].y - v[0].x * v[2].y)
        / (v[1].x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * v[1].y + v[2].x * v[0].y - v[0].x * v[2].y);
    let c3 = (x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * y + v[0].x * v[1].y - v[1].x * v[0].y)
        / (v[2].x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * v[2].y + v[0].x * v[1].y - v[1].x * v[0].y);
    (c1, c2, c3)
}