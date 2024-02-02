/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_initStatus)]
unsafe fn sub_ftstatusuniqprocessguarddamage_initstatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_initStatus_Inner();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        fighter.FighterStatusGuard__set_just_shield_scale();
    }
    else {
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: 1.0, y: 1.0, z: 1.0});
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardDamage_common)]
unsafe fn status_guarddamage_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let shield_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_LR);
    let just_shield_precede_extension = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("just_shield_precede_extension"));
    let just_shield_se = FighterUtil::get_just_shield_se(fighter_kind);
    let team_color = FighterUtil::get_team_color(module_accessor);
    let shield_effect_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("shield_effect_color"));
    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let shield_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    ControlModule::reset_flick_x(fighter.module_accessor);
    ControlModule::reset_flick_sub_x(fighter.module_accessor);
    fighter.global_table[STICK_X].assign(&0xFE.into());
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU);
        ControlModule::set_command_life_extend(fighter.module_accessor, just_shield_precede_extension as u8);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_JUST_SHIELD);
        FighterUtil::flash_eye_info(module_accessor);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL) {
            ModelModule::enable_gold_eye(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE);
        }
        EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_just_shield"), Hash40::new("throw"), &NONE_VECTOR, &NONE_VECTOR, 1.0, &NONE_VECTOR, &NONE_VECTOR, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, *EFFECT_FLIP_NONE, 1);
        ColorBlendModule::set_last_attack_direction(fighter.module_accessor, &Vector3f{x: -shield_lr, y: 0.0, z: 0.0});
        EffectModule::req_common(fighter.module_accessor, Hash40::new("just_shield"), 0.0);
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_GUARD_OFF {
            EffectModule::req_screen(fighter.module_accessor, Hash40::new("just_shield_screen"), false, false, false);
        }
        /*   NEW ADDITIONS   */
        WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
        if fighter_kind == *FIGHTER_KIND_CAPTAIN {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_captain_appeal03"), true, false, false, false, enSEType(0));
            SoundModule::play_se(fighter.module_accessor, just_shield_se, true, false, false, false, enSEType(0));
        }
        else if fighter_kind == *FIGHTER_KIND_SONIC {
            SoundModule::play_se(fighter.module_accessor, just_shield_se, true, false, false, false, enSEType(0));
            WorkModule::add_int(fighter.module_accessor, 5, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE);
        }
        else if fighter_kind == *FIGHTER_KIND_LITTLEMAC {
            SoundModule::play_se(fighter.module_accessor, just_shield_se, true, false, false, false, enSEType(0));
            WorkModule::add_float(fighter.module_accessor, 34.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        }
        /*   END OF NEW ADDITIONS  */
        else {
            SoundModule::play_se(fighter.module_accessor, just_shield_se, true, false, false, false, enSEType(0));
        }
    }
    else {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_IS_DONE_GUARD_DAMAGE_NUM) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_GUARD_DAMAGE_NUM);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_IS_DONE_GUARD_DAMAGE_NUM);
        }
        if param_1.get_bool() {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new_raw(0x12c9377e3d), Hash40::new("throw"), &NONE_VECTOR, &NONE_VECTOR, 0.1, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, -1, *EFFECT_FLIP_NONE, 1, false, true);
            let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new_raw(0x12be304eab), Hash40::new("throw"), &NONE_VECTOR, &NONE_VECTOR, 0.1, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, -1, *EFFECT_FLIP_NONE, 1, false, true);
            EffectModule::set_rgb_partial_last(fighter.module_accessor, shield_effect_color.x, shield_effect_color.y, shield_effect_color.z);
            WorkModule::set_int(fighter.module_accessor, handle as i32, *FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE2_EFFECT_HANDLE);
            let handle_1 = EffectModule::req_follow(fighter.module_accessor, Hash40::new_raw(0x113434cb66), Hash40::new("throw"), &NONE_VECTOR, &NONE_VECTOR, 1.0, false, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, -1, *EFFECT_FLIP_NONE, 1, false, true);
            EffectModule::set_rgb_partial_last(fighter.module_accessor, shield_effect_color.x, shield_effect_color.y, shield_effect_color.z);
            WorkModule::set_int(fighter.module_accessor, handle_1 as i32, *FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE_EFFECT_HANDLE);
            if handle_1 != 0 {
                let diff = (shield_hp/shield_max).clamp(0.1, 1.0)*0.1;
                EffectModule::set_scale(fighter.module_accessor, handle_1 as u32, &Vector3f{x: diff, y: diff, z: diff});
            }
        }
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_GuardDamageUniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_GuardDamageUniq as *const () as _));
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_ftstatusuniqprocessguarddamage_initstatus,
            status_guarddamage_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}