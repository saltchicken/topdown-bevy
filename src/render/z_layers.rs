pub const Y_SORT_MULTIPLIER: f32 = 0.0001;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZLayer {
    Background,
    Shadows,
    Entities,
    Ui,
}

impl ZLayer {
    pub fn to_f32(self) -> f32 {
        match self {
            Self::Background => 0.0,
            Self::Shadows => 1.0,
            Self::Entities => 2.0,
            Self::Ui => 100.0,
        }
    }
}
