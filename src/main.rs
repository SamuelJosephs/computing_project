mod vec3;
use Object::object::{Ob, kind};
use integrator::integrator::{euler_step,euler_step2};
use vec3::vec3d::IsVec3d;
use vec3::vec3d::{Vec3d};
mod Object;
mod integrator;
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

fn main(){
    let planet = Ob::<f64,Vec3d<f64>>::new(1.,Vec3d{x:100.,y:100.,z:100.},Vec3d{x:1.,y:1.,z:1.},Vec3d{x:0.,y:0.,z:0.},kind::Planet,0.);
    let star = Ob::<f64,Vec3d<f64>>::new(
        1000.,
        Vec3d{x:0.,y:0.,z:0.},
        Vec3d{x:0., y:0., z:0.},
        Vec3d{x:0.,y:0.,z:0.},
        kind::Star,
        0.,
    
    );
    let mut inputs = vec![planet,star];
    

    let T:i32 = 1000;
    let dt = 0.001;
    let epsilon = 0.00000001;
    let mut a_matrix = initialise_mat_with_capacity(inputs.len());

    let g = 6.67e-11;

    let mut file = File::create("Results.csv").expect("Unable to create file, are you certain you are running with correct permissions?\n");
    

    let mut data_string = String::with_capacity(10000000);
    
    let mut total_E = 0.;
    
    let n_iter = 10;
    let mut nth_iter = 0;
    for _ in 0..((T as f64/dt) as usize) {
        
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