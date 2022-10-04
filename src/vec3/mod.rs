pub mod vec3 {
   #[derive(Debug,Copy,Clone)] // Only three numbers so cheap to copy around 
   pub struct Vec3d<T> {
      x: T,
      y: T,
      z: T,
 }

 impl<T> Vec3d<T> where T: std::ops::Mul<T,Output = T> + std::ops::Add<T,Output = T> + Copy{
   pub fn new(x_in: T,y_in: T,z_in: T)->Vec3d<T>{
      Vec3d{
         x: x_in,
         y: y_in,
         z: z_in
      }
   }

   pub fn mag_squared(&self) -> T{
      return self.x*self.x + self.y*self.y + self.z*self.z;
   }

   pub fn getx(&self){
      self.x;
   }

   pub fn gety(&self){
      self.y;
   }
   pub fn getz(&self){
      self.z;
   }
   
 }





impl std::ops::Add<&Vec3d<f64>> for &Vec3d<f64> {
    type Output = Vec3d<f64>;
    fn add(self, rhs: &Vec3d<f64>)->Vec3d<f64>{
        Vec3d::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
 }



impl std::ops::AddAssign<&Vec3d<f64>> for &mut Vec3d<f64> {
   fn add_assign(&mut self, rhs: &Vec3d<f64>){
      self.x += rhs.x;
      self.y += rhs.y;
      self.z += rhs.z;
   }
}


impl std::ops::Mul<f64> for &Vec3d<f64> {
   type Output = Vec3d<f64>;
   fn mul(self, rhs:f64)->Vec3d<f64>{
      Vec3d::new(
         self.x * rhs,
         self.y * rhs,
         self.z * rhs
      )  
   }
}

impl std::ops::Mul<&Vec3d<f64>> for f64 {
   type Output = Vec3d<f64>;
   fn mul(self, rhs: &Vec3d<f64>)->Vec3d<f64>{
      Vec3d::new(
         self * rhs.x, 
         self * rhs.y, 
         self * rhs.z
      )
   }
}
}


pub trait IsVec3d<T> {
   type VariableType;
   fn getx(&self) -> Self::VariableType;
   fn gety(&self) -> Self::VariableType;
   fn getz(&self) -> Self::VariableType;

}