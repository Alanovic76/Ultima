use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

// ============================================================
// 1. PALETTE EGA 16 COULEURS (Ultima III / IV)
// ============================================================
// Palette complète documentée. Certaines couleurs ne sont pas
// encore utilisées mais restent disponibles pour la suite.

const EGA_BLACK:    Color = Color::srgb(0.0, 0.0, 0.0);
#[allow(dead_code)] const EGA_BLUE:     Color = Color::srgb(0.0, 0.0, 170.0/255.0);
#[allow(dead_code)] const EGA_GREEN:    Color = Color::srgb(0.0, 170.0/255.0, 0.0);
#[allow(dead_code)] const EGA_CYAN:     Color = Color::srgb(0.0, 170.0/255.0, 170.0/255.0);
#[allow(dead_code)] const EGA_RED:      Color = Color::srgb(170.0/255.0, 0.0, 0.0);
#[allow(dead_code)] const EGA_MAGENTA:  Color = Color::srgb(170.0/255.0, 0.0, 170.0/255.0);
#[allow(dead_code)] const EGA_BROWN:    Color = Color::srgb(170.0/255.0, 85.0/255.0, 0.0);
#[allow(dead_code)] const EGA_LGRAY:    Color = Color::srgb(170.0/255.0, 170.0/255.0, 170.0/255.0);
#[allow(dead_code)] const EGA_DGRAY:    Color = Color::srgb(85.0/255.0, 85.0/255.0, 85.0/255.0);
#[allow(dead_code)] const EGA_BBLUE:    Color = Color::srgb(85.0/255.0, 85.0/255.0, 255.0/255.0);
#[allow(dead_code)] const EGA_BGREEN:   Color = Color::srgb(85.0/255.0, 255.0/255.0, 85.0/255.0);
#[allow(dead_code)] const EGA_BCYAN:    Color = Color::srgb(85.0/255.0, 255.0/255.0, 255.0/255.0);
#[allow(dead_code)] const EGA_BRED:     Color = Color::srgb(255.0/255.0, 85.0/255.0, 85.0/255.0);
#[allow(dead_code)] const EGA_BMAGENTA: Color = Color::srgb(255.0/255.0, 85.0/255.0, 255.0/255.0);
#[allow(dead_code)] const EGA_YELLOW:   Color = Color::srgb(255.0/255.0, 255.0/255.0, 85.0/255.0);
#[allow(dead_code)] const EGA_WHITE:    Color = Color::srgb(1.0, 1.0, 1.0);

fn color_to_rgba(c: Color) -> [u8; 4] {
    let s = c.to_srgba();
    [
        (s.red * 255.0) as u8,
        (s.green * 255.0) as u8,
        (s.blue * 255.0) as u8,
        (s.alpha * 255.0) as u8,
    ]
}

