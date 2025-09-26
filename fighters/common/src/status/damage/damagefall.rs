use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_DamageFall_Main)]
unsafe extern "C" fn status_damagefall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    if fighter.sub_transition_group_check_air_cliff().get_bool() || fighter.check_damage_fall_transition().get_bool() {
        return 0.into();
    }
    let tech = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD);
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB)
    && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    && WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value")) <= fighter.global_table[STICK_X].get_f32().abs()
    && tech {
        fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_FB.into(), true.into());
        return true.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE)
    && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    && !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor)
    && tech {
        fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE.into(), true.into());
        return true.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN.into(), true.into());
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP != 0 && stick_y < -0.66 && get_sum_speed_y <= -0.5 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }
    fighter.sub_damage_fall_uniq_process_exec_fix_pos();
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(status_damagefall_main);
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}