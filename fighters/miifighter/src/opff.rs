use super::*;

//Set Move Customizer is accredited to WuBor Patch
unsafe extern "C" fn miifighter_waza_customize(fighter: &mut L2CFighterCommon) -> L2CValue {
    let waza_customize_to = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if waza_customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_3 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miifighter_special_n3_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miifighter_special_n3_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_n3_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_RISE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miifighter_special_n3_rise_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_RISE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_n3_rise_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_DIVE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miifighter_special_n3_dive_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_DIVE.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miifighter_special_n3_dive_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_DIVE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_n3_dive_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_LAND.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miifighter_special_n3_land_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_LAND.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_n3_land_main_status as *const ()));
        0.into()
    }
    else if waza_customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_s1_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_s1_end_main_status as *const ()));
        0.into()
    }
    else if waza_customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miifighter_special_lw1_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miifighter_special_lw1_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_lw1_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_CHARGE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miifighter_special_lw1_charge_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_CHARGE.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miifighter_special_lw1_charge_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_CHARGE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_lw1_charge_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miifighter_special_lw1_attack_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miifighter_special_lw1_attack_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miifighter_special_lw1_attack_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miifighter_special_lw1_attack_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(miifighter_special_lw1_attack_exit_status as *const ()));
        0.into()
    }
    else if let Some(original) = get_original_customizer(fighter) {
        original(fighter)
    } 
    else {
        0.into()
    }
}

unsafe extern "C" fn miifighter_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || is_damaged(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER);
    }
    0.into()
}

unsafe extern "C" fn miifighter_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    miifighter_var(&mut *boma);
    fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(miifighter_end_control as *const () as _));
    set_move_customizer(fighter, miifighter_waza_customize);
    miifighter_waza_customize(fighter);
}

pub fn install() {
    Agent::new("miifighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(miifighter_on_start)
    .install()
    ;
}