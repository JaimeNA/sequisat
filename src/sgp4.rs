use mathru::{elementary::power::Power, elementary::trigonometry::Trigonometry};
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, Utc, Duration};

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

    // Function to parse the TLE epoch (YYDDD.DDDDDDDD) - cortesy of chatgpt
    pub fn parse_tle_epoch(epoch_str: &str) -> NaiveDateTime {
    // Parse year (YY)
    let year_part = &epoch_str[..2];
    let year: i32 = 24;
    
    // Handle year assumption (20xx or 19xx)
    let full_year = if year >= 57 { 1900 + year } else { 2000 + year };

    // Parse day of the year (DDD)
    let day_of_year: u32 = 272;
    
    // Parse the fractional part of the day (.DDDDDDDD)
    let fractional_day: f64 = 0.20796705;
    
    // Calculate hours, minutes, and seconds from fractional day
    let total_seconds = (fractional_day * 86400.0).round() as u32;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    // Get the start of the year as a NaiveDate
    let start_of_year = NaiveDate::from_ymd(full_year, 1, 1);
    
    // Add the day of the year (adjusted by -1 since day_of_year starts from 1)
    let epoch_date = start_of_year + Duration::days((day_of_year - 1) as i64);

    // Create a NaiveDateTime for the epoch time
    NaiveDateTime::new(epoch_date, chrono::NaiveTime::from_hms(hours, minutes, seconds))
}

