use super::*;

unsafe extern "C" fn edge_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let frame = MotionModule::frame(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    //Sephiroth Taunt Holding
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
        if [hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&motion_kind)
        && frame >= 67.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) 
            || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R)
            || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                MotionModule::set_frame_sync_anim_cmd(boma, 67.0, true, true, false);
            }
        }
        if [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind)
        && frame >= 80.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                MotionModule::set_frame_sync_anim_cmd(boma, 40.0, true, true, false);
            }
        }
    }
    //Up Special Early Cancel
    if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            WorkModule::set_flag(boma, true, FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_RUSH_CANCEL);
        }
    }
    if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING
    && WorkModule::is_flag(boma, FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_RUSH_CANCEL) {
        if frame >= 9.0 {
            CancelModule::enable_cancel(boma);
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if [*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
            if WorkModule::is_flag(boma, FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_RUSH_CANCEL) {
                CancelModule::enable_cancel(boma);
            }
        }
        //Early Detonation
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if (7.0..27.0).contains(&frame)
            && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_HIT, false);
            }
        }
    }
}

unsafe extern "C" fn edge_flaredummy_frame(weapon: &mut L2CFighterBase) {
    let owner_boma = get_owner_boma(weapon);
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    let owner_prev_status_kind = StatusModule::prev_status_kind(owner_boma, 0);
    if StatusModule::status_kind(weapon.module_accessor) == *WEAPON_EDGE_FLAREDUMMY_STATUS_KIND_FLY {
        if WorkModule::is_flag(owner_boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
            if owner_status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL
            && owner_prev_status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CHARGE {
                weapon.change_status(WEAPON_EDGE_FLAREDUMMY_STATUS_KIND_TRY.into(), false.into());
            }
        }
    }
}

pub fn install() {
    Agent::new("edge")
    .on_line(Main, edge_frame)
    .install()
    ;
    Agent::new("edge_flaredummy")
    .on_line(Main, edge_flaredummy_frame)
    .install()
    ;
}