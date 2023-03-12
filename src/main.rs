use bevy::prelude::*;
use rand::prelude::*;

const CELL_SIZE: Vec2 = Vec2::new(200.0, 200.0);
const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 800.0);

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
enum AppState {
    GameOver,
    Win,
    #[default]
    InGame,
}

impl States for AppState {
    type Iter = std::array::IntoIter<AppState, 3>;

    fn variants() -> Self::Iter {
        [AppState::Win, AppState::GameOver, AppState::InGame].into_iter()
    }
}

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

#[derive(Resource, Copy, Clone)]
struct Grid {
    cells: [usize; 16],
}

impl Grid {
    #[must_use]
    fn new() -> Self {
        Self { cells: [0; 16] }
    }

    #[must_use]
    fn add_random_tile(&mut self) -> Option<UVec2> {
        let mut rng = thread_rng();
        let mut empty_cells = self
            .cells
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if c == 0 { Some(i) } else { None })
            .collect::<Vec<usize>>();
        if empty_cells.is_empty() {
            return None;
        }
        empty_cells.shuffle(&mut rng);

        self.cells[empty_cells[0]] = if rng.gen::<f32>() < 0.9 { 2 } else { 4 };
        let index = empty_cells[0];

        return Some(UVec2::new((index % 4) as u32, (index / 4) as u32));
    }

    #[must_use]
    fn move_left(&mut self) -> (Vec<(UVec2, UVec2)>, usize) {
        let mut moved = Vec::new();
        let mut score = 0;
        let mut has_merged = [false; 16];
        for j in 0..4 {
            for i in 1..4 {
                let from = UVec2::new(i, j);
                let prev = Self::index_2d(i as usize, j as usize, 4, 4);
                if self.cells[prev] == 0 {
                    continue;
                }
                let mut furthest = None;
                // TODO: can be simplified
                for k in 1..=i {
                    let index = Self::index_2d((i - k) as usize, j as usize, 4, 4);
                    if self.cells[index] == 0 || self.cells[index] == self.cells[prev] {
                        furthest = Some(index);
                    } else {
                        break;
                    }
                }
                if let Some(index) = furthest {
                    if !has_merged[index] {
                        if self.cells[index] == self.cells[prev] {
                            self.cells[index] += self.cells[prev];
                            score += self.cells[index];
                            has_merged[index] = true;
                        } else {
                            self.cells[index] = self.cells[prev];
                        }
                        self.cells[prev] = 0;
                        moved.push((from, UVec2::new((index % 4) as u32, (index / 4) as u32)));
                    }
                }
            }
        }
        (moved, score)
    }

    #[must_use]
    fn move_right(&mut self) -> (Vec<(UVec2, UVec2)>, usize) {
        let mut moved = Vec::new();
        let mut score = 0;
        let mut has_merged = [false; 16];
        for j in 0..4 {
            for i in (0..=2).rev() {
                let from = UVec2::new(i, j);
                let prev = Self::index_2d(i as usize, j as usize, 4, 4);
                if self.cells[prev] == 0 {
                    continue;
                }
                let mut furthest = None;
                // TODO: can be simplified
                for k in 1..=(3 - i) {
                    let index = Self::index_2d((i + k) as usize, j as usize, 4, 4);
                    if self.cells[index] == 0 || self.cells[index] == self.cells[prev] {
                        furthest = Some(index);
                    } else {
                        break;
                    }
                }
                if let Some(index) = furthest {
                    if !has_merged[index] {
                        if self.cells[index] == self.cells[prev] {
                            self.cells[index] += self.cells[prev];
                            score += self.cells[index];
                            has_merged[index] = true;
                        } else {
                            self.cells[index] = self.cells[prev];
                        }
                        self.cells[prev] = 0;
                        moved.push((from, UVec2::new((index % 4) as u32, (index / 4) as u32)));
                    }
                }
            }
        }
        (moved, score)
    }

    #[must_use]
    fn move_down(&mut self) -> (Vec<(UVec2, UVec2)>, usize) {
        let mut moved = Vec::new();
        let mut score = 0;
        let mut has_merged = [false; 16];
        for j in 1..4 {
            for i in 0..4 {
                let from = UVec2::new(i, j);
                let prev = Self::index_2d(i as usize, j as usize, 4, 4);
                if self.cells[prev] == 0 {
                    continue;
                }
                let mut furthest = None;
                // TODO: can be simplified
                for k in 1..=j {
                    let index = Self::index_2d(i as usize, (j - k) as usize, 4, 4);
                    if self.cells[index] == 0 || self.cells[index] == self.cells[prev] {
                        furthest = Some(index);
                    } else {
                        break;
                    }
                }
                if let Some(index) = furthest {
                    if !has_merged[index] {
                        if self.cells[index] == self.cells[prev] {
                            self.cells[index] += self.cells[prev];
                            score += self.cells[index];
                            has_merged[index] = true;
                        } else {
                            self.cells[index] = self.cells[prev];
                        }
                        self.cells[prev] = 0;
                        moved.push((from, UVec2::new((index % 4) as u32, (index / 4) as u32)));
                    }
                }
            }
        }
        (moved, score)
    }

    #[must_use]
    fn move_up(&mut self) -> (Vec<(UVec2, UVec2)>, usize) {
        let mut moved = Vec::new();
        let mut score = 0;
        let mut has_merged = [false; 16];
        for j in (0..=2).rev() {
            for i in 0..4 {
                let from = UVec2::new(i, j);
                let prev = Self::index_2d(i as usize, j as usize, 4, 4);
                if self.cells[prev] == 0 {
                    continue;
                }
                let mut furthest = None;
                // TODO: can be simplified
                for k in 1..=(3 - j) {
                    let index = Self::index_2d(i as usize, (j + k) as usize, 4, 4);
                    if self.cells[index] == 0 || self.cells[index] == self.cells[prev] {
                        furthest = Some(index);
                    } else {
                        break;
                    }
                }
                if let Some(index) = furthest {
                    if !has_merged[index] {
                        if self.cells[index] == self.cells[prev] {
                            self.cells[index] += self.cells[prev];
                            score += self.cells[index];
                            has_merged[index] = true;
                        } else {
                            self.cells[index] = self.cells[prev];
                        }
                        self.cells[prev] = 0;
                        moved.push((from, UVec2::new((index % 4) as u32, (index / 4) as u32)));
                    }
                }
            }
        }
        (moved, score)
    }

    #[must_use]
    fn index_2d(i: usize, j: usize, w: usize, _h: usize) -> usize {
        j * w + i
    }

    // TODO: coord to index

    fn has_legal_move(&self) -> bool {
        let mut grid = *self;

        let (up, _) = grid.move_up();
        let (down, _) = grid.move_down();
        let (left, _) = grid.move_left();
        let (right, _) = grid.move_right();

        return up.len() > 0 || down.len() > 0 || left.len() > 0 || right.len() > 0;
    }
}

