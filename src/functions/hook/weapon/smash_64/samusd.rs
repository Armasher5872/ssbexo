//use super::*;

/*
//Dark Samus Charge Shot On Attack Offset
unsafe extern "C" fn samusd_cshot_on_attack(vtable: u64, weapon: *mut smash::app::Weapon, log: u32) -> u64 {
    let weapon_kind = (*weapon).battle_object.kind;
    if weapon_kind == *WEAPON_KIND_SAMUSD_CSHOT as u32 {
        let boma = (*weapon).battle_object.module_accessor;
        let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_boma = &mut *sv_battle_object::module_accessor(owner_id as u32);
        let owner_kind = utility::get_kind(&mut *owner_boma);
        let collision_log = *(log as *const u64).add(0x10/0x8);
        let collision_log = collision_log as *const CollisionLog;
        let opponent_id = (*collision_log).opponent_battle_object_id;
        let charge = WorkModule::get_float(boma, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            if opponent_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                let opponent_boma = sv_battle_object::module_accessor(opponent_id as u32);
                let opponent_kind = utility::get_kind(&mut *opponent_boma);
                if owner_kind != opponent_kind {
                    if charge == 1.0 {
                        StopModule::set_hit_stop_frame(opponent_boma, 20, true);
                    }
                }
            }
        }
        *(weapon as *mut bool).add(0x90) = true;
    }
    if weapon_kind == *WEAPON_KIND_SAMUS_CSHOT as u32 {
        *(weapon as *mut bool).add(0x90) = false;
    }
    normal_weapon_hit_handler(vtable, weapon, log)
}
*/

pub fn install() {
    //let _ = skyline::patching::Patch::in_text(0x5216060).data(samusd_cshot_on_attack as u64);
}