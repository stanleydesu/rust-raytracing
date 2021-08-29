pub struct Vec3 {
    v: [f64; 3],
}

impl Vec3 {
    pub fn new(first: f64, second: f64, third: f64) -> Vec3 {
        Vec3 {
            v: [first, second, third],
        }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.v[0]
    }

    pub fn y(&self) -> f64 {
        self.v[1]
    }

    pub fn z(&self) -> f64 {
        self.v[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_constructor() {
        let vec = Vec3::zero();
        assert_eq!(vec.v, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn new_constructor() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.v, [1.0, 2.0, 3.0]);
    }
}