#[cfg(test)]
mod grid_tests {

    use super::*;

    #[test]
    fn move_left_simple() {
        let mut grid = Grid::new();
        #[rustfmt::skip]
      let test = [0, 2, 0, 0,
                  2, 0, 0, 0,
                  0, 0, 2, 0,
                  0, 0, 0, 2];

        grid.cells = test;

        #[rustfmt::skip]
        let res = [2, 0, 0, 0,
                    2, 0, 0, 0,
                    2, 0, 0, 0,
                    2, 0, 0, 0];

        let _ = grid.move_left();

        assert_eq!(grid.cells, res);
    }

    #[test]
    fn move_left_four_in_a_row() {
        let mut grid = Grid::new();
        #[rustfmt::skip]
      let test = [2,  2,  2,  2,
                             4,  4,  4,  4,
                             8,  8,  8,  8,
                            16, 16, 16, 16];

        grid.cells = test;

        #[rustfmt::skip]
        let res = [4, 4, 0, 0,
                   8, 8, 0, 0,
                    16, 16, 0, 0,
                    32, 32, 0, 0];

        let _ = grid.move_left();

        assert_eq!(grid.cells, res);
    }

    #[test]
    fn move_left_more() {
        let mut grid = Grid::new();
        #[rustfmt::skip]
      let test = [0, 2, 0, 4,
                  2, 4, 0, 0,
                  4, 0, 2, 0,
                  4, 0, 0, 2];

        grid.cells = test;

        #[rustfmt::skip]
        let res = [2, 4, 0, 0,
                    2, 4, 0, 0,
                    4, 2, 0, 0,
                    4, 2, 0, 0];

        let _ = grid.move_left();

        assert_eq!(grid.cells, res);
    }

