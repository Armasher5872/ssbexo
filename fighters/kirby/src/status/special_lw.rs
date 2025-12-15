use super::*;

/*   STONE STATUS SCRIPTS   */

unsafe extern "C" fn kirby_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Makes it so Kirby retains his previous momentum before he activated Stone
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_lw") as i64, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw") as i64, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_AIR_MOT);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, false, -1);
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, false, ArticleOperationTarget(0));
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        fun_71001a6740(fighter);
    }
    else {
        let speed = smash::phx::Vector3f{ x: KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)*PostureModule::lr(fighter.module_accessor), y: 0.0, z: 0.0 };
        KineticModule::add_speed(fighter.module_accessor, &speed);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_lw_loop as *const () as _))
}

unsafe extern "C" fn kirby_special_lw_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Vanilla (to the best of my knowledge) (╯°□°)╯︵ ┻━┻
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let effect_handle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_STONE_EFFECT_HANDLE);
    if !fun_7100229ec0(fighter) {
        if situation_kind != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_ESCAPE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
        fun_7100229610(fighter);
    }
    if fun_710022a090(fighter) {
        fun_71001a6740(fighter);
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_EFFECT_ONOFF) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_PREV_EFFECT_ONOFF);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_PREV_EFFECT_ONOFF);
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_PREV_EFFECT_ONOFF) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_EFFECT_ONOFF) {
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_EFFECT_REMOVE, effect_handle);
            sv_module_access::effect(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_STONE_EFFECT_HANDLE);
        }
        else {
            //Normally called as fun_7100191510
            let stone_effect_handle = {fighter.clear_lua_stack(); lua_args!(fighter, Hash40::new("kirby_stone_s"), Hash40::new("rot"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, false, 0, 0, 0); sv_module_access::effect(fighter.lua_state_agent); fighter.pop_lua_stack(1)};
            WorkModule::set_int(fighter.module_accessor, stone_effect_handle.into(), *FIGHTER_KIRBY_STATUS_WORK_ID_INT_STONE_EFFECT_HANDLE);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_STONE_STONE.into(), false.into());
        return true.into();
    }    
    0.into()
}

unsafe extern "C" fn fun_7100229ec0(fighter: &mut L2CFighterCommon) -> bool {
    //Vanilla (to the best of my knowledge)
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND) {
            return true.into();
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END) {
            return false.into();
        }
    }
    else {
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn fun_7100229610(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Vanilla (to the best of my knowledge)
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let ground_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_GROUND_MOT);
    let air_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_AIR_MOT);
    if situation_kind != *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(air_mot), -1.0, 1.0, 0.0, false, false);
            return air_mot.into();
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(air_mot), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT) {
            return ground_mot.into();
        }
        else{
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(ground_mot), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
        }
    }
    return (*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT).into()
}

unsafe extern "C" fn fun_710022a090(fighter: &mut L2CFighterCommon) -> bool {
    //Vanilla (to the best of my knowledge)
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND) {
            if situation_kind == *SITUATION_KIND_GROUND {
                return true.into();
            }
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END) {
            return false.into();
        }
        else {
            if MotionModule::is_end(fighter.module_accessor) {
                return false.into();
            }
        }
    }
    else {
        return true.into();
    }
    false.into()
}

pub fn install() {
    Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, kirby_special_lw_main_status)
    .install()
    ;
}