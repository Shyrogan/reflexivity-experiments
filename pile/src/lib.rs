pub trait Pile<T> {
    /// Fonction qui initialise la pile, prenant une taille en paramètre et retournant `Self`.
    ///
    /// Notons que `Self` signifie que si le trait est implémenté par exemple sur une structure
    /// [`Vec`], le type de retour de cette méthode sera [`Vec`]
    fn initialize(capacite: usize) -> Self;

    /// Retourne `true` si la pile est vide, `false` sinon.
    fn is_empty(&self) -> bool;

    /// Retourne `true` si la pile est pleine, `false` sinon.
    fn is_full(&self) -> bool;

    /// Grossis la pile (double la taille), deux cas sont possibles:
    /// - L'opération se fait correctement et on retourne [`Result::Ok`], equivalent de, c'est à
    ///   dire que l'opération s'est bien terminée.
    /// - On renvoie Result::Err(String), une erreur qui a pour type un String, contenant un
    ///    message
    fn grow(&mut self) -> Result<(), String>;

    /// Ajoute un élément à la pile. Le résultat est similaire à celui de [`Pile::grow`].
    fn push(&mut self, e: T) -> Result<(), String>;
}

#[allow(dead_code)]
impl<T> Pile<T> for Vec<T> {
    /// Initialise la pile en créant une instance de la structure de données.
    fn initialize(capacite: usize) -> Vec<T> {
        Vec::with_capacity(capacite)
    }

    /// Vérifie si la pile est vide.
    fn is_empty(&self) -> bool {
        // On s'assure qu'on appel la bonne méthode, deux méthodes ont le même nom,
        // [Vec::is_empty] et [Pile::is_empty], on veut celle de [`Vec`].
        Vec::is_empty(self)
    }

    /// Vérifie si la pile est pleine.
    fn is_full(&self) -> bool {
        self.capacity() == self.len()
    }

    /// Ajoute un élément dans la pile. Si celle-ci est pleine, on double son volume à l'aide de [`Vec`].
    fn push(&mut self, e: T) -> Result<(), String> {
        if self.is_full() {
            self.grow()?;
        }
        Vec::push(self, e);
        Ok(())
    }

    fn grow(&mut self) -> Result<(), String> {
        // On s'assure qu'on appel la bonne méthode, deux méthodes ont le même nom, on veut celle de [`Vec`].
        Vec::reserve(self, 1);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::Pile;

    #[test]
    pub fn pile_push_single_element() {
        let mut pile: Vec<String> = Vec::initialize(1);
        assert!(Pile::push(&mut pile, String::from("Hello")).is_ok());
        assert_eq!(1, pile.len());
        assert!(pile.is_full());
    }

    #[test]
    pub fn pile_grow() {
        let mut pile: Vec<String> = Vec::initialize(1);
        assert!(Pile::push(&mut pile, String::from("Hello")).is_ok());
        let _ = pile.grow();
        assert!(Pile::push(&mut pile, String::from("Hello")).is_ok());
        assert_eq!(4, pile.capacity());
    }
}
