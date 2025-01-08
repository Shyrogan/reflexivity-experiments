use std::sync::OnceLock;

/// Contient une référence vers notre Singleton, notons qu'il y a un
/// lock et qu'en plus, cet accès est assuré entre plusieurs threads
static INSTANCE: OnceLock<Singleton> = OnceLock::new();

/// Un singleton qui contient un entier
#[derive(Debug, PartialEq, Eq, Clone)]
struct Singleton(i32);

impl Singleton {
    /// Retourne la valeur du singleton
    pub fn new() -> Singleton {
        INSTANCE.get_or_init(|| Singleton(42)).clone()
    }
}

/// Implémentation de Default avec un envoi de
/// message à la fonction Singleton::new
impl Default for Singleton {
    fn default() -> Singleton {
        Singleton::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let singleton1 = Singleton::default();
        let singleton2 = Singleton::default();

        assert_eq!(singleton1, singleton2);
    }
}
