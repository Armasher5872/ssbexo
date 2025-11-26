pub mod attack;
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
mod thrown;

pub fn install() {
    attack::install();
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
    thrown::install();
}