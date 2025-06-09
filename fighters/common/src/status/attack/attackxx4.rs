use super::*;

//Common Forward Smash Hold Exec Status
unsafe extern "C" fn attack_s4_hold_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let loop_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_FRAME);
    let hard_break_frame = match fighter_kind {
        0x0 => 43, //Mario
        0x1 => 60, //Donkey
        0x2 => 60, //Link
        0x3 => 60, //Samus
        0x4 => 46, //SamusD
        0x5 => 60, //Yoshi
        0x6 => 44, //Kirby
        0x7 => 41, //Fox
        0x8 => 43, //Pikachu
        0x9 => 40, //Luigi
        0xA => 60, //Ness
        0xB => 60, //Captain
        0xC => 44, //Purin
        0xD => 60, //Peach
        0xE => 60, //Daisy
        0xF => 60, //Koopa
        0x10 => 40, //Sheik
        0x11 => 60, //Zelda
        0x12 => 43, //MarioD
        0x13 => 44, //Pichu
        0x14 => 60, //Falco
        0x15 => 38, //Marth
        0x16 => 48, //Lucina
        0x17 => 60, //Younglink
        0x18 => 60, //Armstrong
        0x19 => 47, //Mewtwo
        0x1A => 60, //Roy
        0x1B => 60, //Chrom
        0x1C => 45, //Gamewatch
        0x1D => 60, //Metaknight
        0x1E => 60, //Pit
        0x1F => 60, //PitB
        0x20 => 41, //SZerosuit
        0x21 => 46, //Wario
        0x22 => 45, //Snake
        0x23 => 60, //Ike
        0x24 => 48, //Pzenigame
        0x25 => 43, //Pfushigisou
        0x26 => 60, //Plizardon
        0x27 => 40, //Diddy
        0x28 => 42, //Lucas
        0x29 => 60, //Sonic
        0x2A => 60, //Dedede
        0x2B => 39, //Pikmin
        0x2C => 60, //Lucario
        0x2D => 60, //Robot
        0x2E => 60, //Toonlink
        0x2F => 60, //Wolf
        0x30 => 60, //Murabito
        0x31 => 43, //Rockman
        0x32 => 60, //Wiifit
        0x33 => 60, //Rosetta
        0x34 => 60, //Littlemac
        0x35 => 60, //Gekkouga
        0x36 => 46, //Palutena
        0x37 => 60, //Pacman
        0x38 => 60, //Reflet
        0x39 => 60, //Shulk
        0x3A => 46, //Koopajr
        0x3B => 60, //Duckhunt
        0x3C => 60, //Ryu
        0x3D => 60, //Ken
        0x3E => 49, //Cloud
        0x3F => 60, //Kamui
        0x40 => 45, //Bayonetta
        0x41 => 60, //Inkling
        0x42 => 60, //Ridley
        0x43 => 60, //Simon
        0x44 => 60, //Richter
        0x45 => 60, //Krool
        0x46 => 60, //Shizue
        0x47 => 60, //Gaogaen
        0x48 => 60, //Miifighter
        0x49 => 60, //Miiswordsman
        0x4A => 45, //Miigunner
        0x4B => 60, //Popo
        0x4C => 60, //Nana
        0x51 => 60, //Packun
        0x52 => 44, //Jack
        0x53 => 60, //Brave
        0x54 => 45, //Buddy
        0x55 => 60, //Dolly
        0x56 => 60, //Master
        0x57 => 60, //Tantan
        0x58 => 60, //Pickel
        0x59 => 52, //Edge
        0x5A => 60, //Eflame
        0x5B => 40, //Elight
        0x5C => 60, //Demon
        0x5D => 44, //Trail
        _ => 0
    };
    if loop_frame <= hard_break_frame {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
    }
    0.into()
}

