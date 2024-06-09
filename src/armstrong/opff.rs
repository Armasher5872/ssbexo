use super::*;

unsafe extern "C" fn armstrong_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
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
                    fighter.clear_lua_stack();
                    lua_args!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, (armor*armor_multiplier));
                    sv_module_access::damage(fighter.lua_state_agent);
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
                    fighter.clear_lua_stack();
                    lua_args!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, (9.0*armor_multiplier));
                    sv_module_access::damage(fighter.lua_state_agent);
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

unsafe extern "C" fn armstrong_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    }
    0.into()
}

unsafe extern "C" fn armstrong_init(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    //Universal
    fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(armstrong_end_control as *const () as _));
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
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
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
    //Armstrong
    WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S);
    WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_HI);
    WorkModule::set_int(boma, 0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    WorkModule::set_float(boma, 1.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
    WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
    WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
    WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
    WorkModule::set_float(boma, 1.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
}

pub fn install() {
    Agent::new("ganon")
    .on_start(armstrong_init)
    .on_line(Main, armstrong_frame)
    .install()
    ;
}