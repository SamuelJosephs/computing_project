use crate::vec3::vec3d;

pub mod object {
    use std::path::Component;

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
        pub acc: V,
        pub kind: kind,
        



    }
    #[derive(Clone,Copy)]
    pub enum kind{
        Planet,
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
                acc: V::clone(&self.acc),
                kind: self.kind
            }
        }

    }

    impl<T,V> Ob<T,V> 
    where
        T: Real,
        V: IsVec3d<Component = T>
    {
        pub fn new(mass: T, pos: V, vel: V,acc: V,kind: kind) -> Self{
            return Ob { mass: mass, pos: pos, vel: vel,acc: acc, kind: kind};
        }
    }
}