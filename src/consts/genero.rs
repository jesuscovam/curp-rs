use std::fmt;

pub enum Genero {
    Femenino,
    Masculino,
}

impl fmt::Display for Genero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Genero::Femenino => write!(f, "F"),
            Genero::Masculino => write!(f, "M"),
        }
    }
}
