

pub mod vec3d {
    use num_traits::real::Real;
    pub struct Vec3d<T: Real>{
        pub x: T,
        pub y: T,
        pub z: T
    } 

    pub trait IsVec3d{
        type Component;
        fn add(&self, rhs: &Self) -> Self;
        fn sub(&self, rhs: &Self) -> Self;
        fn scalarmul(&self, rhs: Self::Component) -> Self;
        fn mag_squared(&self) -> Self::Component;
        fn clone(&self) -> Self;
        


    } 

    impl<T> IsVec3d for Vec3d<T> 
    where 
        T: Real
    {
        type Component = T;
        fn add(&self,rhs: &Self) -> Self {
            return Vec3d {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z
            }
        }
        fn sub(&self,rhs: &Self) -> Self {
            return Vec3d {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z
            }
        }

        fn scalarmul(&self, rhs: Self::Component) -> Self{
         return Vec3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
         }
        }

        fn mag_squared(&self) -> T {
            return self.x*self.x + self.y*self.y + self.z*self.z;
        }

        fn clone(&self) -> Self {
            Vec3d {
                x: self.x,
                y: self.y,
                z: self.z
            }
        }

 

    }


    
}