//The reason all the opffs are in this file instead of their respective fighters is because for some reason, the game won't allow the auras to work if I put them in there
#![allow(unused_macros)]
use {
    crate::functions::variables::*,
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    },
    smashline::*,
    smash_script::*,
};

#[fighter_frame( agent = FIGHTER_KIND_PLIZARDON )]
fn charizard_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let pos_x = PostureModule::pos_x(module_accessor);
        let pos_y = PostureModule::pos_y(module_accessor);
        let lr = PostureModule::lr(module_accessor);
        //Squirtle's Rain Dance Region Detection
        let point_offset_x1 = lr * (RAIN_DANCE_X1[entry_id] - pos_x);
        let point_offset_x2 = lr * (RAIN_DANCE_X2[entry_id] - pos_x);
        let point_offset_y1 = RAIN_DANCE_Y1[entry_id] - pos_y;
        if (pos_x <= RAIN_DANCE_X2[entry_id] && pos_x >= RAIN_DANCE_X1[entry_id]) && (pos_y >= RAIN_DANCE_Y1[entry_id] && pos_y <= RAIN_DANCE_Y2[entry_id]) && IN_RAIN_DANCE[entry_id] != true {
            IN_RAIN_DANCE[entry_id] = true;
        }
        else {
            IN_RAIN_DANCE[entry_id] = false;
        }
        //Rain Dance Effects
        if RAIN_DANCE_ACTIVE[entry_id] == true {
            RAIN_DANCE_FRAME[entry_id] -= 1.0;
            if RAIN_DANCE_FRAME[entry_id] <= 600.0 && RAIN_DANCE_FRAME[entry_id] >= 598.0 {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_ground_shockwave"), false, false);
            }
            else if RAIN_DANCE_FRAME[entry_id] < 598.0 && RAIN_DANCE_FRAME[entry_id] % 30.0 == 0.0 {
                macros::EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), point_offset_x1, point_offset_y1, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), point_offset_x2, point_offset_y1, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), point_offset_x1 - (12.0*lr), point_offset_y1, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), point_offset_x2 + (12.0*lr), point_offset_y1, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            }
            if RAIN_DANCE_FRAME[entry_id] <= 0.0 {
                RAIN_DANCE_ACTIVE[entry_id] = false;
            }
        }
        if RAIN_DANCE_ACTIVE[entry_id] == false {
            RAIN_DANCE_FRAME[entry_id] = -1.0;
            RAIN_DANCE_X1[entry_id] = 0.0;
            RAIN_DANCE_X2[entry_id] = 0.0;
            RAIN_DANCE_Y1[entry_id] = 0.0;
            RAIN_DANCE_Y2[entry_id] = 0.0;
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_ground_shockwave"), false, false);
        }
    }
}

pub fn install() {
    install_agent_frames!(charizard_frame);
}