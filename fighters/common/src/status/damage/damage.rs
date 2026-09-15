use super::*;

static mut IS_CALCULATING: Option<(u32, u32)> = None;

//Credited to HDR, used for calculating Finishing Zoom

//Related to the updated finishing zoom function
#[skyline::hook(offset = 0x402f00, inline)]
unsafe extern "C" fn calculate_knockback(ctx: &InlineCtx) {
    let damage_module = ctx.registers[19].x();
    let our_boma = *((damage_module + 0x8) as *mut *mut smash::app::BattleObjectModuleAccessor);
    let ptr = ctx.registers[20].x() as *mut u8;
    let id = *(ptr.add(0x24) as *const u32);
    IS_CALCULATING = Some(((*our_boma).battle_object_id, id));
}

#[skyline::hook(offset = 0x403950, inline)]
unsafe extern "C" fn process_knockback(ctx: &InlineCtx) {
    if let Some((defender, attacker)) = IS_CALCULATING {
        let boma = ctx.registers[20].x() as *mut smash::app::BattleObjectModuleAccessor;
        if (*boma).battle_object_id == defender {
            calculate_finishing_hit(defender, attacker, ctx.registers[19].x() as *const f32);
        }
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x633de0).nop(); //Removes the vanilla kill zoom in favor of the updated function. This one handles normal hits
    let _ = skyline::patching::Patch::in_text(0x6373a4).data(0xD503201Fu32); //Removes the vanilla kill zoom in favor of the updated function. This one handles throws
    skyline::install_hooks!(
        process_knockback,
        calculate_knockback
    );
}