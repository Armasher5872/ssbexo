pub mod game;
pub mod status;

pub fn install() {
    game::install();
    status::install();
}