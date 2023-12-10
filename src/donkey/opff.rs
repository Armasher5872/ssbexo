use super::*;

unsafe extern "C" fn donkey_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let timer = WorkModule::get_int(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
    //Taunts
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
        if [hash40("appeal_hi_r"), hash40("appeal_hi_l")].contains(&motion_kind) && frame >= 48.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                MotionModule::set_frame_sync_anim_cmd(boma, 32.0, true, true, false);
            };
        }
    };
    //Cargo
    if status_kind == *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START {
        if motion_kind == hash40("throw_f") {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_hi"), 0.0, 1.0, false, 0.0, false, false);
        }
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_CATCH_PULL {
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        }
    }
    //Up B
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
        let barrel_boma = get_article_boma(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL);
        let barrel_frame = MotionModule::frame(barrel_boma);
        if barrel_frame > 40.0 {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    //Down Special
    if timer > 0 {
        WorkModule::dec_int(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
    }
    if timer <= 0 && WorkModule::is_flag(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE) {
        WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
        fighter.gimmick_flash();
    }
    println!("Has Catch: {}", HAS_CATCH[entry_id]);
}

pub fn install() {
    Agent::new("donkey")
    .on_line(Main, donkey_frame)
    .install()
    ;
}