mod interpretation;
pub use interpretation::*;

/* use m43lang_derive::{AsCode, Decodable};

pub trait AsCode {
    fn as_code(&self) -> String;

    fn as_code_depth(&self, _depth: u8) -> String {
        self.as_code()
    }
}

pub trait Decodable {
    fn decode<'a, I: Iterator<Item = &'a str>>(iter: &mut I) -> Self;
}

#[derive(PartialEq, Eq, Debug, AsCode, Decodable)]
enum Teste {
    Alpha,
    Beta(Hue),
    Gamma(Hue, Hue),
}

#[derive(PartialEq, Eq, Debug, AsCode, Decodable)]
enum Hue {
    H,
    S,
    L,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_as_code() {
        assert_eq!(Teste::Alpha.as_code(), "Teste::Alpha");
        assert_eq!(Teste::Beta(Hue::S).as_code(), "Teste::Beta(Hue::S)");
        assert_eq!(Teste::Gamma(Hue::H, Hue::L).as_code(), "Teste::Gamma(Hue::H, Hue::L)");
    }

    #[test]
    fn test_decodable() {
        assert_eq!(Teste::decode(&mut vec!["A"].into_iter()), Teste::Alpha);
        assert_eq!(Teste::decode(&mut vec!["Beta", "S"].into_iter()), Teste::Beta(Hue::S));
        assert_eq!(Teste::decode(&mut vec!["G", "H", "L"].into_iter()), Teste::Gamma(Hue::H, Hue::L));
    }
} */