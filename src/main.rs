mod vec3;
use vec3::vec3::*;
mod integrator;
use integrator::integrator::Integrate;
mod Object;
use Object::Object::{Ob,kind};



fn main() {
    let planet = Ob::<f64>::new(1.,
                                        kind::planet,
                                        Vec3d::new(1.,1.,1.),
                                        Vec3d::new(1.,1.,1.),
                                    );
    let sun = Ob::<f64>::new(100000.,
                                    kind::star,
                                    Vec3d::new(0.,0.,0.),
                                    Vec3d::new(0.,0.,0.)

    );
    let objects = vec![planet,sun];
    //TODO: impliment integrator
}
