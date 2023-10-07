/// Rellena con ceros un string, para que quede de un ancho determinado.
pub fn zeropad(ancho: usize, num: u32) -> String {
    let pad: String = (0..ancho).map(|_| '0').collect();
    let num_str = num.to_string();

    if num_str.len() >= ancho {
        num_str
    } else {
        format!("{}{}", &pad[..ancho - num_str.len()], num_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeropad() {
        let result = zeropad(10, 10);

        assert_eq!(result, "0000000010");

        let result2 = zeropad(5, 100);
        assert_eq!(result2, "00100");

        let result3 = zeropad(5, 30);
        assert_eq!(result3, "00030");
    }
}
