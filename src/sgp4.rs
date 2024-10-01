use mathru::{elementary::power::Power, elementary::trigonometry::Trigonometry};
use chrono::prelude::*;

const KE: f64 = 0.07436685316871385;
const S: f64 = 1.0122292763545218;
const Q0MS2T: f64 = 0.00000000188027916;
const J2: f64 = 0.00108262998905;     // Second Gravitational Zonal Harmonic of the Earth
const J3: f64 = -0.00000253215306;   // Third Gravitational Zonal Harmonic of the Earth
const J4: f64 = -0.00000161098761;   // Forth Gravitational Zonal Harmonic of the Earth
const AE: f64 = 1.0;            // Equatorial radius of the earth
const K2: f64 = 0.5*J2*AE*AE;
const K4: f64 = (-3.0/8.0)*J4*AE*AE*AE*AE;
const A30: f64 = -J3*AE*AE*AE;

pub struct SGP4
{
    a02:    f64,
    n02:    f64,
    phita:  f64,
    bstar:  f64,
    exilon: f64,
    n:      f64,
    e0:     f64,
    b0:     f64,
    w0:     f64,
    i0:     f64,
    t0:     f64,
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
            bstar:  0.0,
            exilon:  0.0,
            n:     0.0,
            e0:     0.0,
            b0:     0.0,
            w0:     0.0,
            i0:     0.0,
            t0:     0.0,
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

    // Function to calculate the time difference between two NaiveDateTime in minutes
    pub fn time_since_epoch_in_minutes() -> f64 {
        // Parse the TLE epoch in YYDDD.DDDDDDDD format
        let year_part = 24;
        let day_of_year_part = 274;
        let fractional_day_part = 0.84428341;

        // Handle YY (TLE epoch year part)
        let current_year = Utc::now().year() % 100; // Get current year last two digits
        let full_year = if year_part <= current_year {
            2000 + year_part
        } else {
            1900 + year_part
        };

        // Convert the day of the year to a NaiveDate
        let tle_date = NaiveDate::from_yo(full_year, day_of_year_part);

        // Calculate the time from the fractional day part (fraction of 24 hours)
        let seconds_in_day = 86400.0 * fractional_day_part;
        let tle_time = NaiveTime::from_num_seconds_from_midnight(seconds_in_day as u32, 0);

        // Create a full TLE epoch DateTime in UTC
        let tle_datetime = Utc
            .from_utc_datetime(&NaiveDate::and_time(&tle_date, tle_time))
            .with_timezone(&Utc);

        // Get the current time in UTC
        let now = Utc::now();

        // Calculate the delta in minutes
        let delta = now.signed_duration_since(tle_datetime).num_seconds() as f64 / 60.0;

        delta
    }

    // Recover original mean motion and semimajor axis from input
    // i0   : Mean Inclination At Epoch
    // a02  : Semimayor Axis
    // n02  : Mean Motion
    // e0   : Mean Eccentricity At Epoch
    // bstar: SGP4 Type Drag Coefficient
    // q0   : Parameter for SGP4 Density Function
    // w0   : Mean Argument of perigee at epoch
    pub fn initialize(&mut self, n0: f64, i0: f64, e0: f64, bstar: f64, w0: f64)
    {
        self.w0 = w0;       // TODO: Add this to the constructor
        self.bstar = bstar;
        self.i0 = i0;

        let a1:f64 = (KE/n0).powf(2.0/3.0);

        // Trigonometric funtions use radians
        let temp_delta1: f64 = 3.0*i0.cos()*i0.cos() - 1.0;
        let temp_delta2: f64 = (1.0 - e0*e0).powf(3.0/2.0);
        let delta1: f64 = 1.5*K2*temp_delta1 / (a1*a1*temp_delta2);
        
        let a0 = a1 * (1.0 - (1.0/3.0)*delta1 - delta1*delta1 - (134.0/81.0)*delta1*delta1*delta1);

        let delta0 = (3.0/2.0)*(K2 / (a0*a0)) * (temp_delta1/temp_delta2);

        self.n02 = n0;
        //self.a02 = n0 / (1.0 - delta0);

        self.a02 = (KE/self.n02).powf(2.0/3.0);

        self.set_constants(i0, e0, bstar, w0);
    }

