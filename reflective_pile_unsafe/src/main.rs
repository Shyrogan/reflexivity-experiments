use pile::Pile;

/// Cette méthode va être exécutée à la place de [`Vec::grow`].
pub fn growing_error(pile: &mut Vec<String>) -> Result<(), String> {
    Err("Eh non! Pas cette fois! Passe par une `GrowablePile` !".to_string())
}

/// Cependant, rien ne nous empêche d'appliquer le pattern de nouveau type :)
pub struct GrowablePile<T>(Vec<T>);

impl<T> Pile<T> for GrowablePile<T> {
    fn initialize(capacite: usize) -> Self {
        GrowablePile(Vec::<T>::initialize(capacite))
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn is_full(&self) -> bool {
        self.0.is_full()
    }

    fn push(&mut self, e: T) -> Result<(), String> {
        if self.is_full() {
            self.grow()?;
        }
        Vec::push(&mut self.0, e);
        Ok(())
    }

    fn grow(&mut self) -> Result<(), String> {
        Vec::reserve(&mut self.0, 1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{growing_error, GrowablePile};
    use pile::Pile;
    use retour::RawDetour;

    #[test]
    pub fn pile_not_growable() {
        let mut hook = unsafe {
            let hook = RawDetour::new(Vec::<String>::grow as *const (), growing_error as *const ())
                .expect("Failed to create detour");
            hook.enable().expect("Should be able to enable detour");

            hook
        };
        let mut pile: Vec<String> = Vec::initialize(5);
        assert!(pile.grow().is_err());
    }

    pub fn growable_pile_grows() {
        let mut growable_pile: GrowablePile<String> = GrowablePile::initialize(2);
        assert!(growable_pile.grow().is_ok());
        assert!(growable_pile.0.capacity() > 2);
    }
}
