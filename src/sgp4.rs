use mathru::{elementary::power::Power, elementary::trigonometry::Trigonometry};

const KE: f64 = 0.0743669161;
const S: f64 = 1.01222928;
const Q0MS2T: f64 = 0.00000000188027916;
const J2: f64 = 0.00108264;     // Second Gravitational Zonal Harmonic of the Earth
const J3: f64 = -0.000253881;   // Third Gravitational Zonal Harmonic of the Earth
const AE: f64 = 1.0;            // Equatorial radius of the earth
const K2: f64 = 0.5*J2*AE*AE;
const A30: f64 = -J3*AE*AE*AE;

pub struct SGP4
{
    a02:    f64,
    n02:    f64,
    phita:  f64,
    b0:     f64,
    c1:     f64,
    c2:     f64,
    c3:     f64,
    c4:     f64,
    c5:     f64,
    d2:     f64,
    d3:     f64,
    d4:     f64,

}

impl SGP4
{
    pub fn new() -> Self
    {
        SGP4 {
            a02:    0.0, 
            n02:    0.0,
            phita:  0.0,
            b0:     0.0,
            c1:     0.0,
            c2:     0.0,
            c3:     0.0,
            c4:     0.0,
            c5:     0.0,
            d2:     0.0,
            d3:     0.0,
            d4:     0.0,
        }
    }    

    // Recover original mean motion and semimajor axis from input
    pub fn initialize(&mut self, n0: f64, e0: f64, i0: f64)
    {
    // a1, phi1, a0, phi0, n02 a02: f64;    // TODO: Improve naming

        let a1:f64 = Power::pow(KE/n0, 2.0/3.0);

        // Trigonometric funtions use radians
        let temp_delta1: f64 = 3.0*Trigonometry::cos(i0)*Trigonometry::cos(i0) - 1.0;
        let temp_delta2: f64 = Power::pow(1.0 - e0*e0, 3.0/2.0);
        let delta1: f64 = 1.5*K2*temp_delta1 / (a1*a1*temp_delta2);
        
        let a0 = a1 * (1.0 - (1.0/3.0)*delta1 - delta1*delta1 - (134.0/81.0)*delta1*delta1*delta1);

        let delta0 = (3.0/2.0)*(K2 / (a0*a0)) * (temp_delta1/temp_delta2);

        self.n02 = n0 / (1.0 + delta0);
        self.a02 = n0 / (1.0 + delta0);

        println!("{}", delta1);
    }

    // i0   : Mean Inclination At Epoch
    // a02  : Semimayor Axis
    // n02  : Mean Motion
    // e0   : Mean Eccentricity At Epoch
    // B    : SGP4 Type Drag Coefficient
    // q0   : Parameter for SGP4 Density Function
    // w0   : Mean Argument of perigee at epoch
    pub fn set_constant(&mut self, i0: f64, e0: f64, bstar: f64, w0: f64)
    {
        self.phita = Trigonometry::cos(i0);
        let exilon = 1.0 / (self.a02 - S);
        self.b0 = Power::pow(1.0 - e0*e0, 0.5);
        let n = self.a02 * e0 * exilon;

        self.c2 = Q0MS2T*Power::pow(exilon, 4.0) * self.n02 * Power::pow(1.0 - n*n, -7.0/2.0)
            * (self.a02*(1.0 + 1.5*n*n + 4.0*e0*n + e0*n*n*n) + 1.5*((K2*exilon)/(1.0-n*n))
            * (-0.5+1.5*self.phita*self.phita) * (8.0 + 24.0*n*n + 3.0*n*n*n*n));

        self.c1 = bstar*self.c2;

        self.c3 = (Q0MS2T*Power::pow(exilon, 5.0)*A30*self.n02*AE* Trigonometry::sin(i0)) / K2*e0;

        self.c4 = 2.0*self.n02 * Q0MS2T*Power::pow(exilon, 4.0) * self.a02 * self.b0*self.b0 * Power::pow(1.0-n*n, -3.5) 
            * ((2.0*n*(1.0 + e0*n) + 0.5*e0 + 0.5*n*n*n) - ((2.0*K2*exilon) / self.a02*(1.0-n*n))
            * (3.0*(1.0 - 3.0*self.phita*self.phita) * (1.0 + 1.5*n*n - 2.0*e0*n - 0.5*e0*n*n*n) + 0.75*(1.0-self.phita*self.phita)
            * (2.0*n*n - e0*n - e0*n*n*n)*Trigonometry::cos(2.0*w0)));

        self.c5 = 2.0*Q0MS2T*Power::pow(exilon, 4.0) * self.a02 * self.b0*self.b0 * Power::pow(1.0-n*n, -3.5) 
            * (1.0 + (11.0/4.0)*n*(n + e0) + e0*n*n*n);

        self.d2 = 4.0 * self.a02 * exilon * self.c1*self.c1;
        self.d3 = (4.0/3.0)*self.a02 * exilon*exilon * (17.0*self.a02 + S) * self.c1*self.c1*self.c1;
        self.d4 = (2.0/3.0) * self.a02 * exilon*exilon*exilon * (221.0*self.a02 + 32.0*S) * self.c1*self.c1*self.c1*self.c1;

        
        println!("Initialized variables: ");
        println!("  C1 = {}: ", self.c1);
        println!("  C2 = {}: ", self.c2);
        println!("  C3 = {}: ", self.c3);
        println!("  C4 = {}: ", self.c4);
        println!("  C5 = {}: ", self.c5);
        println!();
        println!("  D2 = {}: ", self.d2);
        println!("  D3 = {}: ", self.d3);
        println!("  D4 = {}: ", self.d4);
    }

    fn update_gravity_and_atm_drag(&mut self, m0: f64)
    {
        let mdf = m0 + (1.0 + (3.0*K2 * (-1.0+3.0*self.phita*self.phita))/(2.0*self.a02*self.a02*Power::pow(self.b0, 4.0))
            + ) * n02*(t - t0);
    }
}