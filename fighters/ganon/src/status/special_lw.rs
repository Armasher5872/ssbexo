use super::*;

unsafe extern "C" fn ganon_special_lw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    WorkModule::set_float(fighter.module_accessor, get_sum_speed_x, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_GANON_KICK_SPEED);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_GANOND, ArticleOperationTarget(0));
    0.into()
}

unsafe extern "C" fn ganon_special_lw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_GANOND, ArticleOperationTarget(0));
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, ganon_special_lw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, ganon_special_lw_exit_status)
    .install()
    ;
}