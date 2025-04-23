use leptos::*;
use leptos_meta::Title;
use web_sys::DragEvent;
use crate::model::{gate_type::GateType, logic_gate::LogicGate};
use crate::view::LogicGates::{RenderGate, RotatableGate};

#[component]
pub fn render() -> impl IntoView {
    let dropped_gates = create_rw_signal::<Vec<LogicGate>>(vec![]);
    view! {
        <Title text="Logic Gates"/>
        <div style="user-select: none; display: flex; height: 100vh; font-family: sans-serif;" >
            <DragZone />
            <DropZone dropped_gates=dropped_gates />
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
                children=move |gate| view! {<RenderGate gate=gate draggable=true/>}
            />
        </div>
    }
}

#[component]
fn DropZone(dropped_gates: RwSignal<Vec<LogicGate>>) -> impl IntoView {
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
                    children=move |gate| view! {<RotatableGate gate=gate />}
                />
            </Show>
        </div>
    }
}