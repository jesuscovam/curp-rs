use std::fmt;
pub enum Comunes {
    MariaDel,
    MariaDeLos,
    Maria,
    JoseDe,
    Jose,
    MaPunto,
    Ma,
    M,
    JPunto,
    J,
}

impl fmt::Display for Comunes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Comunes::MariaDel => write!(f, "MARIA DEL "),
            Comunes::MariaDeLos => write!(f, "MARIA DE LOS "),
            Comunes::Maria => write!(f, "MARIA "),
            Comunes::JoseDe => write!(f, "JOSE DE "),
            Comunes::Jose => write!(f, "JOSE "),
            Comunes::MaPunto => write!(f, "MA. "),
            Comunes::Ma => write!(f, "MA "),
            Comunes::M => write!(f, "M. "),
            Comunes::JPunto => write!(f, "J. "),
            Comunes::J => write!(f, "J "),
        }
    }
}
