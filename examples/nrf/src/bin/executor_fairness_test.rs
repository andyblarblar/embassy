#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::task::Poll;

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn run1() {
    loop {
        info!("DING DONG");
        Timer::after(Duration::from_ticks(16000)).await;
    }
}

#[embassy_executor::task]
async fn run2() {
    loop {
        Timer::at(Instant::from_ticks(0)).await;
    }
}

#[embassy_executor::task]
async fn run3() {
    futures::future::poll_fn(|cx| {
        cx.waker().wake_by_ref();
        Poll::<()>::Pending
    })
    .await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let _p = embassy_nrf::init(Default::default());
    unwrap!(spawner.spawn(run1()));
    unwrap!(spawner.spawn(run2()));
    unwrap!(spawner.spawn(run3()));
}
