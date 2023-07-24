use std::collections::HashMap;
use leptos::*;

fn main() {
  _ = console_log::init_with_level(log::Level::Debug);
  console_error_panic_hook::set_once();
  // well IT CRASHES .. just less often
  mount_to_body(|cx| view! { cx,  <><Something/><SuperReloader/></> })
}

#[component]
pub fn Something(cx: Scope) -> impl IntoView {
    let mut some_mem = HashMap::new();
    for i in 0..100000 {
      some_mem.insert(i, i.to_string());
    }
  view!(cx,
    <>
        <h1>Some title</h1>
        <p>Something</p>
    </>
    )
}

#[component]
pub fn SuperReloader(cx: Scope) -> impl IntoView {
    let reload_after_delay = create_action(cx, move |_| async move {
      gloo_timers::future::TimeoutFuture::new(200).await;
      let window = window();
      log!("RELOADING THE UI !");
      window.location().reload().unwrap();
    });

    reload_after_delay.dispatch(());

    view! { cx,
      <span>THE UI WILL RELOAD !</span>
    }
}