    #[test]
    fn move_left_no_move() {
        let mut grid = Grid::new();
        #[rustfmt::skip]
      let test = [8, 2, 16, 4,
                  2, 4, 16, 8,
                  4, 256, 2, 256,
                  4, 16, 8, 16];

        grid.cells = test;

        let _ = grid.move_left();

        assert_eq!(grid.cells, test);
    }

    #[test]
    fn move_right_simple() {
        let mut grid = Grid::new();
        #[rustfmt::skip]
      let test = [0, 2, 0, 0,
                  2, 0, 0, 0,
                  0, 0, 2, 0,
                  0, 0, 0, 2];

        grid.cells = test;

        #[rustfmt::skip]
        let res = [0, 0, 0, 2,
                    0, 0, 0, 2,
                    0, 0, 0, 2,
                    0, 0, 0, 2];

        let _ = grid.move_right();

        assert_eq!(grid.cells, res);
    }

    #[test]
    fn move_right_four_in_a_row() {
        let mut grid = Grid::new();
        #[rustfmt::skip]
      let test = [2,  2,  2,  2,
                             4,  4,  4,  4,
                             8,  8,  8,  8,
                            16, 16, 16, 16];

        grid.cells = test;

        #[rustfmt::skip]
        let res = [ 0, 0,4, 4,
                               0, 0, 8, 8,
                               0, 0, 16, 16,
                               0, 0, 32, 32,];

        let _ = grid.move_right();

        assert_eq!(grid.cells, res);
    }

    #[test]
    fn move_right_more() {
        let mut grid = Grid::new();
        #[rustfmt::skip]
      let test = [0, 2, 0, 4,
                  2, 4, 0, 0,
                  4, 0, 2, 0,
                  4, 0, 0, 2];

        grid.cells = test;

        #[rustfmt::skip]
        let res = [ 0, 0, 2, 4,
                               0, 0, 2, 4,
                               0, 0, 4, 2,
                               0, 0, 4, 2,];

        let _ = grid.move_right();

        assert_eq!(grid.cells, res);
    }

    #[test]
    fn move_right_no_move() {
        let mut grid = Grid::new();
        #[rustfmt::skip]
      let test = [8, 2, 16, 4,
                  2, 4, 16, 8,
                  4, 256, 2, 256,
                  4, 16, 8, 16];

        grid.cells = test;

        let _ = grid.move_right();

        assert_eq!(grid.cells, test);
    }

    #[test]
    fn move_down_simple() {
        let mut grid = Grid::new();
        #[rustfmt::skip]
      let test = [0, 2, 0, 0,
                  2, 0, 0, 0,
                  0, 0, 2, 0,
                  0, 0, 0, 2];

        grid.cells = test;

        #[rustfmt::skip]
        let res = [2, 2, 2, 2,
                    0, 0, 0, 0,
                    0, 0, 0, 0,
                    0, 0, 0, 0];

        let _ = grid.move_down();

        assert_eq!(grid.cells, res);
    }

