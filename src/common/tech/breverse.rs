use super::*;

//=================================================================
//== FighterStatusModuleImpl::set_fighter_status_data
//=================================================================
#[skyline::hook(replace=FighterStatusModuleImpl::set_fighter_status_data)]
unsafe fn set_fighter_status_data_hook(boma: &mut BattleObjectModuleAccessor, arg2: bool, treaded_kind: i32, arg4: bool, arg5: bool, arg6: bool, log_mask_flag: u64, status_attr: u32, power_up_attack_bit: u32, arg10: u32) {
    let mut new_status_attr = status_attr;
    if boma.is_fighter() {
        // this handles turnaround special/b-reversible moves
        if (boma.kind() == *FIGHTER_KIND_LINK && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_YOUNGLINK && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_TOONLINK && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_MIIFIGHTER && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N])) {
            // if b-reverse flag does not already exist in status_attr bitmask
            if status_attr & *FIGHTER_STATUS_ATTR_START_TURN as u32 == 0 {
                // add b-reverse flag to status_attr bitmask
                new_status_attr = status_attr + *FIGHTER_STATUS_ATTR_START_TURN as u32;
            }
        }

    }
    original!()(boma, arg2, treaded_kind, arg4, arg5, arg6, log_mask_flag, new_status_attr, power_up_attack_bit, arg10)
}

pub fn install() {
    skyline::install_hooks!(
        set_fighter_status_data_hook,
    );
}