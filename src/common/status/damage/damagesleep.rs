use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageSleep_Main)]
unsafe fn status_damagesleep_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let hash_1 = Hash40::new_raw(0x1a786b5238);
    let hash_2 = Hash40::new_raw(0xca8fd5eb1);
    let hash_3 = Hash40::new_raw(0xf8c14990b);
    let hash_4 = Hash40::new_raw(0xb95b751c9);
    let hash_5 = Hash40::new_raw(0xbff07713b);
    let damage_song_effect_interval = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_song_effect_interval"));
    let sleep_effect_bone_hash = WorkModule::get_param_int64(fighter.module_accessor, hash_1.hash as u64, 0);
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT) {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    }
    asdi_check(fighter);
    asdi_function(fighter);
    if situation_kind != *SITUATION_KIND_AIR {
        if ControlModule::get_clatter_time(fighter.module_accessor, 0) <= 0.0 {
            if situation_kind == *SITUATION_KIND_GROUND {
                if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_STATUS_KIND_SLEEP {
                    fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END.into(), false.into());
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_SLEEP_END.into(), false.into());
                }
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
            if motion_kind != hash40("down_bound_u")
            || motion_kind == hash_2.hash as u64 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0xbff07713b), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0xb95b751c9), 0.0, 1.0, false, 0.0, false, false);
            }
            if MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("fill_blank_sleep")) {
                FighterUtil::set_face_motion_by_priority(module_accessor, FighterFacial(*FIGHTER_FACIAL_SLEEP), Hash40::new("invalid"));
            }
            FighterUtil::play_sleep_voice(module_accessor);
        }
        if motion_kind != hash_3.hash as u64 {
            if current_frame as i32 % damage_song_effect_interval == 0 {
                //Don't know exactly what the original is doing
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new_raw(sleep_effect_bone_hash), &NONE_VECTOR, &NONE_VECTOR, 1.0, false, 0.0 as u32, -1, 0, 0, 0, false, false);
            }
        }
        if ![hash_4.hash as u64, hash_5.hash as u64].contains(&motion_kind) {
            return 0.into();
        }
        if (current_frame % (end_frame*2.0)) as i32 == 0 {
            FighterUtil::play_sleep_voice(module_accessor);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_sleep"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_DamageSleep)]
unsafe fn status_end_damagesleep(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x15a773446b), false);
    fighter.sub_DamageSongExit();
    if MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("fill_blank_sleep")) {
        FighterUtil::cancel_face_motion_by_priority(module_accessor, FighterFacial(*FIGHTER_FACIAL_SLEEP));
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_damagesleep_main,
            status_end_damagesleep
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}