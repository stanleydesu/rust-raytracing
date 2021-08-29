pub struct Vec3 {
    v: [f64; 3],
}

impl Vec3 {
    pub fn new(first: f64, second: f64, third: f64) -> Vec3 {
        Vec3 {
            v: [first, second, third],
        }
    }

    pub fn new_default() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_constructor() {
        let vec = Vec3::new_default();
        assert_eq!(vec.v, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn new_constructor() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.v, [1.0, 2.0, 3.0]);
    }
}
