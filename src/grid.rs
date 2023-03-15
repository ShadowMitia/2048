use bevy::prelude::*;
use rand::prelude::*;

#[derive(Resource, Default, Copy, Clone)]
pub struct Grid {
    pub cells: [usize; 16],
}

impl Grid {
    #[must_use]
    pub fn new() -> Self {
        default()
    }

    #[must_use]
    pub fn default() -> Self {
        Self { cells: [0; 16] }
    }

    #[must_use]
    pub fn add_random_tile(&mut self) -> Option<UVec2> {
        if !self.has_empty_cells() {
            return None;
        }
        let mut rng = thread_rng();

        let mut empty_cells = self
            .cells
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if c == 0 { Some(i) } else { None })
            .collect::<Vec<usize>>();
        empty_cells.shuffle(&mut rng);

        self.cells[empty_cells[0]] = if rng.gen::<f32>() < 0.9 { 2 } else { 4 };
        let index = empty_cells[0];

        return Some(Grid::index_to_coord(index, 4, 4));
    }

    #[must_use]
    pub fn move_left(&mut self) -> (Vec<(UVec2, UVec2)>, usize) {
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
    pub fn move_right(&mut self) -> (Vec<(UVec2, UVec2)>, usize) {
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
    pub fn move_down(&mut self) -> (Vec<(UVec2, UVec2)>, usize) {
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
    pub fn move_up(&mut self) -> (Vec<(UVec2, UVec2)>, usize) {
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
    pub fn index_2d(i: usize, j: usize, w: usize, _h: usize) -> usize {
        j * w + i
    }

    #[must_use]
    pub fn index_to_coord(index: usize, w: usize, _h: usize) -> UVec2 {
        UVec2::new((index % w) as u32, (index / w) as u32)
    }

    pub fn has_legal_move(&self) -> bool {
        let mut grid = *self;

        let (up, _) = grid.move_up();
        let (down, _) = grid.move_down();
        let (left, _) = grid.move_left();
        let (right, _) = grid.move_right();

        return up.len() > 0 || down.len() > 0 || left.len() > 0 || right.len() > 0;
    }

    pub fn has_empty_cells(&self) -> bool {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if c == 0 { Some(i) } else { None })
            .count()
            != 0
    }

    pub fn max_value(&self) -> usize {
        self.cells.iter().max().cloned().unwrap()
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
