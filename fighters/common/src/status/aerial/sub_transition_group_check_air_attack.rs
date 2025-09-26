//Credited to WuBoyTH
use super::*;

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_air_attack)]
unsafe extern "C" fn sub_transition_group_check_air_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if fighter.global_table[CHECK_AIR_ATTACK_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_AIR_ATTACK_UNIQ].get_ptr());
        if callable(fighter).get_bool() {
            return 1.into();
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
            if fighter.sub_transition_group_check_air_jump_attack().get_bool() {
                return 1.into();
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_AIR) && fighter.sub_is_item_shoot_air().get_bool() {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SHOOT_AIR.into(), true.into());
                return 1.into();
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
                if fighter_kind == *FIGHTER_KIND_BAYONETTA {
                    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0 {
                        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_SMASH.into(), true.into());
                        return 1.into();
                    }
                    else if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 {
                        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_U_SMASH.into(), true.into());
                        return 1.into();
                    }
                    else if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0 {
                        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_D_SMASH.into(), true.into());
                        return 1.into();
                    }
                    else {
                        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
                        return 1.into();
                    }
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
                    return 1.into();
                }
            }
        }
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(sub_transition_group_check_air_attack);
    }
}

pub fn install() {
	let _ = skyline::nro::add_hook(nro_hook).unwrap();
}