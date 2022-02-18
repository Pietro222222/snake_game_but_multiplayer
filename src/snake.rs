use crate::apple::Apple;
use crate::constants::*;
use crate::grid::*;
use crate::utils::Coord;

pub struct SnakePiece /* as you can see, im not good with names*/ {
    pub coord: Coord,
    pub head: bool,
}

impl SnakePiece {
    pub fn new(c: Coord, head: bool) -> Self {
        Self {
            coord: c,
            head: head,
        }
    }
    pub fn append(&self, other: &mut SnakePiece) {
        other.coord.height = self.coord.height;
        other.coord.width = self.coord.width - 1;
    }
}

pub enum SnakeDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq)]
pub enum SnakePlayers {
    Player1, // ====@
    Player2, // ++++0
}

pub struct Snake {
    pub direction: SnakeDirection,
    pub pieces: Vec<SnakePiece>,
    pub player: SnakePlayers,
}

impl Snake {
    pub fn new(player: SnakePlayers) -> Self {
        let head = if player == SnakePlayers::Player1 {
            SnakePiece::new(Coord::new(0, GRID_WIDTH / 2), true)
        } else {
            SnakePiece::new(Coord::new(GRID_HEIGHT - 1, GRID_WIDTH / 2), true)
        };
        let mut body = SnakePiece::new(Coord::new(0, 0), false);
        head.append(&mut body);
        let pieces = vec![head, body];
        Self {
            direction: SnakeDirection::Right,
            pieces: pieces,
            player: player,
        }
    }
    pub fn append_piece(&mut self, mut piece: SnakePiece) {
        let last_piece = self.pieces.get(self.pieces.len() - 1).unwrap();
        last_piece.append(&mut piece);
        self.pieces.push(piece)
    }
    pub fn add_new_piece(&mut self) {
        self.append_piece(SnakePiece::new(Coord::new(0, 0), false));
    }

    fn move_up(&mut self) {
        let mut last_coord = Coord::new(0, 0);
        let mut is_da_head = false;
        for i in &mut self.pieces {
            if (!is_da_head) {
                is_da_head = true;
                last_coord = i.coord.to_owned();
                if (i.coord.height <= 0) {
                    i.coord.height = GRID_HEIGHT - 1;
                } else {
                    i.coord.height -= 1;
                }
            } else {
                let mut tmp_coord = last_coord.clone();
                last_coord = i.coord.to_owned();
                i.coord.height = tmp_coord.height;
                i.coord.width = tmp_coord.width;
            }
        }
    }

    fn move_down(&mut self) {
        let mut last_coord = Coord::new(0, 0);
        let mut is_da_head = false;
        for i in &mut self.pieces {
            if (!is_da_head) {
                is_da_head = true;
                last_coord = i.coord.to_owned();
                if (i.coord.height >= GRID_HEIGHT - 1) {
                    i.coord.height = 0;
                } else {
                    i.coord.height += 1;
                }
            } else {
                let mut tmp_coord = last_coord.clone();
                last_coord = i.coord.to_owned();
                i.coord.height = tmp_coord.height;
                i.coord.width = tmp_coord.width;
            }
        }
    }

    fn move_right(&mut self) {
        let mut last_coord = Coord::new(0, 0);
        let mut is_da_head = false;
        for i in &mut self.pieces {
            if (!is_da_head) {
                is_da_head = true;
                last_coord = i.coord.to_owned();
                if (i.coord.width >= GRID_WIDTH - 1) {
                    i.coord.width = 0;
                } else {
                    i.coord.width += 1;
                }
            } else {
                let mut tmp_coord = last_coord.clone();
                last_coord = i.coord.to_owned();
                i.coord.height = tmp_coord.height;
                i.coord.width = tmp_coord.width;
            }
        }
    }

    fn move_left(&mut self) {
        let mut last_coord = Coord::new(0, 0);
        let mut is_da_head = false;
        for i in &mut self.pieces {
            if (!is_da_head) {
                is_da_head = true;
                last_coord = i.coord.to_owned();
                if (i.coord.width <= 0) {
                    i.coord.width = GRID_WIDTH - 1;
                } else {
                    i.coord.width -= 1;
                }
            } else {
                let mut tmp_coord = last_coord.clone();
                last_coord = i.coord.to_owned();
                i.coord.height = tmp_coord.height;
                i.coord.width = tmp_coord.width;
            }
        }
    }

    pub fn move_snake(&mut self) {
        match self.direction {
            SnakeDirection::Left => {
                self.move_left();
            }
            SnakeDirection::Right => {
                self.move_right();
            }
            SnakeDirection::Up => {
                self.move_up();
            }
            SnakeDirection::Down => {
                self.move_down();
            }
        }
    }

    pub fn snake_in_apple(&self, apple: &Apple) -> bool /* checks if the snake ate the apple */ {
        let head = self.pieces.get(0).unwrap();
        return head.coord.height == apple.coord.height && head.coord.width == apple.coord.width;
    }

    pub fn is_game_over(&self) -> bool {
        let head = self.pieces.get(0).unwrap();
        for i in &self.pieces[1..] {
            if head.coord.height == i.coord.height && head.coord.width == i.coord.width {
                return true;
            }
        }
        return false;
    }
    pub fn is_colliding(&self, snake: &Snake) -> bool {
        let other_head = snake.pieces.get(0).unwrap();
        for i in &self.pieces[1..] {
            if other_head.coord.height == i.coord.height && other_head.coord.width == i.coord.width
            {
                return true;
            }
        }
        return false;
    }
}

impl GridDrawable for Snake {
    fn draw_in_grid(&self, grid: &mut Grid) {
        for i in &self.pieces {
            if (i.coord.height >= 0 && i.coord.width >= 0) {
                grid.draw_in_pos(
                    i.coord.height,
                    i.coord.width,
                    /*ATTENTION GORE CODE*/
                    if i.head == true {
                        if self.player == SnakePlayers::Player1 {
                            '@'
                        } else {
                            '0'
                        }
                    } else {
                        if self.player == SnakePlayers::Player1 {
                            '='
                        } else {
                            '+'
                        }
                    },
                );
            }
        }
    }
}
