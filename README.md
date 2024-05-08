\[PoC\] Use async with the savvy framework
================

<!-- README.md is generated from README.Rmd. Please edit that file -->

This repository is an experiment to use ALTREP to wrap async with the
power of my [savvy framework](https://github.com/yutannihilation/savvy).

<!-- badges: start -->
<!-- badges: end -->

## Implementation

This function returns `c(1L, 2L, 3L)` after sleeping for 3 seconds.

``` rust
async fn vec_i32_async() -> Vec<i32> {
    async {
        Timer::after(std::time::Duration::from_secs(3)).await;
        vec![1, 2, 3]
    }
    .await
}

// ...snip...

/// @export
#[savvy]
fn sleepy_vec() -> savvy::Result<savvy::Sexp> {
    let f = smol::spawn(vec_i32_async());
    SleepyVec {
        future: Box::pin(f),
        result: None,
    }
    .into_altrep()
}
```

## Behavior

``` r
library(savvyAsyncPoC)

x <- sleepy_vec()

# While I'm waiting for 1 second, the task is executed in background.
Sys.sleep(1)

# It takes 3 seconds to finish the task, and 1 second has passed.
# So, this should take about 2 seconds.
system.time(x[1])
#>    user  system elapsed 
#>    0.00    0.00    1.95

# After that, the result is cached.
system.time(sum(x))
#>    user  system elapsed 
#>       0       0       0
```
