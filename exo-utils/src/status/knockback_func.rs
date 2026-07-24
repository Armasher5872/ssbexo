use super::*;

pub unsafe extern "C" fn calculate_finishing_hit(defender: u32, attacker: u32, knockback_info: *const f32) {
    *(knockback_info.add(0x4C/4) as *mut u32) = 60; //Hitstop Frames forcibly set to 60
    let defender_battle_object = *get_battle_object_from_id(defender);
    let defender_boma = defender_battle_object.module_accessor;
    let defender_agent = get_fighter_common_from_accessor(&mut *defender_boma);
    let attacker_battle_object = *get_battle_object_from_id(attacker);
    let attacker_boma = attacker_battle_object.module_accessor;
    WorkModule::off_flag(&mut *defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STOCK);
    if !is_potential_finishing_hit(defender_battle_object, attacker_battle_object) { 
        return;
    }
    if !is_valid_finishing_hit(knockback_info, &mut *defender_boma) { 
        return;
    }
    WorkModule::on_flag(&mut *defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STOCK);
    call_finishing_hit_effects(&mut *defender_boma, defender_agent, &mut *attacker_boma);
}

unsafe extern "C" fn is_potential_finishing_hit(defender_battle_object: BattleObject, attacker_battle_object: BattleObject) -> bool {
    let defender_boma = &mut *(defender_battle_object.module_accessor);
    let defender_category = sv_battle_object::category(defender_battle_object.battle_object_id);
    let attacker_boma = &mut *(attacker_battle_object.module_accessor);
    let attacker_category = sv_battle_object::category(attacker_battle_object.battle_object_id);
    if defender_category != *BATTLE_OBJECT_CATEGORY_FIGHTER { 
        return false; 
    }
    if attacker_category != *BATTLE_OBJECT_CATEGORY_FIGHTER && attacker_category != *BATTLE_OBJECT_CATEGORY_WEAPON { 
        return false; 
    }
    if WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER) > 0 {
        return false; 
    }
    if attacker_category == *BATTLE_OBJECT_CATEGORY_FIGHTER && is_no_finishing_hit(attacker_boma) { 
        return false; 
    }
    return true;
}

unsafe extern "C" fn is_no_finishing_hit(attacker_boma: &mut BattleObjectModuleAccessor) -> bool {
    //for some reason this function always returns true for weapons
    for is_abs in [false, true] {
        for id in 0..8 {
            let attack_data = smash::app::lua_bind::AttackModule::attack_data(attacker_boma, id, is_abs);
            let attr = (*attack_data).attr;
            let off = if is_abs { 0xd9 } else { 0xc9 };
            if smash::app::lua_bind::AttackModule::is_attack(attacker_boma, id, is_abs) 
            && *attack_data.cast::<bool>().add(off)
            && ![
                hash40("collision_attr_bind"), hash40("collision_attr_bind_extra"), hash40("collision_attr_bury"), hash40("collision_attr_bury_f"), hash40("collision_attr_bury_r"),
                hash40("collision_attr_fist_down"), hash40("collision_attr_fist_down2"), hash40("collision_attr_fist_down3"), hash40("collision_attr_lay"), hash40("collision_attr_saving"),
                hash40("collision_attr_saving_ken"), hash40("collision_attr_search"), hash40("collision_attr_sleep"), hash40("collision_attr_sleep_ex"), hash40("collision_attr_slip"),
                hash40("collision_attr_stop"), hash40("collision_attr_turn")
            ].contains(&attr) {
                return true;
            }
        }
    }
    return false;
}

