use std::ops::{Add, Mul};

type Scalar = f64;
type ScalarTuple = (Scalar, Scalar, Scalar);
#[derive(Debug)]
struct Vec {
    x: Scalar,
    y: Scalar,
    z: Scalar,
}

impl Add for Vec {
    type Output = Vec;
    fn add(self, rhs: Vec) -> Vec {
        Vec::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Mul for Vec {
    type Output = Vec;

    fn mul(self, rhs: Self) -> Vec {
        Vec::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Vec {
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { x, y, z }
    }
    pub fn i() -> Vec {
        Vec::new(1., 0., 0.)
    }
    pub fn j() -> Vec {
        Vec::new(0., 1., 0.)
    }
    pub fn k() -> Vec {
        Vec::new(0., 0., 1.)
    }
    pub fn magnitude(&self) -> Scalar {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn as_unit_vector(&self) -> Vec {
        let m = self.magnitude();
        self.scalar_mul(m)
    }
    pub fn scalar_mul(&self, s: Scalar) -> Vec {
        Vec::new(self.x * s, self.y * s, self.z * s)
    }
    pub fn dot(&self, rhs: Vec) -> Vec {
        Vec::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
#[test]
fn add() {
    let v1 = Vec {
        x: 2.,
        y: 3.,
        z: 4.,
    };
    let v2 = Vec {
        x: 5.,
        y: 6.,
        z: 7.,
    };
    let v3 = Vec::new(1., 2., 2.);
    let v4 = Vec::new(1., 2., 3.);
    let x = 2 ^ 3;
    println!("{:?}", v1 + v2);

    let v1 = Vec {
        x: 2.,
        y: 3.,
        z: 4.,
    };
    let v2 = Vec {
        x: 5.,
        y: 6.,
        z: 7.,
    };

    println!("{:?}", v1 * v2);
}
