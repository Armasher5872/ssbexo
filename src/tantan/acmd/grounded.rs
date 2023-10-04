use super::*;

#[acmd_script( agent = "tantan", scripts = [
    "0x172240c033", "0x1778b4545c", "0x18bc531aa8", "game_aircatchhang", "game_aircatchhit", "game_aircatchhitloop", "game_aircatchpose", "game_attacklongstartl2", "game_attacklongstartl3", "game_attacklongstartlb2", "game_attacklongstartlb3",
    "game_attacklongstartr2", "game_attacklongstartr3", "game_attacklongstartrb2", "game_attacklongstartrb3", "game_attackshortstartl2", "game_attackshortstartl3", "game_attackshortstartlb1", "game_attackshortstartlb2", "game_attackshortstartlb3",
    "game_attackshortstartr2", "game_attackshortstartr3", "game_attackshortstartrb1", "game_attackshortstartrb2", "game_attackshortstartrb3", "game_catchpull", "game_damageair2", "game_specialairhi2", "game_specialhilong", "game_specialhishort"
], category = ACMD_GAME)]
unsafe fn ssbuexo_tantan_hurtbox_disable_acmd(_fighter: &mut L2CAgentBase) {}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_tantan_hurtbox_disable_acmd
    );
}