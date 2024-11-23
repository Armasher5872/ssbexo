use super::*;

unsafe extern "C" fn lucariom_frame(weapon: &mut L2CFighterBase) {
    let lucario_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let lucario_boma = sv_battle_object::module_accessor(lucario_id);
    let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
    let status_kind = StatusModule::status_kind(weapon.module_accessor);
    let situation_kind = StatusModule::situation_kind(weapon.module_accessor);
    let rate = MotionModule::rate(weapon.module_accessor);
    let lr = PostureModule::lr(weapon.module_accessor);
    let scale = PostureModule::scale(weapon.module_accessor);
    let lucario_situation_kind = StatusModule::situation_kind(lucario_boma);
    let lucario_motion_kind = MotionModule::motion_kind(lucario_boma);
    let lucario_frame = MotionModule::frame(lucario_boma);
    let lucario_rate = MotionModule::rate(lucario_boma);
    let lucario_lr = PostureModule::lr(lucario_boma);
    let lucario_scale = PostureModule::scale(lucario_boma);
    if ![*WEAPON_LUCARIO_LUCARIOM_STATUS_KIND_FINAL_START, *WEAPON_LUCARIO_LUCARIOM_STATUS_KIND_FINAL_ENTRY, *WEAPON_LUCARIO_LUCARIOM_STATUS_KIND_FINAL, *WEAPON_LUCARIO_LUCARIOM_STATUS_KIND_FINAL_END].contains(&status_kind) {
        if motion_kind != lucario_motion_kind {
            MotionModule::change_motion(weapon.module_accessor, Hash40::new_raw(lucario_motion_kind), lucario_frame, lucario_rate, false, 0.0, false, false);
        }
        if situation_kind != lucario_situation_kind {
            StatusModule::set_situation_kind(weapon.module_accessor, SituationKind(lucario_situation_kind), true);
        }
        if rate != lucario_rate {
            MotionModule::set_rate(weapon.module_accessor, lucario_rate);
        }
        if lr != lucario_lr {
            PostureModule::set_lr(weapon.module_accessor, lucario_lr);
        }
        if scale != lucario_scale {
            PostureModule::set_scale(weapon.module_accessor, lucario_scale, true);
        }
        if [hash40("wait"), hash40("wait_item"), hash40("squat_wait"), hash40("attack_100")].contains(&motion_kind) {
            if !StatusModule::is_changing(weapon.module_accessor) && !StatusModule::is_situation_changed(weapon.module_accessor) && MotionModule::is_end(weapon.module_accessor) {
                MotionModule::change_motion(weapon.module_accessor, Hash40::new_raw(lucario_motion_kind), lucario_frame, lucario_rate, false, 0.0, false, false);
            }
        }
    }
    if status_kind == WEAPON_LUCARIO_LUCARIOM_STATUS_KIND_MEGA_EVOLVE {
        if [hash40("mega_evolve"), hash40("mega_evolve_air")].contains(&motion_kind) {
            if MotionModule::is_end(weapon.module_accessor) {
                if lucario_situation_kind == *SITUATION_KIND_AIR {
                    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(weapon.module_accessor, Hash40::new("wait"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
    }
}

pub fn install() {
    Agent::new("lucario_lucariom")
    .on_line(Main, lucariom_frame)
    .install()
    ;
}