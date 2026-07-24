/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Sub Ft Status Uniq Process Guard Off Init Status. Makes Parry only happen from Parry Input
#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOff_initStatus)]
unsafe extern "C" fn sub_ft_status_uniq_process_guard_off_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let hit_stop_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64);
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_PARRY) {
        ShieldModule::set_status(boma, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
        ShieldModule::set_shield_type(boma, ShieldType(*SHIELD_TYPE_JUST_SHIELD), *FIGHTER_SHIELD_KIND_GUARD, 0);
        fighter.FighterStatusGuard__set_just_shield_scale();
        ShieldModule::set_hit_stop_mul(boma, hit_stop_mul);
    }
    0.into()
}

//Sub Guard Off Uniq. Makes Parry only happen from Parry Input
#[skyline::hook(replace = L2CFighterCommon_sub_guard_off_uniq)]
unsafe extern "C" fn sub_guard_off_uniq(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let boma = fighter.module_accessor;
    let get_shield_type_of_guard = FighterUtil::get_shield_type_of_guard(fighter.global_table[FIGHTER_KIND].get_i32()) as i32;
    let end_frame = MotionModule::end_frame_from_hash(boma, Hash40::new("guard_off"));
    let rate = (end_frame*0.8)/(37.0-current_frame);
    if param_2.get_bool() {
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_PARRY) {
            let just_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
            if 0 < just_frame {
                WorkModule::dec_int(boma, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
                if just_frame == 1 {
                    ShieldModule::set_status(boma, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NONE), 0);
                    ShieldModule::set_shield_type(boma, ShieldType(get_shield_type_of_guard), *FIGHTER_SHIELD_KIND_GUARD, 0);
                    if FighterUtil::is_valid_just_shield_reflector(boma) {
                        ReflectorModule::set_status(boma, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
                    }
                    EffectModule::remove_common(boma, Hash40::new("just_shield"));
                    MotionModule::change_motion(boma, Hash40::new("guard_off"), end_frame*0.2, rate, false, 0.0, false, false);
                }
                else if just_frame == 0 {
                    SoundModule::stop_se(boma, Hash40::new("se_common_guardoff"), 0);
                }
            }
        }
        let cancel_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
        if 0 < cancel_frame {
            WorkModule::dec_int(boma, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
            if cancel_frame == 0 {
                CancelModule::enable_cancel(boma);
            }
        }
    }
    0.into()
}

//Status Guard Off. Parry stuff
#[skyline::hook(replace = L2CFighterCommon_status_GuardOff)]
unsafe extern "C" fn status_guard_off(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let rate = fighter.status_GuardOff_Common().get_f32();
    let shield_radius = WorkModule::get_param_float(boma, hash40("shield_radius"), 0);
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_PARRY) {
        MotionModule::change_motion(boma, Hash40::new("guard_damage"), 2.0, 0.0, false, 0.0, false, false);
        EffectModule::req_follow(boma, Hash40::new("sys_genesis_end"), Hash40::new("throw"), &Vector3f::zero(), &Vector3f::zero(), shield_radius*0.06, true, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, 0, *EFFECT_FLIP_NONE, 0, false, false);
        EffectModule::set_rate_last(boma, 1.2);
        EffectModule::req_common(boma, Hash40::new("just_shield"), 0.0);
        let se_handle = SoundModule::play_se(boma, Hash40::new("se_item_backshield_guard01"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(boma, se_handle as i32, 0.9, 0);
        SoundModule::stop_se(boma, Hash40::new("se_common_guardon"), 0);
    }
    else {
        MotionModule::change_motion(boma, Hash40::new("guard_off"), 0.0, rate, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_GuardOff_Main as *const () as _))
}

//Sub Status Guard Off Main Common Cancel. Parry stuff
#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_off_main_common_cancel)]
unsafe extern "C" fn sub_status_guard_off_main_common_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_i32();
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_PARRY) {
        if current_frame >= 37 {
            return 0.into();
        }
    }
    if CancelModule::is_enable_cancel(boma) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_PARRY) {
        return fighter.sub_wait_ground_check_common(false.into());
    }
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_ground_special().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_ground_attack().get_bool() {
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD) {
        if ItemModule::is_have_item(boma, 0) {
            return check_item_guard_off(fighter);
        }
    }
    0.into()
}

unsafe extern "C" fn check_item_guard_off(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    let cmd_cat3 = fighter.global_table[CMD_CAT3].get_i32();
    let boma = fighter.module_accessor;
    let throw = {fighter.clear_lua_stack(); lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW); sv_module_access::item(fighter.lua_state_agent); !fighter.pop_lua_stack(1).get_bool()};
    let item_throwable = ItemModule::is_have_item(boma, 0) && situation_kind == SITUATION_KIND_GROUND && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD) && throw;
    if item_throwable && ((pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) != 0 || (cmd_cat3 & (*FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI | *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI4)) != 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 1.into();
    }
    0.into()
}

//Sub Ft Status Uniq Process Guard Off Exit Status. Parry stuff
#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOff_exitStatus)]
unsafe extern "C" fn sub_ft_status_uniq_process_guard_off_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let shield_type = FighterUtil::get_shield_type_of_guard(fighter.global_table[FIGHTER_KIND].get_i32()) as i32;
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_PARRY) {
        ShieldModule::set_status(boma, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NONE), 0);
        ShieldModule::set_shield_type(boma, ShieldType(shield_type), *FIGHTER_SHIELD_KIND_GUARD, 0);
        if FighterUtil::is_valid_just_shield_reflector(boma) {
            ReflectorModule::set_status(boma, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        }
        ShieldModule::set_hit_stop_mul(boma, 1.0);
    }
    if status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
        if !WorkModule::is_flag(boma, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
            effect!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sys_shield"), true, true);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
        }
    }
    else {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_PARRY);
    }
    ShieldModule::set_shield_type(boma, ShieldType(shield_type), *FIGHTER_SHIELD_KIND_GUARD, 0);
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_ATTACK_HI4_START].contains(&status_kind) {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME);
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_ft_status_uniq_process_guard_off_init_status,
            sub_guard_off_uniq,
            status_guard_off,
            sub_status_guard_off_main_common_cancel,
            sub_ft_status_uniq_process_guard_off_exit_status
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);

}