mod cell;
mod cells;
mod field;
mod game;
mod messages;
fn main() {
    game::game_of_life(10, 10);
}
