
use super::orbit::Orbit;
use super::vector::PositionVector;

const KE: f64 = 0.07436685316871385;
const S: f64 = 1.0122292763545218;
const ER: f64 = 6371.0;
const Q0MS2T: f64 = 0.00000000188027916;
const J2: f64 = 0.00108262998905;     // Second Gravitational Zonal Harmonic of the Earth
const J3: f64 = -0.00000253215306;   // Third Gravitational Zonal Harmonic of the Earth
const J4: f64 = -0.00000161098761;   // Forth Gravitational Zonal Harmonic of the Earth
const AE: f64 = 1.0;            // Equatorial radius of t ehe earth
const K2: f64 = 0.5*J2*AE*AE;
const K4: f64 = (-3.0/8.0)*J4*AE*AE*AE*AE;
const A30: f64 = -J3*AE*AE*AE;

pub trait Propagate {
    fn initialize(&mut self);
    fn propagate(&mut self, delta_time: f64) -> PositionVector;
}

pub struct SGP4 {
    pub orbit_0: Orbit,
    semimayor_axis:    f64,
    phita:  f64,
    exilon: f64,
    eta:    f64,
    beta0:  f64,
    c1:     f64,
    c2:     f64,
    c3:     f64,
    c4:     f64,
    c5:     f64,
    d2:     f64,
    d3:     f64,
    d4:     f64,
}

pub struct SDP4 {
    eccentricity_dot: f64,
    inclination_dot: f64,
}

impl Propagate for SGP4 {

    // TODO: Separate into smaller functions and comment/document it
    fn initialize(&mut self)
    {
        self.recover_a02_n02();

        self.phita = self.orbit_0.inclination.cos();
        self.exilon = 1.0 / (self.semimayor_axis - S);
        self.beta0 = (1.0 - self.orbit_0.eccentricity.powi(2)).sqrt();
        self.eta = self.semimayor_axis * self.orbit_0.eccentricity * self.exilon;

        let psisq = (1.0 - self.eta*self.eta).abs();

        let coef = Q0MS2T*self.exilon.powi(4);
        let coef1 = coef / (psisq.powf(3.5));

        self.c2 = coef1 * self.orbit_0.mean_motion
            * (self.semimayor_axis*(1.0 + 1.5*self.eta*self.eta + 4.0*self.orbit_0.eccentricity*self.eta + self.orbit_0.eccentricity*self.eta.powi(3))
            + 1.5*((K2*self.exilon)/(1.0-self.eta*self.eta))
            * (-0.5+1.5*self.phita*self.phita) * (8.0 + 24.0*self.eta*self.eta + 3.0*self.eta.powi(4)));

        self.c1 = self.orbit_0.drag_term*self.c2;

        self.c3 = (coef * self.exilon * A30 * self.orbit_0.mean_motion * AE * self.orbit_0.inclination.sin()) / (K2*self.orbit_0.eccentricity);

        self.c4 = 2.0*self.orbit_0.mean_motion * coef1 * self.semimayor_axis * self.beta0*self.beta0 
            * ((2.0*self.eta*(1.0 + self.orbit_0.eccentricity*self.eta) + 0.5*self.orbit_0.eccentricity + 0.5*self.eta.powi(3))
            - ((2.0*K2*self.exilon) / (self.semimayor_axis*psisq))
            * (3.0*(1.0 - 3.0*self.phita*self.phita) * (1.0 + 1.5*self.eta*self.eta - 2.0*self.orbit_0.eccentricity*self.eta - 0.5*self.orbit_0.eccentricity*self.eta.powi(3))
            + 0.75*(1.0-self.phita*self.phita)
            * (2.0*self.eta*self.eta - self.orbit_0.eccentricity*self.eta - self.orbit_0.eccentricity*self.eta.powi(3)) * (2.0*self.orbit_0.argument_of_perigee).cos()));
        
        self.c5 = 2.0 * coef1 * self.semimayor_axis * self.beta0*self.beta0 
            * (1.0 + (11.0/4.0)*self.eta*(self.eta + self.orbit_0.eccentricity) + self.orbit_0.eccentricity*self.eta.powi(3));

        self.d2 = 4.0 * self.semimayor_axis * self.exilon * self.c1*self.c1;
        self.d3 = (4.0/3.0) * self.semimayor_axis * self.exilon*self.exilon * (17.0*self.semimayor_axis + S) * self.c1.powi(3);
        self.d4 = (2.0/3.0) * self.semimayor_axis * self.exilon.powi(3) * (221.0*self.semimayor_axis + 32.0*S) * self.c1.powi(4);

    }