fn make_image(size: usize, data: Vec<u8>) -> Image {
    Image::new(
        Extent3d {
            width: size as u32,
            height: size as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

// ============================================================
// 2. GÉNÉRATION DES TUILES (16x16 pixels, style EGA)
// ============================================================
fn generate_tile_image(tile_index: usize) -> Image {
    const SIZE: usize = 16;
    let mut data = vec![0u8; SIZE * SIZE * 4];

    for y in 0..SIZE {
        for x in 0..SIZE {
            let color = match tile_index {
                // 0 : Herbe
                0 => {
                    if (x * 3 + y * 5) % 11 == 0 { EGA_GREEN }
                    else if (x + y) % 7 == 0 { Color::srgb(0.0, 120.0/255.0, 0.0) }
                    else { EGA_BGREEN }
                }
                // 1 : Eau
                1 => {
                    if (x + y * 2) % 9 == 0 || (y + x * 3) % 13 == 0 { EGA_BCYAN }
                    else if (x * 2 + y) % 5 == 0 { EGA_CYAN }
                    else { EGA_BLUE }
                }
                // 2 : Mur de pierre
                2 => {
                    let brick_y = y / 4;
                    let offset = if brick_y % 2 == 0 { 0 } else { 4 };
                    let brick_x = (x + offset) % 8;
                    if y % 4 == 0 || brick_x == 0 { EGA_DGRAY }
                    else if (brick_x + brick_y) % 3 == 0 { EGA_LGRAY }
                    else { EGA_WHITE }
                }
                // 3 : Chemin de terre
                3 => {
                    if (x * 7 + y * 3) % 17 == 0 { EGA_DGRAY }
                    else if (x + y * 2) % 5 == 0 { Color::srgb(140.0/255.0, 70.0/255.0, 0.0) }
                    else { EGA_BROWN }
                }
                // 4 : Arbre
                4 => {
                    let cx = x as f32 - 7.5;
                    let cy = y as f32 - 7.5;
                    let dist = (cx * cx + cy * cy).sqrt();
                    if dist < 5.5 && y < 13 {
                        if (x + y) % 4 == 0 { EGA_GREEN }
                        else { Color::srgb(0.0, 100.0/255.0, 0.0) }
                    } else if (6..=9).contains(&x) && y >= 11 {
                        EGA_BROWN
                    } else {
                        EGA_BLACK
                    }
                }
                // 5 : Maison
                5 => {
                    if y < 8 {
                        if (x + y) % 3 == 0 { Color::srgb(140.0/255.0, 70.0/255.0, 0.0) }
                        else { EGA_BROWN }
                    } else if y < 12 {
                        if x == 0 || x == 15 || y == 8 { EGA_DGRAY } else { EGA_RED }
                    } else if (6..=9).contains(&x) && y >= 12 {
                        EGA_BLACK
                    } else if (x == 2 || x == 13) && y == 10 {
                        EGA_YELLOW
                    } else {
                        EGA_RED
                    }
                }
                _ => EGA_BLACK,
            };
            let rgba = color_to_rgba(color);
            let idx = (y * SIZE + x) * 4;
            data[idx..idx + 4].copy_from_slice(&rgba);
        }
    }
    make_image(SIZE, data)
}

/// Génère un sprite humanoïde (16x16) avec tunique de la couleur donnée.
fn generate_human_image(tunic: Color) -> Image {
    const SIZE: usize = 16;
    let mut data = vec![0u8; SIZE * SIZE * 4];

    // Couleur claire dérivée de la tunique pour l'ombrage
    let s = tunic.to_srgba();
    let tunic_light = Color::srgb(
        (s.red * 1.3).min(1.0),
        (s.green * 1.3).min(1.0),
        (s.blue * 1.3).min(1.0),
    );

    for y in 0..SIZE {
        for x in 0..SIZE {
            let color = if (2..=6).contains(&y) && (5..=10).contains(&x) {
                // Tête
                if y == 2 || y == 6 || x == 5 || x == 10 { EGA_BROWN }
                else if (x == 6 || x == 9) && (y == 4 || y == 5) { EGA_BLACK }
                else { Color::srgb(255.0/255.0, 200.0/255.0, 150.0/255.0) }
            } else if (7..=12).contains(&y) && (4..=11).contains(&x) {
                // Corps
                if y == 7 || x == 4 || x == 11 { tunic } else { tunic_light }
            } else if y >= 13 && (4..=11).contains(&x) {
                // Jambes
                if (6..=9).contains(&x) { EGA_DGRAY } else { EGA_BLACK }
            } else {
                EGA_BLACK
            };
            let rgba = color_to_rgba(color);
            let idx = (y * SIZE + x) * 4;
            data[idx..idx + 4].copy_from_slice(&rgba);
        }
    }
    make_image(SIZE, data)
}

// ============================================================
// 3. COMPOSANTS ET RESSOURCES
// ============================================================
#[derive(Component)]
struct Joueur;

#[derive(Component)]
struct PNJ {
    dialogue: String,
}

#[derive(Resource)]
struct TileHandles {
    handles: Vec<Handle<Image>>,
}

#[derive(Component)]
struct DialogueUI;

const TAILLE_TUILE: f32 = 32.0;
const VITESSE_JOUEUR: f32 = 150.0;

// Carte : 20x15 tuiles
// 0=herbe, 1=eau, 2=mur, 3=chemin, 4=arbre, 5=maison
const CARTE: [[u8; 20]; 15] = [
    [4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,4,4],
    [4,0,0,0,2,2,2,2,2,0,0,0,3,3,3,0,0,0,0,4],
    [0,0,0,2,5,5,5,5,5,2,0,3,3,3,3,3,0,0,0,0],
    [0,0,2,5,5,5,5,5,5,5,2,3,3,0,3,3,3,0,0,0],
    [0,0,2,5,5,5,5,5,5,5,2,3,3,0,3,3,3,3,0,0],
    [0,0,2,5,5,5,5,5,5,5,2,3,3,3,3,3,3,3,3,0],
    [0,0,0,2,2,2,2,2,2,2,2,3,3,3,3,3,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [4,4,0,0,0,0,0,0,3,3,3,3,0,0,0,0,0,0,4,4],
    [4,0,0,0,0,0,0,0,3,3,3,3,0,0,0,0,0,0,0,4],
];

// ============================================================
// 4. MAIN
// ============================================================
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Mon RPG — Style Ultima".into(),
                        resolution: (640.0_f32, 480.0_f32).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(EGA_BLACK))
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_tuiles,
                setup_monde,
                setup_joueur,
                setup_pnj,
            )
                .chain(),
        )
        .add_systems(Update, (deplacer_joueur, afficher_dialogue))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_tuiles(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut handles = Vec::new();
    for i in 0..6 {
        handles.push(images.add(generate_tile_image(i)));
    }
    commands.insert_resource(TileHandles { handles });
}

fn setup_monde(mut commands: Commands, tile_handles: Res<TileHandles>) {
    let largeur = CARTE[0].len() as f32;
    let hauteur = CARTE.len() as f32;
    let offset_x = -(largeur * TAILLE_TUILE) / 2.0 + TAILLE_TUILE / 2.0;
    let offset_y = (hauteur * TAILLE_TUILE) / 2.0 - TAILLE_TUILE / 2.0;

    for (y, ligne) in CARTE.iter().enumerate() {
        for (x, &tuile) in ligne.iter().enumerate() {
            let pos_x = offset_x + x as f32 * TAILLE_TUILE;
            let pos_y = offset_y - y as f32 * TAILLE_TUILE;
            commands.spawn((
                Sprite {
                    image: tile_handles.handles[tuile as usize].clone(),
                    custom_size: Some(Vec2::splat(TAILLE_TUILE)),
                    ..default()
                },
                Transform::from_xyz(pos_x, pos_y, 0.0),
            ));
        }
    }
}

fn setup_joueur(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let handle = images.add(generate_human_image(EGA_BLUE));
    commands.spawn((
        Sprite {
            image: handle,
            custom_size: Some(Vec2::splat(TAILLE_TUILE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
        Joueur,
    ));
}

fn setup_pnj(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let handle = images.add(generate_human_image(EGA_GREEN));
    commands.spawn((
        Sprite {
            image: handle,
            custom_size: Some(Vec2::splat(TAILLE_TUILE)),
            ..default()
        },
        Transform::from_xyz(-96.0, 64.0, 10.0),
        PNJ {
            dialogue: "Bienvenue, aventurier ! Le royaume a besoin de toi...".into(),
        },
    ));
}

fn deplacer_joueur(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Joueur>>,
) {
    let mut direction = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let deplacement = direction.normalize() * VITESSE_JOUEUR * time.delta_secs();
        for mut transform in &mut query {
            transform.translation.x += deplacement.x;
            transform.translation.y += deplacement.y;
        }
    }
}

fn afficher_dialogue(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    joueur_query: Query<&Transform, With<Joueur>>,
    pnj_query: Query<(&Transform, &PNJ)>,
    dialogue_existant: Query<Entity, With<DialogueUI>>,
    mut commands: Commands,
) {
    let Ok(joueur_transform) = joueur_query.single() else {
        return;
    };

    for (pnj_transform, pnj) in &pnj_query {
        let distance = joueur_transform
            .translation
            .truncate()
            .distance(pnj_transform.translation.truncate());

        if distance < 48.0 && keyboard_input.just_pressed(KeyCode::Space) {
            for entity in &dialogue_existant {
                commands.entity(entity).despawn();
            }
            commands.spawn((
                DialogueUI,
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    left: Val::Px(20.0),
                    right: Val::Px(20.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(EGA_BLACK),
                children![(
                    Text::new(&pnj.dialogue),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(EGA_WHITE),
                )],
            ));
        }
    }
}

