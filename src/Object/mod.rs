use crate::vec3::vec3d;

pub mod object {
    use num_traits::real::Real;

    use crate::vec3::vec3d::IsVec3d;
    
    pub struct Ob<T,V> 
    where 
        V: IsVec3d,
        T: num_traits::real::Real
    {
        pub mass: T,
        pub pos: V,
        pub vel: V,
        pub kind: kind,
        



    }
    #[derive(Clone,Copy)]
    pub enum kind{
        Star,
        BlackHole

    }

    impl<T,V> Clone for Ob<T,V>
    where 
        T: Real,
        V: IsVec3d
    {
        fn clone(&self) -> Self {
            Ob {
                mass: self.mass,
                pos: V::clone(&self.pos),
                vel: V::clone(&self.vel),
                kind: self.kind
            }
        }
    }
}