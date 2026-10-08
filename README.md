# Ultima-like

Un petit jeu de rôle en Rust avec Bevy, inspiré des premiers Ultima (I-IV).

## Style graphique

Palette EGA 16 couleurs, tuiles 16x16 pixels générées procéduralement.
Aucun asset externe n'est requis : tous les sprites sont dessinés par code.

## Prérequis

- Rust (édition 2021 ou plus récente)
- Un GPU supportant Vulkan

## Compilation

    cargo run

Pour une version optimisée :

    cargo run --release

## Contrôles

| Touche | Action |
|--------|--------|
| Flèches / ZQSD / WASD | Déplacer le personnage |
| F | Attaquer (si un ennemi est à portée) |
| Espace | Parler au PNJ (si à proximité) |
| Échap | Quitter |

## État actuel

- Carte de 20x15 tuiles avec 6 types de terrain
- Joueur déplaçable
- 2 ennemis avec IA de poursuite et attaque au contact
- 1 PNJ avec dialogue
- Système de points de vie avec affichage en temps réel
- Réapparition automatique du joueur à sa mort

## Palette EGA

Les 16 couleurs de l'Enhanced Graphics Adapter sont définies en constantes
dans main.rs et utilisées pour la génération procédurale des tuiles et
sprites.