//Common Up Smash Hold Exec Status
unsafe extern "C" fn attack_hi4_hold_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let loop_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_FRAME);
    let hard_break_frame = match fighter_kind {
        0x0 => 43, //Mario
        0x1 => 39, //Donkey
        0x2 => 38, //Link
        0x3 => 39, //Samus
        0x4 => 39, //SamusD
        0x5 => 39, //Yoshi
        0x6 => 60, //Kirby
        0x7 => 36, //Fox
        0x8 => 38, //Pikachu
        0x9 => 37, //Luigi
        0xA => 38, //Ness
        0xB => 50, //Captain
        0xC => 40, //Purin
        0xD => 42, //Peach
        0xE => 40, //Daisy
        0xF => 44, //Koopa
        0x10 => 60, //Sheik
        0x11 => 37, //Zelda
        0x12 => 37, //MarioD
        0x13 => 35, //Pichu
        0x14 => 35, //Falco
        0x15 => 41, //Marth
        0x16 => 41, //Lucina
        0x17 => 38, //Younglink
        0x18 => 60, //Armstrong
        0x19 => 35, //Mewtwo
        0x1A => 40, //Roy
        0x1B => 17, //Chrom
        0x1C => 49, //Gamewatch
        0x1D => 34, //Metaknight
        0x1E => 33, //Pit
        0x1F => 33, //PitB
        0x20 => 60, //SZerosuit
        0x21 => 39, //Wario
        0x22 => 39, //Snake
        0x23 => 60, //Ike
        0x24 => 47, //Pzenigame
        0x25 => 60, //Pfushigisou
        0x26 => 34, //Plizardon
        0x27 => 33, //Diddy
        0x28 => 60, //Lucas
        0x29 => 43, //Sonic
        0x2A => 60, //Dedede
        0x2B => 40, //Pikmin
        0x2C => 36, //Lucario
        0x2D => 40, //Robot
        0x2E => 39, //Toonlink
        0x2F => 41, //Wolf
        0x30 => 40, //Murabito
        0x31 => 36, //Rockman
        0x32 => 39, //Wiifit
        0x33 => 36, //Rosetta
        0x34 => 38, //Littlemac
        0x35 => 40, //Gekkouga
        0x36 => 46, //Palutena
        0x37 => 39, //Pacman
        0x38 => 60, //Reflet
        0x39 => 46, //Shulk
        0x3A => 35, //Koopajr
        0x3B => 40, //Duckhunt
        0x3C => 37, //Ryu
        0x3D => 37, //Ken
        0x3E => 40, //Cloud
        0x3F => 41, //Kamui
        0x40 => 46, //Bayonetta
        0x41 => 37, //Inkling
        0x42 => 40, //Ridley
        0x43 => 44, //Simon
        0x44 => 44, //Richter
        0x45 => 34, //Krool
        0x46 => 37, //Shizue
        0x47 => 41, //Gaogaen
        0x48 => 36, //Miifighter
        0x49 => 39, //Miiswordsman
        0x4A => 39, //Miigunner
        0x4B => 40, //Popo
        0x4C => 40, //Nana
        0x51 => 60, //Packun
        0x52 => 38, //Jack
        0x53 => 41, //Brave
        0x54 => 37, //Buddy
        0x55 => 38, //Dolly
        0x56 => 41, //Master
        0x57 => 36, //Tantan
        0x58 => 36, //Pickel
        0x59 => 50, //Edge
        0x5A => 60, //Eflame
        0x5B => 37, //Elight
        0x5C => 40, //Demon
        0x5D => 39, //Trail
        _ => 0
    };
    if loop_frame <= hard_break_frame {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
    }
    0.into()
}

