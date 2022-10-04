pub mod integrator {
    use crate::vec3::{vec3::Vec3d, IsVec3d};

    pub trait Integrate{
        type Object;
        fn integrate(position: Vec<Vec3d<Self::Object>>,velocity: Vec<Vec3d<Self::Object>>) -> Vec<Vec3d<Self::Object>>;
        
    }
}