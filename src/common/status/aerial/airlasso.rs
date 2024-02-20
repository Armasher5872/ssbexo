use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_AirLasso)]
unsafe fn status_airlasso(fighter: &mut L2CFighterCommon, article_id: &mut L2CValue, article2_id: &mut L2CValue, cliff_data: &mut L2CValue, param_4: &mut L2CValue) -> L2CValue {
    let bool_check = true;
    let landing_air_lasso_motion_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("landing_air_lasso_motion_rate"));
    let lr = PostureModule::lr(fighter.module_accessor);
    WorkModule::set_int(fighter.module_accessor, article_id.get_i32(), *FIGHTER_STATUS_AIR_LASSO_WORK_INT_ARTICLE_ID);
    WorkModule::set_int(fighter.module_accessor, article2_id.get_i32(), *FIGHTER_STATUS_AIR_LASSO_WORK_INT_ARTICLE2_ID);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::set_int(fighter.module_accessor, landing_air_lasso_motion_rate-1, *FIGHTER_INSTANCE_WORK_ID_INT_AIR_LASSO_LANDING_FRAME);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_air_lasso_uniq(false.into());
    }
    if *FIGHTER_CLIFF_HANG_DATA_TERM <= cliff_data.get_i32() {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, cliff_data.get_u32());
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_air_lasso_uniq as *const () as _));
    if param_4.get_bool() != bool_check {
        return 0.into();
    }
    else {
        if fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_LUIGI {
            if MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("special_air_lw_end")) {
                if lr == -1.0 {
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("special_lw_shoot_l"), false, -1.0);
                }
                else {
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("special_lw_shoot"), false, -1.0);
                }
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_end"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        else {
            if MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("air_catch")) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("air_catch"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_air_lasso_main as *const () as _));
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_airlasso
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}   