// Function to calculate the time difference between two NaiveDateTime in minutes
pub fn time_since_epoch_in_seconds(epoch: NaiveDateTime, current_time: NaiveDateTime) -> f64 {
    let duration = current_time - epoch;
    duration.num_seconds() as f64 / 60.0
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

        let a1:f64 = Power::pow(KE/n0, 2.0/3.0);

        // Trigonometric funtions use radians
        let temp_delta1: f64 = 3.0*Trigonometry::cos(i0)*Trigonometry::cos(i0) - 1.0;
        let temp_delta2: f64 = Power::pow(1.0 - e0*e0, 3.0/2.0);
        let delta1: f64 = 1.5*K2*temp_delta1 / (a1*a1*temp_delta2);
        
        let a0 = a1 * (1.0 - (1.0/3.0)*delta1 - delta1*delta1 - (134.0/81.0)*delta1*delta1*delta1);

        let delta0 = (3.0/2.0)*(K2 / (a0*a0)) * (temp_delta1/temp_delta2);

        self.n02 = n0 / (1.0 + delta0);
        //self.a02 = n0 / (1.0 - delta0);

        self.a02 = Power::pow(KE/self.n02, 2.0/3.0);

        self.set_constants(i0, e0, bstar, w0);
    }

    pub fn set_constants(&mut self, i0: f64, e0: f64, bstar: f64, w0: f64)
    {
        self.e0 = e0;
        self.phita = Trigonometry::cos(i0);
        self.exilon = 1.0 / (self.a02 - S);
        self.b0 = Power::pow(1.0 - e0*e0, 0.5);
        let n = self.a02 * e0 * self.exilon;
        self.n = n; // TODO: refactor

        self.c2 = Q0MS2T*Power::pow(self.exilon, 4.0) * self.n02 * Power::pow(1.0 - n*n, -7.0/2.0)
            * (self.a02*(1.0 + 1.5*n*n + 4.0*e0*n + e0*n*n*n) + 1.5*((K2*self.exilon)/(1.0-n*n))
            * (-0.5+1.5*self.phita*self.phita) * (8.0 + 24.0*n*n + 3.0*n*n*n*n));

        self.c1 = bstar*self.c2;

        self.c3 = (Q0MS2T*Power::pow(self.exilon, 5.0)*A30*self.n02*AE* Trigonometry::sin(i0)) / K2*e0;

        self.c4 = bstar*2.0*self.n02 * Q0MS2T*Power::pow(self.exilon, 4.0) * self.a02 * self.b0*self.b0 * Power::pow(1.0-n*n, -3.5) 
            * ((2.0*n*(1.0 + e0*n) + 0.5*e0 + 0.5*n*n*n) - ((2.0*K2*self.exilon) / self.a02*(1.0-n*n))
            * (3.0*(1.0 - 3.0*self.phita*self.phita) * (1.0 + 1.5*n*n - 2.0*e0*n - 0.5*e0*n*n*n) + 0.75*(1.0-self.phita*self.phita)
            * (2.0*n*n - e0*n - e0*n*n*n)*Trigonometry::cos(2.0*w0)));
        // TODO: Why bstar on C4?
        
        self.c5 = 2.0*Q0MS2T*Power::pow(self.exilon, 4.0) * self.a02 * self.b0*self.b0 * Power::pow(1.0-n*n, -3.5) 
            * (1.0 + (11.0/4.0)*n*(n + e0) + e0*n*n*n);

        self.d2 = 4.0 * self.a02 * self.exilon * self.c1*self.c1;
        self.d3 = (4.0/3.0)*self.a02 * self.exilon*self.exilon * (17.0*self.a02 + S) * self.c1*self.c1*self.c1;
        self.d4 = (2.0/3.0) * self.a02 * self.exilon*self.exilon*self.exilon * (221.0*self.a02 + 32.0*S) * self.c1*self.c1*self.c1*self.c1;

    }

    pub fn update_gravity_and_atm_drag(&mut self, m0: f64, omega0: f64, deltaTime: f64)
    {
        let mdf = m0 + (1.0 + (3.0*K2 * (-1.0+3.0*self.phita*self.phita))/(2.0*self.a02*self.a02*Power::pow(self.b0, 4.0))
            + (3.0*K2*K2 * (13.0 - 78.0*self.phita*self.phita
            + 137.0*Power::pow(self.phita, 4.0)))/(16.0*Power::pow(self.a02, 4.0)*Power::pow(self.b0, 7.0)))
            * self.n02*deltaTime;

        let wdf = self.w0 + (-(3.0*K2 * (-1.0-5.0*self.phita*self.phita))/(2.0*self.a02*self.a02*self.b0*self.b0*self.b0*self.b0) 
            + (3.0*K2*K2 * (7.0 - 114.0*self.phita*self.phita + 395.0*Power::pow(self.phita, 4.0)))/(16.0*Power::pow(self.a02, 4.0)*Power::pow(self.b0, 8.0))
            + (5.0*K4 * (3.0-36.0*self.phita*self.phita+49.0*Power::pow(self.phita, 4.0)))/(16.0*Power::pow(self.a02, 4.0)*Power::pow(self.b0, 8.0)))
            * self.n02*deltaTime;

        let omegadf = omega0 + (-(3.0*K2*self.phita)/(self.a02*self.a02*Power::pow(self.b0, 4.0))
            + (3.0*K2*K2*(4.0*self.phita - 19.0*Power::pow(self.phita, 3.0)))/(2.0*Power::pow(self.a02, 4.0)*Power::pow(self.b0, 8.0))
            + (5.0*K4*self.phita*(3.0 - 7.0*self.phita*self.phita))/(2.0*Power::pow(self.a02, 4.0)*Power::pow(self.b0, 8.0)))
            * self.n02*deltaTime;

        let deltaw = self.bstar * self.c3 * Trigonometry::cos(self.w0) * deltaTime;
        let deltaM = -(2.0/3.0) * Q0MS2T * self.bstar * Power::pow(self.exilon, 4.0)
                * (AE / (self.e0*self.n) * (Power::pow((1.0 + self.n*Trigonometry::cos(mdf)), 3.0)
                - Power::pow((1.0 + self.n*Trigonometry::cos(m0)), 3.0)));

        let mp = mdf + deltaw + deltaM;

        let w = wdf - deltaw - deltaM;
        let omega = omegadf - (21.0/2.0)*((self.n02*K2*self.phita) / (self.a02*self.a02*self.b0*self.b0))
                * self.c1*deltaTime*deltaTime;
        let e = self.e0 - self.bstar*self.c4*deltaTime
            - self.bstar*self.c5*(Trigonometry::sin(mp) - Trigonometry::sin(m0));
        let a = self.a02*Power::pow((1.0 - self.c1*deltaTime - self.d2*deltaTime*deltaTime - self.d3*Power::pow(deltaTime, 3.0)
            - self.d4*Power::pow(deltaTime, 4.0)), 2.0);

        let il = mp + w + omega + self.n02*(1.5*self.c1*deltaTime*deltaTime + (self.d2+2.0*self.c2*self.c2)*Power::pow(deltaTime, 3.0)
            + 0.25*(3.0*self.d3 + 12.0*self.c1*self.d2 + 10.0*Power::pow(self.c1, 3.0))*Power::pow(deltaTime, 4.0)
            + 0.2*(3.0*self.d4 + 12.0*self.c1*self.d3 + 6.0*self.d2*self.d2 + 30.0*self.c1*self.c1*self.d2 + 15.0*Power::pow(self.c1, 4.0))
            *Power::pow(deltaTime, 5.0));

        let b = Power::pow(1.0-e*e, 0.5);
        let n1 = KE / Power::pow(a, 1.5);

        //Add the long-period periodic terms

        println!("  e = {}: ", e);
        println!("  w = {}: ", w);
        let axn = e*Trigonometry::cos(w);

        let ill = ((A30*Trigonometry::sin(self.i0)) / (8.0*K2*a*b*b)) * e*Trigonometry::cos(w)
            * ((3.0+5.0*self.phita) / (1.0+self.phita));
        let aynl = (A30*Trigonometry::sin(self.i0)) / (4.0*K2*a*b*b);
        let ilt = il + ill;
        let ayn = e*Trigonometry::sin(w) + aynl;

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
            let delta = (up - ayn*Trigonometry::cos(eo1) + axn*Trigonometry::sin(eo1) - eo1)
                / (-ayn*Trigonometry::sin(eo1) - axn*Trigonometry::cos(eo1) + 1.0);

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
        let ecose = axn*Trigonometry::cos(eo1) + ayn*Trigonometry::sin(eo1);
		let esine = axn*Trigonometry::sin(eo1) - ayn*Trigonometry::cos(eo1);
		let el2 = axn*axn + ayn*ayn;
		let pl = a*(1.0 - el2);

        let r = a*(1.0 - ecose);

        let rdot = KE*(Power::pow(a, 0.5)/r) * esine;
        let rfdot = KE*(pl / r);

        let cosu = (a/r) * (Trigonometry::cos(eo1) - axn + ayn*(esine / (1.0 + (1.0 - el2).sqrt())));
        let sinu = (a/r) * (Trigonometry::sin(eo1) - ayn - axn*(esine / (1.0 + (1.0 - el2).sqrt())));

        let u =  Trigonometry::arctan(sinu / cosu);

        let deltar = (K2 / (2.0*pl)) * (1.0-self.phita*self.phita) * Trigonometry::cos(2.0*u);
        let deltau = -0.25 * (K2 / (pl*pl)) * (7.0*self.phita*self.phita - 1.0) * Trigonometry::sin(2.0*u);
        let deltaomega = ((3.0*K2*self.phita) / (2.0*pl*pl)) * Trigonometry::sin(2.0*u);
        let deltai = ((3.0*K2*self.phita) / (2.0*pl*pl)) * Trigonometry::sin(self.i0) * Trigonometry::cos(2.0*u);
        let deltardot = -((K2*n1) / pl) * (1.0-self.phita*self.phita) * Trigonometry::sin(2.0*u); // Note that n1 is the actual n and n is the greek n-like letter
        let deltarfdot = ((K2*n1) / pl) * ((1.0-self.phita*self.phita) * Trigonometry::cos(2.0*u) 
            - 1.5*(1.0 - 3.0*self.phita*self.phita));

        // The short period periodics are added to give the osculation quantities
        let rk = r*(1.0 - 1.5*K2*(Power::pow(1.0 - el2, 0.5) / (pl*pl)) * (3.0*self.phita*self.phita - 1.0)) + deltar;
        let uk = u + deltau;
        let omegak = omega + deltaomega;
        let ik = self.i0 + deltai;
        let rdotk = rdot + deltardot;
        let rfdotk = rfdot + deltarfdot;

        let ux = -Trigonometry::sin(omegak) * Trigonometry::cos(ik) * Trigonometry::sin(uk)
            + Trigonometry::cos(omegak) * Trigonometry::cos(uk);
        

        let rx = rk * ux * 6378.137;

        println!("  rx = {}: ", rx);
        // println!("  ry = {}: ", ry);
        // println!("  rz = {}: ", rz);
    }
}