use super::*;

unsafe extern "C" fn mewtwo_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.global_table[IS_STOP].get_bool() {
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
            if fighter.is_cat_flag(Cat1::AttackS3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
            }
            if fighter.is_cat_flag(Cat1::Dash) && fighter.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            if fighter.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
            }
            if fighter.is_cat_flag(Cat1::Dash) && fighter.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
            if fighter.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
            }
            if fighter.is_cat_flag(Cat1::Dash) && fighter.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
            if fighter.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
            }
            if fighter.is_cat_flag(Cat1::Dash) && fighter.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            if fighter.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
            if fighter.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            if fighter.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
            if fighter.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if fighter.is_cat_flag(Cat1::AttackAirN) || fighter.is_cat_flag(Cat1::AttackAirF) || fighter.is_cat_flag(Cat1::AttackAirB)
            || fighter.is_cat_flag(Cat1::AttackAirHi) || fighter.is_cat_flag(Cat1::AttackAirLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if fighter.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackAirN) || fighter.is_cat_flag(Cat1::AttackAirF) || fighter.is_cat_flag(Cat1::AttackAirB)
            || fighter.is_cat_flag(Cat1::AttackAirHi) || fighter.is_cat_flag(Cat1::AttackAirLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if fighter.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackAirN) || fighter.is_cat_flag(Cat1::AttackAirF) || fighter.is_cat_flag(Cat1::AttackAirB)
            || fighter.is_cat_flag(Cat1::AttackAirHi) || fighter.is_cat_flag(Cat1::AttackAirLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if fighter.is_cat_flag(Cat1::AttackN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
            }
            if fighter.is_cat_flag(Cat1::AttackS4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackHi4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackLw4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }
            if fighter.is_cat_flag(Cat1::AttackAirN) || fighter.is_cat_flag(Cat1::AttackAirF) || fighter.is_cat_flag(Cat1::AttackAirB)
            || fighter.is_cat_flag(Cat1::AttackAirHi) || fighter.is_cat_flag(Cat1::AttackAirLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
            }
            if fighter.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
        }
    }
}

unsafe extern "C" fn mewtwo_init(fighter: &mut L2CFighterCommon) {
}

pub fn install() {
    Agent::new("mewtwo")
    //.on_start(mewtwo_init)
    .on_line(Main, mewtwo_frame)
    .install()
    ;
}