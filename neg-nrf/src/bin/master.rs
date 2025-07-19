#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use nazo6_neg_nrf::{hooks::create_hooks, keymap::KEYMAP};
use neg_nrf::{init_peri, start_master};

use neg_nrf as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = init_peri();

    let hooks = create_hooks(unsafe { p.P0_31.clone_unchecked() });
    start_master(p, hooks, &KEYMAP).await;
}
