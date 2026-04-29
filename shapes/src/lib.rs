pub mod shapes;
pub use shapes::{Shape, Shaper};
pub mod prefab;
pub use prefab::{
    polygon,
    platonic_solid,
    kepler_poinsot,
};
