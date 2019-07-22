use num_traits::identities::Zero;
use num_traits::Float;
use rand_distr::{Distribution, UnitBall};
use std::cmp;
use std::iter;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Vec3<F> {
    pub fn new(x: F, y: F, z: F) -> Self {
        Vec3 { x, y, z }
    }
    pub fn from_array(xyz: [F; 3]) -> Self {
        Vec3 {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        }
    }
    pub fn sqrt(&self) -> Self {
        Vec3 {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }
    pub fn length(&self) -> F {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn squared_length(&self) -> F {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn normalize(&mut self) {
        *self /= self.length();
    }
    pub fn unit(&self) -> Self {
        let mut v = *self;
        v.normalize();
        v
    }
    pub fn rgb(&self) -> Option<[u8; 3]> {
        let x = (self.x * F::from(255.999).unwrap()).floor().to_u8();
        let y = (self.y * F::from(255.999).unwrap()).floor().to_u8();
        let z = (self.z * F::from(255.999).unwrap()).floor().to_u8();
        match (x, y, z) {
            (Some(x), Some(y), Some(z)) => Some([x, y, z]),
            (_, _, _) => None,
        }
    }
    pub fn dot(&self, other: &Self) -> F {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    pub fn rand_in_unit_ball() -> Vec3<F> {
        let xyz: [f32; 3] = UnitBall.sample(&mut rand::thread_rng());
        Vec3::new(
            F::from(xyz[0]).unwrap(),
            F::from(xyz[1]).unwrap(),
            F::from(xyz[2]).unwrap(),
        )
    }
    pub fn reflect(&self, n: &Self) -> Self {
        *self - *n * self.dot(n) * F::from(2).unwrap()
    }
    pub fn refract(&self, n: &Self, r: F) -> Option<Self> {
        let uv = self.unit();
        let dt = uv.dot(n);
        let discr = F::one() - r * r * (F::one() - dt * dt);
        if discr > F::zero() {
            Some((uv - *n * dt) * r - *n * discr.sqrt())
        } else {
            None
        }
    }
}

impl<F: Float> ops::Neg for Vec3<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<F: Float> ops::Add<Self> for Vec3<F> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<F: Float> ops::Sub<Self> for Vec3<F> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<F: Float> ops::Mul<F> for Vec3<F> {
    type Output = Self;
    fn mul(self, other: F) -> Self::Output {
        Self::new(self.x * other, self.y * other, self.z * other)
    }
}

impl<F: Float> ops::Mul<Self> for Vec3<F> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl<F: Float> ops::Div<F> for Vec3<F> {
    type Output = Self;
    fn div(self, other: F) -> Self::Output {
        Self::new(self.x / other, self.y / other, self.z / other)
    }
}

impl<F: Float> ops::AddAssign<Self> for Vec3<F> {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl<F: Float> ops::SubAssign<Self> for Vec3<F> {
    fn sub_assign(&mut self, other: Self) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
    }
}

impl<F: Float> ops::MulAssign<F> for Vec3<F> {
    fn mul_assign(&mut self, other: F) {
        self.x = self.x * other;
        self.y = self.y * other;
        self.z = self.z * other;
    }
}

impl<F: Float> ops::DivAssign<F> for Vec3<F> {
    fn div_assign(&mut self, other: F) {
        self.x = self.x / other;
        self.y = self.y / other;
        self.z = self.z / other;
    }
}

impl<F: Float> iter::Sum for Vec3<F> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), |acc, c| acc + c)
    }
}

impl<F: Float> cmp::PartialEq for Vec3<F> {
    fn eq(&self, other: &Self) -> bool {
        (*self - *other).is_zero()
    }
}

impl<F: Float> cmp::Eq for Vec3<F> {}

impl<F: Float> Zero for Vec3<F> {
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
    fn zero() -> Self {
        Vec3 {
            x: F::zero(),
            y: F::zero(),
            z: F::zero(),
        }
    }
}
