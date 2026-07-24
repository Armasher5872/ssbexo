use super::*;

unsafe extern "C" fn edge_guard_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_guard_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_guard_main_loop as *const () as _))
}

unsafe extern "C" fn edge_guard_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let min_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
    let shield_min_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("shield_min_frame"));
    if !fighter.status_guard_main_common_air().get_bool() {
        if !fighter.sub_guard_cont().get_bool() {
            if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if min_frame <= 0 {
                    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                        fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), true.into());
                        return 1.into();
                    }
                }
            }
            if min_frame > 0 && min_frame < shield_min_frame {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD_HOLD) {
                    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_PARRY);
                    fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), false.into());
                }
            }
            if edge_check_valid_wing_enable(boma) {
                fighter.change_status(FIGHTER_EDGE_STATUS_KIND_WING_ACTIVATE.into(), false.into());
                return 0.into();
            }
            return 0.into();
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_GUARD, edge_guard_main_status)
    .install()
    ;
}