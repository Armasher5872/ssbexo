use super::*;

unsafe extern "C" fn krool_frame(fighter: &mut L2CFighterCommon) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(module_accessor);
    let motion_kind = MotionModule::motion_kind(module_accessor);
    let frame = MotionModule::frame(module_accessor);
    if motion_kind == hash40("attack_air_hi") 
    && frame >= 58.0 {
        CancelModule::enable_cancel(module_accessor);
    };
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        KROOL_HAS_UAIR[entry_id] = false;
    };
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START].contains(&status_kind) {
        if frame >= 10.0 {
            CancelModule::enable_cancel(module_accessor);
        }
        if CancelModule::is_enable_cancel(module_accessor) {
            KROOL_UP_SPECIAL_CANCEL[entry_id] = true;
        };
    };
    if KROOL_UP_SPECIAL_CANCEL[entry_id] == true {
        WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
        WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
        WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    };
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR
    && fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
    && KROOL_UP_SPECIAL_CANCEL[entry_id] != false {
        if motion_kind == hash40("attack_air_n")
        && frame >= 43.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL, true);
        };
        if motion_kind == hash40("attack_air_f") 
        && frame >= 52.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL, true);
        };
        if motion_kind == hash40("attack_air_b")
        && frame >= 48.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL, true);
        };
        if motion_kind == hash40("attack_air_hi") 
        && frame >= 57.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL, true);
        };
        if motion_kind == hash40("attack_air_lw") 
        && frame >= 59.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL, true);
        };
    };
    if status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL
    || fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        KROOL_UP_SPECIAL_CANCEL[entry_id] = false;
    };
}

pub fn install() {
    Agent::new("krool")
    .on_line(Main, krool_frame)
    .install()
    ;
}