    pub fn set_constants(&mut self, i0: f64, e0: f64, bstar: f64, w0: f64)
    {
        self.e0 = e0;
        self.phita = i0.cos();
        self.exilon = 1.0 / (self.a02 - S);
        self.b0 = (1.0 - self.e0*self.e0).sqrt();
        self.n = self.a02 * self.e0 * self.exilon;

        self.c2 = Q0MS2T*self.exilon.powi(4) * self.n02 * (1.0 - self.n*self.n).powf(-3.5)
            * (self.a02*(1.0 + 1.5*self.n*self.n + 4.0*self.e0*self.n + self.e0*self.n.powi(3)) + 1.5*((K2*self.exilon)/(1.0-self.n*self.n))
            * (-0.5+1.5*self.phita*self.phita) * (8.0 + 24.0*self.n*self.n + 3.0*self.n.powi(4)));

        self.c1 = bstar*self.c2;

        self.c3 = (Q0MS2T*self.exilon.powi(5)*A30*self.n02*AE * self.i0.sin()) / (K2*self.e0);

        self.c4 = 2.0*self.n02 * Q0MS2T * self.exilon.powi(4) * self.a02 * self.b0*self.b0 * (1.0 - self.n*self.n).powf(-3.5) 
            * ((2.0*self.n*(1.0 + self.e0*self.n) + 0.5*self.e0 + 0.5*self.n.powi(3)) - ((2.0*K2*self.exilon) / (self.a02*(1.0-self.n*self.n)))
            * (3.0*(1.0 - 3.0*self.phita*self.phita) * (1.0 + 1.5*self.n*self.n - 2.0*self.e0*self.n - 0.5*self.e0*self.n.powi(3)) + 0.75*(1.0-self.phita*self.phita)
            * (2.0*self.n*self.n - self.e0*self.n - self.e0*self.n.powi(3)) * (2.0*w0).cos()));
        // TODO: Why bstar on C4?
        
        self.c5 = 2.0*Q0MS2T * self.exilon.powi(4) * self.a02 * self.b0*self.b0 * (1.0 - self.n*self.n).powf(-3.5) 
            * (1.0 + (11.0/4.0)*self.n*(self.n + self.e0) + self.e0*self.n.powi(3));

        self.d2 = 4.0 * self.a02 * self.exilon * self.c1*self.c1;
        self.d3 = (4.0/3.0) * self.a02 * self.exilon*self.exilon * (17.0*self.a02 + S) * self.c1.powi(3);
        self.d4 = (2.0/3.0) * self.a02 * self.exilon.powi(3) * (221.0*self.a02 + 32.0*S) * self.c1.powi(4);

    }

