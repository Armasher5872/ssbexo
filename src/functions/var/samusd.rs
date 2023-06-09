//Dark Samus Variables
pub static mut CHARGE_SHOT_TIMER: [i32; 8] = [0; 8];
pub static mut HAS_FIRE_CHARGE_SHOT: [bool; 8] = [false; 8];
pub static mut SAMUSD_HAS_FLOAT: [bool; 8] = [false; 8];
pub static mut SAMUSD_X_ACCEL_ADD: f32 = 0.01; //Air Accel Add
pub static mut SAMUSD_X_ACCEL_MUL: f32 = 0.02; //Air Accel Mul
pub static mut SAMUSD_Y_ACCEL_ADD: f32 = 0.02; //Air Accel Add
pub static mut SAMUSD_Y_ACCEL_MUL: f32 = 0.04; //Air Accel Mul
pub static mut SAMUSD_X: [f32; 8] = [0.0; 8]; //Logs Horizontal speed
pub static mut SAMUSD_Y: [f32; 8] = [0.0; 8]; //Logs Vertical speed
pub static mut SAMUSD_X_MAX: f32 = 0.25; //Max Horizontal movespeed
pub static mut SAMUSD_Y_MAX: f32 = 0.5; //Max Vertical movespeed