    // Note: this provides the coordinates in TEME, meaning that it doesnt have an earth-fixed frame, that would be the ECEF
    fn propagate(&mut self, delta_time: f64) -> PositionVector
    {
        let mdf = self.orbit_0.mean_anomaly + (1.0 + (3.0*K2 * (-1.0+3.0*self.phita*self.phita))/(2.0*self.semimayor_axis*self.semimayor_axis*self.beta0.powi(3))
            + (3.0*K2*K2 * (13.0 - 78.0*self.phita*self.phita + 137.0*self.phita.powi(4)))/(16.0*self.semimayor_axis.powi(4)*self.beta0.powi(7)))
            * self.orbit_0.mean_motion*delta_time;
 
        let wdf = self.orbit_0.argument_of_perigee + (-(3.0*K2 * (1.0-5.0*self.phita*self.phita))/(2.0*self.semimayor_axis*self.semimayor_axis*self.beta0.powi(4)) 
            + (3.0*K2*K2 * (7.0 - 114.0*self.phita*self.phita + 395.0*self.phita.powi(4)))/(16.0*self.semimayor_axis.powi(4)*self.beta0.powi(7))
            + (5.0*K4 * (3.0-36.0*self.phita*self.phita+49.0*self.phita.powi(4)))/(4.0*self.semimayor_axis.powi(4)*self.beta0.powi(8)))
            * self.orbit_0.mean_motion*delta_time;

        let omegadf = self.orbit_0.right_ascension + (-( 3.0*K2*self.phita ) / ( self.semimayor_axis*self.semimayor_axis*self.beta0.powi(4) )
            + ( 3.0*K2*K2*(4.0*self.phita - 19.0*self.phita.powi(3)) ) / ( 2.0*self.semimayor_axis.powi(4)*self.beta0.powi(8) )
            + (5.0*K4*self.phita*(3.0 - 7.0*self.phita*self.phita))/(2.0*self.semimayor_axis.powi(4)*self.beta0.powi(8)))
            * self.orbit_0.mean_motion*delta_time;

        let deltaw = self.orbit_0.drag_term * self.c3 * self.orbit_0.argument_of_perigee.cos() * delta_time;
        let delta_m = -(2.0/3.0) * Q0MS2T * self.orbit_0.drag_term * self.exilon.powi(4)
                * (AE / (self.orbit_0.eccentricity*self.eta)) 
                * ((1.0 + self.eta*mdf.cos()).powi(3) - (1.0 + self.eta*self.orbit_0.mean_anomaly.cos()).powi(3));

        let mp = mdf + deltaw + delta_m;

        let w = wdf - deltaw - delta_m;
        let omega = omegadf - 10.5*((self.orbit_0.mean_motion*K2*self.phita) / (self.semimayor_axis*self.semimayor_axis*self.beta0*self.beta0))
                * self.c1*delta_time*delta_time;
        let e = self.orbit_0.eccentricity - self.orbit_0.drag_term*self.c4*delta_time
            - self.orbit_0.drag_term*self.c5*(mp.sin() - self.orbit_0.mean_anomaly.sin()); // Does using radians instead of degrees affect the propagation?
        let a = self.semimayor_axis*(1.0 - self.c1*delta_time - self.d2*delta_time*delta_time - self.d3*delta_time.powi(3)
            - self.d4*delta_time.powi(4)).powi(2);

        let il = mp + w + omega + self.orbit_0.mean_motion*(1.5*self.c1*delta_time*delta_time + (self.d2+2.0*self.c1*self.c1)*delta_time.powi(3)
            + 0.25*(3.0*self.d3 + 12.0*self.c1*self.d2 + 10.0*self.c1.powi(3))*delta_time.powi(4)
            + 0.2*(3.0*self.d4 + 12.0*self.c1*self.d3 + 6.0*self.d2*self.d2 + 30.0*self.c1*self.c1*self.d2 + 15.0*self.c1.powi(4))
            *delta_time.powi(5));

        let b = (1.0-e*e).sqrt();
        //let n = KE / a.powf(1.5);

        //Add the long-period periodic terms

        let axn = e*w.cos(); // TODO: change value depending on the perigee (see spacetrk.pdf)

        let ill = ((A30*self.orbit_0.inclination.sin()) / (8.0*K2*a*b*b)) * axn
            * ((3.0+5.0*self.phita) / (1.0+self.phita));
        let aynl = (A30*self.orbit_0.inclination.sin()) / (4.0*K2*a*b*b);
        let ilt = il + ill;
        let ayn = e*w.sin() + aynl; 

        // Solve Kepler's equation for (E + w) by defining:
        let up = (ilt - omega) % (2.0 * core::f64::consts::PI);    // Going to be reused later, not for U 

        let mut eo1 = up;

        // And using the iterator equation with:
        //   the following iteration needs better limits on corrections
		for _ in 0..10 
		{
            let delta = (up - ayn*eo1.cos() + axn*eo1.sin() - eo1)
                / (-ayn*eo1.sin() - axn*eo1.cos() + 1.0);

            if delta.abs() < 1.0e-12 {
                break;
            }

			eo1 += if delta < -0.95 {
                -0.95
            } else if delta > 0.95 {
                0.95
            } else {
                delta
            };
		}

        // Calculate preliminary quantities needed por short-period periodics
        let ecose = axn*eo1.cos() + ayn*eo1.sin();
		let esine = axn*eo1.sin() - ayn*eo1.cos();
		let el2 = axn*axn + ayn*ayn;
		let pl = a*(1.0 - el2);

        let r = a*(1.0 - ecose);

        //let rdot = KE*(a.sqrt()/r) * esine;
        // let rfdot = KE*(pl.sqrt()/r);

        let cosu = (a/r) * (eo1.cos() - axn + ayn*(esine / (1.0 + (1.0 - el2).sqrt())));
        let sinu = (a/r) * (eo1.sin() - ayn - axn*(esine / (1.0 + (1.0 - el2).sqrt())));

        let u =  sinu.atan2(cosu);

        let deltar = (K2 / (2.0*pl)) * (1.0-self.phita*self.phita) * (2.0*u).cos();
        let deltau = -0.25 * (K2 / (pl*pl)) * (7.0*self.phita*self.phita - 1.0) * (2.0*u).sin();
        let deltaomega = ((3.0*K2*self.phita) / (2.0*pl*pl)) * (2.0*u).sin();
        let deltai = ((3.0*K2*self.phita) / (2.0*pl*pl)) * self.orbit_0.inclination.sin() * (2.0*u).cos();
        //let deltardot = -((K2*n) / pl) * (1.0-self.phita*self.phita) * (2.0*u).sin(); // Note that n is the actual n and n is the greek n-like letter
        //let deltarfdot = ((K2*n) / pl) * ((1.0-self.phita*self.phita) * (2.0*u).cos() 
        //    - 1.5*(1.0 - 3.0*self.phita*self.phita));

        // The short period periodics are added to give the osculation quantities
        let rk = r*(1.0 - 1.5*K2*((1.0 - el2).sqrt() / (pl*pl)) * (3.0*self.phita*self.phita - 1.0)) + deltar;
        let uk = u + deltau;
        let omegak = omega + deltaomega;
        let ik = self.orbit_0.inclination + deltai;
        //let rdotk = rdot + deltardot;
        //let rfdotk = rfdot + deltarfdot;

        let ux = -omegak.sin() * ik.cos() * uk.sin()
            + omegak.cos() * uk.cos();
        let uy = omegak.cos() * ik.cos() * uk.sin()
            + omegak.sin() * uk.cos();
        let uz = ik.sin() * uk.sin();
        

        let rx = rk * ux;
        let ry = rk * uy;
        let rz = rk * uz;

        PositionVector::new(rx*ER, ry*ER, rz*ER)
    }

}

