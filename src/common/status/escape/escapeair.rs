/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Airdodge Pre
#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE, symbol = "_ZN7lua2cpp16L2CFighterCommon20status_pre_EscapeAirEv")]
pub unsafe fn status_pre_escapeair(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Automatically forces you to the ground if you're buffering Wavedashes during the startup of Airdodge
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let y = ControlModule::get_stick_y(boma);
    //Handles wavedash
    if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH) && y < 0.5 && (ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI)) {
        GroundModule::attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }
    call_original!(fighter)
}

//Escape Air
#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN, symbol = "_ZN7lua2cpp16L2CFighterCommon16status_EscapeAirEv")]
unsafe fn status_escapeair(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_escape_air_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_slide"), 0.0, 1.0, false, 0.0, false, false);
    } 
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(status_escapeair_main as *const () as _))
}

unsafe extern "C" fn status_escapeair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let escape_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
    let escape_throw_item_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("escape_throw_item_frame"));
    let agt_window = {escape_frame <= escape_throw_item_frame};
    let throwable = !fighter.pop_lua_stack(1).get_bool();
    let lasso_type = WorkModule::get_param_int(fighter.module_accessor, hash40("air_lasso_type"), 0);
    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    /* START OF NEW ADDITION */
    //Rivals of Aether Momentum
    if (18.0..=34.0).contains(&frame) {
        KineticModule::unable_energy_all(boma);
        KineticModule::clear_speed_all(boma);
    }
    if frame > 34.0 {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.sub_transition_group_check_air_cliff();
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    //Aerial Glide Tossing
    if ![hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) 
        && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
        && ItemModule::is_have_item(fighter.module_accessor, 0)
        && agt_window {
            fighter.clear_lua_stack(); 
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW); 
            sv_module_access::item(fighter.lua_state_agent); 
            if throwable {
                let stick_x = fighter.global_table[STICK_X].get_f32();
                let stick_y = fighter.global_table[STICK_Y].get_f32();
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 5.0*stick_x.abs());
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 5.0*stick_y.abs());
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                return 1.into();
            }
        }
    }
    //Airdodge Canceled Zair
    if agt_window {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO)
        && lasso_type != *FIGHTER_AIR_LASSO_TYPE_NONE
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
        && !LinkModule::is_link(fighter.module_accessor, *FIGHTER_LINK_NO_CONSTRAINT) {
            fighter.change_status(FIGHTER_STATUS_KIND_AIR_LASSO.into(), false.into());
            return 1.into();
        }
    }
    else {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
    }
    /* END OF NEW ADDITION */
    0.into()
}

//Sub Escape Air Common
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_escape_air_common)]
unsafe fn sub_escape_air_common(fighter: &mut L2CFighterCommon) {
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    let transition_term = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL
    ];
    for x in transition_term.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_NO_WATER_INOUT_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_escape_air_uniq(L2CValue::Bool(false));
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_escape_air_uniq as *const () as _));
}

//Sub Escape Air Uniq
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_escape_air_uniq)]
pub unsafe fn sub_escape_air_uniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
    }
    else {
        let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
        if frame <= WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("item_air_catch_frame_escape")) {
            fighter.sub_GetLightItemImm(L2CValue::Void());
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE)
            && ItemModule::is_have_item(fighter.module_accessor, 0)
            && frame <= WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("escape_throw_item_frame")) {
            fighter.sub_AIRChkDropItemImm();
        }
        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
                fighter.exec_escape_air_slide();
            }
            if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME) {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_FLAG_HIT_XLU) {
                    let stale_motion_rate = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_FLOAT_MOTION_RATE_PENALTY);
                    MotionModule::set_rate(fighter.module_accessor, stale_motion_rate);
                    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
                }
            }
            if StatusModule::is_changing(fighter.module_accessor) && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_STIFF) {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_END_STIFF)
                    && CancelModule::is_enable_cancel(fighter.module_accessor) {
                    MotionModule::set_rate(fighter.module_accessor, 1.0);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_END_STIFF);
                }
            } 
            else if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_STIFF) {
                let start_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
                let frame = MotionModule::frame(fighter.module_accessor);
                let end_frame = MotionModule::end_frame(fighter.module_accessor);
                if 0.0 <= start_frame && start_frame <= frame {
                    let mut cancel_frame = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
                        WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_cancel_frame"))
                    } 
                    else {
                        WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_cancel_frame"))
                    };
                    if cancel_frame < 0.0 {
                        cancel_frame = end_frame;
                    }
                    if WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME) <= frame {
                        WorkModule::set_float(fighter.module_accessor, end_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
                    }
                    let stiff_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
                    let ratio = (cancel_frame - frame) / (stiff_frame - frame);
                    let new_rate = ratio * MotionModule::rate(fighter.module_accessor);
                    MotionModule::set_rate(fighter.module_accessor, new_rate);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_STIFF);
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_FALL) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL);
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
                    fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
                }
            }
            fighter.sub_fall_common_uniq(param_1);
        }
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_escape_air_common,
            sub_escape_air_uniq
        );
    }
}

pub fn install() {
	install_status_scripts!(
        status_pre_escapeair,
        status_escapeair
    );
    skyline::nro::add_hook(nro_hook);
}