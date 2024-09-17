use super::*;

const MIISWORDSMAN_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xd99a30; //Mii Swordfighter only
const MIISWORDSMAN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xd99d30; //Mii Swordfighter only
const MIISWORDSMAN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xd99d50; //Mii Swordfighter only
const MIISWORDSMAN_VTABLE_ONCE_PER_FIGHTER_FRAME: usize = 0x68d670; //Shared

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

//Mii Swordfighter Startup Initialization
#[skyline::hook(offset = MIISWORDSMAN_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn miiswordsman_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    WorkModule::set_flag(boma, false, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_BLURRING_SLASHES_CANCEL);
    WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_AIRBORNE_ASSAULT_ANGLE);
    WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_BLURRING_SLASHES_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
    WorkModule::set_int(boma, 1, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_LIGHT_SHURIKEN_COUNT);
    WorkModule::set_float(boma, 1.0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_GUARD_BREAKER_POWER);
    set_move_customizer(lua_module_fighter, miiswordsman_waza_customize);
    miiswordsman_waza_customize(lua_module_fighter);
    original!()(vtable, fighter)
}

//Mii Swordfighter Reset Initialization
#[skyline::hook(offset = MIISWORDSMAN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn miiswordsman_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
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
    WorkModule::set_flag(boma, false, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_BLURRING_SLASHES_CANCEL);
    WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_AIRBORNE_ASSAULT_ANGLE);
    WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_BLURRING_SLASHES_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
    WorkModule::set_int(boma, 1, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_LIGHT_SHURIKEN_COUNT);
    WorkModule::set_float(boma, 1.0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_GUARD_BREAKER_POWER);
    original!()(vtable, fighter)
}

//Mii Swordfighter Death Initialization
#[skyline::hook(offset = MIISWORDSMAN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn miiswordsman_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    WorkModule::set_flag(boma, false, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_BLURRING_SLASHES_CANCEL);
    WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_AIRBORNE_ASSAULT_ANGLE);
    WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_BLURRING_SLASHES_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
    WorkModule::set_int(boma, 1, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_LIGHT_SHURIKEN_COUNT);
    WorkModule::set_float(boma, 1.0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_GUARD_BREAKER_POWER);
    original!()(vtable, fighter)
}

//Mii Swordfighter Once Per Fighter Frame
#[skyline::hook(offset = MIISWORDSMAN_VTABLE_ONCE_PER_FIGHTER_FRAME)]
unsafe extern "C" fn miiswordsman_opff(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let blurring_slashes_timer = WorkModule::get_int(boma, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_BLURRING_SLASHES_TIMER);
    let long_sword_scale = Vector3f{x: 1.015, y: 1.15, z: 1.045};
    if fighter.battle_object.kind == *FIGHTER_KIND_MIISWORDSMAN as u32 {
        ModelModule::set_joint_scale(boma, Hash40::new("havel"), &long_sword_scale);
        ModelModule::set_joint_scale(boma, Hash40::new("haver"), &long_sword_scale);
        if WorkModule::is_flag(boma, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_BLURRING_SLASHES_CANCEL) && blurring_slashes_timer > 0 {
            WorkModule::dec_int(boma, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_BLURRING_SLASHES_TIMER);
        }
        if blurring_slashes_timer <= 0 {
            WorkModule::set_flag(boma, false, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_BLURRING_SLASHES_CANCEL);
        }
    }
}

pub fn install() {
	skyline::install_hooks!(
        miiswordsman_start_initialization,
        miiswordsman_reset_initialization,
        miiswordsman_death_initialization,
        miiswordsman_opff
    );
}