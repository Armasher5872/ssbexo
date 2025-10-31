use super::*;

unsafe extern "C" fn ness_appeal_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if status_kind != *FIGHTER_STATUS_KIND_SMASH_APPEAL {
        if 0 < log_attack_kind {
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log_attack_kind);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
        CANCEL_FILL_SCREEN(fighter, 1, 4);
        CANCEL_FILL_SCREEN(fighter, 2, 4);
    }
    0.into()
}

unsafe extern "C" fn ness_attack_s4_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
                EFFECT(fighter, Hash40::new("starman_smash"), Hash40::new("top"), 0, 16, -10, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
                SoundModule::play_se(fighter.module_accessor, Hash40::new("se_ness_smash"), false, false, false, false, enSEType(0));
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("ness")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_APPEAL, ness_appeal_end_status)
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_S4, ness_attack_s4_check_attack_status)
    .install()
    ;
}