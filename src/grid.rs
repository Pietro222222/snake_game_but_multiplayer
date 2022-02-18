use pancurses::{has_colors, init_pair, start_color, Window, COLOR_PAIR};

pub struct Grid {
    height: i8,
    width: i8,
    grid: Vec<Vec<char>>,
    color: bool,
}

impl Grid {
    pub fn new(h: i8, w: i8) -> Self {
        let grid: Vec<Vec<char>> = vec![vec!['#'; w as usize]; h as usize];
        let color = has_colors();
        if color {
            start_color();
        }

        init_pair(1, 7, 2);
        init_pair(2, 7, 1);
        init_pair(3, 7, 0);
        init_pair(4, 7, 3);
        Self {
            height: h,
            width: w,
            grid: grid,
            color: color,
        }
    }

    pub fn init(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let character = self
                    .grid
                    .get_mut(i as usize)
                    .unwrap()
                    .get_mut(j as usize)
                    .unwrap();
                *character = '#';
            }
        }
    }

    pub fn draw_in_pos(&mut self, y: i8, x: i8, c: char) {
        let character = self
            .grid
            .get_mut(y as usize)
            .unwrap()
            .get_mut(x as usize)
            .unwrap();
        *character = c;
    }

    pub fn draw(&self, window: &mut Window) {
        for i in 0..self.height {
            for j in 0..self.width {
                let character = self.grid.get(i as usize).unwrap().get(j as usize).unwrap();
                window.mv(i.into(), j.into());
                if self.color {
                    if *character == '=' || *character == '@' {
                        window.attrset(COLOR_PAIR(1));
                        window.addch(character.to_owned());
                        window.attroff(COLOR_PAIR(1));
                    } else if *character == '+' || *character == '0' {
                        window.attrset(COLOR_PAIR(4));
                        window.addch(character.to_owned());
                        window.attroff(COLOR_PAIR(4));
                    } else if *character == 'O' {
                        window.attrset(COLOR_PAIR(2));
                        window.addch(character.to_owned());
                        window.attroff(COLOR_PAIR(2));
                    } else {
                        window.attrset(COLOR_PAIR(3));
                        window.addch(character.to_owned());
                        window.attroff(COLOR_PAIR(3));
                    }
                } else {
                    window.addch(character.to_owned());
                }
            }
        }
        window.refresh();
    }
}

pub trait GridDrawable {
    fn draw_in_grid(&self, grid: &mut Grid);
}
