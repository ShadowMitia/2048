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
struct Score {
    current: u32,
    highscore: u32,
}

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

fn setup_score(mut score: ResMut<Score>) {
    use directories::ProjectDirs;

    if let Some(proj_dirs) = ProjectDirs::from("eu", "shadowmitia", "2048") {
        let highscore = proj_dirs.data_dir().join("highscore.txt");
        // TODO: robustness
        if dbg!(highscore.exists()) {
            score.highscore = std::fs::read_to_string(highscore).unwrap().parse().unwrap();
        }
    }
}

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
        .spawn((NodeBundle {
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
        },))
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

enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

struct TouchTracking {
    id: Option<u64>,
    start: Option<Vec2>,
    end: Option<Vec2>,
}

fn input(
    input: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    touches: Res<Touches>,
    mut mouse_coords: Local<Vec2>,
    mut current_touch: Local<Option<TouchTracking>>,
    mut grid: ResMut<Grid>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Cell, &Transform)>,
    text_style: Res<GameStyle>,
    mut next_state: ResMut<NextState<AppState>>,
    mut has_won: ResMut<HasWon>,
    mut score_events: EventWriter<ScoreEvent>,
    mut gizmos: Gizmos,
) {
    let mut released = false;

    if buttons.just_released(MouseButton::Left) {
        let window = window.single();
        let (camera, camera_transform) = camera.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            *mouse_coords = world_position;
        }
        if let Some(current) = &mut *current_touch {
            current.end = Some(*mouse_coords);
        }
        released = true;
    }

    let mut debug_end = None;

    if buttons.pressed(MouseButton::Left) {
        let window = window.single();
        let (camera, camera_transform) = camera.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            *mouse_coords = world_position;
        }
        if current_touch.is_none() {
            *current_touch = Some(TouchTracking {
                id: None,
                start: Some(*mouse_coords),
                end: None,
            });
        } else {
            debug_end = Some(*mouse_coords);
        }
    }

    const PI: f32 = std::f32::consts::PI;

    let up_start = 3.0 * PI / 4.0;
    let up_end = PI / 4.0;

    let down_start = -3.0 * PI / 4.0;
    let down_end = -PI / 4.0;

    let left_start = 3.0 * PI / 4.0;
    let left_end = -3.0 * PI / 4.0;

    let right_start = PI / 4.0;
    let right_end = -PI / 4.0;

    /*
                    Some(MoveDirection::Left) => Color::GREEN,
                      Some(MoveDirection::Right) => Color::BLUE,
                      Some(MoveDirection::Up) => Color::PURPLE,
    Some(MoveDirection::Down) => Color::ORANGE,
    */
    /*
         gizmos.arc_2d(Vec2::ZERO, up_start, PI / 2.0, 75.0, Color::PURPLE);
         gizmos.arc_2d(Vec2::ZERO, down_start, PI / 2.0, 75.0, Color::ORANGE);
         gizmos.arc_2d(Vec2::ZERO, -left_start, PI / 2.0, 75.0, Color::GREEN);
         gizmos.arc_2d(Vec2::ZERO, right_start, PI / 2.0, 75.0, Color::BLUE);
    */

    let move_direction = {
        for touch in touches.iter_just_pressed() {
            *current_touch = Some(TouchTracking {
                id: Some(touch.id()),
                start: Some(touch.position()),
                end: None,
            });
            // Grab first touch and use that
            break;
        }
        if let Some(current_touch) = &mut *current_touch {
            for touch in touches.iter_just_released() {
                if current_touch.id == Some(touch.id()) {
                    current_touch.end = Some(touch.position());
                    released = true;
                } else {
                    continue;
                }
            }
        };

        let dir = {
            if let Some(current) = &*current_touch {
                if current.end.is_some() {
                    let diff = current.start.unwrap() - current.end.unwrap();
                    let diff = diff.normalize_or_zero();

                    if diff != Vec2::ZERO {
                        // https://stackoverflow.com/questions/34658253/for-the-point-inside-circle-find-in-which-quarter-it-is
                        if diff.y > 0.0 && diff.x.abs() < diff.y {
                            // TODO: Why flipped on web?
                            if current.id.is_none() {
                                Some(MoveDirection::Down)
                            } else {
                                Some(MoveDirection::Up)
                            }
                        } else if diff.y < 0.0 && diff.x.abs() < -diff.y {
                            // TODO: Why flipped on web?
                            if current.id.is_none() {
                                Some(MoveDirection::Up)
                            } else {
                                Some(MoveDirection::Down)
                            }
                        } else if diff.x < 0.0 && diff.y.abs() < -diff.x {
                            Some(MoveDirection::Right)
                        } else if diff.x > 0.0 && diff.y.abs() < diff.x {
                            Some(MoveDirection::Left)
                        } else {
                            None
                        }
                    } else {
                        None
                    }

                    /*
                        if diff.length() > 50.0 {
                            let angle = diff.to_angle();
                            if angle < up_start && angle > up_end {
                                Some(MoveDirection::Up)
                            } else if angle < down_start && angle >= down_end {
                                Some(MoveDirection::Down)
                            } else if angle >= left_start && angle >= left_end {
                                Some(MoveDirection::Left)
                            } else if angle <= right_start && angle <= right_end {
                                Some(MoveDirection::Right)
                            } else {
                                None
                            }
                        } else {
                            None
                    }
                      */
                } else {
                    None
                }
            } else {
                None
            }
        };

        #[cfg(debug_assertions)]
        if let Some(current) = &*current_touch {
            if let Some(start) = current.start {
                gizmos.circle_2d(start, 50.0, Color::RED);
            }
            if let Some(end) = debug_end {
                let color = match dir {
                    Some(MoveDirection::Left) => Color::GREEN,
                    Some(MoveDirection::Right) => Color::BLUE,
                    Some(MoveDirection::Up) => Color::PURPLE,
                    Some(MoveDirection::Down) => Color::ORANGE,
                    None => Color::RED,
                };
                gizmos.circle_2d(end, 50.0, color);
            }
        }

        if released {
            *current_touch = None;
        }

        if dir.is_some() {
            dir
        } else {
            if input.just_pressed(KeyCode::ArrowLeft) {
                Some(MoveDirection::Left)
            } else if input.just_pressed(KeyCode::ArrowRight) {
                Some(MoveDirection::Right)
            } else if input.just_pressed(KeyCode::ArrowUp) {
                Some(MoveDirection::Up)
            } else if input.just_pressed(KeyCode::ArrowDown) {
                Some(MoveDirection::Down)
            } else {
                None
            }
        }
    };

    if move_direction.is_none() {
        return;
    }

    let (moved, score) = match move_direction.unwrap() {
        MoveDirection::Left => grid.move_left(),
        MoveDirection::Right => grid.move_right(),
        MoveDirection::Up => grid.move_up(),
        MoveDirection::Down => grid.move_down(),
    };

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

    if !moved.is_empty() {
        if !add_tile(&mut commands, &mut grid, &text_style.0) && !grid.has_legal_move() {
            next_state.set(AppState::GameOver);
        }
    }
}