    #[test]
    fn move_down_four_in_a_row() {
        let mut grid = Grid::new();

        #[rustfmt::skip]
      let test = [2, 4, 8, 16,
                             2, 4, 8, 16,
                             2, 4, 8, 16,
                             2, 4, 8, 16];

        grid.cells = test;

        #[rustfmt::skip]
        let res = [ 4, 8, 16, 32,
                               4, 8, 16, 32,
                               0, 0,  0,  0,
                               0, 0,  0,  0];

        let _ = grid.move_down();

        assert_eq!(grid.cells, res);
    }

    #[test]
    fn move_down_more() {
        let mut grid = Grid::new();
        #[rustfmt::skip]
      let test =       [
        0, 2, 4, 4,
        2, 4, 0, 0,
        0, 0, 2, 0,
        4, 0, 0, 2
      ];

        grid.cells = test;

        #[rustfmt::skip]
        let res = [2, 2, 4, 4,
                    4, 4, 2, 2,
                    0, 0, 0, 0,
                    0, 0, 0, 0];

        let _ = grid.move_down();

        assert_eq!(grid.cells, res);
    }

    #[test]
    fn move_down_no_move() {
        let mut grid = Grid::new();
        #[rustfmt::skip]
      let test = [8, 2, 8, 4,
                  2, 4, 16, 8,
                  4, 256, 2, 256,
                  8, 16, 8, 16];

        grid.cells = test;

        let _ = grid.move_down();

        assert_eq!(grid.cells, test);
    }

    #[test]
    fn move_up_simple() {
        let mut grid = Grid::new();
        #[rustfmt::skip]
      let test = [0, 2, 0, 0,
                  2, 0, 0, 0,
                  0, 0, 2, 0,
                  0, 0, 0, 2];

        grid.cells = test;

        #[rustfmt::skip]
        let res = [0, 0, 0, 0,
                    0, 0, 0, 0,
                    0, 0, 0, 0,
                    2, 2, 2, 2];

        let _ = grid.move_up();

        assert_eq!(grid.cells, res);
    }

    #[test]
    fn move_up_four_in_a_row() {
        let mut grid = Grid::new();
        #[rustfmt::skip]
      let test = [2, 4, 8, 16,
                             2, 4, 8, 16,
                             2, 4, 8, 16,
                             2, 4, 8, 16];

        grid.cells = test;

        #[rustfmt::skip]
      let res = [
        0, 0,  0,  0,
        0, 0,  0,  0,
        4, 8, 16, 32,
        4, 8, 16, 32,
                               ];

        let _ = grid.move_up();

        assert_eq!(grid.cells, res);
    }

    #[test]
    fn move_up_more() {
        let mut grid = Grid::new();

        #[rustfmt::skip]
      let test = [0, 2, 4, 4,
                  2, 4, 0, 0,
                  0, 0, 2, 0,
                  4, 0, 0, 2];

        grid.cells = test;

        #[rustfmt::skip]
      let res = [
        0, 0, 0, 0,
        0, 0, 0, 0,
        2, 2, 4, 4,
        4, 4, 2, 2];
        let _ = grid.move_up();

        assert_eq!(grid.cells, res);
    }

    #[test]
    fn move_up_no_move() {
        let mut grid = Grid::new();
        #[rustfmt::skip]
      let test = [8, 2, 8, 4,
                  2, 4, 16, 8,
                  4, 256, 2, 256,
                  8, 16, 8, 16];

        grid.cells = test;

        let _ = grid.move_up();

        assert_eq!(grid.cells, test);
    }
}

#[derive(Component)]
struct TweenTranslation {
    duration: f32,
    to: Vec3,
    from: Vec3,
    completed: bool,
    // private
    _starttime: Option<f32>,
}

#[derive(Component)]
struct TweenScale {
    duration: f32,
    to: Vec3,
    from: Vec3,
    completed: bool,
    // private
    _starttime: Option<f32>,
}

#[derive(Component)]
struct Cell {
    coord: UVec2,
}

