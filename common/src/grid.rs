// New Position type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    // Move in a direction
    pub fn step(&self, direction: Direction) -> Self {
        let (dx, dy) = direction.delta();
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    // Get row/col indices (matching array indexing order [y][x])
    pub fn row_col(&self) -> Option<(usize, usize)> {
        if self.x < 0 || self.y < 0 {
            None
        } else {
            Some((self.y as usize, self.x as usize))
        }
    }
}

impl From<(i64, i64)> for Position {
    fn from((x, y): (i64, i64)) -> Self {
        Self::new(x, y)
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x as i64, y as i64)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    // Type-safe index for use in arrays
    pub fn index(&self) -> usize {
        *self as usize
    }

    pub const ALL: [Direction; 4] = [
        Direction::NORTH,
        Direction::EAST,
        Direction::SOUTH,
        Direction::WEST,
    ];

    pub fn delta(&self) -> (i64, i64) {
        match self {
            Direction::NORTH => (0, -1),
            Direction::EAST => (1, 0),
            Direction::SOUTH => (0, 1),
            Direction::WEST => (-1, 0),
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }
}

// Update cell_at to work with Position
pub fn cell_at(grid: &[Vec<char>], pos: Position) -> Option<char> {
    let (row, col) = pos.row_col()?;
    grid.get(row)?.get(col).copied()
}
