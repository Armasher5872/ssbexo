use super::*;

//Universal Variables
pub static mut BALL_BOUNCED: Vector3f = Vector3f{x: 0.0, y: 0.0, z: 9999.0}; //Tracks stats about the volleyball to determine who to KO
pub static mut BALL_ID: u32 = 0; //The battle object ID of the ball itself
pub static mut BALL_OWNER: i32 = 9; //Which player will start with the ball
pub static mut BALL_VICTIMS: [i32; 8] = [9; 8]; //Which players are to be KOd
pub static mut COUNTERHIT_CHECK: [bool; 8] = [false; 8]; //Checks if a fighter is in the startup of an attack. Used for GGST COUNTER!
pub static mut COUNTERHIT_SUCCESS: [bool; 8] = [false; 8]; //Checks if a fighter has landed a successful counterhit. Used for GGST COUNTER!
pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
pub static mut HIGH_SPAWN_POS: Vector3f = Vector3f{x: 0.0, y: 0.0, z: 1.0}; //Determines where to spawn the right net
pub static mut HIT_PLAYER: i32 = -1; //Tracks which players need to be respawned
pub static mut LAST_ALT_STICK: [f32; 2] = [0.0, 0.0];
pub static mut LAST_ANALOG: f32 = 0.0;
pub static mut LAST_ATTACK_HITBOX_ID: i32 = 0;
pub static mut LAST_ATTACK_HITBOX_LOCATION_X: f32 = 0.0;
pub static mut LAST_ATTACK_HITBOX_LOCATION_Y: f32 = 0.0;
pub static mut LAST_ATTACK_HITBOX_LOCATION_Z: f32 = 0.0;
pub static mut LAST_DAMAGE: [f32; 8] = [0.0; 8];
pub static mut LAST_TO_HIT_BALL: usize = 9; //The last player to have hit the ball
pub static mut LOW_SPAWN_POS: Vector3f = Vector3f{x: 0.0, y: 0.0, z: 1.0}; //Determines where to spawn the left net
pub static mut READY_GO_TIMER: i32 = 0; //Determines how many frames to suspend all players while respawning in Tennis and One-Hit modes
pub static mut SIZE0: [f32; 9] = [0.0; 9];
pub static mut SIZE1: [f32; 9] = [0.0; 9];
pub static mut SPAWN_POS: [Vector3f;9] = [Vector3f{x: 0.0, y: 0.0, z: 0.0};9]; //Tracks what position to spawn the nets in in Basketball mode, and where to respawn players in Volleyball and One-Hit mode
pub static mut SPAWN_SIDE: [bool; 9] = [false; 9];
pub static mut SPECIAL_SMASH_BODY: i32 = 0; //Etc.
pub static mut SPECIAL_SMASH_GRAVITY: i32 = 0; //Etc.
pub static mut SPECIAL_SMASH_HEAD: i32 = 0; //Etc.
pub static mut SPECIAL_SMASH_RATE: i32 = 0; //Etc.
pub static mut SPECIAL_SMASH_SIZE: i32 = 0; //Checks which mode was selected in the "Size" slot
pub static mut SPECIAL_SMASH_STATUS: i32 = 0; //Etc.
pub static mut STOCK_COUNT: [u64;9] = [99;9];
pub static mut TEMP_SPAWN_POS: [Vector3f;9] = [Vector3f{x: 0.0, y: 0.0, z: 0.0};9]; //Used to randomize spawn pos in Volleyball mode
pub static mut TOTAL_FIGHTER: i32 = 1; //Tracks how many fighters are present

//Ridley Variables
pub static mut POGO_GROUND_BOUNCE: [bool; 8] = [false; 8];
pub static mut POGO_OPPONENT_BOUNCE: [bool; 8] = [false; 8];
pub static mut RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT: [i32; 8] = [0; 8];
pub static mut RIDLEY_VEC2_SPECIAL_LW_BOUNCE_POS_CHECK_PREV : [Vector2f; 8] = [Vector2f{x:0.0,y:0.0}; 8];