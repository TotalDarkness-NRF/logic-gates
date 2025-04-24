use leptos::*;
use leptos_meta::Title;
use web_sys::{DragEvent, MouseEvent};
use crate::model::{gate_type::GateType, logic_gate::LogicGate};
use crate::view::LogicGates::{RenderGate, RotatableGate};

#[component]
pub fn render() -> impl IntoView {
    view! {
        <Title text="Logic Gates"/>
        <div style="user-select: none; display: flex; height: 100vh; font-family: sans-serif;">
            <DragZone />
            <DropZone />
            <CircuitConnector />  
        </div>
    }
}

#[component]
fn DragZone() -> impl IntoView {
    let all_gates = GateType::get_all_gates();
    view! {
        <div style="width: 150px; padding: 10px; border-right: 2px solid #ccc;">
            <h3>"Logic Gates"</h3>
            <For
                each=move || all_gates.clone()
                key=|gate| gate.as_str().to_string()
                children=move |gate_type| view! {<RenderGate gate_type draggable=true />}
            />
        </div>
    }
}

#[component]
fn DropZone() -> impl IntoView {
    let dropped_gates = create_rw_signal::<Vec<LogicGate>>(vec![]);
    let handle_drop = move |e: DragEvent| {
        e.prevent_default();
        let x = e.offset_x();
        let y = e.offset_y();
        if let Some(dt) = e.data_transfer() {
            if let Ok(data) = dt.get_data("text/plain") {
                if let Some(gate_type) = GateType::from_str(&data) {
                    let mut gate = LogicGate::get_logic_gate(gate_type);
                    gate.set_pos(x, y);
                    dropped_gates.update(|g| g.push(gate));
                }
            }
        }
    };
    view! {
        <div
            style="flex: 1; padding: 10px; background: #f0f0f0;"
            on:dragover=|e| e.prevent_default()
            on:drop=handle_drop
        >
            <h3>"Dropped Gates"</h3>
            <Show
                when=move || !dropped_gates.get().is_empty()
                fallback=|| view! { <p>"Drop gates here!"</p> }
            >
                <For
                    each=move || dropped_gates.get()
                    key=|gate| gate.gate_type.as_str().to_string() + &uuid::Uuid::new_v4().to_string()
                    children=move |gate| view!{<RotatableGate gate draggable=true />}
                />
            </Show>
        </div>
    }
}

#[component]
pub fn CircuitConnector() -> impl IntoView {
    let (start_point, set_start_point) = create_signal(None::<(f64, f64)>);
    let (hover_point, set_hover_point) = create_signal(None::<(f64, f64)>);
    let (connections, set_connections) = create_signal(Vec::new());

    let on_mouse_move = move |ev: MouseEvent| {
        set_hover_point.set(Some((ev.offset_x() as f64, ev.offset_y() as f64)));
    };

    let on_click = move |ev: MouseEvent| {
            let point = (ev.offset_x() as f64, ev.offset_y() as f64);
            match start_point.get() {
                Some(start) => {
                    set_connections.update(|conns| conns.push((start, point)));
                    set_start_point.set(None);
                    set_hover_point.set(None);
                },
                None => set_start_point.set(Some(point)),
            }
        };

    view! {
        <svg
            style="border: 1px solid gray;"
            on:mousemove=on_mouse_move
            on:click=on_click
        >
            {move || connections.get().into_iter().map(|((x1, y1), (x2, y2))| {
                view! {
                    <line x1=x1 y1=y1 x2=x2 y2=y2 stroke="black" stroke-width="2"/>
                }
            }).collect::<Vec<_>>()}

            {move || {
                if let (Some((x1, y1)), Some((x2, y2))) = (start_point.get(), hover_point.get()) {
                    Some(view! {
                        <line x1=x1 y1=y1 x2=x2 y2=y2 stroke="red" stroke-dasharray="4"/>
                    })
                } else {
                   None
                }
            }}
        </svg>
    }
}
