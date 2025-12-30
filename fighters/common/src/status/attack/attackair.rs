/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Status Pre Attack Air, used to permit momentum transfer for aerials
#[skyline::hook(replace = L2CFighterCommon_status_pre_AttackAir)]
unsafe extern "C" fn status_pre_attackair(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE as u64, *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

//Status Attack Air Main Common, used for continual platform drops and ECB Shifts
#[skyline::hook(replace = L2CFighterCommon_status_AttackAir_Main_common)]
unsafe extern "C" fn status_attackair_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.attack_air_common_strans().get_bool() {
        /* START OF NEW ADDITIONS */
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
        if prev_status_kind == *FIGHTER_STATUS_KIND_PASS {
            if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                GroundModule::set_passable_check(boma, true);
            }
        }
        /* END OF NEW ADDITIONS */
        if !CancelModule::is_enable_cancel(boma) {
            if MotionModule::is_end(boma) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            return false.into();
        }
        else {
            if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return true.into();
                }
                if !MotionModule::is_end(boma) {
                    return false.into();
                }
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
    }
    true.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_attackair,
            status_attackair_main_common
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}