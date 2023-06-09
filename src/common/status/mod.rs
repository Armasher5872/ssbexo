pub mod aerial;
pub mod attack;
mod catch;
mod damage;
mod escape;
pub mod grounded;
mod guard;
mod item;
mod jump;
mod landing;
mod special;

pub fn install() {
    aerial::install();
    attack::install();
    catch::install();
    damage::install();
    escape::install();
    grounded::install();
    guard::install();
    item::install();
    jump::install();
    landing::install();
    special::install();
}