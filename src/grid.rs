use std::collections::HashMap;

pub struct Grid<T> {
    pub elements: HashMap<(usize, usize), T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(elements: HashMap<(usize, usize), T>, width: usize, height: usize) -> Self {
        Grid { elements, width, height }
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator {
            grid: self,
            current_row: 0,
            current_col: 0,
        }
    }

    pub fn parse(lines: &[String], transform: fn(char) -> T) -> Grid<T> {
        let mut elements = HashMap::new();
        lines.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                elements.insert((x, y), transform(c));
            });
        });
        Grid { elements, width: lines[0].len(), height: lines.len() }
    }

    pub fn get(&self, pos: (usize, usize)) -> Option<&T> {
        self.elements.get(&pos)    }

    pub fn adjacent(&self, (x, y): (usize, usize), diagonal: bool) -> Vec<((usize, usize), &T)> {
        let mut neighbors = Vec::new();
        let mut offsets = vec![
            (0, 1),  // Down
            (0, -1), // Up
            (1, 0),  // Right
            (-1, 0), // Left
        ];

        if diagonal {
            offsets.extend_from_slice(&[
                (-1, -1), // Top-left
                (-1, 1),  // Top-right
                (1, -1),  // Bottom-left
                (1, 1),   // Bottom-right
            ]);
        }

        for (dx, dy) in offsets {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            if let Some(value) = self.get((nx, ny)) {
                neighbors.push(((nx, ny), value));
            }
        }

        neighbors
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    current_row: usize,
    current_col: usize,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = ((usize, usize), Option<&'a T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row >= self.grid.height {
            return None;
        }

        let key = (self.current_col, self.current_row);
        let value = self.grid.get((self.current_col, self.current_row));

        self.current_col += 1;
        if self.current_col >= self.grid.width {
            self.current_col = 0;
            self.current_row += 1;
        }

        Some((key, value))
    }
}