fn lerp(from: f32, to: f32, t: f32) -> f32 {
    (1.0 - t) * from + t * to
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

#[must_use]
fn scale_tween(duration: f32, from: Vec3, to: Vec3) -> TweenScale {
    TweenScale {
        duration,
        to,
        from,
        completed: false,
        _starttime: None,
    }
}

#[must_use]
fn translation_tween(duration: f32, from: Vec3, to: Vec3) -> TweenTranslation {
    TweenTranslation {
        duration,
        to,
        from,
        completed: false,
        _starttime: None,
    }
}

#[must_use]
fn coord_to_position(v: Vec3) -> Vec3 {
    let mut transform = Vec3::new(0.0, 0.0, 0.0);
    transform.x += CELL_SIZE.x * (v.x as f32) - WINDOW_SIZE.x / 2.0 + CELL_SIZE.x / 2.0;
    transform.y += CELL_SIZE.y * (v.y as f32) - WINDOW_SIZE.y / 2.0 + CELL_SIZE.y / 2.0;
    transform.z = v.z;
    transform
}

#[must_use]
fn add_tile(commands: &mut Commands, grid: &mut Grid, text_style: &TextStyle) -> bool {
    if let Some(UVec2 { x: i, y: j }) = grid.add_random_tile() {
        let score = grid.cells[Grid::index_2d(i as usize, j as usize, 4, 4) as usize] as u32;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: score_to_colour(score),
                    custom_size: Some(CELL_SIZE),
                    ..Default::default()
                },
                transform: Transform {
                    translation: coord_to_position(Vec3::new(i as f32, j as f32, 0 as f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            scale_tween(0.2, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 1.0, 1.0)),
            Cell {
                coord: UVec2 { x: i, y: j },
            },
        ));

        let c = coord_to_position(Vec3::new(i as f32, j as f32, 0.));

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(score.to_string(), text_style.clone())
                    .with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(c.x, c.y, 2.0),
                ..Default::default()
            },
            Cell {
                coord: UVec2 { x: i, y: j },
            },
        ));
        return true;
    }
    return false;
}

#[derive(Resource)]
pub struct GameStyle(pub TextStyle);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_dark: Color = Color::hex("776e65").unwrap();
    let font = asset_server.load("fonts/Kenney Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: text_dark,
    };

    commands.insert_resource(GameStyle(text_style.clone()));

    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn game_setup(
    mut commands: Commands,
    mut sprite_query: Query<Entity, With<Sprite>>,
    mut text_cell_query: Query<Entity, With<Text>>,
    mut grid: ResMut<Grid>,
    text_style: Res<GameStyle>,
) {
    // Cleanup every tile and text
    for t in sprite_query.iter_mut().chain(text_cell_query.iter_mut()) {
        commands.entity(t).despawn_recursive();
    }

    // Add two random tiles to the grid
    for _ in 0..2 {
        let _ = add_tile(&mut commands, &mut grid, &text_style.0);
    }
}

