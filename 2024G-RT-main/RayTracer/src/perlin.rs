use crate::vec3::Vec3;

pub struct Perlin{
    pub rand_float:Vec<f64>,
    pub perm_x:Vec<i32>,
    pub perm_y:Vec<i32>,
    pub perm_z:Vec<i32>,
}
impl Perlin {
    fn permute(p: &mut Vec<i32>, n: i32) {
        for i in (0..n).rev() {
            let target = (crate::camera::random()*(i as f64+1.0)) as i32;
            p.swap(i as usize, target as usize);
        }
    }
    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::new();
        for i in 0..256 {
            p.push(i);
        }
        Perlin::permute(&mut p, 256);
        p
    }
    pub fn new() -> Self {
        let mut rand_float = Vec::new();
        for _ in 0..256 {
            rand_float.push(crate::camera::random());
        }
        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();
        Perlin {
            rand_float,
            perm_x,
            perm_y,
            perm_z,
        }

    }
    fn trilinear_interp(c: Vec<Vec<Vec<f64>>>,v:f64,u:f64,w:f64)->f64{
        let mut accum=0.0;
        for i in 0..2{
            for j in 0..2{
                for k in 0..2{
                    accum+=(i as f64*v+(1.0-i as f64)*(1.0-v))*
                        (j as f64*u+(1.0-j as f64)*(1.0-u))*
                        (k as f64*w+(1.0-k as f64)*(1.0-w))*c[i][j][k];
                }
            }
        }
        accum
    }
    pub fn noise(&self,p:Vec3)->f64{
     let mut u=p.x-p.x.floor();
        let mut v=p.y-p.y.floor();
        let mut w=p.z-p.z.floor();
        u=u*u*(3.0-2.0*u);
        v=v*v*(3.0-2.0*v);
        w=w*w*(3.0-2.0*w);
        let i=  p.x.floor() as i32;
        let j=p.y.floor() as i32;
        let k=p.z.floor() as i32;
            let mut c=Vec::<Vec<Vec<f64>>>::new();
            for di in 0..2{
                let mut temp=Vec::<Vec<f64>>::new();
                for dj in 0..2{
                    let mut temp2=Vec::<f64>::new();
                    for dk in 0..2 {
                        temp2.push(self.rand_float[(self.perm_x[((i + di) & 255) as usize] ^
                            self.perm_y[((j + dj) & 255) as usize] ^
                            self.perm_z[((k + dk) & 255) as usize]) as usize]);
                    }
                temp.push(temp2);
                }
                c.push(temp);}
        Self::trilinear_interp(c,u,v,w)
    }


}