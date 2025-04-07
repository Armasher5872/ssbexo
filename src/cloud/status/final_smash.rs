use super::*;

unsafe extern "C" fn cloud_final_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let limit_level = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
    if status_kind != *FIGHTER_CLOUD_STATUS_KIND_FINAL_DASH {
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("cloud_final_limitbreak"));
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("cloud_final_aura"), false, true);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_FINAL_FLAG_DISP_WINDOW_PREV) {
            display_final_window(false);
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d68));
    }
    if limit_level <= 4 && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM);
    }
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .status(End, *FIGHTER_STATUS_KIND_FINAL, cloud_final_end_status)
    .install()
    ;
}