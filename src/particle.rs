#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Particle {
    Blank,
    Sand,
    Stone,
}
impl Particle {
    pub fn to_id(self) -> u8 {
        use Particle::*;
        match self {
            Blank => 0,
            Sand => 1,
            Stone => 2,
        }
    }
}
