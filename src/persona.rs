use crate::consts::estado::Estado;
use crate::consts::genero::Genero;

pub struct Persona<'a> {
    pub nombre: &'a str,
    pub apellido_paterno: &'a str,
    pub apellido_materno: &'a str,
    pub genero: Genero,
    pub estado: Estado,
    pub fecha_de_nacimiento: &'a str,
}
