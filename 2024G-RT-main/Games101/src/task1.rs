#![allow(warnings)]

pub use std::env;
use std::io;
pub use nalgebra::Vector3;
pub use opencv::{
    Result,
};
pub use opencv::core::Vector;
pub use crate::rasterizer1::{Buffer, Rasterizer,Primitive};
pub use crate::utils::*;
pub use crate::shader::FragmentShaderPayload;
pub use crate::texture::Texture;
use opencv::imgcodecs::imwrite;
use opencv::highgui::{imshow, wait_key};
use nalgebra::Matrix4;

pub fn t1()-> Result<()>{
    println!("选择任务1");
    let mut angle = 0.0;
    let mut r = Rasterizer::new(700, 700);
    let eye_pos = Vector3::new(0.0, 0.0, 5.0);
    let pos = vec![Vector3::new(2.0, 0.0, -2.0),
                   Vector3::new(0.0, 2.0, -2.0),
                   Vector3::new(-2.0, 0.0, -2.0)];
    let ind = vec![Vector3::new(0, 1, 2)];

    let pos_id = r.load_position(&pos);
    let ind_id = r.load_indices(&ind);

    let mut k = 0;
    let mut frame_count = 0;
    let mut flag=false;
    let mut arbitrary_rotation:Matrix4<f64>=Matrix4::identity();

    while k != 27 {
        r.clear(Buffer::Both);
        if flag{
            r.set_model(arbitrary_rotation);
        }
        else{
            r.set_model(get_model_matrix(angle,1.0));
        }
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw_triangle(pos_id, ind_id, Primitive::Triangle);

        let frame_buffer = r.frame_buffer();
        let image = frame_buffer2cv_mat(frame_buffer);
        imshow("image", &image).unwrap();

        k = wait_key(80).unwrap();
        println!("frame count: {}", frame_count);
        if k == 'a' as i32 {
            angle += 10.0;
        } else if k == 'd' as i32 {
            angle -= 10.0;
        }
        else if k =='r' as i32{
            let mut axis1 = String::new();
            let mut axis2 = String::new();
            let mut axis3 = String::new();
            let mut angle1 = String::new();
            println!("Please input the axis and angle of rotation or r to exit this mode:");
            println!("axis1:");
            io::stdin().read_line(&mut axis1).expect("Failed to read line");
            if axis1.trim() == "r"{
                flag=false;
                continue;
            }
            println!("axis2:");
            io::stdin().read_line(&mut axis2).expect("Failed to read line");
            println!("axis3:");
            io::stdin().read_line(&mut axis3).expect("Failed to read line");
            println!("angle:");
            io::stdin().read_line(&mut angle1).expect("Failed to read line");
            let mut axis: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
            axis[0] = axis1.trim().parse().expect("Please type a number!");
            axis[1] = axis2.trim().parse().expect("Please type a number!");
            axis[2] = axis3.trim().parse().expect("Please type a number!");
            let mut angle: f64 = angle1.trim().parse().expect("Please type a number!");
            arbitrary_rotation=get_rotation(axis,angle);
            flag=true;
        }
        frame_count += 1;
    }
    Ok(())
}