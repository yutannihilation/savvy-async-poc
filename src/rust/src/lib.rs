use std::pin::Pin;

use async_io::Timer;
use futures_lite::Future;
use savvy::{
    altrep::{register_altinteger_class, AltInteger},
    ffi::DllInfo,
    savvy, savvy_init,
};

async fn vec_i32_async() -> Vec<i32> {
    async {
        Timer::after(std::time::Duration::from_secs(3)).await;
        vec![1, 2, 3]
    }
    .await
}

struct SleepyVec {
    future: Pin<Box<dyn Future<Output = Vec<i32>>>>,
    result: Option<Vec<i32>>,
}

impl savvy::IntoExtPtrSexp for SleepyVec {}

impl AltInteger for SleepyVec {
    const CLASS_NAME: &'static str = "SleepyVec";
    const PACKAGE_NAME: &'static str = "savvyAsyncPoC";

    fn length(&mut self) -> usize {
        3
    }

    fn elt(&mut self, i: usize) -> i32 {
        match &self.result {
            Some(result) => result[i],
            None => {
                let f = self.future.as_mut();
                let result = futures_lite::future::block_on(f);
                let out = result[i];
                self.result = Some(result);
                out
            }
        }
    }
}

/// @export
#[savvy]
fn sleepy_vec() -> savvy::Result<savvy::Sexp> {
    let f = vec_i32_async();
    SleepyVec {
        future: Box::pin(f),
        result: None,
    }
    .into_altrep()
}

#[savvy_init]
fn init_altrep_class(dll_info: *mut DllInfo) -> savvy::Result<()> {
    register_altinteger_class::<SleepyVec>(dll_info)?;
    Ok(())
}
