use super::*;

#[fighter_frame( agent = FIGHTER_KIND_LITTLEMAC )]
fn littlemac_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(boma);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let ko_gauge = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        //Hitstun Check
        if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
            MAC_HITSTUN[get_player_number(boma)] += 1;
        }
        else {
            MAC_HITSTUN[get_player_number(boma)] = 0;
        }
        if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT && AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
			WorkModule::add_float(boma, 34.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
		}
        if ko_gauge < 0.0 || ko_gauge == f32::NAN {
            WorkModule::set_float(boma, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        }
        //Credit to CSK and Scythe for figuring this out
        let x9 = **(((text_addr+0x52b7000) + 0x4f8) as *const *const u64);
        let mut x8 = x9 + ((entry_id as u64) << 3);
        x8 = *((x8 + 0x20) as *const u64);
        let x0 = (x8 + 0x41e4);
        update_ui(x0 as *const u64, ko_gauge as u32);
    }
}

pub fn install() {
    install_agent_frames!(
        littlemac_frame
    );
}