
mod sgp4;
// ISS (ZARYA)             <- Test TLE data
// 1 25544U 98067A   24272.20796705  .00058591  00000+0  10319-2 0  9996
// 2 25544  51.6377 165.1137 0006922  38.3252  99.3437 15.49843852474523

fn main() {

    // Data from TLE
    let i0: f64 = (51.6377 * 3.14159265) / 180.0;   // Converted to degrees
    let e0: f64 = 0.0006922;
    let n0: f64 = 15.49843852;
    let w0: f64 = (38.3252 * 3.14159265) / 180.0;
    let bstar: f64 = 0.010319;

    let mut iss = sgp4::SGP4::new();

    iss.initialize(n0, e0, i0);
    iss.set_constant(i0, e0, bstar, w0);

}
