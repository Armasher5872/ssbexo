use super::*;

pub unsafe extern "C" fn kirby_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INPUT);
}

pub unsafe extern "C" fn fun_71001a6740(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
}

pub unsafe extern "C" fn fun_710022ad50(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let finalcutter_ground_mot = WorkModule::get_int64(boma, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_GROUND_MOT);
    let finalcutter_air_mot = WorkModule::get_int64(boma, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_AIR_MOT);
    if situation_kind != *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(boma, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT) {
            MotionModule::change_motion_inherit_frame(boma, Hash40::new_raw(finalcutter_air_mot), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(boma, Hash40::new_raw(finalcutter_air_mot), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    }
    else {
        if WorkModule::is_flag(boma, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT) {
            MotionModule::change_motion_inherit_frame(boma, Hash40::new_raw(finalcutter_ground_mot), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(boma, Hash40::new_raw(finalcutter_ground_mot), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(boma, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    }
}