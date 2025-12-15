use super::*;

//Up Special Jump Check Attack Status
unsafe extern "C" fn wario_special_hi_jump_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let current_frame = MotionModule::frame(fighter.module_accessor);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            if current_frame >= 24.0 {
                if LAST_ATTACK_HITBOX_ID != 0 {
                    let coin_sfx = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_coin"), true, false, false, false, smash::app::enSEType(0));
                    SoundModule::set_se_vol(fighter.module_accessor, coin_sfx as i32, 4.0, 0);
                }
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(CheckAttack, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_HI_JUMP, wario_special_hi_jump_check_attack_status)
    .install()
    ;
}