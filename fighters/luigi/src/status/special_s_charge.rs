use super::*;

unsafe extern "C" fn luigi_special_s_charge_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CHARGE_MELEE_NO_RANDOM) {
        let discharge_prob = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_S_DISCHARGE_CHANCE);
        if discharge_prob > 1 {
            let rand_int = sv_math::rand(hash40("fighter"), discharge_prob);
            if rand_int <= 1 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE);
            }
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE);
        }
    }
    EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, luigi_special_s_charge_end_status)
    .install()
    ;
}