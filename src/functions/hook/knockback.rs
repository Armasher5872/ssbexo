//Credited to HDR, used for calculating Finishing Zoom
#![allow(dead_code, improper_ctypes)]
use super::*;

extern "C" {
    #[link_name = "_ZN3app6camera13get_dead_areaEv"]
    fn get_dead_area() -> Rect;

    #[link_name = "_ZN3app10sv_animcmd25EFFECT_GLOBAL_BACK_GROUNDEP9lua_State"]
    fn effect_global_back_ground(lua_state: u64);
}

#[derive(Debug, Copy, Clone)]
struct KnockbackCalcContext {
    knockback: f32,
    x_launch_speed: f32,
    y_launch_speed: f32,
    y_chara_speed: f32,
    tumble: bool,
    is_damage_fly_top: bool,
    hitstun: f32,
    gravity: f32,
    damageflytop_gravity: f32,
    fall_speed: f32,
    damageflytop_fall_speed: f32,
    x_pos: f32,
    y_pos: f32,
    decay_x: f32,
    decay_y: f32,
}

impl KnockbackCalcContext {
    fn step(&mut self) {
        self.x_pos += self.x_launch_speed;
        self.y_pos += self.y_launch_speed + self.y_chara_speed;
        if self.x_launch_speed != 0.0 {
            let dir = self.x_launch_speed.signum();
            self.x_launch_speed = self.x_launch_speed.abs() - self.decay_x;
            if self.x_launch_speed < 0.0 {
                self.x_launch_speed = 0.0;
            } 
            else {
                self.x_launch_speed *= dir;
            }
        }
        if self.y_launch_speed != 0.0 {
            let dir = self.y_launch_speed.signum();
            self.y_launch_speed = self.y_launch_speed.abs() - self.decay_y;
            if self.y_launch_speed < 0.0 {
                self.y_launch_speed = 0.0;
            } 
            else {
                self.y_launch_speed *= dir;
            }
        }
        if self.is_damage_fly_top {
            self.y_chara_speed -= self.damageflytop_gravity;
            if self.y_chara_speed < -self.damageflytop_fall_speed {
                self.y_chara_speed = -self.damageflytop_fall_speed;
            }
        } 
        else {
            self.y_chara_speed -= self.gravity;
            if self.y_chara_speed < -self.fall_speed {
                self.y_chara_speed = -self.fall_speed;
            }
        }
    }
}

#[repr(simd)]
#[derive(Debug)]
struct Rect {
    direction: [f32;4]
}

impl Rect {
    fn contains(&self, x: f32, y: f32) -> bool {
        (x >= self.direction[0] && x <= self.direction[1]) && (y <= self.direction[2] && y >= self.direction[3])
    }
}

static mut IS_CALCULATING: Option<(u32, u32)> = None;

//Related to the updated finishing zoom function
#[skyline::hook(offset = 0x402f00, inline)]
unsafe extern "C" fn calculate_knockback(ctx: &InlineCtx) {
    let damage_module = *ctx.registers[19].x.as_ref();
    let our_boma = *((damage_module + 0x8) as *mut *mut smash::app::BattleObjectModuleAccessor);
    let ptr = *ctx.registers[20].x.as_ref() as *mut u8;
    let id = *(ptr.add(0x24) as *const u32);
    IS_CALCULATING = Some(((*our_boma).battle_object_id, id));
}

#[skyline::hook(offset = 0x403950, inline)]
unsafe extern "C" fn process_knockback(ctx: &InlineCtx) {
    if let Some((defender, attacker)) = IS_CALCULATING {
        let boma = *ctx.registers[20].x.as_ref() as *mut smash::app::BattleObjectModuleAccessor;
        if (*boma).battle_object_id == defender {
            calculate_finishing_hit(defender, attacker, *ctx.registers[19].x.as_ref() as *const f32);
        }
    }
}

