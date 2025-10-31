use super::*;

pub static mut LAST_ATTACK_HITBOX_ID: i32 = 0;
pub static mut LAST_ATTACK_HITBOX_LOCATION_X: f32 = 0.0;
pub static mut LAST_ATTACK_HITBOX_LOCATION_Y: f32 = 0.0;
pub static mut LAST_ATTACK_HITBOX_LOCATION_Z: f32 = 0.0;
pub static mut MARKED_COLORS: [bool; 256] = [false; 256];

//Ridley Variables
pub static mut POGO_GROUND_BOUNCE: [bool; 8] = [false; 8];
pub static mut POGO_OPPONENT_BOUNCE: [bool; 8] = [false; 8];
pub static mut RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT: [i32; 8] = [0; 8];
pub static mut RIDLEY_VEC2_SPECIAL_LW_BOUNCE_POS_CHECK_PREV : [Vector2f; 8] = [Vector2f{x:0.0,y:0.0}; 8];