#![allow(non_snake_case)]
use common::Row;
use leptos::*;

#[component]
pub fn Grid(data: ReadSignal<Vec<Row>>, set_data: WriteSignal<Vec<Row>>) -> impl IntoView {
    let (changed, set_changed) = create_signal(String::from("nothing changed"));

    view! {
        <div class="grid-container">
            <div class="grid">
                <div>"Changed: " {move || changed.get()}</div>
                <For each=move || data.get()
                 key=|r| r.idx
                 children=move |row| view! {
                     <div class="row">
                         <For each=move || row.columns.clone()
                              key=|c| c.idx
                              children=move |col| view! {
                                  <input type="text" on:input=move |ev| {
                                      let val = event_target_value(&ev);
                                      logging::log!("ev: {:?}", ev);
                                      logging::log!("val: {val}");
                                      set_changed.update(|v| *v = format!("{} {} changed to {val}", row.idx, col.idx));
                                      set_data.update(|d| d[row.idx].columns[col.idx].value = val);
                                  }
                                  prop:value={col.value} />
                              }/>
                     </div>
                }/>
            </div>
        </div>
    }
}