/// Knockback log
/// 0x8a -> the opponent was grounded (bool)
/// 0x90 -> backslash (bool)
/// 0x60 -> stop delay (f32) 
/// 0x50 -> collision attr (Hash40)
/// 0x40 -> launch angle in rad (f32)
/// 0x4 -> level (?)
/// 0x4c -> hitstop frame
pub unsafe extern "C" fn calculate_finishing_hit(defender: u32, attacker: u32, knockback_info: *const f32) {
    *(knockback_info.add(0x4c/4) as *mut u32) = 60;
    let defender_boma = &mut *(*get_battle_object_from_id(defender)).module_accessor;
    let attacker_boma = &mut *(*get_battle_object_from_id(attacker)).module_accessor;
    if !defender_boma.is_fighter() { 
        return; 
    }
    if !attacker_boma.is_fighter() && !attacker_boma.is_weapon() { 
        return; 
    }
    if is_invalid_finishing_hit(attacker_boma) {
        return;
    }
    if smash2::app::FighterCutInManager::is_vr_mode() {
        return;
    }
    if !FighterCutInManager__is_one_on_one() {
        return;
    }
    let entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let fighter_info = smash::app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), FighterEntryID(entry_id));
    if smash::app::lua_bind::FighterInformation::stock_count(fighter_info) != 1 { 
        WorkModule::off_flag(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK);
    }
    else {
        WorkModule::on_flag(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK);
    }
    let knockback = *knockback_info;
    let initial_speed_x = *knockback_info.add(4);
    let initial_speed_y = *knockback_info.add(5);
    let reaction = *knockback_info.add(0x48/4);
    let angle = *knockback_info.add(0x10);
    let counter = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    let fly_top_angle_hi = WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("fly_top_angle_lw"));
    let fly_top_angle_lw = WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("fly_top_angle_hi"));
    let damage_air_brake = WorkModule::get_param_float(defender_boma, hash40("common"), hash40("damage_air_brake"));
    let damage_fly_correction_max = WorkModule::get_param_float(defender_boma, hash40("common"), hash40("damage_fly_correction_max"));
    let air_accel_y = WorkModule::get_param_float(defender_boma, hash40("air_accel_y"), 0);
    let damage_fly_top_air_accel_y = WorkModule::get_param_float(defender_boma, hash40("damage_fly_top_air_accel_y"), 0);
    let air_speed_y_stable = WorkModule::get_param_float(defender_boma, hash40("air_speed_y_stable"), 0);
    let damage_fly_top_speed_y_stable = WorkModule::get_param_float(defender_boma, hash40("damage_fly_top_speed_y_stable"), 0);
    let mut context = KnockbackCalcContext{knockback, x_launch_speed: initial_speed_x, y_launch_speed: initial_speed_y, y_chara_speed: 0.0, tumble: *(knockback_info.add(1) as *const u32) >= 3, is_damage_fly_top: fly_top_angle_lw <= angle && angle <= fly_top_angle_hi, hitstun: reaction, gravity: air_accel_y, damageflytop_gravity: damage_fly_top_air_accel_y, fall_speed: air_speed_y_stable, damageflytop_fall_speed: damage_fly_top_speed_y_stable, x_pos: PostureModule::pos_x(defender_boma), y_pos: PostureModule::pos_y(defender_boma), decay_x: damage_air_brake*angle.cos().abs(), decay_y: damage_air_brake*angle.sin().abs()};
    let blastzones = get_dead_area();
    let mag = (context.y_launch_speed.powi(2)+context.x_launch_speed.powi(2)).sqrt();
    let kb_angle = context.y_launch_speed.atan2(context.x_launch_speed).to_degrees();
    let min_di = kb_angle-damage_fly_correction_max;
    let step = (damage_fly_correction_max*2.0)/10.0;
    let context_ref = context;
    //checks 10 different DI angles and sees how many of them will result in a kill
    let mut kill_angle_num = 0;
    for idx in 0..10 {
        let ang = (min_di+(idx as f32*step)).to_radians();
        context.x_launch_speed = ang.cos()*mag;
        context.y_launch_speed = ang.sin()*mag;
        let mut x = 0;
        let mut will_touch_stage = false;
        while context.hitstun > x as f32 {
            context.step();
            if GroundModule::ray_check(defender_boma, &Vector2f{x: context.x_pos, y: (context.y_pos + 4.0)}, &Vector2f{x: 0.0, y: -6.0}, true) == 1 && !(30.0..150.0).contains(&ang.to_degrees()) {
                will_touch_stage = true;
            }
            if !blastzones.contains(context.x_pos, context.y_pos) && !will_touch_stage {
                kill_angle_num += 1;
                break;
            }
            x += 1;
        }
        context = context_ref;
    }
    if kill_angle_num >= 9 && counter == 0 {
        let attacker_kind = (*attacker_boma).kind();
        if WorkModule::is_flag(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
            let handle = EffectModule::req_screen(defender_boma, Hash40::new("bg_finishhit"), false, true, true);
            EffectModule::set_billboard(defender_boma, handle as u32, true);
            EffectModule::set_disable_render_offset_last(defender_boma);
            WorkModule::set_int(attacker_boma, 20, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
            SoundModule::play_se(attacker_boma, Hash40::new("se_common_finishhit"), false, false, false, false, enSEType(0));
            SlowModule::set_whole(attacker_boma, 8, 25);
            StopModule::set_hit_stop_frame(defender_boma, 20, true);
            macros::CAM_ZOOM_IN_arg5(get_fighter_common_from_accessor(&mut *defender_boma), /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
            macros::QUAKE(get_fighter_common_from_accessor(&mut *attacker_boma), *CAMERA_QUAKE_KIND_XL);
            set_vis_hud(false);
            set_stage_visibility(attacker_boma, 0);
            effect_global_back_ground(get_fighter_common_from_accessor(&mut *defender_boma).lua_state_agent);
        }
        else {
            let handle = match attacker_kind {
                _ if [*FIGHTER_KIND_MARIO, *WEAPON_KIND_MARIO_FIREBALL, *WEAPON_KIND_MARIO_HUGE_FLAME, *WEAPON_KIND_MARIO_PUMP_WATER].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_mario_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_DONKEY => EffectModule::req_screen(attacker_boma, Hash40::new("bg_donkey_final"), false, true, true),
                _ if [*FIGHTER_KIND_LINK, *WEAPON_KIND_LINK_SWORD_BEAM, *WEAPON_KIND_LINK_BOWARROW, *WEAPON_KIND_LINK_BOOMERANG].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_link_final"), false, true, true),
                _ if [*FIGHTER_KIND_SAMUS, *WEAPON_KIND_SAMUS_CSHOT, *WEAPON_KIND_SAMUS_MISSILE, *WEAPON_KIND_SAMUS_SUPERMISSILE, *WEAPON_KIND_SAMUS_BOMB].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_samus_final"), false, true, true),
                _ if [*FIGHTER_KIND_SAMUSD, *WEAPON_KIND_SAMUSD_CSHOT, *WEAPON_KIND_SAMUSD_MISSILE, *WEAPON_KIND_SAMUSD_SUPERMISSILE, *WEAPON_KIND_SAMUSD_BOMB].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_samusd_final"), false, true, true),
                _ if [*FIGHTER_KIND_YOSHI, *WEAPON_KIND_YOSHI_TAMAGO, *WEAPON_KIND_YOSHI_STAR].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_yoshi_final"), false, true, true),
                _ if [*FIGHTER_KIND_KIRBY, *WEAPON_KIND_KIRBY_HAMMER, *WEAPON_KIND_KIRBY_FINALCUTTERSHOT, *WEAPON_KIND_KIRBY_ROSETTATICOMISSILE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_kirby_final"), false, true, true),
                _ if [*FIGHTER_KIND_PIKACHU, *WEAPON_KIND_PIKACHU_DENGEKI, *WEAPON_KIND_PIKACHU_DENGEKIDAMA, *WEAPON_KIND_PIKACHU_KAMINARI].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_pikachu_final"), false, true, true),
                _ if [*FIGHTER_KIND_LUIGI, *WEAPON_KIND_LUIGI_FIREBALL].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_luigi_final"), false, true, true),
                _ if [*FIGHTER_KIND_NESS, *WEAPON_KIND_NESS_YOYO_HEAD, *WEAPON_KIND_NESS_PK_FLASH, *WEAPON_KIND_NESS_PK_FIRE, *WEAPON_KIND_NESS_PK_THUNDER].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ness_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_CAPTAIN => EffectModule::req_screen(attacker_boma, Hash40::new("bg_captain_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_PURIN => EffectModule::req_screen(attacker_boma, Hash40::new("bg_purin_final"), false, true, true),
                _ if [*FIGHTER_KIND_PEACH, *WEAPON_KIND_PEACH_KINOPIOSPORE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_peach_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_DAISY => EffectModule::req_screen(attacker_boma, Hash40::new("bg_daisy_final"), false, true, true),
                _ if [*FIGHTER_KIND_KOOPA, *WEAPON_KIND_KOOPA_BREATH].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_koopa_final"), false, true, true),
                _ if [*FIGHTER_KIND_POPO, *WEAPON_KIND_POPO_ICESHOT, *FIGHTER_KIND_NANA].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_popo_final"), false, true, true),
                _ if [*FIGHTER_KIND_SHEIK, *WEAPON_KIND_SHEIK_NEEDLE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_sheik_final"), false, true, true),
                _ if [*FIGHTER_KIND_ZELDA, *WEAPON_KIND_ZELDA_DEIN, *WEAPON_KIND_ZELDA_DEIN_S, *WEAPON_KIND_ZELDA_PHANTOM].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_zelda_final"), false, true, true),
                _ if [*FIGHTER_KIND_MARIOD, *WEAPON_KIND_MARIOD_DRCAPSULE, *WEAPON_KIND_MARIOD_HUGE_CAPSULE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_mariod_final"), false, true, true),
                _ if [*FIGHTER_KIND_PICHU, *WEAPON_KIND_PICHU_DENGEKI, *WEAPON_KIND_PICHU_DENGEKIDAMA, *WEAPON_KIND_PICHU_KAMINARI].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_pichu_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_MARTH => EffectModule::req_screen(attacker_boma, Hash40::new("bg_marth_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_LUCINA => EffectModule::req_screen(attacker_boma, Hash40::new("bg_lucina_final"), false, true, true),
                _ if [*FIGHTER_KIND_YOUNGLINK, *WEAPON_KIND_YOUNGLINK_BOWARROW, *WEAPON_KIND_YOUNGLINK_HOOKSHOT_HAND, *WEAPON_KIND_YOUNGLINK_BOOMERANG].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_younglink_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_GANON => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ganon_final"), false, true, true),
                _ if [*FIGHTER_KIND_MEWTWO, *WEAPON_KIND_MEWTWO_BINDBALL].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_mewtwo_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_ROY => EffectModule::req_screen(attacker_boma, Hash40::new("bg_roy_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_CHROM => EffectModule::req_screen(attacker_boma, Hash40::new("bg_chrom_final"), false, true, true),
                _ if [*FIGHTER_KIND_GAMEWATCH, *WEAPON_KIND_GAMEWATCH_NORMAL_WEAPON, *WEAPON_KIND_GAMEWATCH_PARACHUTE, *WEAPON_KIND_GAMEWATCH_BREATH, *WEAPON_KIND_GAMEWATCH_RESCUE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_gamewatch_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_METAKNIGHT => EffectModule::req_screen(attacker_boma, Hash40::new("bg_metaknight_final"), false, true, true),
                _ if [*FIGHTER_KIND_PIT, *WEAPON_KIND_PIT_BOWARROW].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_pit_final"), false, true, true),
                _ if [*FIGHTER_KIND_PITB, *WEAPON_KIND_PITB_BOWARROW].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_pitb_final"), false, true, true),
                _ if [*FIGHTER_KIND_SZEROSUIT, *WEAPON_KIND_SZEROSUIT_PARALYZER_BULLET, *WEAPON_KIND_SZEROSUIT_WHIP, *WEAPON_KIND_SZEROSUIT_WHIP2].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_szerosuit_final"), false, true, true),
                _ if [*FIGHTER_KIND_WARIO, *WEAPON_KIND_WARIO_WARIOBIKE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_wario_final"), false, true, true),
                _ if [*FIGHTER_KIND_SNAKE, *WEAPON_KIND_SNAKE_C4, *WEAPON_KIND_SNAKE_NIKITA_MISSILE, *WEAPON_KIND_SNAKE_TRENCHMORTAR_BULLET].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_snake_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_IKE => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ike_final"), false, true, true),
                _ if [*FIGHTER_KIND_PZENIGAME, *WEAPON_KIND_PZENIGAME_WATER].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ptrainer_final"), false, true, true),
                _ if [*FIGHTER_KIND_PFUSHIGISOU, *WEAPON_KIND_PFUSHIGISOU_LEAFCUTTER, *WEAPON_KIND_PFUSHIGISOU_VINE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ptrainer_final"), false, true, true),
                _ if [*FIGHTER_KIND_PLIZARDON, *WEAPON_KIND_PLIZARDON_EXPLOSION].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ptrainer_final"), false, true, true),
                _ if [*FIGHTER_KIND_DIDDY, *WEAPON_KIND_DIDDY_PEANUTS, *WEAPON_KIND_DIDDY_EXPLOSION].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_diddy_final"), false, true, true),
                _ if [*FIGHTER_KIND_LUCAS, *WEAPON_KIND_LUCAS_PK_FREEZE, *WEAPON_KIND_LUCAS_PK_FIRE, *WEAPON_KIND_LUCAS_PK_THUNDER].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_lucas_final"), false, true, true),
                _ if [*FIGHTER_KIND_SONIC, *WEAPON_KIND_SONIC_SUPERSONIC].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_sonic_final"), false, true, true),
                _ if [*FIGHTER_KIND_DEDEDE, *WEAPON_KIND_DEDEDE_GORDO, *WEAPON_KIND_DEDEDE_STAR].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_dedede_final"), false, true, true),
                _ if [*FIGHTER_KIND_PIKMIN, *WEAPON_KIND_PIKMIN_PIKMIN].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_pikmin_final"), false, true, true),
                _ if [*FIGHTER_KIND_LUCARIO, *WEAPON_KIND_LUCARIO_AURABALL, *WEAPON_KIND_LUCARIO_QIGONG].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_lucario_final"), false, true, true),
                _ if [*FIGHTER_KIND_ROBOT, *WEAPON_KIND_ROBOT_BEAM].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_robot_final"), false, true, true),
                _ if [*FIGHTER_KIND_TOONLINK, *WEAPON_KIND_TOONLINK_HOOKSHOT, *WEAPON_KIND_TOONLINK_BOWARROW, *WEAPON_KIND_TOONLINK_BOOMERANG].contains(&attacker_kind)  => EffectModule::req_screen(attacker_boma, Hash40::new("bg_toonlink_final"), false, true, true),
                _ if [*FIGHTER_KIND_MURABITO, *WEAPON_KIND_MURABITO_WEEDS, *WEAPON_KIND_MURABITO_FLOWERPOT, *WEAPON_KIND_MURABITO_BOWLING_BALL, *WEAPON_KIND_MURABITO_FIREWORK, *WEAPON_KIND_MURABITO_BULLET, *WEAPON_KIND_MURABITO_CLAYROCKET, *WEAPON_KIND_MURABITO_TREE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_murabito_final"), false, true, true),
                _ if [*FIGHTER_KIND_ROCKMAN, *WEAPON_KIND_ROCKMAN_CHARGESHOT, *WEAPON_KIND_ROCKMAN_AIRSHOOTER, *WEAPON_KIND_ROCKMAN_HARDKNUCKLE, *WEAPON_KIND_ROCKMAN_CRASHBOMB, *WEAPON_KIND_ROCKMAN_LEAFSHIELD].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_rockman_final"), false, true, true),
                _ if [*FIGHTER_KIND_WIIFIT, *WEAPON_KIND_WIIFIT_SUNBULLET, *WEAPON_KIND_WIIFIT_HULAHOOP].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_wiifit_final"), false, true, true),
                _ if [*FIGHTER_KIND_ROSETTA, *WEAPON_KIND_ROSETTA_METEOR, *WEAPON_KIND_ROSETTA_STARPIECE, *WEAPON_KIND_ROSETTA_TICO].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_rosetta_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_LITTLEMAC => EffectModule::req_screen(attacker_boma, Hash40::new("bg_littlemac_final"), false, true, true),
                _ if [*FIGHTER_KIND_GEKKOUGA, *WEAPON_KIND_GEKKOUGA_SHURIKEN, *WEAPON_KIND_GEKKOUGA_WATER].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_gekkouga_final"), false, true, true),
                _ if [*FIGHTER_KIND_PALUTENA, *WEAPON_KIND_PALUTENA_EXPLOSIVEFLAME].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_palutena_final"), false, true, true),
                _ if [*FIGHTER_KIND_PACMAN, *WEAPON_KIND_PACMAN_FIREHYDRANT].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_pacman_final"), false, true, true),
                _ if [*FIGHTER_KIND_REFLET, *WEAPON_KIND_REFLET_THUNDER, *WEAPON_KIND_REFLET_GIGAFIRE, *WEAPON_KIND_REFLET_ELWIND].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_reflet_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_SHULK => EffectModule::req_screen(attacker_boma, Hash40::new("bg_shulk_final"), false, true, true),
                _ if [*FIGHTER_KIND_KOOPAJR, *WEAPON_KIND_KOOPAJR_CANNONBALL, *WEAPON_KIND_KOOPAJR_HAMMER].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_koopajr_final"), false, true, true),
                _ if [*FIGHTER_KIND_DUCKHUNT, *WEAPON_KIND_DUCKHUNT_CAN, *WEAPON_KIND_DUCKHUNT_CLAY, *WEAPON_KIND_DUCKHUNT_GUNMANBULLET].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_duckhunt_final"), false, true, true),
                _ if [*FIGHTER_KIND_RYU, *WEAPON_KIND_RYU_HADOKEN].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ryu_final_shinsyoryu"), false, true, true),
                _ if [*FIGHTER_KIND_KEN, *WEAPON_KIND_KEN_HADOKEN].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ken_final_shinryuken"), false, true, true),
                _ if [*FIGHTER_KIND_CLOUD, *WEAPON_KIND_CLOUD_WAVE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_cloud_final"), false, true, true),
                _ if [*FIGHTER_KIND_KAMUI, *WEAPON_KIND_KAMUI_DRAGONHAND, *WEAPON_KIND_KAMUI_RYUSENSYA].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_kamui_final"), false, true, true),
                _ if [*FIGHTER_KIND_BAYONETTA, *WEAPON_KIND_BAYONETTA_WICKEDWEAVEARM, *WEAPON_KIND_BAYONETTA_WICKEDWEAVELEG, *WEAPON_KIND_BAYONETTA_SPECIALN_BULLET].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_bayonetta_final"), false, true, true),
                _ if [*FIGHTER_KIND_INKLING, *WEAPON_KIND_INKLING_BRUSH, *WEAPON_KIND_INKLING_INKBULLET, *WEAPON_KIND_INKLING_SPLASHBOMB].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_inkling_final_l"), false, true, true),
                _ if [*FIGHTER_KIND_RIDLEY, *WEAPON_KIND_RIDLEY_BREATH].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_ridley_final"), false, true, true),
                _ if [*FIGHTER_KIND_SIMON, *WEAPON_KIND_SIMON_AXE, *WEAPON_KIND_SIMON_CROSS, *WEAPON_KIND_SIMON_WHIP, *WEAPON_KIND_SIMON_WHIP2, *WEAPON_KIND_SIMON_WHIPWIRE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_simon_final"), false, true, true),
                _ if [*FIGHTER_KIND_RICHTER, *WEAPON_KIND_RICHTER_AXE, *WEAPON_KIND_RICHTER_CROSS, *WEAPON_KIND_RICHTER_WHIP, *WEAPON_KIND_RICHTER_WHIP2, *WEAPON_KIND_RICHTER_WHIPWIRE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_richter_final"), false, true, true),
                _ if [*FIGHTER_KIND_KROOL, *WEAPON_KIND_KROOL_IRONBALL, *WEAPON_KIND_KROOL_CROWN].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_krool_final"), false, true, true),
                _ if [*FIGHTER_KIND_SHIZUE, *WEAPON_KIND_SHIZUE_PICOPICOHAMMER, *WEAPON_KIND_SHIZUE_WEEDS, *WEAPON_KIND_SHIZUE_POT, *WEAPON_KIND_SHIZUE_TRAFFICSIGN, *WEAPON_KIND_SHIZUE_POMPON, *WEAPON_KIND_SHIZUE_BULLET, *WEAPON_KIND_SHIZUE_CLAYROCKET].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_shizue_final"), false, true, true),
                _ if attacker_kind == *FIGHTER_KIND_GAOGAEN => EffectModule::req_screen(attacker_boma, Hash40::new("bg_gaogaen_final"), false, true, true),
                _ if [*FIGHTER_KIND_PACKUN, *WEAPON_KIND_PACKUN_SPIKEBALL].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_packun_final1"), false, true, true),
                _ if [*FIGHTER_KIND_JACK, *WEAPON_KIND_JACK_FIRE, *WEAPON_KIND_JACK_FIRE2, *WEAPON_KIND_JACK_WIREROPE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_jack_final"), false, true, true),
                _ if [*FIGHTER_KIND_BRAVE, *WEAPON_KIND_BRAVE_CRASH, *WEAPON_KIND_BRAVE_DEATHBALL, *WEAPON_KIND_BRAVE_EXPLOSION, *WEAPON_KIND_BRAVE_FIREBALL, *WEAPON_KIND_BRAVE_FLASH, *WEAPON_KIND_BRAVE_LIGHTNING, *WEAPON_KIND_BRAVE_SPARK, *WEAPON_KIND_BRAVE_TORNADO].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_brave_final"), false, true, true),
                _ if [*FIGHTER_KIND_BUDDY, *WEAPON_KIND_BUDDY_BULLET, *WEAPON_KIND_BUDDY_PAD].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_buddy_final"), false, true, true),
                _ if [*FIGHTER_KIND_DOLLY, *WEAPON_KIND_DOLLY_WAVE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_dolly_final"), false, true, true),
                _ if [*FIGHTER_KIND_MASTER, *WEAPON_KIND_MASTER_ARROW1, *WEAPON_KIND_MASTER_ARROW2, *WEAPON_KIND_MASTER_AXE, *WEAPON_KIND_MASTER_BOW, *WEAPON_KIND_MASTER_SPEAR, *WEAPON_KIND_MASTER_SWORD, *WEAPON_KIND_MASTER_SWORD2].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_master_final"), false, true, true),
                _ if [*FIGHTER_KIND_TANTAN, *WEAPON_KIND_TANTAN_BEAM, *WEAPON_KIND_TANTAN_PUNCH1, *WEAPON_KIND_TANTAN_PUNCH2, *WEAPON_KIND_TANTAN_PUNCH3, *WEAPON_KIND_TANTAN_RING, *WEAPON_KIND_TANTAN_SPIRALLEFT, *WEAPON_KIND_TANTAN_SPIRALLEFTLOUPE, *WEAPON_KIND_TANTAN_SPIRALRIGHT, *WEAPON_KIND_TANTAN_SPIRALRIGHTLOUPE, *WEAPON_KIND_TANTAN_SPIRALSIMPLE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_tantan_final_l"), false, true, true),
                _ if [*FIGHTER_KIND_PICKEL, *WEAPON_KIND_PICKEL_AXE, *WEAPON_KIND_PICKEL_FIRE, *WEAPON_KIND_PICKEL_MELT, *WEAPON_KIND_PICKEL_PICK, *WEAPON_KIND_PICKEL_PUSHOBJECT, *WEAPON_KIND_PICKEL_STUFF, *WEAPON_KIND_PICKEL_SWORD, *WEAPON_KIND_PICKEL_TROLLEY].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_pickel_final"), false, true, true),
                _ if [*FIGHTER_KIND_EDGE, *WEAPON_KIND_EDGE_FIRE, *WEAPON_KIND_EDGE_FLARE2, *WEAPON_KIND_EDGE_FLASH].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_edge_final"), false, true, true),
                _ if [*FIGHTER_KIND_EFLAME, *WEAPON_KIND_EFLAME_ESWORD].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_eflame_final"), false, true, true),
                _ if [*FIGHTER_KIND_ELIGHT, *WEAPON_KIND_ELIGHT_EXPROSIVESHOT, *WEAPON_KIND_ELIGHT_METEOR, *WEAPON_KIND_ELIGHT_SPREADBULLET].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_eelight_final"), false, true, true),
                _ if [*FIGHTER_KIND_DEMON, *WEAPON_KIND_DEMON_BLASTER].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_demon_final"), false, true, true),
                _ if [*FIGHTER_KIND_TRAIL, 0x25F, 0x261, 0x262].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_trail_final"), false, true, true),
                _ if [*FIGHTER_KIND_MIIFIGHTER, *WEAPON_KIND_MIIFIGHTER_IRONBALL].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_miifighter_final"), false, true, true),
                _ if [*FIGHTER_KIND_MIISWORDSMAN, *WEAPON_KIND_MIISWORDSMAN_LIGHTSHURIKEN, *WEAPON_KIND_MIISWORDSMAN_CHAKRAM].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_miiswordsman_final"), false, true, true),
                _ if [*FIGHTER_KIND_MIIGUNNER, *WEAPON_KIND_MIIGUNNER_ATTACKAIRF_BULLET, *WEAPON_KIND_MIIGUNNER_FLAMEPILLAR, *WEAPON_KIND_MIIGUNNER_GRENADELAUNCHER, *WEAPON_KIND_MIIGUNNER_GROUNDBOMB, *WEAPON_KIND_MIIGUNNER_LASER, *WEAPON_KIND_MIIGUNNER_GUNNERCHARGE, *WEAPON_KIND_MIIGUNNER_RAPIDSHOT_BULLET, *WEAPON_KIND_MIIGUNNER_STEALTHBOMB_S, *WEAPON_KIND_MIIGUNNER_SUPERMISSILE].contains(&attacker_kind) => EffectModule::req_screen(attacker_boma, Hash40::new("bg_miigunner_final"), false, true, true),
                _ => EffectModule::req_screen(attacker_boma, Hash40::new("bg_criticalhit"), false, true, true)
            };
            EffectModule::set_billboard(attacker_boma, handle as u32, true);
            EffectModule::set_disable_render_offset_last(attacker_boma);
            EffectModule::set_rate(attacker_boma, handle as u32, 8.0);
            WorkModule::set_int(attacker_boma, 20, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
            WorkModule::set_int(attacker_boma, handle as i32, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
            SoundModule::play_se(attacker_boma, Hash40::new("se_common_boss_down"), false, false, false, false, enSEType(0));
            SlowModule::set_whole(attacker_boma, 8, 25);
            StopModule::set_hit_stop_frame(defender_boma, 20, true);
            effect_global_back_ground(get_fighter_common_from_accessor(&mut *defender_boma).lua_state_agent);
        }
        match attacker_kind {
            _ if attacker_kind == *FIGHTER_KIND_PZENIGAME => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_ptrainer_win_pzenigame")),
            _ if attacker_kind == *FIGHTER_KIND_PFUSHIGISOU => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_ptrainer_win_pfushigisou")),
            _ if attacker_kind == *FIGHTER_KIND_PLIZARDON => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_ptrainer_win_plizardon")),
            _ if attacker_kind == *FIGHTER_KIND_LITTLEMAC => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_littlemac_win_dl06")),
            _ if attacker_kind == *FIGHTER_KIND_REFLET => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_reflet_final_chrom09")),
            _ if attacker_kind == *FIGHTER_KIND_JACK => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_jack_appeal01")),
            _ if attacker_kind == *FIGHTER_KIND_EFLAME => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_eflame_diver_apeal01")),
            _ if attacker_kind == *FIGHTER_KIND_ELIGHT => macros::PLAY_SE(get_fighter_common_from_accessor(&mut *attacker_boma), Hash40::new("vc_elight_diver_apeal01")),
            _ => {}
        };
    }
}

unsafe extern "C" fn is_invalid_finishing_hit(boma: &mut BattleObjectModuleAccessor) -> bool {
    for id in 0..8 {
        let attack_data = AttackModule::attack_data(boma, id, false);
        let attribute = (*attack_data).attr;
        let fixed_knockback = (*attack_data).r_fix;
        if [hash40("collision_attr_saving"), hash40("collision_attr_lay"), hash40("collision_attr_bury")].contains(&attribute) {
            return true;
        }
        if fixed_knockback > 0 {
            return true;
        }
    }
    return false;
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x633de0).nop(); //Removes the vanilla kill zoom in favor of the updated function. This one handles normal hits
    let _ = skyline::patching::Patch::in_text(0x6373a4).data(0xD503201Fu32); //Removes the vanilla kill zoom in favor of the updated function. This one handles throws
	skyline::install_hooks!(
        process_knockback,
        calculate_knockback
    );
}