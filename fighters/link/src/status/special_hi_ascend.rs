use super::*;

unsafe extern "C" fn link_special_hi_ascend_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_CLIFF, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    GroundModule::set_ignore_boss(fighter.module_accessor, true);
    GroundModule::set_passable_check(fighter.module_accessor, false);
    GroundModule::set_collidable(fighter.module_accessor, false);
    JostleModule::set_status(fighter.module_accessor, false);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_ascend_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_ascend"), 0.0, 1.0, false, 0.0, false, false);
    let cbm_t_vec1 = Vector4f{ /* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 1.0};
    let cbm_t_vec2 = Vector4f{ /* Red */ x: 0.25, /* Green */ y: 1.0, /* Blue */ z: 0.75, /* Alpha */ w: 0.5};
    ColorBlendModule::set_main_color(fighter.module_accessor, /* Brightness */ &cbm_t_vec1, /* Diffuse */ &cbm_t_vec2, 0.21, 2.2, 3, /* Display Color */ true);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_ascend_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_ascend_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ASCEND_FRAME);
    let target_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
    let height = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
    let mut max_y = target_y+height+20.0;
    let modulo = current_frame % 10.0;
    WorkModule::add_float(fighter.module_accessor, 1.0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ASCEND_FRAME);
    macros::SET_SPEED_EX(fighter, 0.0, 3.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if GroundModule::ray_check_hit_pos(fighter.module_accessor, &Vector2f{x: pos_x, y: pos_y+4.0}, &Vector2f{x: 0.0, y: -height/1.5}, ground_hit_pos, true) && pos_y >= max_y-(height*2.0) {
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos_x, y: ground_hit_pos.y, z: 0.0});
        GroundModule::set_attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND_END.into(), false.into());
        return 0.into();
    }
    if modulo < 1.0 {
        if GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos_x, y: target_y+5.0}, &Vector2f{x: 0.0, y: -10.0}, true) != 1 {
            if GroundModule::ray_check_hit_pos(fighter.module_accessor, &Vector2f{x: pos_x, y: target_y+20.0}, &Vector2f{x: 0.0, y: -40.0}, ground_hit_pos, true) {
                WorkModule::set_float(fighter.module_accessor, ground_hit_pos.y, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
                max_y = ground_hit_pos.y;
            }
            else {
                max_y = -999.0;
            }
        }
    }
    if pos_y > max_y {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_damage_paralyze"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::PLAY_SE(fighter, Hash40::new("vc_link_damage01"));
        macros::SET_SPEED_EX(fighter, 0.0, 2.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        PostureModule::add_pos(fighter.module_accessor, &Vector3f{ x: 0.0, y: 11.6, z: 0.0});
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_ascend"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_ascend_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_special_hi_ascend_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
    GroundModule::set_rhombus_offset(fighter.module_accessor, &Vector2f{ x: 0.0, y: 0.0});
    macros::COL_NORMAL(fighter);
    macros::BURN_COLOR_NORMAL(fighter);
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"),false,false);
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND, link_special_hi_ascend_pre_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND, link_special_hi_ascend_main_status)
    .status(Exec, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND, link_special_hi_ascend_exec_status)
    .status(Exit, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_ASCEND, link_special_hi_ascend_exit_status)
    .install()
    ;
}