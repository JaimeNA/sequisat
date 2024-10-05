
pub struct Orbit {
    /// Angle between the equator and the orbit plane in rad
    pub inclination: f64,

    /// Angle between vernal equinox and the point where the orbit crosses the equatorial plane in rad
    pub right_ascension: f64,

    /// Shape of the orbit
    pub eccentricity: f64,

    /// Angle between the ascending node and the orbit's point of closest approach to the earth in rad
    pub argument_of_perigee: f64,

    /// Angle of the satellite location measured from perigee in rad
    pub mean_anomaly: f64,

    /// Mean number of orbits per day in rad.min⁻¹
    pub mean_motion: f64,
}

pub struct Constants {
    pub geopotential: model::Geopotential,
    pub right_ascension_dot: f64,
    pub argument_of_perigee_dot: f64,
    pub mean_anomaly_dot: f64,
    pub c1:         f64,
    pub c4:         f64,
    pub k0:         f64,
    pub k1:         f64,
    pub method:     Method,
    pub orbit_0:    Orbit,
}