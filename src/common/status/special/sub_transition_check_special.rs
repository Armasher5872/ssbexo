//The following hooks are accredited to WuBoyTH's source code from the WuBor Patch
use super::*;

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_special)]
unsafe extern "C" fn sub_transition_group_check_ground_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if fighter.global_table[CHECK_GROUND_SPECIAL_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_GROUND_SPECIAL_UNIQ].get_ptr());
        if callable(fighter).get_bool() {
            return true.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.sub_transition_group_check_special_command().get_bool() {
            return true.into();
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SMASH) {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, Hash40::new_raw(0x229a8a3811));
                    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                    if fighter.pop_lua_stack(1).get_bool() {
                        fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(), true.into());
                        return true.into();
                    }   
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) {
                let cont = if fighter.global_table[CHECK_SPECIAL_N_UNIQ].get_bool() {
                    let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_SPECIAL_N_UNIQ].get_ptr());
                    callable(fighter).get_bool()
                }
                else {
                    true
                };
                if cont {
                    fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(), true.into());
                    return true.into();
                }
            }
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) {
            let cont = if fighter.global_table[CHECK_SPECIAL_S_UNIQ].get_bool() {
                let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_SPECIAL_S_UNIQ].get_ptr());
                callable(fighter).get_bool()
            }
            else {
                true
            };
            if cont {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
                return true.into();
            }
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) {
            let cont = if fighter.global_table[CHECK_SPECIAL_HI_UNIQ].get_bool() {
                let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_SPECIAL_HI_UNIQ].get_ptr());
                callable(fighter).get_bool()
            }
            else {
                true
            };
            if cont {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
                return true.into();
            }
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
            let cont = if fighter.global_table[CHECK_SPECIAL_LW_UNIQ].get_bool() {
                let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_SPECIAL_LW_UNIQ].get_ptr());
                callable(fighter).get_bool()
            }
            else {
                true
            };
            if cont {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_air_special)]
unsafe extern "C" fn sub_transition_group_check_air_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].get_ptr());
        if callable(fighter).get_bool() {
            return true.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if fighter.sub_transition_group_check_special_command().get_bool() {
            return true.into();
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SMASH) {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, Hash40::new_raw(0x229a8a3811));
                    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                    if fighter.pop_lua_stack(1).get_bool() {
                        fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(), true.into());
                        return true.into();
                    }   
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) {
                let cont = if fighter.global_table[CHECK_SPECIAL_N_UNIQ].get_bool() {
                    let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_SPECIAL_N_UNIQ].get_ptr());
                    callable(fighter).get_bool()
                }
                else {
                    true
                };
                if cont {
                    fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(), true.into());
                    return true.into();
                }
            }
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) {
            let cont = if fighter.global_table[CHECK_SPECIAL_S_UNIQ].get_bool() {
                let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_SPECIAL_S_UNIQ].get_ptr());
                callable(fighter).get_bool()
            }
            else {
                true
            };
            if cont {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
                return true.into();
            }
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) {
            let cont = if fighter.global_table[CHECK_SPECIAL_HI_UNIQ].get_bool() {
                let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_SPECIAL_HI_UNIQ].get_ptr());
                callable(fighter).get_bool()
            }
            else {
                true
            };
            if cont {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
                return true.into();
            }
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
            let cont = if fighter.global_table[CHECK_SPECIAL_LW_UNIQ].get_bool() {
                let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_SPECIAL_LW_UNIQ].get_ptr());
                callable(fighter).get_bool()
            }
            else {
                true
            };
            if cont {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_transition_group_check_ground_special,
            sub_transition_group_check_air_special
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}