    pub fn update_gravity_and_atm_drag(&mut self, m0: f64, omega0: f64, deltaTime: f64)
    {
        let mdf = m0 + (1.0 + (3.0*K2 * (-1.0+3.0*self.phita*self.phita))/(2.0*self.a02*self.a02*self.b0.powi(3))
            + (3.0*K2*K2 * (13.0 - 78.0*self.phita*self.phita + 137.0*self.phita.powi(4)))/(16.0*self.a02.powi(4)*self.b0.powi(7)))
            * self.n02*deltaTime;

        let wdf = self.w0 + (-(3.0*K2 * (1.0-5.0*self.phita*self.phita))/(2.0*self.a02*self.a02*self.b0.powi(4)) 
            + (3.0*K2*K2 * (7.0 - 114.0*self.phita*self.phita + 395.0*self.phita.powi(4)))/(16.0*self.a02.powi(4)*self.phita.powi(4))
            + (5.0*K4 * (3.0-36.0*self.phita*self.phita+49.0*self.phita.powi(4)))/(16.0*self.a02.powi(4)*self.b0.powi(8)))
            * self.n02*deltaTime;

        let omegadf = omega0 + (-(3.0*K2*self.phita)/(self.a02*self.a02*self.b0.powi(4))
            + (3.0*K2*K2*(4.0*self.phita - 19.0*self.phita.powi(3)))/(2.0*self.a02.powi(4)*self.b0.powi(8))
            + (5.0*K4*self.phita*(3.0 - 7.0*self.phita*self.phita))/(2.0*self.a02.powi(4)*self.b0.powi(8)))
            * self.n02*deltaTime;

        let deltaw = self.bstar * self.c3 * self.w0.cos() * deltaTime;
        let deltaM = -(2.0/3.0) * Q0MS2T * self.bstar * self.exilon.powi(4)
                * (AE / (self.e0*self.n)) * ((1.0 + self.n*mdf.cos()).powi(3)
                - (1.0 + self.n*m0.cos()).powi(3));

        let mp = mdf + deltaw + deltaM;

        let w = wdf - deltaw - deltaM;
        let omega = omegadf - 10.5*((self.n02*K2*self.phita) / (self.a02*self.a02*self.b0*self.b0))
                * self.c1*deltaTime*deltaTime;
        let e = self.e0 - self.bstar*self.c4*deltaTime
            - self.bstar*self.c5*(mp.sin() - m0.sin()); // Does using radians instead of degrees affect the propagation?
        let a = self.a02*(1.0 - self.c1*deltaTime - self.d2*deltaTime*deltaTime - self.d3*deltaTime.powi(3)
            - self.d4*deltaTime.powi(4)).powi(2);

        let il = mp + w + omega + self.n02*(1.5*self.c1*deltaTime*deltaTime + (self.d2+2.0*self.c2*self.c2)*deltaTime.powi(3)
            + 0.25*(3.0*self.d3 + 12.0*self.c1*self.d2 + 10.0*self.c1.powi(3))*deltaTime.powi(4)
            + 0.2*(3.0*self.d4 + 12.0*self.c1*self.d3 + 6.0*self.d2*self.d2 + 30.0*self.c1*self.c1*self.d2 + 15.0*self.c1.powi(4))
            *deltaTime.powi(5));

        let b = (1.0-e*e).sqrt();
        let n1 = KE / a.powf(1.5);

        //Add the long-period periodic terms

        println!("  e = {}: ", e);
        println!("  w = {}: ", w);
        let axn = e*w.cos();

        let ill = ((A30*self.i0.sin()) / (8.0*K2*a*b*b)) * axn
            * ((3.0+5.0*self.phita) / (1.0+self.phita));
        let aynl = (A30*self.i0.sin()) / (4.0*K2*a*b*b);
        let ilt = il + ill;
        let ayn = e*w.sin() + aynl;

        // Solve Kepler's equation for (E + w) by defining:
        let mut up = (ilt - omega) % (2.0 * core::f64::consts::PI);    // Going to be reused later, not for U 

        println!("  axn = {}: ", axn);
        println!("  ayn = {}: ", ayn);
        println!("  up = {}: ", ilt - omega);
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

        println!("  eo1 = {}: ", eo1);
        // Calculate preliminary quantities needed por short-period periodics
        let ecose = axn*eo1.cos() + ayn*eo1.sin();
		let esine = axn*eo1.sin() - ayn*eo1.cos();
		let el2 = axn*axn + ayn*ayn;
		let pl = a*(1.0 - el2);

        let r = a*(1.0 - ecose);

        let rdot = KE*(a.sqrt()/r) * esine;
        let rfdot = KE*(pl.sqrt()/r);

        let cosu = (a/r) * (eo1.cos() - axn + ayn*(esine / (1.0 + (1.0 - el2).sqrt())));
        let sinu = (a/r) * (eo1.sin() - ayn - axn*(esine / (1.0 + (1.0 - el2).sqrt())));

        let u =  sinu.atan2(cosu);

        let deltar = (K2 / (2.0*pl)) * (1.0-self.phita*self.phita) * (2.0*u).cos();
        let deltau = -0.25 * (K2 / (pl*pl)) * (7.0*self.phita*self.phita - 1.0) * (2.0*u).sin();
        let deltaomega = ((3.0*K2*self.phita) / (2.0*pl*pl)) * (2.0*u).sin();
        let deltai = ((3.0*K2*self.phita) / (2.0*pl*pl)) * self.i0.sin() * (2.0*u).cos();
        let deltardot = -((K2*n1) / pl) * (1.0-self.phita*self.phita) * (2.0*u).sin(); // Note that n1 is the actual n and n is the greek n-like letter
        let deltarfdot = ((K2*n1) / pl) * ((1.0-self.phita*self.phita) * (2.0*u).cos() 
            - 1.5*(1.0 - 3.0*self.phita*self.phita));

        // The short period periodics are added to give the osculation quantities
        let rk = r*(1.0 - 1.5*K2*((1.0 - el2).sqrt() / (pl*pl)) * (3.0*self.phita*self.phita - 1.0)) + deltar;
        let uk = u + deltau;
        let omegak = omega + deltaomega;
        let ik = self.i0 + deltai;
        let rdotk = rdot + deltardot;
        let rfdotk = rfdot + deltarfdot;

        let ux = -omegak.sin() * ik.cos() * uk.sin()
            + omegak.cos() * uk.cos();
        let uy = omegak.cos() * ik.cos() * uk.sin()
            + omegak.sin() * uk.cos();
        let uz = ik.sin() * uk.sin();
        

        let rx = rk * ux * 6378.137;
        let ry = rk * uy * 6378.137;
        let rz = rk * uz * 6378.137;

        let altitude = (rx*rx + ry*ry + rz*rz).sqrt();
        let latitude = (rz/altitude).asin();

        print!("  rx = {}: ", rx);
        print!("  ry = {}: ", ry);
        println!("  rz = {}: ", rz);
        println!("  ---  ");
        println!("  altitude = {}: ", altitude - 6378.137);
        println!("  laltitude = {}: ", (latitude * 180.0) / core::f64::consts::PI);
    }
}