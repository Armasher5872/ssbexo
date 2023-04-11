use {
    crate::functions::variables::*,
    smash::{
        app::lua_bind::*,
        hash40,
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*,
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

#[fighter_frame( agent = FIGHTER_KIND_ROY )]
fn roy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(module_accessor);
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        let stick_y = ControlModule::get_stick_y(module_accessor);
        ROY_GFX_COUNTER[entry_id] += 1;
        if ROY_GFX_COUNTER[entry_id] >= 5 {
            if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind)
            && (1.0..=9.0).contains(&frame) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
            && (3.0..=9.0).contains(&frame) {
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
            && (6.0..=31.0).contains(&frame) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
            && (1.0..=13.0).contains(&frame) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;      
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_START
            && (1.0..=6.0).contains(&frame) {
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;      
            }
            if [*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind) {
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;      
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4
            && (1.0..=16.0).contains(&frame) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;       
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4_START
            && (1.0..=4.0).contains(&frame) {
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;     
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
            && (1.0..=19.0).contains(&frame) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;       
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4_START
            && (1.0..=2.0).contains(&frame) {
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;     
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4
            && (1.0..=14.0).contains(&frame) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;      
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
                if motion_kind == hash40("attack_air_n")
                && (1.0..=21.0).contains(&frame) {
                    macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                    macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                    smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                    fighter.pop_lua_stack(1);
                    ROY_GFX_COUNTER[entry_id] = 0;
                }
                if motion_kind == hash40("attack_air_f")
                && (1.0..=18.0).contains(&frame) {
                    macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                    macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                    smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                    fighter.pop_lua_stack(1);
                    ROY_GFX_COUNTER[entry_id] = 0;
                }
                if [hash40("attack_air_b"), hash40("attack_air_hi")].contains(&motion_kind)
                && (1.0..=12.0).contains(&frame) {
                    macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                    macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                    smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                    fighter.pop_lua_stack(1);
                    ROY_GFX_COUNTER[entry_id] = 0;
                }
                if motion_kind == hash40("attack_air_lw")
                && (1.0..=23.0).contains(&frame) {
                    macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                    macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                    smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                    fighter.pop_lua_stack(1);
                    ROY_GFX_COUNTER[entry_id] = 0;
                }
            }
            if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;       
            }
            if [*FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3].contains(&status_kind)
            && (1.0..=14.0).contains(&frame) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;      
            }
            if status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX
            && (1.0..=17.0).contains(&frame) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;       
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
            && (1.0..=11.0).contains(&frame) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;       
            }
            if status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3
            && (1.0..=6.0).contains(&frame) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;       
            }
            if status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4
            && (1.0..=8.0).contains(&frame) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;        
            }
            if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;        
            }
            if status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4
            && (1.0..=10.0).contains(&frame) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;        
            }
            if status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT
            && (1.0..=12.0).contains(&frame) {
                macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
                fighter.clear_lua_stack();
                lua_args!(fighter, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                ROY_GFX_COUNTER[entry_id] = 0;      
            }
        }
        else {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("roy_fire"), false, false);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("roy_attack_fire"), false, false);
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind)
        && frame > 59.0 {
            FULL_SMASH_ATTACK[entry_id] = true;
        }
        if motion_kind == hash40("special_s1")
        && (12.0..=32.0).contains(&frame)
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, true);
                MotionModule::change_motion(module_accessor, Hash40::new("special_s3_hi"), 1.0, 1.0, false, 0.0, false, false);
            }
            else if stick_y < -0.75 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, true);
                MotionModule::change_motion(module_accessor, Hash40::new("special_s3_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, true);
                MotionModule::change_motion(module_accessor, Hash40::new("special_s3_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_air_s1")
        && (12.0..=32.0).contains(&frame)
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, true);
                MotionModule::change_motion(module_accessor, Hash40::new("special_air_s3_hi"), 1.0, 1.0, false, 0.0, false, false);
            }
            else if stick_y < -0.75 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, true);
                MotionModule::change_motion(module_accessor, Hash40::new("special_air_s3_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, true);
                MotionModule::change_motion(module_accessor, Hash40::new("special_air_s3_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        roy_frame
    );
}