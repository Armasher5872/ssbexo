/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Status Pre Attack Air, used to permit momentum transfer for aerials
#[skyline::hook(replace = L2CFighterCommon_status_pre_AttackAir)]
unsafe extern "C" fn status_pre_attackair(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE as u64, *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

//Sub Attack Air Inherit Jump Aerial Motion Uniq Process Init, DJC related stuff
#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_inherit_jump_aerial_motion_uniq_process_init)]
unsafe extern "C" fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let valid_djc_kind = [*FIGHTER_KIND_NESS, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_LUCAS].contains(&kind);
    let boma = fighter.module_accessor;
    let motion_kind = MotionModule::motion_kind(boma);
    let frame  = MotionModule::frame(boma);
    fighter.sub_attack_air_kind();
    if [hash40("jump_aerial_f"), hash40("jump_aerial_b")].contains(&motion_kind) {
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
        }
        else {
            MotionModule::add_motion_2nd(boma, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
            MotionModule::set_weight(boma, 1.0, true);
            if valid_djc_kind {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || frame < 2.0 {
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
                }
                else {
                    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL); 
                }
            }
            else {
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL); 
            }
        }
    }
    else {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);   
    }
    fighter.sub_attack_air_uniq_process_init();
    0.into()
}

//Sub Attack Air Inherit Jump Aerial Motion Uniq Process Exec, DJC related stuff
#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec)]
unsafe extern "C" fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let valid_djc_kind = [*FIGHTER_KIND_NESS, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_LUCAS].contains(&kind);
    let boma = fighter.module_accessor;
    if valid_djc_kind {
        if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND {
            if MotionModule::frame_2nd(boma) >= 2.0 {
                if fighter.global_table[CURRENT_FRAME].get_i32() <= 6 {
                    if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) {
                        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
                        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                    }
                }
            }
        }
    }
    FighterUtil::check_cloud_through_out(boma);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_attackair,
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_init,
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}