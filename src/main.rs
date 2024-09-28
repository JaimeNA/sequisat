use mathru::{elementary::power::Power, elementary::trigonometry::Trigonometry, elementary::exponential::Exponential};
// Constants
const XKE: f64 = 0.12455;    // TODO: Replace with real values
const CK2: f64 = 0.12345;

// Recover original mean motion and semimajor axis from input - TLE sets already provide de mean motion and semimayor axis
pub fn initialization(n0: f64, e0: f64, i0: f64, m0: f64, w0: f64, omega0: f64)
{
   // a1, phi1, a0, phi0, n02 a02: f64;    // TODO: Improve naming

    let a1:f64 = Power::pow(XKE/n0, 2.0/3.0);

    // Trigonometric funtions use radians
    let temp_delta1: f64 = 3.0*Trigonometry::cos(i0)*Trigonometry::cos(i0) - 1.0;
    let temp_delta2: f64 = Power::pow(1.0 - e0*e0, 3.0/2.0);
    let delta1: f64 = 1.5*CK2*temp_delta1 / (a1*a1*temp_delta2);
    
    println!("{}", delta1);
}

pub fn setConstant(f64: i0, a0_2: f64,n0_2: f64, s: f64, e0: f64, B: f64, q0: f64)
{
    let phita = Trigonometry::cos(i0);
    let exilon = 1 / (a0_2 - s)
    let beta0 = Power::pow(1 - e0*e0, 0.5);
    let n = a0_2 * e0 * exilon;

    const C2: f64 = Power::pow((q0 - s)*(exilon), 4.0);
}

fn main() {
    initialization(0.23, 0.23, 0.23, 0.23, 0.23, 0.23);
}
