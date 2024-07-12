
use std::sync::Arc;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::vec3::Vec3;
use crate::material::{Lambertian, Material};
use crate::hittable::{Hittable};
use crate::bvh::BvhNode;
use crate::hittable_list::HittableList;
use crate::obj::Obj;
use crate::quad::quad;
use crate::texture::Texture;
use crate::texture::Texture::SolidColor;
use crate::triangle::Triangle;

/// 从OBJ文件加载模型，创建三角形列表，并生成BVH优化的hittable_list
pub(crate) fn load_obj_to_hittable_list<P: AsRef<Path>, Mat: Material + Sync + Send + Clone + 'static>(
    path: P,
    material: Mat,
) -> HittableList {
    let obj: Obj = Obj::load(path).expect("Failed to load OBJ file");
    let mut hittable_list = HittableList::new();
    // 遍历每个网格中的面
    for object in &obj.data.objects{
        for group in &object.groups {
            for poly in &group.polys {
                let mut vertices = Vec::new();

                // 获取顶点位置
                for &index in &poly.0 {
                    let vertex = obj.data.position[index.0];
                    vertices.push(Vec3::new(vertex[0] as f64, vertex[1] as f64, vertex[2]as f64));
                }

                if vertices.len() == 3 { // 确保是三角形
                    hittable_list.add(Arc::new(Triangle::new(
                        vertices[0],
                        vertices[1],
                        vertices[2],
                        material.clone(),
                    )));
                }
            }
        }
    }

    let bvh = BvhNode::set(hittable_list.clone());
    let  mut hittable_list = HittableList::new();
    hittable_list.add(Arc::new(bvh));
    hittable_list
}
pub(crate) fn load_obj<P: AsRef<Path>>(
    path: P,
    mtl_path: P,
) -> HittableList {
    let mut obj: Obj = Obj::load(path).expect("Failed to load OBJ file");
    let mtl_file = File::open(mtl_path).expect("Failed to open MTL file");
    obj.data.material_libs[0].reload(mtl_file).expect("Failed to load MTL file");
    let mut triangles:Vec<Arc<dyn Hittable + Sync + Send>> = Vec::new();
    for object in &obj.data.objects{
        for group in &object.groups {
            let mat_name=group.material.as_ref().unwrap().name();
            let color_f32=obj.data.material_libs[0].materials.iter().find(|&x| x.name==mat_name).unwrap().kd;
            let color=match color_f32{
                Some(color_f32)=>Vec3::new(color_f32[0] as f64,color_f32[1] as f64,color_f32[2] as f64),
                None=>Vec3::new(0.0,0.0,0.0),
            };
            for poly in &group.polys {
                let mut vertices = Vec::new();

                // 获取顶点位置
                for &index in &poly.0 {
                    let vertex = obj.data.position[index.0];
                    vertices.push(Vec3::new(vertex[0] as f64, vertex[1] as f64, vertex[2]as f64));
                }

                if vertices.len() == 3 { // 确保是三角形
                    triangles.push(Arc::new(Triangle::new(
                        vertices[0],
                        vertices[1],
                        vertices[2],
                        Lambertian::new(color),
                    )));
                }
                else if vertices.len() == 4{
                    triangles.push(Arc::new(Triangle::new(
                        vertices[0],
                        vertices[1]-vertices[0],
                        vertices[3]-vertices[0],
                        Lambertian::new(color),
                    )));

                }

            }
        }
    }

    let hittables: Vec<Arc<dyn Hittable + Sync + Send>> = triangles.iter().map(|t| t.clone() as Arc<dyn Hittable + Sync + Send>).collect();
    let bvh = BvhNode::new(&hittables, 0, hittables.len());

    let mut hittable_list = HittableList::new();
    hittable_list.add(Arc::new(bvh));

    hittable_list
}
