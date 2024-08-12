use std::borrow::ToOwned;
use std::sync::LazyLock;

// T = tundra, G = grasslands, B = Taiga / boreal forest, D = desert,
// S = savanna, F = temperate forest, R = temperate rainforest,
// W = Xeric shrubland and dry forest, E = tropical dry forest,
// O = tropical rainforest, I = icecap
pub static biomes: LazyLock<[[u8; 45]; 45]> = LazyLock::new(|| {
    [
        b"IIITTTTTGGGGGGGGDDDDDDDDDDDDDDDDDDDDDDDDDDDDD".to_owned(),
        b"IIITTTTTGGGGGGGGDDDDGGDSDDSDDDDDDDDDDDDDDDDDD".to_owned(),
        b"IITTTTTTTTTBGGGGGGGGGGGSSSSSSDDDDDDDDDDDDDDDD".to_owned(),
        b"IITTTTTTTTBBBBBBGGGGGGGSSSSSSSSSWWWWWWWDDDDDD".to_owned(),
        b"IITTTTTTTTBBBBBBGGGGGGGSSSSSSSSSSWWWWWWWWWWDD".to_owned(),
        b"IIITTTTTTTBBBBBBFGGGGGGSSSSSSSSSSSWWWWWWWWWWW".to_owned(),
        b"IIIITTTTTTBBBBBBFFGGGGGSSSSSSSSSSSWWWWWWWWWWW".to_owned(),
        b"IIIIITTTTTBBBBBBFFFFGGGSSSSSSSSSSSWWWWWWWWWWW".to_owned(),
        b"IIIIITTTTTBBBBBBBFFFFGGGSSSSSSSSSSSWWWWWWWWWW".to_owned(),
        b"IIIIIITTTTBBBBBBBFFFFFFGGGSSSSSSSSWWWWWWWWWWW".to_owned(),
        b"IIIIIIITTTBBBBBBBFFFFFFFFGGGSSSSSSWWWWWWWWWWW".to_owned(),
        b"IIIIIIIITTBBBBBBBFFFFFFFFFFGGSSSSSWWWWWWWWWWW".to_owned(),
        b"IIIIIIIIITBBBBBBBFFFFFFFFFFFFFSSSSWWWWWWWWWWW".to_owned(),
        b"IIIIIIIIIITBBBBBBFFFFFFFFFFFFFFFSSEEEWWWWWWWW".to_owned(),
        b"IIIIIIIIIITBBBBBBFFFFFFFFFFFFFFFFFFEEEEEEWWWW".to_owned(),
        b"IIIIIIIIIIIBBBBBBFFFFFFFFFFFFFFFFFFEEEEEEEEWW".to_owned(),
        b"IIIIIIIIIIIBBBBBBRFFFFFFFFFFFFFFFFFEEEEEEEEEE".to_owned(),
        b"IIIIIIIIIIIIBBBBBBRFFFFFFFFFFFFFFFFEEEEEEEEEE".to_owned(),
        b"IIIIIIIIIIIIIBBBBBRRRFFFFFFFFFFFFFFEEEEEEEEEE".to_owned(),
        b"IIIIIIIIIIIIIIIBBBRRRRRFFFFFFFFFFFFEEEEEEEEEE".to_owned(),
        b"IIIIIIIIIIIIIIIIIBRRRRRRRFFFFFFFFFFEEEEEEEEEE".to_owned(),
        b"IIIIIIIIIIIIIIIIIRRRRRRRRRRFFFFFFFFEEEEEEEEEE".to_owned(),
        b"IIIIIIIIIIIIIIIIIIRRRRRRRRRRRRFFFFFEEEEEEEEEE".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIRRRRRRRRRRRRRFRREEEEEEEEEE".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIRRRRRRRRRRRRRRRREEEEEEEE".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIRRRRRRRRRRRRRROOEEEEEE".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIRRRRRRRRRRRROOOOOEEEE".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIRRRRRRRRRROOOOOOEEE".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIRRRRRRRRROOOOOOOEE".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIRRRRRRRROOOOOOOEE".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIRRRRRRROOOOOOOOE".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIRRRRROOOOOOOOOO".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIRROOOOOOOOOOO".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIROOOOOOOOOOO".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIROOOOOOOOOOO".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIOOOOOOOOOOO".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIOOOOOOOOOO".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIOOOOOOOOO".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIOOOOOOOOO".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIOOOOOOOO".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIOOOOOOOO".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIOOOOOOOO".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIOOOOOOOO".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIOOOOOOO".to_owned(),
        b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIOOOOOOO".to_owned(),
    ]
});

// Character table for XPM output
static XPMCharacters: LazyLock<[u8; 64]> = LazyLock::new(|| {
    b"@$.,:;-+=#*&ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".to_owned()
});

#[derive(Clone)]
pub struct LatLong {
    pub latitude: f64,
    pub lat_sin: f64,
    pub lat_cos: f64,
    pub longitude: f64,
    pub long_sin: f64,
    pub long_cos: f64,
}

impl LatLong {
    pub fn new(lat: f64, long: f64) -> Self {
        Self {
            latitude: lat,
            longitude: long,
            lat_sin: 0.0,
            lat_cos: 0.0,
            long_sin: 0.0,
            long_cos: 0.0,
        }
    }

    pub fn new_with_trig(lat: f64, long: f64) -> Self {
        Self {
            latitude: lat,
            longitude: long,
            lat_sin: lat.sin(),
            lat_cos: lat.cos(),
            long_sin: long.sin(),
            long_cos: long.cos(),
        }
    }
}
