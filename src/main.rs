mod apple;
mod constants;
mod game;
mod grid;
mod snake;
mod utils;

use apple::Apple;
use constants::*;
use game::*;
use grid::*;
use pancurses::{endwin, napms, Input};
use snake::*;

fn get_apples(amount: u8) -> Vec<Apple> {
    let mut vec: Vec<Apple> = vec![];
    for i in 0..amount {
        vec.push(Apple::new(Apple::get_random_coord()));
    }
    vec
}

fn draw_array_in_grid<T: GridDrawable>(arr: &Vec<T>, grid: &mut Grid) {
    for i in arr {
        i.draw_in_grid(grid);
    }
}

fn did_snake_eat_any<'a>(
    apples: &'a mut Vec<Apple>,
    snake: &Snake,
) -> (bool, Option<&'a mut Apple>) {
    for i in apples {
        if snake.snake_in_apple(i) {
            return (true, Some(i));
        }
    }
    (false, None)
}

fn main() {
    let mut window = init_game();
    window.keypad(true);

    let mut grid = Grid::new(GRID_HEIGHT, GRID_WIDTH);
    let mut snake = Snake::new(SnakePlayers::Player1);
    let mut snake2 = Snake::new(SnakePlayers::Player2);
    let mut timer_count = 0;
    let mut apples = get_apples(40);
    snake.draw_in_grid(&mut grid);

    grid.draw(&mut window);

    'mainloop: loop {
        if let Some(ch) = window.getch() {
            match ch {
                Input::KeyExit => {
                    break 'mainloop;
                }
                Input::KeyUp => {
                    snake2.direction = SnakeDirection::Up;
                }
                Input::KeyDown => {
                    snake2.direction = SnakeDirection::Down;
                }
                Input::KeyLeft => {
                    snake2.direction = SnakeDirection::Left;
                }
                Input::KeyRight => {
                    snake2.direction = SnakeDirection::Right;
                }
                Input::Character(c) => {
                    if c == 'q' || c == 'Q' {
                        break 'mainloop;
                    } else if c == 'w' || c == 'W' {
                        snake.direction = SnakeDirection::Up;
                    } else if c == 'a' || c == 'A' {
                        snake.direction = SnakeDirection::Left;
                    } else if c == 's' || c == 'S' {
                        snake.direction = SnakeDirection::Down;
                    } else if c == 'd' || c == 'D' {
                        snake.direction = SnakeDirection::Right;
                    }
                }
                _ => {}
            }
        }
        if (timer_count >= 4) {
            snake.move_snake();
            snake2.move_snake();
            if snake.is_game_over() {
                window.mvprintw(GRID_HEIGHT as i32 + 1, 0, "Player1 died!");
                break 'mainloop;
            } else if snake2.is_game_over() {
                window.mvprintw(GRID_HEIGHT as i32 + 1, 0, "Player2 died!");
                break 'mainloop;
            } else if snake2.is_colliding(&snake) {
                window.mvprintw(
                    GRID_HEIGHT as i32 + 1,
                    0,
                    "Player2 died after colliding with player1!",
                );
                break 'mainloop;
            } else if snake.is_colliding(&snake2) {
                window.mvprintw(
                    GRID_HEIGHT as i32 + 1,
                    0,
                    "Player1 died after colliding with player2!",
                );
                break 'mainloop;
            }

            let res = did_snake_eat_any(&mut apples, &snake);
            if res.0 == true {
                let mut apple = res.1.unwrap();
                apple.set_coord(Apple::get_random_coord());
                snake.add_new_piece();
            }
            let res_snake2 = did_snake_eat_any(&mut apples, &snake2);
            if res_snake2.0 == true {
                let mut apple = res_snake2.1.unwrap();
                apple.set_coord(Apple::get_random_coord());
                snake2.add_new_piece();
            }
            timer_count = 0;
        } else {
            timer_count += 1;
        }
        grid.init();
        draw_array_in_grid(&apples, &mut grid);
        snake.draw_in_grid(&mut grid);
        snake2.draw_in_grid(&mut grid);
        grid.draw(&mut window);
        window.refresh();
        napms(20);
    }

    window.mvprintw(
        GRID_HEIGHT as i32 + 2,
        0,
        "GAME OVER! PRESS ANYTHING TO QUIT",
    );
    window.refresh();

    window.nodelay(false);
    window.getch();

    endwin();
}
