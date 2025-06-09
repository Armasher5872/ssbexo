mod game;
pub mod status;
//mod customgamemodes;

pub fn install() {
    game::install();
    status::install();
    //customgamemodes::install();
}