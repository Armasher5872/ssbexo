const PTRAINER_DEATH_SWAP_OFFSET: usize = 0xf96330;

//Removes the death swap from PT
#[skyline::hook(offset = PTRAINER_DEATH_SWAP_OFFSET)]
unsafe fn ptrainer_death_swap() {}

pub fn install() {
    skyline::install_hook!(ptrainer_death_swap);
}