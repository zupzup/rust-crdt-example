#![allow(non_snake_case)]
use common::{ChangeEvent, Row};
use leptos::*;

#[component]
pub fn Grid(
    data: ReadSignal<Vec<Row>>,
    set_data_change: WriteSignal<Option<ChangeEvent>>,
) -> impl IntoView {
    view! {
        <div class="grid-container">
            <div class="grid">
                <For each=move || data.get()
                 key=|r| r.idx
                 children=move |row| view! {
                     <div class="row">
                         <For each=move || row.columns.clone()
                              key=move |c| format!("{}{}", row.idx, c.idx)
                              children=move |col| view! {
                                  <input type="text" on:input=move |ev| {
                                      let val = event_target_value(&ev);
                                      set_data_change.update(|dc| *dc = Some(ChangeEvent { row: row.idx, column: col.idx, value: val }));
                                  }
                                  prop:value=move || data.get()[row.idx].columns[col.idx].value.clone()/>
                              }/>
                     </div>
                }/>
            </div>
        </div>
    }
}