//Common Down Smash Hold Exec Status
unsafe extern "C" fn attack_lw4_hold_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let loop_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_FRAME);
    let hard_break_frame = match fighter_kind {
        0x0 => 43, //Mario
        0x1 => 42, //Donkey
        0x2 => 36, //Link
        0x3 => 37, //Samus
        0x4 => 60, //SamusD
        0x5 => 35, //Yoshi
        0x6 => 34, //Kirby
        0x7 => 34, //Fox
        0x8 => 36, //Pikachu
        0x9 => 34, //Luigi
        0xA => 40, //Ness
        0xB => 47, //Captain
        0xC => 47, //Purin
        0xD => 34, //Peach
        0xE => 39, //Daisy
        0xF => 40, //Koopa
        0x10 => 36, //Sheik
        0x11 => 33, //Zelda
        0x12 => 60, //MarioD
        0x13 => 36, //Pichu
        0x14 => 37, //Falco
        0x15 => 34, //Marth
        0x16 => 34, //Lucina
        0x17 => 37, //Younglink
        0x18 => 60, //Armstrong
        0x19 => 60, //Mewtwo
        0x1A => 34, //Roy
        0x1B => 38, //Chrom
        0x1C => 40, //Gamewatch
        0x1D => 32, //Metaknight
        0x1E => 33, //Pit
        0x1F => 33, //PitB
        0x20 => 48, //SZerosuit
        0x21 => 60, //Wario
        0x22 => 60, //Snake
        0x23 => 60, //Ike
        0x24 => 46, //Pzenigame
        0x25 => 41, //Pfushigisou
        0x26 => 60, //Plizardon
        0x27 => 35, //Diddy
        0x28 => 48, //Lucas
        0x29 => 38, //Sonic
        0x2A => 41, //Dedede
        0x2B => 38, //Pikmin
        0x2C => 44, //Lucario
        0x2D => 35, //Robot
        0x2E => 37, //Toonlink
        0x2F => 42, //Wolf
        0x30 => 37, //Murabito
        0x31 => 60, //Rockman
        0x32 => 46, //Wiifit
        0x33 => 34, //Rosetta
        0x34 => 38, //Littlemac
        0x35 => 39, //Gekkouga
        0x36 => 60, //Palutena
        0x37 => 43, //Pacman
        0x38 => 60, //Reflet
        0x39 => 60, //Shulk
        0x3A => 60, //Koopajr
        0x3B => 40, //Duckhunt
        0x3C => 33, //Ryu
        0x3D => 33, //Ken
        0x3E => 36, //Cloud
        0x3F => 45, //Kamui
        0x40 => 50, //Bayonetta
        0x41 => 39, //Inkling
        0x42 => 52, //Ridley
        0x43 => 42, //Simon
        0x44 => 42, //Richter
        0x45 => 60, //Krool
        0x46 => 36, //Shizue
        0x47 => 60, //Gaogaen
        0x48 => 37, //Miifighter
        0x49 => 34, //Miiswordsman
        0x4A => 37, //Miigunner
        0x4B => 37, //Popo
        0x4C => 37, //Nana
        0x51 => 38, //Packun
        0x52 => 40, //Jack
        0x53 => 36, //Brave
        0x54 => 38, //Buddy
        0x55 => 36, //Dolly
        0x56 => 60, //Master
        0x57 => 34, //Tantan
        0x58 => 36, //Pickel
        0x59 => 60, //Edge
        0x5A => 60, //Eflame
        0x5B => 36, //Elight
        0x5C => 60, //Demon
        0x5D => 60, //Trail
        _ => 0
    };
    if loop_frame <= hard_break_frame {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
    }
    0.into()
}

//Sub Attack XX4 Common Unique Process Exit, clears the full smash attack flags and COUNTER! related stuff
#[skyline::hook(replace = L2CFighterCommon_sub_attack_xx4_common_uniq_process_exit)]
unsafe fn sub_attack_xx4_common_uniq_process_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind_interrupt)
    || ![*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
        AttackModule::set_shield_stiff_mul(fighter.module_accessor, 1.0);
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        SlowModule::clear_whole(boma);
        CameraModule::reset_all(boma);
        macros::CAM_ZOOM_OUT(fighter);
        COUNTERHIT_SUCCESS[get_player_number(boma)] = false;
        return 0.into();
    }
    if [*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind_interrupt) {
        if ![*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
            fighter.sub_end_attack_s4_turn_rev();
        }
    }
    if log_attack_kind < 0 {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log_attack_kind);
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(sub_attack_xx4_common_uniq_process_exit);
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
    Agent::new("fighter")
    .status(Exec, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attack_s4_hold_exec_status)
    .status(Exec, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, attack_hi4_hold_exec_status)
    .status(Exec, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, attack_lw4_hold_exec_status)
    .install()
    ;
}