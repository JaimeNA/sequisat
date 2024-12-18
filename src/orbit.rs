pub struct Orbit {
    /// B star
    pub drag_term: f64,

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

impl Orbit{
    pub fn from_tle(tle :TLE) -> Self
    {
        Orbit{
            drag_term:  tle.drag_term,
            inclination: tle.inclination,
            right_ascension: tle.right_ascension,
            eccentricity: tle.eccentricity,
            argument_of_perigee: tle.argument_of_perigee,
            mean_anomaly: tle.mean_anomaly,
            mean_motion: tle.mean_motion
        }
    }
}
pub struct Geopotential {
    /// Equatorial radius of the earth in km
    // aₑ
    pub ae: f64,

    /// square root of earth's gravitational parameter in earth radii³ min⁻²
    // kₑ
    pub ke: f64,

    /// un-normalised second zonal harmonic
    // J₂
    pub j2: f64,

    /// un-normalised third zonal harmonic
    // J₃
    pub j3: f64,

    /// un-normalised fourth zonal harmonic
    // J₄
    pub j4: f64,
}

