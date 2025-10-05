use std::sync::Once;

mod ch02_p013_creating;
mod ch02_p019_varargs;
mod ch02_p023_newtypes;
mod ch02_p028_dsls;
mod ch02_p031_composing;
mod ch02_p038_ex1;

static TRACING_INIT: Once = Once::new();

#[allow(dead_code)]
fn init_tracing() {
  TRACING_INIT.call_once(|| {
    // https://www.reddit.com/r/rust/
    //  comments/18shil2/idiomatic_way_to_use_tracing_log_framework_in/

    tracing_subscriber::fmt::init();
  });
}
