use super::*;

unsafe extern "C" fn gamewatch_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_SPECIAL_HI);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, false, -1);
    if situation_kind == *SITUATION_KIND_AIR {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, Hash40::new("special_air_hi"), false, -1.0);
    }
    else {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, Hash40::new("special_hi"), false, -1.0);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi"), L2CValue::Hash40s("special_air_hi"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(gamewatch_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn gamewatch_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if !MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE) {
                    let status;
                    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
                    if 0.0 >= landing_frame {
                        status = *FIGHTER_STATUS_KIND_LANDING;
                    }
                    else {
                        status = *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL;
                    }
                    fighter.change_status(status.into(), false.into());
                    return 1.into();
                }
            }
            return 0.into();
        }
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    1.into()
}

pub fn install() {
    Agent::new("gamewatch")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, gamewatch_special_hi_main_status)
    .install()
    ;
}