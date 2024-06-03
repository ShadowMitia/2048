use bevy::prelude::*;
use grid::*;
use tween::*;

mod grid;
mod tween;

const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 800.0 + 800.0 * 110.0 / 1000.0);
const CELL_SIZE: Vec2 = Vec2::new(200.0, 200.0);

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(States, Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
enum AppState {
    GameOver,
    Win,
    #[default]
    InGame,
}

#[derive(Default, Event)]
struct ScoreEvent(u32);

#[derive(Resource, Default)]
struct Score(u32);

#[derive(Resource, Default)]
struct HighScore(u32);

#[derive(Resource, Default)]
struct HasWon(bool);

#[must_use]
pub fn score_to_colour(score: u32) -> Color {
    match score {
        0 => Color::hex("cdc1b4").unwrap(),
        2 => Color::hex("eee4da").unwrap(),
        4 => Color::hex("ede0c8").unwrap(),
        8 => Color::hex("f2b179").unwrap(),
        16 => Color::hex("f59563").unwrap(),
        32 => Color::hex("f67c5f").unwrap(),
        64 => Color::hex("f65e3b").unwrap(),
        128 => Color::hex("edcf72").unwrap(),
        256 => Color::hex("edcc61").unwrap(),
        512 => Color::hex("edc850").unwrap(),
        1024 => Color::hex("edc53f").unwrap(),
        2048 => Color::hex("edc22e").unwrap(),
        _ => Color::hex("FF00FF").unwrap(),
    }
}

#[must_use]
fn grid_coord_to_position(v: Vec3) -> Vec3 {
    let mut transform = Vec3::new(0.0, 0.0, 0.0);
    transform.x += CELL_SIZE.x * v.x - WINDOW_SIZE.x / 2.0 + CELL_SIZE.x / 2.0;
    transform.y += CELL_SIZE.y * v.y - WINDOW_SIZE.y / 2.0 + CELL_SIZE.y / 2.0;
    transform.z = v.z;
    transform
}

#[must_use]
fn add_tile(commands: &mut Commands, grid: &mut Grid, text_style: &TextStyle) -> bool {
    if let Some(UVec2 { x: i, y: j }) = grid.add_random_tile() {
        let score = grid.cells[Grid::index_2d(i as usize, j as usize, 4, 4)] as u32;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: score_to_colour(score),
                    custom_size: Some(CELL_SIZE),
                    ..Default::default()
                },
                transform: Transform {
                    translation: grid_coord_to_position(Vec3::new(i as f32, j as f32, 0 as f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            tween_scale(0.2, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 1.0, 1.0)),
            Cell {
                coord: UVec2 { x: i, y: j },
            },
        ));

        let c = grid_coord_to_position(Vec3::new(i as f32, j as f32, 0.));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(score.to_string(), text_style.clone()),
                transform: Transform::from_xyz(c.x, c.y, 2.0),

                ..Default::default()
            },
            Cell {
                coord: UVec2 { x: i, y: j },
            },
        ));
        return true;
    }
    false
}

#[derive(Resource)]
pub struct GameStyle(pub TextStyle);

#[derive(Resource)]
pub struct GameFont(pub Handle<Font>);

#[derive(Component)]
struct ScoreUI;

#[derive(Component)]
struct HighScoreUI;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut grid: ResMut<Grid>) {
    let text_dark: Color = Color::hex("776e65").unwrap();
    let font = asset_server.load("fonts/Kenney Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: text_dark,
    };

    commands.insert_resource(GameStyle(text_style.clone()));
    commands.insert_resource(GameFont(font.clone()));

    // Camera
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_basis: Val::Auto,
                    align_content: AlignContent::Stretch,
                    width: Val::Percent(100.0),
                    height: Val::Px(CELL_SIZE.y / 2.0),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 1.0)),
                ..Default::default()
            },
            WinUI,
        ))
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    "Score 0",
                    TextStyle {
                        font: font.clone(),
                        font_size: 42.0,
                        color: Color::BLACK,
                    },
                ),
                ScoreUI,
            ));
            builder.spawn((
                TextBundle::from_section(
                    "High score 0",
                    TextStyle {
                        font: font.clone(),
                        font_size: 42.0,
                        color: Color::BLACK,
                    },
                ),
                HighScoreUI,
            ));
        });

    // Place two random tiles
    for _ in 0..2 {
        let _ = add_tile(&mut commands, &mut grid, &text_style);
    }
}

