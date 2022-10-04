pub mod Object {
    pub struct Ob<T> {
        mass: T,
        kind: kind,
        position: Vec3d,
        velocity: vec3d
    }

    enum kind {
        planet,
        star,
        black_hole

    }

    
}