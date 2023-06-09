/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Sub Escape FB Main, permits the momentum of Glide Tossing
#[skyline::hook(replace = L2CFighterCommon_sub_escape_fb_main)]
unsafe fn sub_escape_fb_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let item_lua_stack_no_throw = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW); sv_module_access::item(fighter.lua_state_agent); !fighter.pop_lua_stack(1).get_bool()};
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && ItemModule::is_have_item(fighter.module_accessor, 0)
    && item_lua_stack_no_throw {
        let escape_work_int_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
        let escape_throw_item_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("escape_throw_item_frame"));
        if escape_work_int_frame <= escape_throw_item_frame {
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 5.0);
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            return 1.into();
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) 
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 
    && !ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), false.into());
        return 0.into()
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(sub_escape_fb_main);
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}