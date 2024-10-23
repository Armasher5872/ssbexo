use super::*;

const ARMSTRONG_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xaa6510; //Armstrong only
const ARMSTRONG_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const ARMSTRONG_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xaa6520; //Armstrong only
const ARMSTRONG_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x68d680; //Shared
const ARMSTRONG_VTABLE_ON_ATTACK_OFFSET: usize = 0xaa6540; //Armstrong only
const ARMSTRONG_VTABLE_ON_DAMAGE_OFFSET: usize = 0x68d9e0; //Shared
const ARMSTRONG_VTABLE_LINK_EVENT_OFFSET: usize = 0xaa6990; //Armstrong only

unsafe extern "C" fn armstrong_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    }
    0.into()
}

//Armstrong Startup Initialization
#[skyline::hook(offset = ARMSTRONG_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn armstrong_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let lua_module_fighter = get_fighter_common_from_accessor(&mut *boma);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    FIGHTER_BOOL_1[entry_id] = false;
    FIGHTER_BOOL_2[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
    WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    SPECIAL_WALL_JUMP = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    LAST_DAMAGE[entry_id] = 0.0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SIZE2[entry_id] = 0.0;
    SIZE3[entry_id] = 0.0;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S);
    WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_HI);
    WorkModule::set_int(boma, 0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    WorkModule::set_float(boma, 1.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
    WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
    WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
    WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
    WorkModule::set_float(boma, 1.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    lua_module_fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    lua_module_fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(armstrong_end_control as *const () as _));
}

//Armstrong Reset Initialization
#[skyline::hook(offset = ARMSTRONG_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn armstrong_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_GANON as u32 {
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
        COUNTERHIT_CHECK[entry_id] = false;
        COUNTERHIT_SUCCESS[entry_id] = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
        FIGHTER_BOOL_1[entry_id] = false;
        FIGHTER_BOOL_2[entry_id] = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
        WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        SPECIAL_WALL_JUMP = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
        WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
        WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
        LAST_DAMAGE[entry_id] = 0.0;
        SIZE0[entry_id] = 0.0;
        SIZE1[entry_id] = 0.0;
        SIZE2[entry_id] = 0.0;
        SIZE3[entry_id] = 0.0;
        FULL_HOP_ENABLE_DELAY[entry_id] = 0;
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S);
        WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_HI);
        WorkModule::set_int(boma, 0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
        WorkModule::set_float(boma, 1.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
        WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
        WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
        WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
        WorkModule::set_float(boma, 1.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    }
    original!()(vtable, fighter)
}

//Armstrong Death Initialization
#[skyline::hook(offset = ARMSTRONG_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn armstrong_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
    WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    FIGHTER_BOOL_1[entry_id] = false;
    FIGHTER_BOOL_2[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    SPECIAL_WALL_JUMP = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    LAST_DAMAGE[entry_id] = 0.0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SIZE2[entry_id] = 0.0;
    SIZE3[entry_id] = 0.0;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S);
    WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_HI);
    WorkModule::set_int(boma, 0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
    WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
    WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
    WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
    WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
}

