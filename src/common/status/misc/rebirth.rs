use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_rebirth_common_pre)]
unsafe extern "C" fn sub_rebirth_common_pre(fighter: &mut L2CFighterCommon) {
    let lr = PostureModule::lr(fighter.module_accessor);
    let kind = fighter.global_table[FIGHTER_KIND].get_i32();
    CameraModule::reset_all(fighter.module_accessor);
    ControlModule::reset_trigger(fighter.module_accessor);
    ControlModule::clear_command(fighter.module_accessor, false);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
    AreaModule::set_whole(fighter.module_accessor, false);
    VisibilityModule::set_whole(fighter.module_accessor, true);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e61567377));
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d68));
    PhysicsModule::set_swing_rebirth(fighter.module_accessor, true);
    if ![*FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PLIZARDON].contains(&kind) {
        if lr == -1.0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("entry_l"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("entry_r"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("respawn"), 0.0, 1.0, false, 0.0, false, false);
    }
    match kind {
        0x0 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x1 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x3 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_TRANSPORTATION, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_TRANSPORTATION, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_TRANSPORTATION, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_TRANSPORTATION, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x6 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x7 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x8 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL, Hash40::new("entry_r"), true, -1.0);
        },
        0x9 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_DOKAN, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_DOKAN, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_DOKAN, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_DOKAN, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0xB => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON, Hash40::new("entry_r"), true, -1.0);
            }
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON);
                StatusModule::change_status_request_from_script(article_boma, *WEAPON_CAPTAIN_BLUEFALCON_STATUS_KIND_ENTRY, false);
            }
        },
        0xC => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PURIN_GENERATE_ARTICLE_MONSTERBALL, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_PURIN_GENERATE_ARTICLE_MONSTERBALL, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PURIN_GENERATE_ARTICLE_MONSTERBALL, Hash40::new("entry"), true, -1.0);
        },
        0xD => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0xE => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x12 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_CAPSULEBLOCK, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_CAPSULEBLOCK, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_CAPSULEBLOCK, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_CAPSULEBLOCK, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x13 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_MONSTERBALL, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_MONSTERBALL, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_MONSTERBALL, Hash40::new("entry_r"), true, -1.0);
        },
        0x14 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ARWING, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ARWING, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ARWING, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ARWING, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x16 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x1A => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ROY_GENERATE_ARTICLE_SWORD, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_ROY_GENERATE_ARTICLE_SWORD, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_ROY_GENERATE_ARTICLE_SWORD, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_ROY_GENERATE_ARTICLE_SWORD, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x1C => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_ENTRY, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_ENTRY, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        },
        0x1D => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x20 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP, Hash40::new("entry_r"), true, -1.0);
            }
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP);
                StatusModule::change_status_request_from_script(article_boma, *WEAPON_SZEROSUIT_GUNSHIP_STATUS_KIND_ENTRY, false);
            }
        },
        0x21 => {
            let bike_hp = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("bike_hp"));
            WorkModule::set_float(fighter.module_accessor, bike_hp, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_BIKE_HP);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE, Hash40::new("entry_r"), true, -1.0);
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_REMOVE_BIKE);
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP);
                StatusModule::change_status_request_from_script(article_boma, *WEAPON_WARIO_WARIOBIKE_STATUS_KIND_ENTRY, false);
            }
        },
        0x27 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x28 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_DOSEITABLE, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_DOSEITABLE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_DOSEITABLE, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_DOSEITABLE, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x2A => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_SHRINE, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_SHRINE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_SHRINE, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_SHRINE, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x2B => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_DOLFIN, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_DOLFIN, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_DOLFIN, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_DOLFIN, Hash40::new("entry_r"), true, -1.0);
            }
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_DOLFIN) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_DOLFIN);
                StatusModule::change_status_request_from_script(article_boma, *WEAPON_PIKMIN_DOLFIN_STATUS_KIND_ENTRY, false);
            }
        },
        0x30 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x32 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_BALANCEBOARD, false, -1);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_WIIBO, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_BALANCEBOARD, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_WIIBO, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_BALANCEBOARD, Hash40::new("entry_l"), true, -1.0);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_WIIBO, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_BALANCEBOARD, Hash40::new("entry_r"), true, -1.0);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_WIIBO, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x33 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO);
                StatusModule::change_status_request_from_script(article_boma, *WEAPON_ROSETTA_TICO_STATUS_KIND_ENTRY, false);
            }
        },
        0x34 => {
            if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_SWEATLITTLEMAC) {
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_SWEATLITTLEMAC, false, -1);
                ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_SWEATLITTLEMAC, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                if lr == -1.0 {
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_SWEATLITTLEMAC, Hash40::new("entry_l"), true, -1.0);
                } 
                else {
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_SWEATLITTLEMAC, Hash40::new("entry_r"), true, -1.0);
                }
            }
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_THROWSWEAT, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_THROWSWEAT, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_THROWSWEAT, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_THROWSWEAT, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x35 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MONSTERBALL, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MONSTERBALL, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MONSTERBALL, Hash40::new("entry"), true, -1.0);
        },
        0x36 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GATE, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GATE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GATE, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GATE, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x37 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN, Hash40::new("entry"), true, -1.0);
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN);
                StatusModule::change_status_request_from_script(article_boma, *WEAPON_PACMAN_BIGPACMAN_STATUS_KIND_ENTRY, false);
            }
        },
        0x46 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_OFFICE, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_OFFICE, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_OFFICE, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_OFFICE, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x47 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GAOGAEN_GENERATE_ARTICLE_MONSTERBALL, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_GAOGAEN_GENERATE_ARTICLE_MONSTERBALL, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_GAOGAEN_GENERATE_ARTICLE_MONSTERBALL, Hash40::new("entry"), true, -1.0);
        },
        0x4B => {
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR, Hash40::new("entry"), true, -1.0);
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR);
                StatusModule::change_status_request_from_script(article_boma, *WEAPON_POPO_CONDOR_STATUS_KIND_ENTRY, false);
            }
        },
        0x4C => {
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR, Hash40::new("entry"), true, -1.0);
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR);
                StatusModule::change_status_request_from_script(article_boma, *WEAPON_POPO_CONDOR_STATUS_KIND_ENTRY, false);
            }
        },
        0x56 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BATON, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BATON, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BATON, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BATON, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x58 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_ENTRYOBJECT, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_ENTRYOBJECT, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_ENTRYOBJECT, Hash40::new("entry_r"), true, -1.0);
        },
        0x5A => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_DIVER) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_DIVER);
                StatusModule::change_status_request_from_script(article_boma, *WEAPON_ELEMENT_DIVER_STATUS_KIND_ENTRY, false);
            }
        },
        0x5B => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_DIVER) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_DIVER);
                StatusModule::change_status_request_from_script(article_boma, *WEAPON_ELEMENT_DIVER_STATUS_KIND_ENTRY, false);
            }
        },
        _ => {}
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_rebirth_uniq_check(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_rebirth_uniq_check as *const () as _));
    GroundModule::set_ignore_boss(fighter.module_accessor, true);
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Rebirth_Main)]
unsafe extern "C" fn status_rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let pos = PostureModule::pos(fighter.module_accessor);
    let pos_x = (*pos).x;
    let pos_y = (*pos).y;
    let pos_z = (*pos).z;
    if fighter.sub_rebirth_common().get_bool() {
        return 1.into();
    }
    if [hash40("entry_l"), hash40("entry_r")].contains(&motion_kind) {
        rebirth_motion_handler(fighter);
    }
    else {
        fighter.sub_wait_motion(false.into());
    }
    if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0 && [hash40("wait"), hash40("wait_2"), hash40("wait_3"), hash40("wait_4")].contains(&motion_kind) {
        if lr >= 0.0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_r"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_l"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if (cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0 || cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0) && [hash40("wait"), hash40("wait_2"), hash40("wait_3"), hash40("wait_4")].contains(&motion_kind) {
        if lr >= 0.0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_l"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0 && [hash40("wait"), hash40("wait_2"), hash40("wait_3"), hash40("wait_4")].contains(&motion_kind) {
        if lr >= 0.0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_l"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    match kind {
        0x0 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x1 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x3 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_TRANSPORTATION) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_TRANSPORTATION);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x6 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x7 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x8 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x9 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_DOKAN) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_DOKAN);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0xB => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0xC => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PURIN_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_PURIN_GENERATE_ARTICLE_MONSTERBALL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0xD => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0xE => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x12 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_CAPSULEBLOCK) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_CAPSULEBLOCK);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x13 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_MONSTERBALL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x14 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ARWING) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ARWING);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x16 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x1A => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ROY_GENERATE_ARTICLE_SWORD) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_ROY_GENERATE_ARTICLE_SWORD);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x1C => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_ENTRY) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_ENTRY);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x1D => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x20 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x21 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x27 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x28 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_DOSEITABLE) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_DOSEITABLE);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x2A => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_SHRINE) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_SHRINE);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x2B => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_DOLFIN) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_DOLFIN);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x30 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x32 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_BALANCEBOARD) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_BALANCEBOARD);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_WIIBO) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_WIIBO);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x34 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_SWEATLITTLEMAC) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_SWEATLITTLEMAC);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_THROWSWEAT) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_THROWSWEAT);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x35 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MONSTERBALL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x36 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GATE) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GATE);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x37 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x46 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_OFFICE) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_OFFICE);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x47 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_GAOGAEN_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_GAOGAEN_GENERATE_ARTICLE_MONSTERBALL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x56 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BATON) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BATON);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x58 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_ENTRYOBJECT) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_ENTRYOBJECT);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x5A => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_DIVER) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_DIVER);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x5B => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_DIVER) {
                let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_DIVER);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        _ => {}
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if !fighter.global_table[IS_STOP].get_bool() {
        fighter.sub_rebirth_uniq_process_exec_fix_pos();
    }
    0.into()
}

unsafe extern "C" fn rebirth_motion_handler(fighter: &mut L2CFighterCommon) {
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait"), 0.0, 1.0, false, 0.0, false, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d68));
        fighter.sub_entry_remove_article();
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_Rebirth)]
unsafe extern "C" fn status_end_rebirth(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Rebirth_common();
    fighter.sub_entry_remove_article();
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_rebirth_common_pre,
            status_rebirth_main,
            status_end_rebirth
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}