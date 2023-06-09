/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Attack S4 Hold Main, declares the Fully Charged Smash Attack variable
#[skyline::hook(replace = L2CFighterCommon_status_end_AttackS4Hold)]
unsafe fn status_attacks4hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_4_hold(fighter);
    0.into()
}

//Attack S4 Main, does the related stuff to fully charged Forward Smashes
#[skyline::hook(replace = L2CFighterCommon_status_AttackS4_Main)]
unsafe fn status_attacks4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        let combo = ComboModule::count(fighter.module_accessor) as i32;
        let s4_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("s4_combo_max"), 0);
        if combo < s4_combo_max
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            fighter.attack_s4_mtrans();
        }
    }
    else {
        fighter.attack_s4_mtrans();
    }
    /* START OF NEW ADDITIONS */
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = smash::app::utility::get_kind(boma);
    let frame = MotionModule::frame(boma);
    let cbm_vec1 = Vector4f{/* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 0.2};
    let cbm_vec2 = Vector4f{/* Red */ x: 0.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha */w: 0.8};
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
        if fighter_kind == *FIGHTER_KIND_NESS {
            if frame > 12.0 {
                ModelModule::set_mesh_visibility(boma, Hash40::new("ness_catch"), false);
                ModelModule::set_mesh_visibility(boma, Hash40::new("ness_eye"), false);
                ModelModule::set_mesh_visibility(boma, Hash40::new("ness_head"), false);
                ModelModule::set_mesh_visibility(boma, Hash40::new("ness_talk"), false);
                ModelModule::set_mesh_visibility(boma, Hash40::new("ness_facen"), false);
                ModelModule::set_mesh_visibility(boma, Hash40::new("ness_patterna"), true);
                ColorBlendModule::set_main_color(boma, /* Brightness */&cbm_vec1, /* Diffuse */&cbm_vec2, 0.21, 2.2, 5, /* Display Color */ true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ness_black_face"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        }
    }
    /* END OF NEW ADDITIONS */
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return true.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into();
    }
    0.into()
}

//Attack S4 End, clears the full smash attack flags
#[skyline::hook(replace = L2CFighterCommon_status_end_AttackS4)]
unsafe fn status_end_attacks4(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = smash::app::utility::get_kind(boma);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    if fighter_kind == *FIGHTER_KIND_NESS {
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_eye"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_facen"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_head"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_patterna"), false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("ness_black_face"), false, false);
        ColorBlendModule::cancel_main_color(boma, 0);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_attacks4hold_end,
            status_attacks4_main,
            status_end_attacks4
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}