unsafe extern "C" fn is_valid_finishing_hit(knockback_info: *const f32, defender_boma: &mut BattleObjectModuleAccessor) -> bool {
    let knockback = *knockback_info;
    let hitstun = *knockback_info.add(0x48 / 4);
    let damage = *knockback_info.add(22);
    let sdi_mul = *knockback_info.add(24);
    let launch_radians = *knockback_info.add(0x10);
    let launch_speed = Vector2f::new(*knockback_info.add(4), *knockback_info.add(5));
    let is_tumble = *(knockback_info.add(1) as *const u32) >= 3;
    let mut context = KnockbackCalcContext::new(
        defender_boma,
        knockback,
        hitstun,
        damage,
        sdi_mul,
        launch_radians,
        launch_speed,
        is_tumble,
    );
    let is_final = is_final_killing_hit(defender_boma);
    return context.is_finishing_hit(is_final);
}

pub unsafe extern "C" fn is_final_killing_hit(defender_boma: &mut BattleObjectModuleAccessor) -> bool {
    if is_teammate_alive(defender_boma) { 
        return false; 
    }
    // check if the defender is on their last stock
    let defender_entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let fighter_info = smash::app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), smash::app::FighterEntryID(defender_entry_id));
    if smash::app::lua_bind::FighterInformation::stock_count(fighter_info) != 1 {
        return false;
    }
    return true;
}

unsafe extern "C" fn is_teammate_alive(defender_boma: &mut BattleObjectModuleAccessor) -> bool {
    for object_id in get_all_active_battle_object_ids() {
        let object = get_battle_object_from_id(object_id);
        if object.is_null() { 
            continue; //skip null
        }
        let other_battle_object_id = (*object).battle_object_id;
        let other_kind = sv_battle_object::kind(other_battle_object_id);
        let other_boma = &mut *(*object).module_accessor;
        if other_kind == *FIGHTER_KIND_NANA { 
            continue; //skip nana
        }
        let defender_entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let other_entry_id = WorkModule::get_int(other_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        if defender_entry_id == other_entry_id { 
            continue; //skip this boma if it belongs to the same player
        }
        let fighter_info = smash::app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), smash::app::FighterEntryID(other_entry_id));
        if FighterUtil::get_team_color(defender_boma) == FighterUtil::get_team_color(other_boma) && smash::app::lua_bind::FighterInformation::stock_count(fighter_info) > 0 { 
            return true; //check if another player on the same team has at least one stock
        }
    }
    return false;
}

