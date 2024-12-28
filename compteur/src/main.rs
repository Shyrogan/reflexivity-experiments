use bevy_reflect::Struct;

#[derive(bevy_reflect::Reflect, Debug)]
pub struct Compteur {
    pub valeur: u32,
}

fn main() {
    let c = Compteur { valeur: 0 };

    // Affiche dans le debug les informations sur le compteur
    // Compteur { valeur: 0 }
    println!("{:?}", c);

    // Nécessaire pour la réflexion
    println!(
        "Attributs: {}",
        c.get_represented_struct_info()
            .expect("Erreur lors de la réflexion sur `c`")
            .field_names()
            .join(", ")
    );
}
