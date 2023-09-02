use leptos::*;

fn main() {
    mount_to_body( |cx| {
        view! {
            cx,
            <App/>
        }
    })
}

#[component]
fn App(cx: Scope) -> impl IntoView{
    let (count, set_count) = create_signal(cx, 0);

    view!{ cx,
        <progress 
            on:click=move |_| set_count.update(|cur| *cur += 2)
            max="50" value={move || count.get()}/>
    }
}
