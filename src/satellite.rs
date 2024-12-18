mod sgp4;

use sgp4::SGP4;

pub struct Satellite 
{
    propagator: SGP4,
    tle: TLE
}  
