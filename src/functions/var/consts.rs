//Custom Universal Consts
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK: i32 = 0x20000116; //Indicates all fighters are on their last stock
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED: i32 = 0x20000117; //Tracks if the ball has bounced at least once since being thrown
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START: i32 = 0x20000118;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE: i32 = 0x20000119;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL: i32 = 0x2000011A;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING: i32 = 0x2000011B;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY: i32 = 0x2000011C;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED: i32 = 0x2000011D;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER: i32 = 0x2000011E;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK: i32 = 0x2000011F; //Tracks if a fighter used a certain special move in the air
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE: i32 = 0x20000120;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD: i32 = 0x20000121;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED: i32 = 0x20000122;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT: i32 = 0x20000123;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT: i32 = 0x20000124;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE: i32 = 0x20000125;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK: i32 = 0x20000126;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE: i32 = 0x20000127; //Allows the throwing player to bounce the ball on their own side once
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK: i32 = 0x20000128;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH: i32 = 0x20000129;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW: i32 = 0x2000012A;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE: i32 = 0x2000012B;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC: i32 = 0x2000012C;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO: i32 = 0x2000012D; //Returns false for exactly one frame after is_ready_go becomes true, used to initiate certain events exactly once at the start of a match
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE: i32 = 0x2000012E;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE: i32 = 0x2000012F;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE: i32 = 0x20000130;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE: i32 = 0x20000131;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH: i32 = 0x20000132;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS: i32 = 0x20000133; //Flags when you just used a Final Smash in Special Smash
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL: i32 = 0x641;
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED: i32 = 0x642;
pub const FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE: i32 = 0x100000ED;
pub const FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER: i32 = 0x100000EE;
pub const FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE: i32 = 0x100000EF;
pub const FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT: i32 = 0x100000F0; //Tracks if a player got hit during One-Hit mode
pub const FIGHTER_INSTANCE_WORK_ID_INT_MASHING: i32 = 0x100000F1;
pub const FIGHTER_INSTANCE_WORK_ID_INT_PARRIED: i32 = 0x100000F2;
pub const FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER: i32 = 0x100000F3;
pub const FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER: i32 = 0x100000F4;
pub const FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE: i32 = 0x100000F5;
pub const FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX: i32 = 0x100000F6;
pub const FIGHTER_STATUS_KIND_SPECIAL_GUARD: i32 = 0x643;
pub const FRAME_FALL: f32 = 34.0;
pub const FRAME_LAND: f32 = 52.0;