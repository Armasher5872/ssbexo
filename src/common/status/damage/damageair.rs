use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageAir_Main)]
unsafe fn status_damageair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let fall_check;
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let prev_pos = PostureModule::prev_pos(fighter.module_accessor);
    let prev_pos_y = (*prev_pos).y;
    let start_fall_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_START_FALL_Y);
    let hit_stop_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_HIT_STOP_FRAME);
    let get_speed_length = {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        smash::app::sv_kinetic_energy::get_speed_length(fighter.lua_state_agent)
    };
    let damage_speed_down = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("damage_speed_down"));
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            if (start_fall_y-pos_y) < 2.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT.into(), false.into());
                return 0.into();
            }
        }
        if fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if fighter.sub_transition_group_check_air_special().get_bool() {
        return 0.into();
    }
    if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
        fall_check = false;
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            fall_check = false;
        }
        else {
            fall_check = MotionModule::is_end(fighter.module_accessor);
        }
    }
    if fall_check {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    asdi_check(fighter);
    asdi_function(fighter);
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT) {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    }
    if fighter_kind == *FIGHTER_KIND_PICKEL {
        if fighter.global_table[IS_STOP].get_bool() {
            macros::COL_NORMAL(fighter);
            macros::FLASH(fighter, 2.55, 0.0, 0.0, 0.25);
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        if !StopModule::is_damage(fighter.module_accessor) {
            if 0 >= hit_stop_frame {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN) {
                    if damage_speed_down < get_speed_length {
                        fighter.change_status(FIGHTER_STATUS_KIND_DOWN.into(), false.into());
                        return 0.into();
                    }
                }
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
                    WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                    return 0.into();
                }
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if pos_y > prev_pos_y {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FALLING);
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FALLING) {
                return 0.into();
            }
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_START_FALL_Y);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FALLING);
        }
    }
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_DamageAir)]
unsafe fn status_end_damageair(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    fighter.check_ryu_final_damage_03(true.into());
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_damageair_main,
            status_end_damageair
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}