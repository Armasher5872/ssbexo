use super::*;

//Status Attack Hi3 Main, adds the momentum to Samus's DACUT
#[skyline::hook(replace = L2CFighterCommon_status_AttackHi3_Main)]
unsafe fn status_attackhi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = smash::app::utility::get_kind(boma);
    let frame = MotionModule::frame(boma);
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 0.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE) {
        if !StatusModule::is_changing(fighter.module_accessor) {
            let combo = ComboModule::count(fighter.module_accessor) as i32;
            let s3_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("s3_combo_max"), 0);
            if combo < s3_combo_max
            || (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE)
            && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)) {
                fighter.attack_s3_mtrans_param(FIGHTER_COMBO_KIND_S3.into());
            }
        }
        else {
            fighter.attack_s3_mtrans_param(FIGHTER_COMBO_KIND_S3.into());
        }
    }
    /* START OF NEW ADDITIONS */
    if fighter_kind == *FIGHTER_KIND_SAMUS && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_DASH && frame == 1.0 {
        macros::SET_SPEED_EX(fighter, 0.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    /* END OF NEW ADDITIONS */
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if fighter.status_AttackHi3_Main_minjump().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(status_attackhi3_main);
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}