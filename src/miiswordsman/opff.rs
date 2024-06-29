use super::*;

unsafe extern "C" fn miiswordsman_waza_customize(fighter: &mut L2CFighterCommon) -> L2CValue {
    let customize_to = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_1 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_n1_start_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_n1_start_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_n1_start_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N1_LOOP.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_n1_loop_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N1_LOOP.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miiswordsman_special_n1_loop_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N1_LOOP.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_n1_loop_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N1_LOOP.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(miiswordsman_special_n1_loop_exec_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N1_LOOP.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_n1_loop_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N1_LOOP.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(miiswordsman_special_n1_loop_exit_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_n1_attack_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miiswordsman_special_n1_attack_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_n1_attack_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(miiswordsman_special_n1_attack_exec_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_n1_attack_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(miiswordsman_special_n1_attack_exit_status as *const ()));
        0.into()
    }
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_n2_start_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miiswordsman_special_n2_start_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_n2_start_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(miiswordsman_special_n2_start_exec_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_n2_start_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N2_HOLD.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_n2_hold_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N2_HOLD.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miiswordsman_special_n2_hold_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N2_HOLD.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_n2_hold_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N2_HOLD.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(miiswordsman_special_n2_hold_exec_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N2_HOLD.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_n2_hold_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N2_HOLD.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(miiswordsman_special_n2_hold_exit_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N2_FIRE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_n2_fire_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N2_FIRE.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miiswordsman_special_n2_fire_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N2_FIRE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_n2_fire_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N2_FIRE.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(miiswordsman_special_n2_fire_exec_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N2_FIRE.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_n2_fire_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N2_FIRE.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(miiswordsman_special_n2_fire_exit_status as *const ()));
        0.into()
    }
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_3 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_n3_start_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_n3_start_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_n3_start_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_SLASH.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_n3_slash_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_SLASH.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miiswordsman_special_n3_slash_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_SLASH.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_n3_slash_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_SLASH.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(miiswordsman_special_n3_slash_exec_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_SLASH.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_n3_slash_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_SLASH.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(miiswordsman_special_n3_slash_exit_status as *const ()));
        0.into()
    }
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_s1_start_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miiswordsman_special_s1_start_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_s1_start_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_s1_start_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_s1_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miiswordsman_special_s1_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_s1_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_s1_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_s1_end_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miiswordsman_special_s1_end_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_s1_end_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_s1_end_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        0.into()
    }
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_2 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_s2_start_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miiswordsman_special_s2_start_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_s2_start_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_s2_start_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_1.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_s2_attack_1_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_1.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miiswordsman_special_s2_attack_1_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_1.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_s2_attack_1_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_1.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_1.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_s2_attack_1_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_2.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_2.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_s2_attack_2_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_2.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miiswordsman_special_s2_attack_2_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_2.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_s2_attack_2_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_2.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_2.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_s2_attack_2_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_2.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_3.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(miiswordsman_special_s2_attack_3_pre_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_3.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), std::mem::transmute(miiswordsman_special_s2_attack_3_init_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_3.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(miiswordsman_special_s2_attack_3_main_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_3.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_3.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(miiswordsman_special_s2_attack_3_end_status as *const ()));
        fighter.sv_set_status_func(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK_3.into(), LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(), std::mem::transmute(empty_waza_customize as *const ()));
        0.into()
    }
    else if let Some(original) = get_original_customizer(fighter) {
        original(fighter)
    } 
    else {
        0.into()
    }
}

unsafe extern "C" fn miiswordsman_init(fighter: &mut L2CFighterCommon) {
    set_move_customizer(fighter, miiswordsman_waza_customize);
    miiswordsman_waza_customize(fighter);
}

pub fn install() {
    Agent::new("miiswordsman")
    .on_start(miiswordsman_init)
    .install()
    ;
}