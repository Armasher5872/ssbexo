use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_DamageFall_common)]
unsafe extern "C" fn sub_damagefall_common(fighter: &mut L2CFighterCommon) {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let rate_counter = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x2f136d6218u64);
    let damage_fly_meteor_reverse_motion = WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("damage_fly_meteor_reverse_motion"));
    let rate = MotionModule::rate(fighter.module_accessor);
    let mut reverse_motion = false;
    if [*FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY].contains(&status_kind) 
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BLACKBALL_AREA) {
        if motion_kind != hash40("fall_damage") {
            WorkModule::set_float(fighter.module_accessor, rate, *FIGHTER_STATUS_DAMAGE_FALL_WORK_FLOAT_PREV_MOT_RATE);
            WorkModule::set_float(fighter.module_accessor, rate_counter, *FIGHTER_STATUS_DAMAGE_FALL_WORK_FLOAT_MOT_RATE_COUNTER);
        }
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN);
    if prev_status_kind != *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB);
    }
    if !MotionModule::is_no_comp(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FALL_FLAG_PREV_NO_COMP);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FALL_FLAG_PREV_NO_COMP);
    }
    if [*FIGHTER_STATUS_KIND_GIMMICK_BARREL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) {
        MotionModule::set_no_comp(fighter.module_accessor, true);
    }
    if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FALL_FLAG_DISABLE_MOTION_RATE_TRANS);
    }
    if status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FLY {
        if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR {
            if motion_kind == hash40("wall_damage") {
                reverse_motion = true;
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR {
            if motion_kind == damage_fly_meteor_reverse_motion as u64 {
                reverse_motion = true;
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY {
            if motion_kind == hash40("damage_fly_lw") {
                reverse_motion = true;
            }
        }
    }
    else {
        if motion_kind == hash40("wall_damage") {
            reverse_motion = true;
        }
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall_damage"), 0.0, 1.0, false, 0.0, false, false);
    if reverse_motion {
        MotionModule::set_reverse(fighter.module_accessor, true);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FALL_FLAG_REVERSE_MOTION);
    }
    if motion_kind == hash40("fall_damage") {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_KNOCKOUT) {
            if !StopModule::is_stop(fighter.module_accessor) {
                fighter.sub_damage_fall_uniq(false.into());
            }
            fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_damage_fall_uniq as *const () as _));
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_DamageFall_Main)]
unsafe extern "C" fn status_damagefall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    fighter.sub_damage_fall_uniq_process_exec_fix_pos();
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_damagefall_common,
            status_damagefall_main
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}