pub mod aerial;
pub mod attack;
//mod capture;
mod catch;
mod cliff;
mod damage;
mod escape;
pub mod grounded;
mod guard;
mod jump;
mod landing;
mod misc;
mod special;
//mod squat;
mod thrown;

pub fn install() {
    aerial::install();
    attack::install();
    //capture::install();
    catch::install();
    cliff::install();
    damage::install();
    escape::install();
    grounded::install();
    guard::install();
    jump::install();
    landing::install();
    misc::install();
    special::install();
    //squat::install();
    thrown::install();
}