use super::*;

unsafe extern "C" fn sonic_final_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_SUPERSONIC, false, ArticleOperationTarget(0));
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_CMD_LINK_CONSTRAINT, *FIGHTER_SONIC_LINK_NO_SUPER_SONIC, Hash40::new("top"), Hash40::new("top"), *CONSTRAINT_FLAG_MTX);
    sv_module_access::link(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    LinkModule::set_attribute(fighter.module_accessor, *FIGHTER_SONIC_LINK_NO_SUPER_SONIC, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_VISIBILITY as u8}, false);
    LinkModule::set_attribute(fighter.module_accessor, *FIGHTER_SONIC_LINK_NO_SUPER_SONIC, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SLOW as u8}, false);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE_DAMAGE);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    AreaModule::set_whole(fighter.module_accessor, false);
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("final_start"), L2CValue::Hash40s("final_air_start"), false.into());
    start_final_sonic(global_fighter);
    fun_71000129e0(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_final_main_loop as *const () as _))
}

unsafe extern "C" fn fun_71000129e0(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_depth_offset(fighter.module_accessor, 0.0);
    LinkModule::set_node_depth_offset(fighter.module_accessor, *LINK_NO_ARTICLE, 0.0);
    0.into()
}

unsafe extern "C" fn sonic_final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("final_start"), -1.0, 1.0, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("final_air_start"), -1.0, 1.0, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_FINAL_FLAG_FINAL_CAMERA) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_FINAL_FLAG_FINAL_CAMERA_DONE) {
            start_final_sonic_camera(global_fighter);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_FINAL_FLAG_FINAL_CAMERA_DONE);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("sonic")
    .status(Main, *FIGHTER_STATUS_KIND_FINAL, sonic_final_main_status)
    .install()
    ;
}