

pub mod integrator {
    use std::rc::Rc;

    use num_traits::real;

    use crate::{Object::object::Ob, vec3::vec3d::{IsVec3d, Vec3d}};

    pub fn euler_step<T,V>(input: Vec<Ob<T,V>>, dt: T, epsilon: T, a_matrix: &mut Vec<Vec<V>>,g: T) -> (Vec<Ob<T,V>>)
    where 
        T: real::Real,
        V: IsVec3d<Component = T>
    {
        // input and a_matrix[0] must have the same length
        // Set up Rc pointers to input as two owners are needed for each part of the for loops
        let mut input1 = Rc::new(input);
        let mut input2 = Rc::clone(&input1);
        let input1_temp = Rc::get_mut(&mut input1).unwrap();
        let input2_temp = Rc::get_mut(&mut input2).unwrap();
        // Iterate through the input objects and memoize
        // Calculate the 1/r^2 part of acceleration 
        let size = input1_temp.len();
        for (n,i) in input1_temp.iter_mut().enumerate() {
            if n >= ((size / 2) + 1) { // Only need to go through half of the array
                break;
            } 
            for (n2, j) in input2_temp.iter().enumerate() {
                if n == n2 {
                    continue;
                }
                let displacement = i.pos.sub(&j.pos);
                let mut mag = displacement.mag_squared() + epsilon*epsilon;
                mag = mag.powi(3);
                mag = mag.sqrt();
                mag = mag.powi(-1); // mag is now the denominator
                a_matrix[n][n2] = displacement.scalarmul(mag);
                a_matrix[n2][n] = a_matrix[n][n2].scalarmul(T::from(-1).unwrap());

            }


        }

        // Now go about updating velocity and position
        for (n,i) in input1_temp.iter_mut().enumerate() {
            for (n2,j) in input2_temp.iter_mut().enumerate() {
                i.vel = i.vel.add(&a_matrix[n][n2].scalarmul(j.mass).scalarmul(g).scalarmul(dt));
            }
            i.pos = i.pos.add(&i.vel.scalarmul(dt));
        }

        let to_return = input1_temp.to_vec();
        return to_return;
    }
}