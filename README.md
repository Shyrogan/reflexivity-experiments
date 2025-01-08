# reflexive experiments

Ce projet contient des expérimentations et exercices effectués dans un premier temps avec le langager de programmation Pharo,
puis dans un second temps avec Rust.

## Pré-requis

Il est nécessaire d'installer la version nightly de Rust afin d'exécuter le projet, cette limitation est dûe
à l'expérimentation avec la librairie `retour-rs`.

https://www.rust-lang.org/tools/install

## Executables

Les exécutables sont les programmes suivants:
- compteur
- operators
- reflective_pile_experiments

Pour exécuter le programme, la commande suivante doit-être exécutée.
```bash
cargo run --bin <programme> 
```

## Librairies

Les autres programmes sont des bibliothèques non exécutables cepedant elles incluent des tests exécutables avec la commande
```bash
cargo test
```