fn input(
    input: Res<ButtonInput<KeyCode>>,
    mut grid: ResMut<Grid>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Cell, &Transform)>,
    text_style: Res<GameStyle>,
    mut next_state: ResMut<NextState<AppState>>,
    mut has_won: ResMut<HasWon>,
    mut score_events: EventWriter<ScoreEvent>,
) {
    let (moved, score) = {
        if input.just_pressed(KeyCode::ArrowLeft) {
            grid.move_left()
        } else if input.just_pressed(KeyCode::ArrowRight) {
            grid.move_right()
        } else if input.just_pressed(KeyCode::ArrowUp) {
            grid.move_up()
        } else if input.just_pressed(KeyCode::ArrowDown) {
            grid.move_down()
        } else {
            (vec![], usize::MAX)
        }
    };

    if score == usize::MAX {
        return;
    }

    score_events.send(ScoreEvent(score as u32));

    if !grid.has_empty_cells() && !grid.has_legal_move() {
        next_state.set(AppState::GameOver);
        return;
    }

    if let HasWon(false) = *has_won {
        if grid.max_value() >= 2048 {
            has_won.0 = true;
            next_state.set(AppState::Win);
            return;
        }
    }

    for (entity, mut cell, trans) in query.iter_mut() {
        if let Some((from_cell, to_cell)) = moved.iter().find(|(f, _)| f == &cell.coord) {
            let from = Vec3::new(from_cell.x as f32, from_cell.y as f32, trans.translation.z);
            let from = grid_coord_to_position(from);
            let to = Vec3::new(to_cell.x as f32, to_cell.y as f32, trans.translation.z);
            let to = grid_coord_to_position(to);
            commands
                .entity(entity)
                .insert(tween_translation(0.2, from, to));

            cell.coord.x = to_cell.x;
            cell.coord.y = to_cell.y;
        }
    }

    if !add_tile(&mut commands, &mut grid, &text_style.0) && !grid.has_legal_move() {
        next_state.set(AppState::GameOver);
    }
}

fn add_score(
    mut ev_score: EventReader<ScoreEvent>,
    mut score: ResMut<Score>,
    mut high_score: ResMut<HighScore>,
    mut score_ui: Query<&mut Text, With<ScoreUI>>,
    mut high_score_ui: Query<&mut Text, (With<HighScoreUI>, Without<ScoreUI>)>,
) {
    for ev in ev_score.read() {
        *score = Score(score.0 + ev.0);
    }

    let mut score_ui = score_ui.get_single_mut().unwrap();
    score_ui.sections[0].value = format!("Score {}", score.0);

    if score.0 > high_score.0 {
        high_score.0 = score.0;

        let mut score_ui = high_score_ui.get_single_mut().unwrap();
        score_ui.sections[0].value = format!("High score {}", score.0);
    }
}

fn update_tile_graphics(
    grid: Res<Grid>,
    mut query: Query<(Ref<Cell>, &mut Sprite)>,
    mut text_query: Query<(Ref<Cell>, &mut Text)>,
) {
    for (c, mut s) in query.iter_mut() {
        let score = grid.cells[Grid::index_2d(c.coord.x as usize, c.coord.y as usize, 4, 4)] as u32;
        s.color = score_to_colour(score);
    }

    for (cell, mut text) in text_query.iter_mut() {
        let score =
            grid.cells[Grid::index_2d(cell.coord.x as usize, cell.coord.y as usize, 4, 4)] as u32;

        text.sections[0].value = score.to_string();
    }
}

fn reset_game(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    mut has_won: ResMut<HasWon>,
    text_style: Res<GameStyle>,
    mut score: ResMut<Score>,
) {
    *grid = Grid::new();

    for _ in 0..2 {
        let _ = add_tile(&mut commands, &mut grid, &text_style.0);
    }

    *has_won = HasWon(false);

    score.0 = 0;
}

fn button_system(
    mut next_state: ResMut<NextState<AppState>>,
    text_style: Res<GameStyle>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(AppState::InGame)
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Component)]
struct GameOverUI;

fn game_over(mut commands: Commands, font: Res<GameFont>) {
    let font = &font.0;
    println!("game over!");
    commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Global(2),
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 0.75)),
                ..Default::default()
            },
            GameOverUI,
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Game Over",
                TextStyle {
                    font: font.clone(),
                    font_size: 42.0,
                    color: Color::BLACK,
                },
            ));
            builder
                .spawn((ButtonBundle {
                    style: Style {
                        // horizontally center child text
                        padding: UiRect {
                            left: Val::Px(15.0),
                            right: Val::Px(15.0),
                            top: Val::Px(15.0),
                            bottom: Val::Px(15.0),
                        },
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                },))
                .with_children(|builder| {
                    builder.spawn((TextBundle::from_section(
                        "Replay",
                        TextStyle {
                            font: font.clone(),
                            font_size: 42.0,
                            color: Color::WHITE,
                        },
                    ),));
                });
        });
}

