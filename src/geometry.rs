mod geomtests;

use rand::Rng;
use std::ops::{Add, Mul, Sub};

pub type Scalar = f64;

#[derive(Debug, PartialEq, Clone)]
pub struct Vect {
    pub(crate) x: Scalar,
    pub(crate) y: Scalar,
    pub(crate) z: Scalar,
}
impl Vect {
    pub fn random(min: f64, max: f64, use_z: bool) -> Vect {
        let mut rng = rand::thread_rng();

        let z_val = if use_z { rng.gen_range(min..max) } else { 0.0 };
        Self {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: z_val,
        }
    }
}

impl Copy for Vect {}

impl Add for Vect {
    type Output = Vect;
    fn add(self, rhs: Vect) -> Vect {
        Vect::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Sub for Vect {
    type Output = Vect;
    fn sub(self, rhs: Vect) -> Vect {
        Vect::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Mul for Vect {
    type Output = Vect;

    fn mul(self, rhs: Self) -> Vect {
        let Self {
            x: u1,
            y: u2,
            z: u3,
        } = self;
        let Vect {
            x: v1,
            y: v2,
            z: v3,
        } = rhs;
        Vect::new(u2 * v3 - u3 * v2, u3 * v1 - u1 * v3, u1 * v2 - u2 * v1)
    }
}

impl Vect {
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { x, y, z }
    }
    pub fn i() -> Vect {
        Vect::new(1., 0., 0.)
    }
    pub fn j() -> Vect {
        Vect::new(0., 1., 0.)
    }
    pub fn k() -> Vect {
        Vect::new(0., 0., 1.)
    }
    pub fn euclid_distance(&self, rhs: &Vect) -> Scalar {
        let Self {
            x: u1,
            y: u2,
            z: u3,
        } = self;
        let Vect {
            x: v1,
            y: v2,
            z: v3,
        } = rhs;

        let d = (u1 - v1).abs().powf(2.0) + (u2 - v2).abs().powf(2.0) + (u3 - v3).abs().powf(2.0);
        d.sqrt()
    }
    pub fn magnitude(&self) -> Scalar {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }
    pub fn as_unit_vector(&self) -> Vect {
        self.scalar_mul(1.0 / self.magnitude())
    }
    pub fn scalar_mul(&self, s: Scalar) -> Vect {
        Vect::new(self.x * s, self.y * s, self.z * s)
    }
    pub fn dot(&self, rhs: Vect) -> Scalar {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
