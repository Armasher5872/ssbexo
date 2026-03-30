//Credit to WuBoyTH
use super::*;

//Autoturn Lr Assignment for Special Command Users. Edited to disable Kazuya's Autoturn, but still allow for back walk/dashing
#[skyline::hook(offset = 0x69a6e0)]
unsafe extern "C" fn autoturn_handler(boma: *mut BattleObjectModuleAccessor, is_landing_special: bool, status: i32, param_4: u32) -> f32 {
    let kind = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    if kind == *FIGHTER_KIND_DEMON && ![*FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status) {
        return 0.0;
    }
    original!()(boma, is_landing_special, status, param_4)
}

//Autoturn Control Module Assignment for Special Command Users. Disables functions related to reversing command inputs based on facing direction
#[skyline::hook(offset = 0x69ad00)]
unsafe extern "C" fn update_1_on_1_common(boma: *mut BattleObjectModuleAccessor, param_2: u32, param_3: u32, mut param_4: f32) {
    let control_module = *(boma as *mut *mut u64).add(0x48/8);
    let dir = WorkModule::get_float(boma, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    let kind = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    let mut final_dir = param_4;
    let mut skip_reverse = false;
    if param_3 & 1 == 0 {
        let lr = PostureModule::lr(boma);
        if param_2 & 0xfffffffe == 0x7e {
            if param_4 != 0.0 {
                if lr != param_4 {
                    final_dir = 0.0;
                    param_4 = 0.0;
                }
            }
            skip_reverse = true;
        }
        else if lr != param_4 && *(control_module as *mut u8).add(0x641) != 0 {
            final_dir = 0.0;
            skip_reverse = true;
        }
        else if -dir != param_4 {
            skip_reverse = true;
        }
    }
    else {
        let mut lr = PostureModule::lr(boma);
        let mut lr_check = lr;
        if param_4 != 0.0 {
            lr_check = param_4;
        }
        if dir != 0.0 {
            lr = dir;
        }
        if lr_check != -lr {
            skip_reverse = true;
        }
        else {
            let command = *(control_module as *mut f32).add(0x654/0x4);
            ControlModule::set_back_command(boma, -command);
            *(control_module as *mut u8).add(0x641) = 0;
        }
    }
    if !skip_reverse && kind != *FIGHTER_KIND_DEMON {
        ControlModule::reverse_special_command(boma);
    }
    WorkModule::set_float(boma, final_dir, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if kind != *FIGHTER_KIND_DEMON {
        *(control_module as *mut f32).add(0x650/0x4) = final_dir;
    }
}

//Installation
pub fn install() {
    //The following disables the reversed stick values when autoturn runs for Kazuya
    let _ = skyline::patching::Patch::in_text(0x934a6c).nop();
    let _ = skyline::patching::Patch::in_text(0x21d7d1c).nop();
    //Disables Reverse Special Command calls for Kazuya
    let _ = skyline::patching::Patch::in_text(0x934a4c).nop();
    let _ = skyline::patching::Patch::in_text(0x21d7cfc).nop();
	skyline::install_hooks!(
        autoturn_handler,
        update_1_on_1_common
    );
}