//Armstrong Once Per Fighter Frame
#[skyline::hook(offset = ARMSTRONG_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn armstrong_opff(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_GANON as u32 {
        let boma = fighter.battle_object.module_accessor;
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        //Charge Mechanics
        if [
            hash40("attack_s3_s"), hash40("attack_s3_hi"), hash40("attack_s3_lw"), hash40("attack_hi3"), hash40("attack_lw3"), hash40("attack_air_f"), hash40("attack_air_b"), hash40("attack_air_hi"), 
            hash40("attack_air_lw"), hash40("special_s_start"), hash40("special_air_s_start"), hash40("special_hi_catch"), hash40("special_hi_throw"), hash40("special_lw"), hash40("special_air_lw")
        ].contains(&motion_kind) {
            let armor_multiplier = WorkModule::get_float(boma, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
            let damage_multiplier = WorkModule::get_float(boma, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
            let charge_frames = WorkModule::get_int(boma, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
            let charging = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK);
            let charging_special = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL);
            let charge_start = WorkModule::get_float(boma, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
            let charge_end = WorkModule::get_float(boma, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
            let max_charge: f32 = 20.0;
            match motion_kind {
                _ if [hash40("attack_s3_s"), hash40("attack_s3_hi"), hash40("attack_s3_lw"), hash40("attack_lw3")].contains(&motion_kind) => {
                    WorkModule::set_float(boma, 3.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
                    WorkModule::set_float(boma, 9.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
                }
                _ if motion_kind == hash40("attack_hi3") => {
                    WorkModule::set_float(boma, 12.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
                    WorkModule::set_float(boma, 20.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
                }
                _ if [hash40("attack_air_f")].contains(&motion_kind) => {
                    WorkModule::set_float(boma, 4.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
                    WorkModule::set_float(boma, 12.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
                }
                _ if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) => {
                    WorkModule::set_float(boma, 5.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
                    WorkModule::set_float(boma, 14.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
                }
                _ if [hash40("attack_air_b"), hash40("attack_air_hi")].contains(&motion_kind) => {
                    WorkModule::set_float(boma, 1.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
                    WorkModule::set_float(boma, 5.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
                }
                _ if [hash40("attack_air_lw"), hash40("special_s_start"), hash40("special_air_s_start")].contains(&motion_kind) => {
                    WorkModule::set_float(boma, 3.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
                    WorkModule::set_float(boma, 11.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
                }
                _ => {}
            }
            if (charge_start..charge_end).contains(&frame) && (charge_frames < (max_charge as i32)) && (charging || charging_special) {
                let mut motion_rate: f32 = 0.0;
                if charging {
                    match motion_kind {
                        _ if [hash40("attack_s3_s"), hash40("attack_s3_hi"), hash40("attack_s3_lw"), hash40("attack_lw3")].contains(&motion_kind) => {
                            motion_rate = 0.03*(charge_frames as f32);
                        }
                        _ if [hash40("attack_hi3"), hash40("attack_air_f"), hash40("attack_air_lw")].contains(&motion_kind) => {
                            motion_rate = 0.045*(charge_frames as f32);
                        }
                        _ if [hash40("attack_air_b"), hash40("attack_air_hi")].contains(&motion_kind) => {
                            motion_rate = 0.025*(charge_frames as f32);
                        }
                        _ => {}
                    }
                    MotionModule::set_rate(boma, motion_rate);
                    WorkModule::set_float(boma, 1.0+((1.0/14.0)*(charge_frames as f32)), FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
                    WorkModule::set_float(boma, 1.0+(0.02*(charge_frames as f32)), FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
                    if [hash40("attack_s3_s"), hash40("attack_s3_hi"), hash40("attack_s3_lw"), hash40("attack_hi3"), hash40("attack_lw3")].contains(&motion_kind) {
                        let mut armor: f32 = 0.0;
                        match motion_kind {
                            _ if [hash40("attack_s3_s"), hash40("attack_s3_hi"), hash40("attack_s3_lw")].contains(&motion_kind) => {
                                armor = 6.0;
                            }
                            _ if motion_kind == hash40("attack_hi3") => {
                                armor = 7.0;
                            }
                            _ if motion_kind == hash40("attack_lw3") => {
                                armor = 4.5;
                            }
                            _ => {}
                        }
                        DamageModule::set_reaction_mul(boma, 0.85/armor_multiplier);
                        DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER as u8}, armor*armor_multiplier, -1.0, -1);
                    }
                    WorkModule::inc_int(boma, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
                }
                if charging_special {
                    match motion_kind {
                        _ if [hash40("special_s_start"), hash40("special_air_s_start")].contains(&motion_kind) => {
                            motion_rate = 0.045*(charge_frames as f32);
                        }
                        _ if motion_kind == hash40("special_hi_throw") => {
                            motion_rate = 1.0;
                        }
                        _ if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) => {
                            motion_rate = 0.045*(charge_frames as f32);
                        }
                        _ => {}
                    }
                    MotionModule::set_rate(boma, motion_rate);
                    WorkModule::set_float(boma, 1.0+((1.0/14.0)*(charge_frames as f32)), FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
                    WorkModule::set_float(boma, 1.0+(0.02*(charge_frames as f32)), FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
                    if [hash40("special_s_start"), hash40("special_lw")].contains(&motion_kind) {
                        DamageModule::set_reaction_mul(boma, 0.85/armor_multiplier);
                        DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER as u8}, 9.0*armor_multiplier, -1.0, -1);
                    }
                    WorkModule::inc_int(boma, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
                }
            }
            else {
                if charge_frames > 0 {
                    AttackModule::set_power_up(boma, damage_multiplier);
                }
                MotionModule::set_rate(boma, 1.0);
            }
        }
        else {
            WorkModule::set_int(boma, 0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
            WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
            WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
            WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
            WorkModule::set_float(boma, 1.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
            WorkModule::set_float(boma, 1.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
        }
    }
    original!()(vtable, fighter)
}

//Armstrong On Attack
#[skyline::hook(offset = ARMSTRONG_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn armstrong_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let frame = MotionModule::frame(boma);
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N && situation_kind != *SITUATION_KIND_AIR && frame < 94.0 {
        call_special_zoom(boma, log, *FIGHTER_KIND_GANON, hash40("param_special_n"), 1, 0, 0, 0, 0);
    }
    if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
        call_special_zoom(boma, log, *FIGHTER_KIND_GANON, hash40("param_special_n"), 1, 0, 0, 0, 0);
    }
    original!()(vtable, fighter, log)
}

//Armstrong On Damage
#[skyline::hook(offset = ARMSTRONG_VTABLE_ON_DAMAGE_OFFSET)]
unsafe extern "C" fn armstrong_on_damage(vtable: u64, fighter: &mut Fighter, on_damage: u64) {
    if fighter.battle_object.kind == *FIGHTER_KIND_GANON as u32 {
        let boma = fighter.battle_object.module_accessor;
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let status_kind = fighter.battle_object.status();
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL
        && [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind)
        && (1.0..=79.0).contains(&frame) {
            HitModule::set_check_catch(boma, true, 0);
            DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
            MotionModule::change_motion(boma, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    original!()(vtable, fighter, on_damage)
}

//Armstrong Link Event
#[skyline::hook(offset = ARMSTRONG_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn armstrong_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    let ret = call_original!(vtable, fighter, event);
    let boma = fighter.battle_object.module_accessor;
    let status = StatusModule::status_kind(boma);
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
        let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
        if status == *FIGHTER_STATUS_KIND_FINAL && capture_event.status == *FIGHTER_STATUS_KIND_CAPTURE_PULLED {
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.result = true;
            capture_event.motion_offset = offset;
            capture_event.motion_offset_lw = offset_lw;
            fighter.battle_object.change_status_req(*FIGHTER_STATUS_KIND_THROW, false);
            return 0;
        }
        return 1;
    }
    ret
}

pub fn install() {
    //Nops the original location where Neutral Special inflicts critical zoom, as I only want the fully charged final hit to inflict critical zoom
    let _ = skyline::patching::Patch::in_text(0xaa6618).nop();
	skyline::install_hooks!(
        armstrong_start_initialization,
        armstrong_reset_initialization,
        armstrong_death_initialization,
        armstrong_opff,
        armstrong_on_attack,
        armstrong_on_damage,
        armstrong_link_event
    );
}