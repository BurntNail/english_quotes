use std::fmt::Display;

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Theme {
    WO,GH,General
}
impl Default for Theme {
    fn default() -> Self {
        Self::General
    }
}
impl TryInto<Theme> for &str {
    type Error = ();
    fn try_into(self) -> Result<Theme, Self::Error> {
        match self {
            "WO" => Ok(Theme::WO),
            "GH" => Ok(Theme::GH),
            _ => Err(())
        }
    }
}
impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WO => write!(f, "Women"),
            Self::GH => write!(f, "Gothic Horror"),
            Self::General => write!(f, "General"),
        }
    }
}