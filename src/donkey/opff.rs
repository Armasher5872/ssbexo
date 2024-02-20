use super::*;

unsafe extern "C" fn donkey_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let frame = MotionModule::frame(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
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
    println!("Has Catch: {}", WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH));
}

unsafe extern "C" fn donkey_init(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    //Universal
    fighter.global_table[THROW_F_STATUS_KIND].assign(&FIGHTER_STATUS_KIND_THROW.into());
	fighter.global_table[THROW_HI_STATUS_KIND].assign(&FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START.into());
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    FIGHTER_BOOL_1[entry_id] = false;
    FIGHTER_BOOL_2[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
    WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    SPECIAL_WALL_JUMP = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    LAST_DAMAGE[entry_id] = 0.0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SIZE2[entry_id] = 0.0;
    SIZE3[entry_id] = 0.0;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    //Donkey Kong
    WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
    WorkModule::set_int(boma, 0, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
}

pub fn install() {
    Agent::new("donkey")
    .on_start(donkey_init)
    .on_line(Main, donkey_frame)
    .install()
    ;
}