impl SGP4 {
    pub fn new(orbit_0 :Orbit) -> Self
    {
        SGP4 {
            orbit_0: orbit_0,
            semimayor_axis:    0.0, 
            phita:  0.0,
            exilon:  0.0,
            eta:     0.0,
            beta0:     0.0,
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

    pub fn print_data(&self)
    {
        println!("\nSGP4 Model Data: ");
        println!("C1:  {}", self.c1);
        println!("C2:  {}", self.c2);
        println!("C3:  {}", self.c3);
        println!("C4:  {}", self.c4);
        println!("C5:  {}", self.c5);
        println!("D2:  {}", self.d2);
        println!("D3:  {}", self.d3);
        println!("D4:  {}", self.d4);
    }
    
    pub fn recover_a02_n02(&mut self) 
    {
        let a1 = (KE/self.orbit_0.mean_motion).powf(2.0/3.0);

        let d_aux = (3.0/2.0) * (K2 * (3.0*(self.orbit_0.inclination.cos()).powi(2) - 1.0))
            / ((1.0 - self.orbit_0.eccentricity.powi(2)).powf(3.0/2.0));

        let d1 = d_aux / (a1*a1);

        let a0 = a1 * (1.0 - (1.0/3.0)*d1 - d1*d1 - (134.0/81.0)*d1.powi(3));

        let d0 = d_aux / (a0*a0);

        self.orbit_0.mean_motion = self.orbit_0.mean_motion / (1.0 + d0);

        self.semimayor_axis = a0 / (1.0 - d0);
    }

}