#[derive(Component)]
struct WinUI;

fn win_screen(mut commands: Commands, font: Res<GameFont>) {
    let font = &font.0;

    commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Global(2),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 0.75)),
                ..Default::default()
            },
            WinUI,
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "You got 2048!",
                TextStyle {
                    font: font.clone(),
                    font_size: 42.0,
                    color: Color::BLACK,
                },
            ));
            builder
                .spawn((ButtonBundle {
                    style: Style {
                        // horizontally center child text
                        padding: UiRect {
                            left: Val::Px(15.0),
                            right: Val::Px(15.0),
                            top: Val::Px(15.0),
                            bottom: Val::Px(15.0),
                        },
                        justify_content: JustifyContent::SpaceEvenly,
                        // vertically center child text
                        align_items: AlignItems::Stretch,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                },))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Replay",
                        TextStyle {
                            font: font.clone(),
                            font_size: 42.0,
                            color: Color::WHITE,
                        },
                    ));
                });

            builder
                .spawn((ButtonBundle {
                    style: Style {
                        // horizontally center child text
                        padding: UiRect {
                            left: Val::Px(15.0),
                            right: Val::Px(15.0),
                            top: Val::Px(15.0),
                            bottom: Val::Px(15.0),
                        },
                        justify_content: JustifyContent::SpaceEvenly,
                        // vertically center child text
                        align_items: AlignItems::Stretch,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                },))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Continue",
                        TextStyle {
                            font: font.clone(),
                            font_size: 42.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

// From https://github.com/bevyengine/bevy/blob/v0.10.0/examples/ecs/generic_system.rs
fn cleanup_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

fn main() {
    App::new()
        .init_state::<AppState>()
        .init_resource::<Grid>()
        .init_resource::<HasWon>()
        .init_resource::<Score>()
        .init_resource::<HighScore>()
        .add_event::<ScoreEvent>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ShadowMitia's 2048".into(),
                resolution: (WINDOW_SIZE.x, WINDOW_SIZE.y).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, add_score)
        .add_systems(Update, (tween_scale_system, tween_translation_system))
        .add_systems(
            Update,
            (update_tile_graphics, input).run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnEnter(AppState::GameOver), game_over)
        .add_systems(
            OnExit(AppState::GameOver),
            (
                cleanup_system::<GameOverUI>,
                cleanup_system::<Cell>,
                reset_game,
            ),
        )
        .add_systems(OnEnter(AppState::Win), (win_screen,))
        .add_systems(OnExit(AppState::Win), cleanup_system::<WinUI>)
        .add_systems(Update, button_system)
        .run();
}

fn tween_translation_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TweenTranslation, &mut Transform)>,
    time: Res<Time>,
) {
    let time = time.elapsed_seconds();
    for (entity, mut tween, mut trans) in query.iter_mut() {
        if tween.completed {
            continue;
        }
        if tween._starttime.is_none() {
            tween._starttime = Some(time);
        }

        let delta = time - tween._starttime.unwrap();
        let t = delta % tween.duration;
        let t = t / tween.duration;

        if delta >= tween.duration {
            trans.translation = tween.from.lerp(tween.to, 1.0);
            tween.completed = true;
        } else {
            trans.translation = tween.from.lerp(tween.to, t);
        }

        if tween.completed {
            commands.entity(entity).remove::<TweenTranslation>();
        }
    }
}

fn tween_scale_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TweenScale, &mut Transform)>,
    time: Res<Time>,
) {
    let time = time.elapsed_seconds();
    for (entity, mut tween, mut trans) in query.iter_mut() {
        if tween.completed {
            continue;
        }
        if tween._starttime.is_none() {
            tween._starttime = Some(time);
        }

        let delta = time - tween._starttime.unwrap();
        let t = delta % tween.duration;
        let t = t / tween.duration;

        if delta >= tween.duration {
            trans.scale = tween.from.lerp(tween.to, 1.0);
            tween.completed = true;
        } else {
            trans.scale = tween.from.lerp(tween.to, t);
        }

        if tween.completed {
            commands.entity(entity).remove::<TweenScale>();
        }
    }
}
