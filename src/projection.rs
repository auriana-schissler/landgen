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