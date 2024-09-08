use crate::geometry::Vertex;

pub mod azimuthal;
pub mod conical;
pub mod gnomonic;
pub mod icosahedral;
pub mod mercator;
pub mod mollweide;
pub mod orthographic;
pub mod peters;
pub mod sinusoidal;
pub mod square;
pub mod stereographic;

#[derive(Clone)]
pub enum ProjectionMode {
    Mercator,
    Peters,
    Square,
    Stereographic,
    Orthographic,
    Gnomonic,
    Azimuthal,
    Conical,
    Mollweide,
    Sinusoidal,
    Icosahedral,
}

pub trait Projector {
    fn pixel_to_coordinate(&self, h: usize, w: usize) -> Option<Vertex>;
    fn get_subdivision_depth(&self, h: usize) -> u8;
}
