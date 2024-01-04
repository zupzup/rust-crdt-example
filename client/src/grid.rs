#![allow(non_snake_case)]
use leptos::*;

type RowCol = Vec<(usize, Vec<(usize, String)>)>;

#[component]
pub fn Grid(data: ReadSignal<RowCol>, set_data: WriteSignal<RowCol>) -> impl IntoView {
    let (changed, set_changed) = create_signal(String::from("nothing changed"));

    view! {
        <div class="grid-container">
            <div class="grid">
                <div>"Changed: " {move || changed.get()}</div>
                <For each=move || data.get()
                 key=|r| r.0
                 children=move |row| view! {
                     <div class="row">
                         <For each=move || row.1.clone()
                              key=|c| c.0
                              children=move |col| view! {
                                  <input type="text" on:input=move |ev| {
                                      let val = event_target_value(&ev);
                                      logging::log!("ev: {:?}", ev);
                                      logging::log!("val: {val}");
                                      set_changed.update(|v| *v = format!("{} {} changed to {val}", row.0, col.0));
                                      set_data.update(|d| d[row.0].1[col.0].1 = val);
                                  }
                                  prop:value={col.1} />
                              }/>
                     </div>
                }/>
            </div>
            <div>
                {move || data.get()}
            </div>
        </div>
    }
}
