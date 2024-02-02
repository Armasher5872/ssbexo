pub mod aerial;
pub mod attack;
mod capture;
mod catch;
mod damage;
mod escape;
pub mod grounded;
mod guard;
mod item;
mod jump;
mod landing;
mod special;
//mod squat;

pub fn install() {
    aerial::install();
    attack::install();
    capture::install();
    catch::install();
    damage::install();
    escape::install();
    grounded::install();
    guard::install();
    item::install();
    jump::install();
    landing::install();
    special::install();
    //squat::install();
}