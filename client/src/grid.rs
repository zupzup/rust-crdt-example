#![allow(non_snake_case)]
use leptos::*;

type RowCol = Vec<(usize, Vec<(usize, String)>)>;

#[component]
pub fn Grid(data: ReadSignal<RowCol>, set_data: WriteSignal<RowCol>) -> impl IntoView {
    let (changed, set_changed) = create_signal(String::from("nothing changed"));

    view! {
        <div class="grid-container">
            <FormatHeader />
            <div class="grid">
            <div>"Changed: " {move || changed.get()}</div>
            <For each=move || data.get()
             key=|r| r.0
             children=move |row| view! {
             <Row>
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
             </Row>
            }/>
            </div>
            <div>
                {move || data.get()}
            </div>
        </div>
    }
}

#[component]
fn Row(children: Children) -> impl IntoView {
    view! {
        <div class="row">
            {children()}
        </div>
    }
}

// #[component]
// fn Column(children: Children, on_change: Fn()) -> impl IntoView {
//     view! {
//         <div class="column">
//             {children()}
//         </div>
//     }
// }

#[component]
fn FormatHeader() -> impl IntoView {
    view! {
        <div class="grid-header">
            <span class="btn"><strong>B</strong></span>
            <span class="btn"><strong>I</strong></span>
            <span class="btn"><strong>U</strong></span>
        </div>
    }
}
