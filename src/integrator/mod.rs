

pub mod integrator {
    use std::rc::Rc;

    use num_traits::real;

    use crate::{Object::object::Ob, vec3::vec3d::{IsVec3d, Vec3d}};

    pub fn euler_step<T,V>(input: &mut Vec<Ob<T,V>>, dt: T, epsilon: T, a_matrix: &mut Vec<Vec<V>>,g: T) -> (Vec<Ob<T,V>>)
    where 
        T: real::Real,
        V: IsVec3d<Component = T>
    {
        // Step through and calculate accelerations, then step through and update velocities then positions
        for i in 0..(input.len()/2) { // Only need to go through the first half of the array
            
            for j in 0..input.len(){
                if i == j {
                    continue;
                }
                
                let displacement = input[j].pos.sub(&input[i].pos);
                let denominator = displacement.mag_squared() + epsilon*epsilon
                                    .powi(3).sqrt();
                a_matrix[i][j] = a_matrix[i][j].add(&displacement.scalardiv(denominator)) ;
                a_matrix[j][i] = a_matrix[i][j].scalarmul(T::from(-1).unwrap());


            }
        }
    
        // Now we have 1/r^2 r hat in a_matrix we can work out accelerations
        for i in 0..input.len() {
            for j in 0..input.len() {
                if i == j{
                    continue;
                }
                input[i].vel = input[i].vel.add(
                    &a_matrix[i][j].scalarmul(dt)
                );
            }
            input[i].pos = input[i].pos.add(
                &input[i].vel.scalarmul(dt)
            )
        }

        return input.to_vec();
    }


    pub fn euler_step2<T,V>(mut input:  Vec<Ob<T,V>>, dt: T, epsilon: T, a_matrix: &mut Vec<Vec<V>>,g: T) -> (Vec<Ob<T,V>>,T)
    where 
        T: real::Real,
        V: IsVec3d<Component = T> + Default
    {
        let mut total_GPE  = T::from(0).unwrap();
        let mut total_KE = T::from(0).unwrap();
        // Step through and calculate accelerations, then step through and update velocities then positions
        for i in 0..input.len(){
            input[i].GPE = T::from(0.).unwrap();
            let mut acceleration = V::default();
            for j in 0..input.len(){
                if i == j {continue};
                
                let r = input[j].pos.sub(
                    &input[i].pos
                );
                let denominator = (r.mag_squared() + epsilon*epsilon).powi(3).sqrt();
                let denominator_potential = r.mag_squared().powi(-1);
                acceleration = acceleration.add(&r.scalardiv(denominator).scalarmul(g).scalarmul(input[j].mass));
                input[i].GPE = input[i].GPE + denominator_potential * g * input[i].mass * input[j].mass;
            }
            input[i].acc = acceleration;
            

        }

        for i in 0..input.len(){
            input[i].vel = input[i].vel.add(&input[i].acc.scalarmul(dt));
            input[i].pos = input[i].pos.add(&input[i].vel.scalarmul(dt));
            
            total_GPE = total_GPE + input[i].GPE;
            total_KE = total_KE + T::from(0.5).unwrap() * input[i].mass * input[i].vel.mag_squared();
        }
        return (input,total_GPE + total_KE);
    }



    


}