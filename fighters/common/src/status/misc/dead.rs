use super::*;

pub static mut SUB_DEAD_UNIQ_PROCESS_INIT_COMMON_DEAD_SE: usize = 0x133f58;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_dead_uniq_process_init)]
unsafe extern "C" fn sub_dead_uniq_process_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dead_mode = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_DEAD_MODE);
    if dead_mode == *FIGHTER_DEAD_MODE_NORMAL {
        let mut common_dead_sound = SoundModule::get_common_sound_label(fighter.module_accessor, *FIGHTER_COMMONSND_DEAD as u32);
        let dead_rare_se_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("dead_rare_se_rate"), 0);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
            if WorkModule::is_flag(fighter.module_accessor, 0x200000dd) {
                WorkModule::off_flag(fighter.module_accessor, 0x200000dd);
            }
            else {
                if dead_rare_se_rate != 0 {
                    let fighter_rand = smash::app::sv_math::rand(hash40("fighter"), dead_rare_se_rate);
                    if fighter_rand == 0 {
                        common_dead_sound = SoundModule::get_common_sound_label(fighter.module_accessor, *FIGHTER_COMMONSND_DEAD_RARE as u32);
                        WorkModule::on_flag(fighter.module_accessor, 0x200000dd);
                    }
                }
            }
            SoundModule::play_se(fighter.module_accessor, Hash40::new_raw(common_dead_sound), true, true, true, false, enSEType(0));
        }
    }
    original!()(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        unsafe {
            let common_offset = (*info.module.ModuleObject).module_base as usize;
            SUB_DEAD_UNIQ_PROCESS_INIT_COMMON_DEAD_SE += common_offset;
            let _ = skyline::patching::nop_pointer(SUB_DEAD_UNIQ_PROCESS_INIT_COMMON_DEAD_SE as *const u8);
        }
        skyline::install_hook!(sub_dead_uniq_process_init);
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}