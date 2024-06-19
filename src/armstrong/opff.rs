use super::*;

//The VTable Offset for where the opff for Ganondorf would normally be doesn't work for whatever reason.
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
    fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(armstrong_end_control as *const () as _));
}

pub fn install() {
    Agent::new("ganon")
    .on_start(armstrong_init)
    .on_line(Main, armstrong_frame)
    .install()
    ;
}