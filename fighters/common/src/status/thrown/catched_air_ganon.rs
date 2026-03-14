use super::*;

//Status Catched Air Ganon
#[skyline::hook(replace = L2CFighterCommon_status_CatchedAirGanon)]
unsafe extern "C" fn status_catched_air_ganon(fighter: &mut L2CFighterCommon) -> L2CValue {
    let parent_id = LinkModule::get_parent_id(fighter.module_accessor, *LINK_NO_CAPTURE, true);
    let parent_boma = sv_battle_object::module_accessor(parent_id as u32);
    let motion_share = WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("motion_share"));
    let ganon_special_s_fall_clatter_add_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("ganon_special_s_fall_clatter_add_frame"));
    let ganon_special_s_fall_clatter_add_frame_up_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("ganon_special_s_fall_clatter_add_frame_up_limit"));
    let ganon_special_s_fall_clatter_add_frame_down_limit /*LStack_e0*/ = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("ganon_special_s_fall_clatter_add_frame_down_limit"));
    let ganon_special_s_fall_hold_frame /*LStack_100*/ = if !is_armstrong_slots(parent_boma) {WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("ganon_special_s_fall_hold_frame"))} else {300.0};
    let ganon_special_s_fall_clatter_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("ganon_special_s_fall_clatter_frame"));
    let damage = DamageModule::damage(fighter.module_accessor, 0);
    let parent_damage = DamageModule::damage(parent_boma, 0);
    let mut motion_kind = hash40("catched_air_ganon");
    let damage_sum = damage-parent_damage;
    let damage_clatter_add = damage_sum*ganon_special_s_fall_clatter_add_frame;
    let base_clatter ;
    if motion_share != *FIGHTER_MOTION_SHARE_TYPE_TARO {
        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
            motion_kind = FighterMotionModuleImpl::add_body_type_hash(fighter.module_accessor, Hash40::new_raw(motion_kind), *BODY_TYPE_MOTION_GIRL);
        }
    }
    else {
        motion_kind = FighterMotionModuleImpl::add_body_type_hash(fighter.module_accessor, Hash40::new_raw(motion_kind), *BODY_TYPE_MOTION_DX);
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_kind), 0.0, 1.0, false, 0.0, false, false);
    if ganon_special_s_fall_clatter_add_frame_up_limit >= damage_clatter_add {
        //lstack_100 = -ganon_special_s_fall_clatter_add_frame_down_limit
        /*
        if damage_clatter_add < -ganon_special_s_fall_clatter_add_frame_down_limit {
            base_clatter = ganon_special_s_fall_clatter_add_frame_down_limit;
            /*
            lib::L2CValue::operator-(&LStack_e0,(L2CValue *)&LStack_100);
            lib::L2CValue::operator=(&LStack_f0,(L2CValue *)&LStack_100);
            */
        }
        */
        base_clatter = ganon_special_s_fall_clatter_add_frame_down_limit;
    }
    else {
        base_clatter = ganon_special_s_fall_clatter_add_frame_up_limit;
    }
    ControlModule::start_clatter(fighter.module_accessor, ganon_special_s_fall_hold_frame+base_clatter, 1.0, ganon_special_s_fall_clatter_frame, 127, 0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchedAirGanon_Main as *const () as _))
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_catched_air_ganon
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}