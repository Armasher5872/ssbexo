use super::*;

unsafe extern "C" fn lucario_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let frame = MotionModule::frame(boma);
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
    && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
        WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    };
    if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW) {
        MotionModule::set_rate(boma, 1.65);
    }
    if MotionModule::end_frame(boma) - frame <= 2.0
    || CancelModule::is_enable_cancel(boma) {
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
        MotionModule::set_rate(boma, 1.0);
    };
    if [hash40("appeal_hi_r"), hash40("appeal_hi_l")].contains(&motion_kind)
    && frame > 16.0
    && MEGA_EVOLVE[entry_id] != true {
        MEGA_EVOLVE[entry_id] = true;
    }
    if [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind)
    && frame > 43.0
    && MEGA_EVOLVE[entry_id] != false {
        MEGA_EVOLVE[entry_id] = false;
    }
    if status_kind == *FIGHTER_STATUS_KIND_FINAL {
        MEGA_EVOLVE[entry_id] = false;
    }
    if MEGA_EVOLVE[entry_id] == true {
        VisibilityModule::set_whole(boma, false);
        ArticleModule::generate_article(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, false, -1);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, true, ArticleOperationTarget(0));
    }
    else {
        VisibilityModule::set_whole(boma, true);
        ArticleModule::remove_exist(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::set_visibility_whole(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, false, ArticleOperationTarget(0));
    }
    //Up Special
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind) {
        fighter.sub_transition_group_check_air_cliff();
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    };
}

unsafe extern "C" fn lucariom_frame(weapon: &mut L2CFighterBase) {
    let lucario_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let lucario_boma = sv_battle_object::module_accessor(lucario_id);
    let lucario_motion = MotionModule::motion_kind(lucario_boma);
    let lucario_frame = MotionModule::frame(lucario_boma);
    let lucario_rate = MotionModule::rate(lucario_boma);
    let lucario_status_kind = StatusModule::status_kind(lucario_boma);
    let lucario_lr = PostureModule::lr(lucario_boma);
    let lucario_scale = PostureModule::scale(lucario_boma);
    let lucario_pos = PostureModule::pos(lucario_boma);
    let lucario_x = PostureModule::pos_x(lucario_boma);
    let lucario_y = PostureModule::pos_y(lucario_boma);
    let lucario_z = PostureModule::pos_z(lucario_boma);
    let mut lucario_vector = Vector3f{x: lucario_x, y: lucario_y, z: lucario_z};
    let lucario_situation_kind = StatusModule::situation_kind(lucario_boma);
    if StatusModule::status_kind(weapon.module_accessor) != lucario_status_kind {
        StatusModule::change_status_force(weapon.module_accessor, lucario_status_kind, true);
    }
    if StatusModule::is_situation_changed(weapon.module_accessor) {
        StatusModule::set_situation_kind(weapon.module_accessor, SituationKind(lucario_situation_kind), true);
    }
    if MotionModule::motion_kind(weapon.module_accessor) != lucario_motion {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new_raw(lucario_motion), lucario_frame, lucario_rate , false, 0.0, false, false);
        ArticleModule::change_motion(weapon.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, Hash40::new_raw(lucario_motion), false, -1.0);
    }
    if MotionModule::frame(weapon.module_accessor) != lucario_frame {
        ArticleModule::set_frame(weapon.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, lucario_frame);
        MotionModule::set_frame(weapon.module_accessor, lucario_frame, true);
    }
    if MotionModule::rate(weapon.module_accessor) != lucario_rate {
        ArticleModule::set_rate(weapon.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, lucario_rate);
        MotionModule::set_rate(weapon.module_accessor, lucario_rate);
    }
    if PostureModule::lr(weapon.module_accessor) != lucario_lr {
        PostureModule::set_lr(weapon.module_accessor, lucario_lr);
        PostureModule::update_rot_y_lr(weapon.module_accessor);
    }
    if PostureModule::scale(weapon.module_accessor) != lucario_scale {
        PostureModule::set_scale(weapon.module_accessor, lucario_scale, false);
    }
    if PostureModule::pos(weapon.module_accessor) != lucario_pos
    || PostureModule::pos_x(weapon.module_accessor) != lucario_x
    || PostureModule::pos_y(weapon.module_accessor) != lucario_y
    || PostureModule::pos_z(weapon.module_accessor) != lucario_z {
        ArticleModule::set_pos(weapon.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, lucario_vector);
        PostureModule::set_pos(weapon.module_accessor, lucario_pos);
        MotionModule::joint_local_tra(weapon.module_accessor, Hash40::new("trans"), true, &mut lucario_vector);
        MotionModule::joint_local_tra(weapon.module_accessor, Hash40::new("rot"), true, &mut lucario_vector);
        MotionModule::joint_local_tra(weapon.module_accessor, Hash40::new("hip"), true, &mut lucario_vector);
    }
}

pub fn install() {
    Agent::new("lucario")
    .on_line(Main, lucario_frame)
    .install()
    ;
    Agent::new("lucario_lucariom")
    .on_line(Main, lucariom_frame)
    .install()
    ;
}