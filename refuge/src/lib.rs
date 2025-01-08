pub enum Animal {
    Chat { couleur: String },
    Chien { race: String },
}

pub struct RefugeAnimalier {
    pub animaux: Vec<Animal>,
}

impl RefugeAnimalier {
    pub fn new() -> RefugeAnimalier {
        RefugeAnimalier { animaux: vec![] }
    }

    pub fn accueillir(&mut self, animal: Animal) {
        self.animaux.push(animal);
    }
}

trait Bruyant {
    fn faire_du_bruit(&self);
}

impl Bruyant for Animal {
    fn faire_du_bruit(&self) {}
}

#[cfg(test)]
pub mod test {
    use crate::{Animal, Bruyant, RefugeAnimalier};
    #[test]
    fn test_accueillir() {
        // Envoie de message (fonction) sur le type `RefugeAnimalier`
        let mut refuge = RefugeAnimalier::new();
        // Envoie de message (méthode) sur une "instance" de `RefugeAnimalier`
        refuge.accueillir(Animal::Chat {
            couleur: String::from("Noir"),
        })
    }
    #[test]
    fn dynamic_dispatch() {
        let bruyants: Vec<Box<dyn Bruyant>> = vec![];
        for animal in bruyants {
            animal.faire_du_bruit();
        }
    }
}
