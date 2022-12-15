use {
    crate::functions::{
        SNAKE_INT_ATTACK_S4_COMBO_COUNT,
        SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE,
        SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED
    },
    smash::{
        app::lua_bind::*,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
    },
    smashline::*,
};

#[fighter_frame( agent = FIGHTER_KIND_SNAKE )]
fn snake_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S4 {
            if SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] == false {
                if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
                || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SMASH) {
                    SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = true;
                }
            }
            if SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id]
            && SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] {
                SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = false;
                SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = false;
                if SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] == 0 {
                    SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] = 1;
                    ControlModule::reset_trigger(fighter.module_accessor);
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s2"), 0.0, 1.0, false, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s3"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
    }
}

#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_side_smash_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    original!(fighter)
}

#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_side_smash_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = false;
    SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = false;
    SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] = 0;
    original!(fighter)
}

pub fn install() {
    install_status_scripts!(
        snake_side_smash_status_main,
        snake_side_smash_status_end
    );
    install_agent_frames!(
        snake_frame
    );
}