use super::*;

//Sub Attack XX4 Common Unique Process Exit, clears the full smash attack flags and COUNTER! related stuff
#[skyline::hook(replace = L2CFighterCommon_sub_attack_xx4_common_uniq_process_exit)]
unsafe fn sub_attack_xx4_common_uniq_process_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind_interrupt)
    || ![*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
        AttackModule::set_shield_stiff_mul(fighter.module_accessor, 1.0);
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
        SlowModule::clear_whole(boma);
        CameraModule::reset_all(boma);
        CAM_ZOOM_OUT(fighter);
        return 0.into();
    }
    if [*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind_interrupt) {
        if ![*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
            fighter.sub_end_attack_s4_turn_rev();
        }
    }
    if 0 < log_attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log_attack_kind);
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(sub_attack_xx4_common_uniq_process_exit);
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}