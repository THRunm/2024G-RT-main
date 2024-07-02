use crate::vec3::Vec3;
use crate::image_::RtwImage;
use crate::interval::Interval;

pub enum Texture{
    SolidColor(Vec3),
    Checker(Box<CheckerTexture>),
    ImageTex(ImageTexture),
}


impl Texture{
    pub fn value(&self,u:f64,v:f64,p:&Vec3)->Vec3{
        match self{
            Texture::Checker(tex)=>tex.value(u,v,p),
            Texture::SolidColor(color)=>*color,
            Texture::ImageTex(tex)=>tex.value(u,v,p),
        }
    }
    pub fn color(scale:f64,c1:Vec3,c2:Vec3)->Self{
        Texture::Checker(Box::new(CheckerTexture::color(scale,c1,c2)) )
    }
    pub fn new(odd:Texture,even:Texture,scale:f64)->Self{
        Texture::Checker(Box::new(CheckerTexture::new(Box::new(odd),Box::new(even),scale)))
    }
    pub fn solid_color(color:Vec3)->Self{
        Texture::SolidColor(color)}

}
pub struct SolidColor{
    color:Vec3,
}
pub struct CheckerTexture{
    inv_scale: f64,
    pub(crate) odd:Box<Texture>,
    pub(crate) even:Box<Texture>,
}
impl CheckerTexture{
    pub fn new(odd:Box<Texture>,even:Box<Texture>,scale:f64)->Self{
        Self{
            inv_scale:1.0/scale,
            odd,
            even,
        }
    }
    pub fn color(scale:f64,c1:Vec3,c2:Vec3)->Self{
        Self{
            inv_scale:1.0/scale,
            even:Box::new(Texture::SolidColor(c1)),
            odd:Box::new(Texture::SolidColor(c2)),
        }
    }
    pub fn value(&self,u:f64,v:f64,p:&Vec3)->Vec3{
        let x_interger =(self.inv_scale*p.x).floor() as i32;
        let y_interger =(self.inv_scale*p.y).floor() as i32;
        let z_interger=(self.inv_scale*p.z).floor() as i32;
        let is_even =(x_interger + y_interger +z_interger)%2==0;
        if is_even {
            self.even.value(u,v,p)
        }else{
            self.odd.value(u,v,p)
        }
    }
}
pub struct ImageTexture{
 image_texture: RtwImage,
}
impl ImageTexture{
    pub fn new(image_filename:&str)->Self{
        Self{
            image_texture:RtwImage::new(image_filename),
        }
    }
    pub fn value(&self,u:f64,v:f64,p:&Vec3)->Vec3{
        let i=self.image_texture.width() as f64;
        let j=self.image_texture.height() as f64;
        if j<=0.0{
            return Vec3::new(0.0,1.0,1.0);
        }
        let u=Interval::set(0.0,1.0).clamp(u);
        let v=1.0-Interval::set(0.0,1.0).clamp(v);
        let i=(i*u) as u32;
        let j=(j*v) as u32;
        let pixel=self.image_texture.pixel_data(i,j);
        Vec3::new(pixel[0] as f64,pixel[1] as f64,pixel[2] as f64)/255.0
    }
}