fn add_score(
    mut ev_score: EventReader<ScoreEvent>,
    mut score: ResMut<Score>,
    mut score_ui: Query<&mut Text, With<ScoreUI>>,
    mut high_score_ui: Query<&mut Text, (With<HighScoreUI>, Without<ScoreUI>)>,
) {
    for ev in ev_score.read() {
        score.current += ev.0;
    }

    if let Ok(mut score_ui) = score_ui.get_single_mut() {
        score_ui.sections[0].value = format!("Score {}", score.current);

        //
        {
            if score.current > score.highscore {
                score.highscore = score.current;
            }

            let mut score_ui = high_score_ui.get_single_mut().unwrap();
            score_ui.sections[0].value = format!("High score {}", score.highscore);

            use directories::ProjectDirs;

            if let Some(proj_dirs) = ProjectDirs::from("eu", "shadowmitia", "2048") {
                let highscore = proj_dirs.data_dir().join("highscore.txt");
                std::fs::write(highscore, score.highscore.to_string());
            }
        }
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

    score.current = 0;
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
                    flex_basis: Val::Auto,
                    align_content: AlignContent::Stretch,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
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
                    flex_basis: Val::Auto,
                    align_content: AlignContent::Stretch,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
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
        .add_event::<ScoreEvent>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ShadowMitia's 2048".into(),
                resolution: (WINDOW_SIZE.x, WINDOW_SIZE.y).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_score, setup))
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
