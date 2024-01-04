#![allow(non_snake_case)]
use common::{ChangeEvent, Row};
use leptos::*;

#[component]
pub fn Grid(
    data: ReadSignal<Vec<Row>>,
    set_data_change: WriteSignal<Option<ChangeEvent>>,
) -> impl IntoView {
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
                              key=move |c| format!("{}{}", row.idx, c.idx)
                              children=move |col| view! {
                                  <input type="text" on:input=move |ev| {
                                      let val = event_target_value(&ev);
                                      logging::log!("ev: {:?}", ev);
                                      logging::log!("val: {val}");
                                      set_changed.update(|v| *v = format!("{} {} changed to {val}", row.idx, col.idx));
                                      set_data_change.update(|dc| *dc = Some(ChangeEvent { row: row.idx, column: col.idx, value: val }));
                                  }
                                  prop:value=move || data.get()[row.idx].columns[col.idx].value.clone()/>
                              }/>
                     </div>
                }/>
            </div>
            // <div>
            //     {move || data.get()[0].columns[0].value.clone()}
            // </div>
        </div>
    }
}