unsafe extern "C" fn call_finishing_hit_effects(defender_boma: &mut BattleObjectModuleAccessor, defender_agent: &mut L2CFighterCommon, attacker_boma: &mut BattleObjectModuleAccessor) {
    let attacker_category = sv_battle_object::category(attacker_boma.battle_object_id);
    let attacker_kind = sv_battle_object::kind(attacker_boma.battle_object_id);
    let attacker_lr = PostureModule::lr(attacker_boma);
    if is_final_killing_hit(defender_boma) {
        let mut handle: u64 = *EFFECT_HANDLE_NULL as u64;
        if attacker_category == *BATTLE_OBJECT_CATEGORY_WEAPON {
            let owner_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
            let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
            handle = EffectModule::req_on_joint(owner_boma, Hash40::new("sys_bg_finishhit"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
        }
        if attacker_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            handle = EffectModule::req_on_joint(attacker_boma, Hash40::new("sys_bg_finishhit"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
        }
        WorkModule::set_int(defender_boma, handle as i32, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
        WorkModule::set_int(defender_boma, 60, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
        CAM_ZOOM_IN_arg5(defender_agent, 2.0, 0.0, 1.8, 0.0, 0.0);
        QUAKE(defender_agent, *CAMERA_QUAKE_KIND_XL);
        set_stage_visibility(defender_boma, 0);
        set_vis_hud(false);
        SoundModule::play_se(defender_boma, Hash40::new("se_common_finishhit"), false, false, false, false, enSEType(0));
    }
    else {
        if attacker_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            let handle;
            match attacker_kind {
                _ if attacker_kind == *FIGHTER_KIND_MARIO => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_mario_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_DONKEY => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_donkey_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_LINK => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_link_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_SAMUS => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_samus_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_SAMUSD => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_samusd_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_YOSHI => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_yoshi_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_KIRBY => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_kirby_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_FOX => {
                    handle = EffectModule::req_on_joint(attacker_boma, Hash40::new("sys_bg_vortex"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
                },
                _ if attacker_kind == *FIGHTER_KIND_PIKACHU => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_pikachu_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_LUIGI => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_luigi_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_NESS => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_ness_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_CAPTAIN => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_captain_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_PURIN => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_purin_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_PEACH => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_peach_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_DAISY => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_daisy_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_KOOPA => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_koopa_final"), false, false, false);
                },
                _ if [*FIGHTER_KIND_POPO, *FIGHTER_KIND_NANA].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_popo_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_SHEIK => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_sheik_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_ZELDA => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_zelda_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_MARIOD => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_mariod_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_PICHU => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_pichu_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_FALCO => {
                    handle = EffectModule::req_on_joint(attacker_boma, Hash40::new("sys_bg_lightning"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0)
                },
                _ if attacker_kind == *FIGHTER_KIND_MARTH => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_marth_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_LUCINA => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_lucina_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_YOUNGLINK => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_younglink_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_GANON => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_ganon_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_MEWTWO => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_mewtwo_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_ROY => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_roy_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_CHROM => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_chrom_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_GAMEWATCH => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_gamewatch_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_METAKNIGHT => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_metaknight_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_PIT => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_pit_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_PITB => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_pitb_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_SZEROSUIT => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_szerosuit_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_WARIO => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_wario_final2"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_SNAKE => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_snake_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_IKE => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_ike_final"), false, false, false);
                },
                _ if [*FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PLIZARDON].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_ptrainer_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_DIDDY => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_diddy_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_LUCAS => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_lucas_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_SONIC => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_sonic_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_DEDEDE => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_dedede_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_PIKMIN => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_pikmin_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_LUCARIO => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_lucario_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_ROBOT => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_robot_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_TOONLINK => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_toonlink_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_WOLF => {
                    handle = EffectModule::req_on_joint(attacker_boma, Hash40::new("sys_bg_vortex2"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
                },
                _ if attacker_kind == *FIGHTER_KIND_MURABITO => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_murabito_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_ROCKMAN => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_rockman_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_WIIFIT => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_wiifit_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_ROSETTA => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_rosetta_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_LITTLEMAC => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_littlemac_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_GEKKOUGA => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_gekkouga_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_PALUTENA => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_palutena_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_PACMAN => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_pacman_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_REFLET => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_reflet_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_SHULK => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_shulk_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_KOOPAJR => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_koopajr_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_DUCKHUNT => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_duckhunt_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_RYU => {
                    handle = EffectModule::req_on_joint(attacker_boma, Hash40::new("critical_hit"), Hash40::new("handr"), &Vector3f{x: 30.0*attacker_lr, y: 0.0, z: 0.0}, &Vector3f::zero(), 30.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
                },
                _ if attacker_kind == *FIGHTER_KIND_KEN => {
                    handle = EffectModule::req_on_joint(attacker_boma, Hash40::new("critical_hit"), Hash40::new("top"), &Vector3f{x: 30.0*attacker_lr, y: 0.0, z: 0.0}, &Vector3f::zero(), 30.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
                },
                _ if attacker_kind == *FIGHTER_KIND_CLOUD => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_cloud_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_KAMUI => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_kamui_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_BAYONETTA => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_bayonetta_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_INKLING => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_inkling_final_l2"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_RIDLEY => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_ridley_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_SIMON => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_simon_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_RICHTER => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_richter_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_KROOL => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_krool_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_SHIZUE => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_shizue_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_GAOGAEN => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_gaogaen_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_PACKUN => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_packun_final2"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_JACK => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_jack_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_BRAVE => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_brave_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_BUDDY => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_buddy_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_DOLLY => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_dolly_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_MASTER => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_master_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_TANTAN => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_tantan_final_endl"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_PICKEL => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_pickel_final_end"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_EDGE => {
                    handle = EffectModule::req_on_joint(attacker_boma, Hash40::new("edge_win_fire"), Hash40::new("top"), &Vector3f{x: 500.0*attacker_lr, y: -300.0, z: 0.0}, &Vector3f::zero(), 20.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
                },
                _ if attacker_kind == *FIGHTER_KIND_EFLAME => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_eflame_final2"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_ELIGHT => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_eelight_final2"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_DEMON => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_demon_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_TRAIL => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_trail_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_MIIFIGHTER => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_miifighter_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_MIISWORDSMAN => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_miiswordsman_final"), false, false, false);
                },
                _ if attacker_kind == *FIGHTER_KIND_MIIGUNNER => {
                    handle = EffectModule::req_screen(attacker_boma, Hash40::new("bg_miigunner_final"), false, false, false);
                },
                _ => {
                    handle = EffectModule::req_on_joint(attacker_boma, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
                }
            };
            EffectModule::set_rate(attacker_boma, handle as u32, 2.0);
            WorkModule::set_int(attacker_boma, handle as i32, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
            WorkModule::set_int(attacker_boma, 80, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
        }
        if attacker_category == *BATTLE_OBJECT_CATEGORY_WEAPON {
            let handle;
            let owner_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
            let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
            match attacker_kind {
                _ if [*WEAPON_KIND_MARIO_FIREBALL, *WEAPON_KIND_MARIO_HUGE_FLAME].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_mario_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_LINK_SWORD_BEAM, *WEAPON_KIND_LINK_BOWARROW, *WEAPON_KIND_LINK_BOOMERANG].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_link_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_SAMUS_CSHOT, *WEAPON_KIND_SAMUS_MISSILE, *WEAPON_KIND_SAMUS_SUPERMISSILE, *WEAPON_KIND_SAMUS_BOMB].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_samus_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_SAMUSD_CSHOT, *WEAPON_KIND_SAMUSD_MISSILE, *WEAPON_KIND_SAMUSD_SUPERMISSILE, *WEAPON_KIND_SAMUSD_BOMB].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_samusd_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_YOSHI_TAMAGO, *WEAPON_KIND_YOSHI_STAR].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_yoshi_final"), false, false, false)
                },
                _ if [*WEAPON_KIND_KIRBY_HAMMER, *WEAPON_KIND_KIRBY_FINALCUTTERSHOT, *WEAPON_KIND_KIRBY_ROSETTATICOMISSILE].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_kirby_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_FOX_BLASTER_BULLET, *WEAPON_KIND_FOX_ILLUSION].contains(&attacker_kind) => {
                    handle = EffectModule::req_on_joint(owner_boma, Hash40::new("sys_bg_vortex"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
                },
                _ if [*WEAPON_KIND_PIKACHU_DENGEKI, *WEAPON_KIND_PIKACHU_DENGEKIDAMA, *WEAPON_KIND_PIKACHU_KAMINARI].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_pikachu_final"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_LUIGI_FIREBALL => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_luigi_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_NESS_YOYO_HEAD, *WEAPON_KIND_NESS_PK_FLASH, *WEAPON_KIND_NESS_PK_FIRE, *WEAPON_KIND_NESS_PK_THUNDER].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_ness_final"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_PEACH_KINOPIOSPORE => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_peach_final"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_KOOPA_BREATH => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_koopa_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_POPO_ICESHOT, *WEAPON_KIND_POPO_BLIZZARD].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_popo_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_SHEIK_NEEDLE, *WEAPON_KIND_SHEIK_FUSIN].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_sheik_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_ZELDA_DEIN, *WEAPON_KIND_ZELDA_DEIN_S, *WEAPON_KIND_ZELDA_PHANTOM].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_zelda_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_MARIOD_DRCAPSULE, *WEAPON_KIND_MARIOD_HUGE_CAPSULE].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_mariod_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_PICHU_DENGEKI, *WEAPON_KIND_PICHU_DENGEKIDAMA, *WEAPON_KIND_PICHU_KAMINARI].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_pichu_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_FALCO_BLASTER_BULLET, *WEAPON_KIND_FALCO_ILLUSION].contains(&attacker_kind) => {
                    handle = EffectModule::req_on_joint(owner_boma, Hash40::new("sys_bg_lightning"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
                },
                _ if [*WEAPON_KIND_YOUNGLINK_BOWARROW, *WEAPON_KIND_YOUNGLINK_HOOKSHOT_HAND, *WEAPON_KIND_YOUNGLINK_BOOMERANG].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_younglink_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_MEWTWO_SHADOWBALL, *WEAPON_KIND_MEWTWO_BINDBALL, *WEAPON_KIND_MEWTWO_BINDBALL, *WEAPON_KIND_MEWTWO_PSYCHOBREAK].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_mewtwo_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_GAMEWATCH_NORMAL_WEAPON, *WEAPON_KIND_GAMEWATCH_BOMB, *WEAPON_KIND_GAMEWATCH_PARACHUTE, *WEAPON_KIND_GAMEWATCH_BREATH, *WEAPON_KIND_GAMEWATCH_FOOD, *WEAPON_KIND_GAMEWATCH_RESCUE, *WEAPON_KIND_GAMEWATCH_OIL].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_gamewatch_final"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_PIT_BOWARROW => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_pit_final"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_PITB_BOWARROW => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_pitb_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_SZEROSUIT_PARALYZER_BULLET, *WEAPON_KIND_SZEROSUIT_WHIP, *WEAPON_KIND_SZEROSUIT_WHIP2].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_szerosuit_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_SNAKE_RPG7, *WEAPON_KIND_SNAKE_TRENCHMORTAR, *WEAPON_KIND_SNAKE_TRENCHMORTAR_BULLET, *WEAPON_KIND_SNAKE_GRENADE, *WEAPON_KIND_SNAKE_NIKITA_MISSILE, *WEAPON_KIND_SNAKE_CYPHER, *WEAPON_KIND_SNAKE_C4].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_snake_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_PZENIGAME_WATER, *WEAPON_KIND_PFUSHIGISOU_LEAFCUTTER, *WEAPON_KIND_PFUSHIGISOU_VINE, *WEAPON_KIND_PLIZARDON_EXPLOSION].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_ptrainer_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_DIDDY_PEANUTS, *WEAPON_KIND_DIDDY_EXPLOSION].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_diddy_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_LUCAS_PK_FREEZE, *WEAPON_KIND_LUCAS_PK_FIRE, *WEAPON_KIND_LUCAS_PK_THUNDER].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_lucas_final"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_SONIC_SUPERSONIC => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_sonic_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_DEDEDE_GORDO, *WEAPON_KIND_DEDEDE_STAR].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_dedede_final"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_PIKMIN_PIKMIN => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_pikmin_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_LUCARIO_AURABALL, *WEAPON_KIND_LUCARIO_QIGONG].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_lucario_final"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_ROBOT_BEAM => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_robot_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_TOONLINK_HOOKSHOT, *WEAPON_KIND_TOONLINK_BOWARROW, *WEAPON_KIND_TOONLINK_BOOMERANG].contains(&attacker_kind)  => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_toonlink_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_WOLF_BLASTER_BULLET, *WEAPON_KIND_WOLF_ILLUSION].contains(&attacker_kind) => {
                    handle = EffectModule::req_on_joint(owner_boma, Hash40::new("sys_bg_vortex2"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
                },
                _ if [*WEAPON_KIND_MURABITO_WEEDS, *WEAPON_KIND_MURABITO_FLOWERPOT, *WEAPON_KIND_MURABITO_BOWLING_BALL, *WEAPON_KIND_MURABITO_FIREWORK, *WEAPON_KIND_MURABITO_BULLET, *WEAPON_KIND_MURABITO_CLAYROCKET, *WEAPON_KIND_MURABITO_TREE].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_murabito_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_ROCKMAN_CHARGESHOT, *WEAPON_KIND_ROCKMAN_AIRSHOOTER, *WEAPON_KIND_ROCKMAN_HARDKNUCKLE, *WEAPON_KIND_ROCKMAN_CRASHBOMB, *WEAPON_KIND_ROCKMAN_LEAFSHIELD].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_rockman_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_WIIFIT_SUNBULLET, *WEAPON_KIND_WIIFIT_HULAHOOP].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_wiifit_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_ROSETTA_METEOR, *WEAPON_KIND_ROSETTA_STARPIECE, *WEAPON_KIND_ROSETTA_TICO].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_rosetta_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_GEKKOUGA_SHURIKEN, *WEAPON_KIND_GEKKOUGA_WATER].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_gekkouga_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_PALUTENA_AUTOAIMBULLET, *WEAPON_KIND_PALUTENA_EXPLOSIVEFLAME, *WEAPON_KIND_PALUTENA_BEAM].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_palutena_final"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_PACMAN_FIREHYDRANT => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_pacman_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_REFLET_THUNDER, *WEAPON_KIND_REFLET_GIGAFIRE, *WEAPON_KIND_REFLET_ELWIND].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_reflet_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_KOOPAJR_CANNONBALL, *WEAPON_KIND_KOOPAJR_HAMMER].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_koopajr_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_DUCKHUNT_CAN, *WEAPON_KIND_DUCKHUNT_CLAY, *WEAPON_KIND_DUCKHUNT_GUNMANBULLET].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_duckhunt_final"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_RYU_HADOKEN => {
                    handle = EffectModule::req_on_joint(owner_boma, Hash40::new("critical_hit"), Hash40::new("handr"), &Vector3f{x: 30.0*attacker_lr, y: 0.0, z: 0.0}, &Vector3f::zero(), 30.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
                },
                _ if attacker_kind == *WEAPON_KIND_KEN_HADOKEN => {
                    handle = EffectModule::req_on_joint(owner_boma, Hash40::new("critical_hit"), Hash40::new("top"), &Vector3f{x: 30.0*attacker_lr, y: 0.0, z: 0.0}, &Vector3f::zero(), 30.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
                },
                _ if attacker_kind == *WEAPON_KIND_CLOUD_WAVE => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_cloud_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_KAMUI_DRAGONHAND, *WEAPON_KIND_KAMUI_RYUSENSYA, *WEAPON_KIND_KAMUI_SPEARHAND, *WEAPON_KIND_KAMUI_WATERDRAGON, *WEAPON_KIND_KAMUI_WATERSTREAM].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_kamui_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_BAYONETTA_WICKEDWEAVEARM, *WEAPON_KIND_BAYONETTA_WICKEDWEAVELEG, *WEAPON_KIND_BAYONETTA_SPECIALN_BULLET].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_bayonetta_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_INKLING_BRUSH, *WEAPON_KIND_INKLING_INKBULLET, *WEAPON_KIND_INKLING_ROLLER, *WEAPON_KIND_INKLING_SPLASHBOMB].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_inkling_final_l2"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_RIDLEY_BREATH => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_ridley_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_SIMON_AXE, *WEAPON_KIND_SIMON_CROSS, *WEAPON_KIND_SIMON_WHIP, *WEAPON_KIND_SIMON_WHIP2, *WEAPON_KIND_SIMON_WHIPWIRE].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_simon_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_RICHTER_AXE, *WEAPON_KIND_RICHTER_CROSS, *WEAPON_KIND_RICHTER_WHIP, *WEAPON_KIND_RICHTER_WHIP2, *WEAPON_KIND_RICHTER_WHIPWIRE].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_richter_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_KROOL_IRONBALL, *WEAPON_KIND_KROOL_CROWN].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_krool_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_SHIZUE_PICOPICOHAMMER, *WEAPON_KIND_SHIZUE_WEEDS, *WEAPON_KIND_SHIZUE_POT, *WEAPON_KIND_SHIZUE_TRAFFICSIGN, *WEAPON_KIND_SHIZUE_POMPON, *WEAPON_KIND_SHIZUE_BULLET, *WEAPON_KIND_SHIZUE_CLAYROCKET].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_shizue_final"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_PACKUN_SPIKEBALL => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_packun_final2"), false, false, false);
                },
                _ if [*WEAPON_KIND_JACK_FIRE, *WEAPON_KIND_JACK_FIRE2, *WEAPON_KIND_JACK_WIREROPE].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_jack_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_BRAVE_BLUE, *WEAPON_KIND_BRAVE_CRASH, *WEAPON_KIND_BRAVE_DEATHBALL, *WEAPON_KIND_BRAVE_EXPLOSION, *WEAPON_KIND_BRAVE_FIREBALL, *WEAPON_KIND_BRAVE_FLASH, *WEAPON_KIND_BRAVE_LIGHTNING, *WEAPON_KIND_BRAVE_SLEEP, *WEAPON_KIND_BRAVE_SPARK, *WEAPON_KIND_BRAVE_TORNADO].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_brave_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_BUDDY_BULLET, *WEAPON_KIND_BUDDY_PAD].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_buddy_final"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_DOLLY_WAVE => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_dolly_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_MASTER_ARROW1, *WEAPON_KIND_MASTER_ARROW2, *WEAPON_KIND_MASTER_AXE, *WEAPON_KIND_MASTER_BOW, *WEAPON_KIND_MASTER_SPEAR, *WEAPON_KIND_MASTER_SWORD, *WEAPON_KIND_MASTER_SWORD2].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_master_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_TANTAN_BEAM, *WEAPON_KIND_TANTAN_PUNCH1, *WEAPON_KIND_TANTAN_PUNCH2, *WEAPON_KIND_TANTAN_PUNCH3, *WEAPON_KIND_TANTAN_RING, *WEAPON_KIND_TANTAN_SPIRALLEFT, *WEAPON_KIND_TANTAN_SPIRALLEFTLOUPE, *WEAPON_KIND_TANTAN_SPIRALRIGHT, *WEAPON_KIND_TANTAN_SPIRALRIGHTLOUPE, *WEAPON_KIND_TANTAN_SPIRALSIMPLE].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_tantan_final_end"), false, false, false);
                },
                _ if [*WEAPON_KIND_PICKEL_AXE, *WEAPON_KIND_PICKEL_FIRE, *WEAPON_KIND_PICKEL_MELT, *WEAPON_KIND_PICKEL_PICK, *WEAPON_KIND_PICKEL_PUSHOBJECT, *WEAPON_KIND_PICKEL_STUFF, *WEAPON_KIND_PICKEL_SWORD, *WEAPON_KIND_PICKEL_TROLLEY].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_pickel_final_end"), false, false, false);
                },
                _ if [*WEAPON_KIND_EDGE_FIRE, *WEAPON_KIND_EDGE_FLARE2, *WEAPON_KIND_EDGE_FLASH].contains(&attacker_kind) => {
                    handle = EffectModule::req_on_joint(owner_boma, Hash40::new("edge_win_fire"), Hash40::new("top"), &Vector3f{x: 500.0*attacker_lr, y: -300.0, z: 0.0}, &Vector3f::zero(), 20.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
                },
                _ if attacker_kind == *WEAPON_KIND_EFLAME_ESWORD => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_eflame_final2"), false, false, false);
                },
                _ if [*WEAPON_KIND_ELIGHT_EXPROSIVESHOT, *WEAPON_KIND_ELIGHT_METEOR, *WEAPON_KIND_ELIGHT_SPREADBULLET].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_eelight_final2"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_DEMON_BLASTER => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_demon_final"), false, false, false);
                },
                _ if [0x25F, 0x261, 0x262].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_trail_final"), false, false, false);
                },
                _ if attacker_kind == *WEAPON_KIND_MIIFIGHTER_IRONBALL => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_miifighter_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_MIISWORDSMAN_LIGHTSHURIKEN, *WEAPON_KIND_MIISWORDSMAN_CHAKRAM].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_miiswordsman_final"), false, false, false);
                },
                _ if [*WEAPON_KIND_MIIGUNNER_ATTACKAIRF_BULLET, *WEAPON_KIND_MIIGUNNER_FLAMEPILLAR, *WEAPON_KIND_MIIGUNNER_GRENADELAUNCHER, *WEAPON_KIND_MIIGUNNER_GROUNDBOMB, *WEAPON_KIND_MIIGUNNER_LASER, *WEAPON_KIND_MIIGUNNER_GUNNERCHARGE, *WEAPON_KIND_MIIGUNNER_RAPIDSHOT_BULLET, *WEAPON_KIND_MIIGUNNER_STEALTHBOMB_S, *WEAPON_KIND_MIIGUNNER_SUPERMISSILE].contains(&attacker_kind) => {
                    handle = EffectModule::req_screen(owner_boma, Hash40::new("bg_miigunner_final"), false, false, false)
                },
                _ => {
                    handle = EffectModule::req_on_joint(owner_boma, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
                }
            };
            EffectModule::set_rate(owner_boma, handle as u32, 2.0);
            WorkModule::set_int(owner_boma, handle as i32, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
            WorkModule::set_int(owner_boma, 80, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
        }
        match attacker_kind {
            _ if attacker_kind == *FIGHTER_KIND_PZENIGAME => SoundModule::play_se(attacker_boma, Hash40::new("vc_ptrainer_win_pzenigame"), true, false, false, false, enSEType(0)),
            _ if attacker_kind == *FIGHTER_KIND_PFUSHIGISOU => SoundModule::play_se(attacker_boma, Hash40::new("vc_ptrainer_win_pfushigisou"), true, false, false, false, enSEType(0)),
            _ if attacker_kind == *FIGHTER_KIND_PLIZARDON => SoundModule::play_se(attacker_boma, Hash40::new("vc_ptrainer_win_plizardon"), true, false, false, false, enSEType(0)),
            _ if attacker_kind == *FIGHTER_KIND_LITTLEMAC => SoundModule::play_se(attacker_boma, Hash40::new("vc_littlemac_win_dl06"), true, false, false, false, enSEType(0)),
            _ if attacker_kind == *FIGHTER_KIND_REFLET => SoundModule::play_se(attacker_boma, Hash40::new("vc_reflet_final_chrom09"), true, false, false, false, enSEType(0)),
            _ if attacker_kind == *FIGHTER_KIND_JACK => SoundModule::play_se(attacker_boma, Hash40::new("vc_jack_appeal01"), true, false, false, false, enSEType(0)),
            _ if attacker_kind == *FIGHTER_KIND_EFLAME => SoundModule::play_se(attacker_boma, Hash40::new("vc_eflame_diver_apeal01"), true, false, false, false, enSEType(0)),
            _ if attacker_kind == *FIGHTER_KIND_ELIGHT => SoundModule::play_se(attacker_boma, Hash40::new("vc_elight_diver_apeal01"), true, false, false, false, enSEType(0)),
            _ => 0
        };
        SoundModule::play_se(defender_boma, Hash40::new("se_common_boss_down"), false, false, false, false, enSEType(0));
    }
    SlowModule::set_whole(attacker_boma, 8, 25);
    SlowModule::set_whole(defender_boma, 8, 25);
    StopModule::set_hit_stop_frame(defender_boma, 20, true);
}