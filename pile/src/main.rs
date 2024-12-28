#[derive(Debug)]
struct Pile<T> {
    // Taille est l'index du dernier élément + 1
    taille: usize,
    capacite: usize,
    contenu: Vec<T>,
}

impl<T> Pile<T> {
    /// Initialise la pile en créant une instance de la structure de données.
    fn initialize(capacite: usize) -> Pile<T> {
        Pile {
            taille: 0,
            capacite,
            contenu: Vec::with_capacity(capacite),
        }
    }

    /// Vérifie si la pile est vide.
    fn is_empty(&self) -> bool {
        self.taille == 0
    }

    /// Vérifie si la pile est pleine.
    fn is_full(&self) -> bool {
        self.taille == self.capacite
    }

    /// Ajoute un élément dans la pile si elle n'est pas pleine.
    fn push(&mut self, e: T) -> Result<(), ()> {
        if self.is_full() {
            Err(())
        } else {
            self.contenu.push(e);
            self.taille += 1;
            Ok(())
        }
    }

    fn grow(&mut self) {
        self.contenu.reserve(self.capacite);
        self.capacite *= 2;
    }
}

fn main() {
    // Crée une pile avec une capacité de 2
    let mut pile: Pile<String> = Pile::initialize(2);

    // Ajoute une instance de Person à la pile
    let _ = pile.push(String::from("Hello"));

    // Ajoute une instance d'Animal à la pile
    let _ = pile.push(String::from("World"));

    println!("La pile contient {} éléments: {:?}", pile.taille, pile);
}

#[cfg(test)]
mod test {
    use crate::Pile;

    #[test]
    pub fn pile_fail_push_when_full() {
        let mut pile: Pile<String> = Pile::initialize(0);
        assert!(pile.is_empty());
        assert!(pile.push(String::from("Hello")).is_err());
    }

    #[test]
    pub fn pile_push_single_element() {
        let mut pile: Pile<String> = Pile::initialize(1);
        assert!(pile.push(String::from("Hello")).is_ok());
        assert_eq!(1, pile.taille);
        assert!(pile.is_full());
    }

    #[test]
    pub fn pile_grow() {
        let mut pile: Pile<String> = Pile::initialize(1);
        assert!(pile.push(String::from("Hello")).is_ok());
        pile.grow();
        assert!(pile.push(String::from("Hello")).is_ok());
        assert_eq!(2, pile.capacite);
    }
}
