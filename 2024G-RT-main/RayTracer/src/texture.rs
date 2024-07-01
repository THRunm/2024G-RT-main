use std::iter::Scan;
use crate::vec3::Vec3;


pub enum Texture{
    SolidColor(Vec3),
    Checker(CheckerTexture),
}


impl Texture{
    pub fn value(&self,u:f64,v:f64,p:&Vec3)->Vec3{
        match self{
            Texture::Checker(tex)=>tex.value(u,v,p),
            Texture::SolidColor(color)=>*color,
        }
    }
    pub fn color(scale:f64,c1:Vec3,c2:Vec3)->Self{
        Texture::Checker(CheckerTexture::color(scale,c1,c2))
    }
    pub fn new(odd:Texture,even:Texture,scale:f64)->Self{
        Texture::Checker(CheckerTexture::new(odd,even,scale))
    }
    pub fn solid_color(color:Vec3)->Self{
        Texture::SolidColor(color)}

}
pub struct SolidColor{
    color:Vec3,
}
pub struct CheckerTexture{
    inv_scale: f64,
    pub(crate) odd:Texture,
    pub(crate) even:Texture,
}
impl CheckerTexture{
    pub fn new(odd:Texture,even:Texture,scale:f64)->Self{
        Self{
            inv_scale:1.0/scale,
            odd,
            even,
        }
    }
    pub fn color(scale:f64,c1:Vec3,c2:Vec3)->Self{
        Self{
            inv_scale:1.0/scale,
            even:Texture::new(c1),
            odd:Texture::new(c2),
        }
    }
    pub fn value(&self,u:f64,v:f64,p:&Vec3)->Vec3{
        let xInterger=(self.inv_scale*p.x).floor() as i32;
        let yInterger=(self.inv_scale*p.y).floor() as i32;
        let zInterger=(self.inv_scale*p.z).floor() as i32;
        let isEven=(xInterger+yInterger+zInterger)%2==0;
        if isEven{
            self.even.value(u,v,p)
        }else{
            self.odd.value(u,v,p)
        }
    }
}