fn input(
    input: Res<Input<KeyCode>>,
    mut grid: ResMut<Grid>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Cell, &Transform)>,
    text_style: Res<GameStyle>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let (moved, score) = {
        if input.just_pressed(KeyCode::Left) {
            grid.move_left()
        } else if input.just_pressed(KeyCode::Right) {
            grid.move_right()
        } else if input.just_pressed(KeyCode::Up) {
            grid.move_up()
        } else if input.just_pressed(KeyCode::Down) {
            grid.move_down()
        } else {
            (vec![], usize::MAX)
        }
    };

    if score == usize::MAX {
        return;
    }

    let empty_cells = grid
        .cells
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == 0 { Some(i) } else { None })
        .count();

    if moved.len() == 0 && empty_cells == 0 && !grid.has_legal_move() {
        next_state.set(AppState::GameOver);
        return;
    }

    for (entity, mut cell, trans) in query.iter_mut() {
        if let Some((from_cell, to_cell)) = moved.iter().find(|(f, _)| f == &cell.coord) {
            let from = coord_to_position(Vec3::new(
                from_cell.x as f32,
                from_cell.y as f32,
                trans.translation.z,
            ));
            let to = coord_to_position(Vec3::new(
                to_cell.x as f32,
                to_cell.y as f32,
                trans.translation.z,
            ));
            commands
                .entity(entity)
                .insert(translation_tween(0.2, from, to));

            cell.coord.x = to_cell.x as u32;
            cell.coord.y = to_cell.y as u32;
        } else {
            // panic!("Should never happen");
        }
    }

    if !add_tile(&mut commands, &mut grid, &text_style.0) {
        println!("Game Over");
        next_state.set(AppState::GameOver);
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

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn reset_game(
    mut commands: &mut Commands,
    mut grid: &mut Grid,
    next_state: &mut bevy::prelude::NextState<AppState>,
    text_style: &GameStyle,
) {
    next_state.set(AppState::InGame);

    *grid = Grid::new();

    for _ in 0..2 {
        let _ = add_tile(&mut commands, &mut grid, &text_style.0);
    }
}

#[derive(Debug)]
enum Action {
    Reset,
    Continue,
}

#[derive(Component, Debug)]
struct ButtonAction(Action);

fn button_system(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    mut next_state: ResMut<NextState<AppState>>,
    text_style: Res<GameStyle>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                println!("clicked {:?}", action);
                match action {
                    ButtonAction(Action::Reset) => {
                        println!("reset");
                        reset_game(&mut commands, &mut *grid, &mut next_state, &text_style)
                    }
                    ButtonAction(Action::Continue) => {
                        println!("continue");
                        next_state.set(AppState::InGame)
                    }
                }
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

fn game_over(mut commands: Commands, text_style: Res<GameStyle>, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Kenney Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // fill the entire window
                    size: Size::all(Val::Percent(100.)),
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
                .spawn((
                    ButtonBundle {
                        style: Style {
                            //                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
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
                    },
                    ButtonAction(Action::Reset),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        TextBundle::from_section(
                            "Replay",
                            TextStyle {
                                font: font.clone(),
                                font_size: 42.0,
                                color: Color::WHITE,
                            },
                        ),
                        ButtonAction(Action::Reset),
                    ));
                });
        });
}

#[derive(Component)]
struct WinUI;

fn win_screen(mut commands: Commands, text_style: Res<GameStyle>, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Kenney Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // fill the entire window
                    size: Size::all(Val::Percent(100.)),
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
                .spawn((
                    ButtonBundle {
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
                    },
                    ButtonAction(Action::Reset),
                ))
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
                .spawn((
                    ButtonBundle {
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
                    },
                    ButtonAction(Action::Continue),
                ))
                .with_children(|builder| {
                    builder.spawn(
                        (TextBundle::from_section(
                            "Continue",
                            TextStyle {
                                font: font.clone(),
                                font_size: 42.0,
                                color: Color::WHITE,
                            },
                        )),
                    );
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
        .insert_resource(Grid::new())
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ShadowMitia's 2048".into(),
                resolution: (WINDOW_SIZE.x, WINDOW_SIZE.y).into(),
                ..default()
            }),
            ..default()
        }))
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        .add_systems((tween_translation_system, tween_scale_system))
        .add_systems((
            game_setup.in_schedule(OnEnter(AppState::InGame)),
            update_tile_graphics.in_set(OnUpdate(AppState::InGame)),
            input.in_set(OnUpdate(AppState::InGame)),
        ))
        .add_systems((
            game_over.in_schedule(OnEnter(AppState::GameOver)),
            cleanup_system::<GameOverUI>.in_schedule(OnExit(AppState::GameOver)),
            button_system.in_set(OnUpdate(AppState::GameOver)),
        ))
        .add_systems((
            win_screen.in_schedule(OnEnter(AppState::Win)),
            cleanup_system::<WinUI>.in_schedule(OnExit(AppState::Win)),
            button_system.in_set(OnUpdate(AppState::Win)),
        ))
        .run();
}
