use super::*;

//Special Hi Main, resets your jump count
#[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn metaknight_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    let jump_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    WorkModule::set_int(fighter.module_accessor, jump_max, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    ret
}

//Special Hi Loop, resets your jump count
#[status_script( agent = "metaknight", status = FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn metaknight_special_hi_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    ret
}

//Glide Start Main
#[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn metaknight_glide_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
    fighter.status_GlideStart()
}

//Glide End Main
#[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn metaknight_glide_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_end"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_glide_end_main_loop as *const () as _))
}

unsafe fn metaknight_glide_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_air_landing().get_bool() {
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

//Glide Attack Main
#[status_script( agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn metaknight_glide_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_attack"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_glide_attack_main_loop as *const () as _))
}

unsafe fn metaknight_glide_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_landing().get_bool() {
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        metaknight_special_hi_main,
        metaknight_special_hi_loop_main,
        metaknight_glide_start_main,
        metaknight_glide_end_main,
        metaknight_glide_attack_main
    );
}