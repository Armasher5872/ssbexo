use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageSleepFall_Main)]
unsafe fn status_damagesleepfall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let damage_song_fall_effect_interval = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_song_fall_effect_interval"));
    let hash_1 = Hash40::new_raw(0x1a786b5238);
    let sleep_effect_bone_hash = WorkModule::get_param_int64(fighter.module_accessor, hash_1.hash as u64, 0);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT) {
        WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    }
    asdi_check(fighter);
    asdi_function(fighter);
    if fighter_kind == *FIGHTER_KIND_PICKEL {
        if fighter.global_table[IS_STOP].get_bool() {
            COL_NORMAL(fighter);
            FLASH(fighter, 2.55, 0.0, 0.0, 0.25);
        }
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        if ControlModule::get_clatter_time(fighter.module_accessor, 0) > 0.0 {
            if fighter.global_table[IS_STOP].get_bool() {
                return 0.into();
            }
            if current_frame as i32 % damage_song_fall_effect_interval != 0 {
                return 0.into();
            }
            //Don't know exactly what the original is doing
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new_raw(sleep_effect_bone_hash), &Vector3f::zero(), &Vector3f::zero(), 1.0, false, 0.0 as u32, -1, 0, 0, 0, false, false);
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    else {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new_raw(0x1cb547446b), -1);
        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_STATUS_KIND_SLEEP_FALL {
            fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_SLEEP.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_SLEEP.into(), false.into());
        }
    }
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_DamageSleepFall)]
unsafe fn status_end_damagesleepfall(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let flags = [
        *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT, *FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE
    ];
    for x in 0..flags.len() {
        WorkModule::off_flag(fighter.module_accessor, flags[x]);
    }
    if ![*FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_SLEEP].contains(&status_kind) {
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("damage_song"));
    }
    if MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("fill_blank_sleep")) {
        FighterUtil::cancel_face_motion_by_priority(module_accessor, FighterFacial(*FIGHTER_FACIAL_SLEEP));
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_damagesleepfall_main,
            status_end_damagesleepfall
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}