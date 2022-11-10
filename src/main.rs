mod vec3;
use Object::object::{Ob, kind};
use integrator::integrator::{euler_step,euler_step2, step_back_v};
use num_traits::Float;
use num_traits::real::Real;
use vec3::vec3d::IsVec3d;
use vec3::vec3d::{Vec3d};
mod Object;
mod integrator;

use std::f64::consts::PI;
use std::fs::{File};
use std::io::{Write};

fn initialise_mat_with_capacity<T:Default>(n:usize) -> Vec<Vec<T>>{
    let mut initial_vec = Vec::<Vec<T>>::with_capacity(n);
    for _ in 0..n {
        let mut temp_vec = Vec::<T>::with_capacity(n);
        for _ in 0..n {
            temp_vec.push(T::default());
        }
        
        initial_vec.push(temp_vec);
    }
    return initial_vec;
}

fn format_inputs_to_string<T,V>(inputs: &Vec<Ob<T,V>>) -> String
where 
    T: num_traits::real::Real + ToString,
    V: IsVec3d<Component = T>
{
    // let mut file = File::create(FileName).expect("Cannot create file, are you running with appropriate perimissions?");
    let comma = String::from(',');
    let eol = String::from("\n");
    let mut string_to_return = String::with_capacity(10000);
    for i in inputs {
        let mass = i.mass.to_string();
        let x = i.pos.x().to_string();
        let y = i.pos.y().to_string();
        let z = i.pos.z().to_string();



        // file.write_all(mass.as_bytes());
        // file.write_all(comma.as_bytes());

        //TODO: continue
        string_to_return += (mass + &comma
                                    + &x + &comma
                                    + &y + &comma
                                    + &z + &comma 
                                    ).as_str();
                                    
        
        // file.write_all(string_to_write.as_bytes()).expect("Unable to write to file, are you running with correct permissions");
      }
    //   file.write_all(eol.as_bytes());
    string_to_return += eol.as_str();
    return string_to_return;
}


// constants
const EARTH_M: f64 = 5.972e24;
const EARTH_R: f64 = 1.4959787e11;

const SUN_M: f64 = 1.989e30;

const JUPITER_M: f64 = 1.898e27;
const JUPITER_R: f64 = 	778340821e3;

const G:f64 = 6.67e-11;
fn main(){
    let planet = Ob::<f64,Vec3d<f64>>::new(EARTH_M,Vec3d{x:EARTH_R,y:0.,z:0.},Vec3d{x:0.,y:(((G*SUN_M / EARTH_R) as f64).sqrt()),z:0.},Vec3d{x:0.,y:0.,z:0.},kind::Planet,0.);
    let jupiter = Ob::<f64,Vec3d<f64>>::new(
        JUPITER_M,
        Vec3d {x:-JUPITER_R, y:0.,z: 0.},
        Vec3d {x: 0., y: ((G * SUN_M) / JUPITER_R), z: 0. },
        Vec3d { x: 0., y: 0., z: 0. },
        kind::Planet,
        0.
    );
    let star = Ob::<f64,Vec3d<f64>>::new(
        SUN_M,
        Vec3d{x:0.,y:0.,z:0.},
        Vec3d{x:0., y:0., z:0.},
        Vec3d{x:0.,y:0.,z:0.},
        kind::Star,
        0.,
    
    );
    let mut inputs = vec![planet,star,jupiter];
    

    

    
    let N_orbits = 100.;
    // Work out T 
    let mut temp_denominator = ((G*SUN_M / EARTH_R) as f64);
    let T: f64 = N_orbits*(2.*PI*EARTH_R)/((temp_denominator.sqrt()));
    // Work out dt
    let dt: f64 = (10. / (N_orbits*365.25)) * T;
    const epsilon: f64 = 0.0000001;
    let mut a_matrix = initialise_mat_with_capacity(inputs.len());

    let g = 6.67e-11;

    let mut file = File::create("Results.csv").expect("Unable to create file, are you certain you are running with correct permissions?\n");
    

    let mut data_string = String::with_capacity(10000000);
    
    let mut total_E = 0.;
    
    let n_iter = 0;
    let mut nth_iter = 0;
    (inputs,total_E) = step_back_v(inputs, dt, epsilon, &mut a_matrix, g); // Step back initial v by half a time step
    // data_string += total_E.to_string().as_str(); data_string += ",";
    // data_string += format_inputs_to_string(&inputs).as_str();
    for _ in 0..((T/dt) as i64 ) {
        
        (inputs, total_E) = euler_step2(inputs, dt, epsilon, &mut a_matrix, g);
        
        
        if nth_iter < n_iter {
            
            nth_iter += 1;
            continue;
        }
        else {
            data_string += total_E.to_string().as_str(); data_string += ",";
            data_string += format_inputs_to_string(&inputs).as_str();
            nth_iter = 0;
        }
        
        
        
    }

  
    
    file.write_all(data_string.as_bytes()).expect("Failed to write to file, are you sure you are using running with appropriate permissions?");
    
    
}