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
            <Legend />
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
                if let Ok(id) = uuid::Uuid::parse_str(&data) {
                    dropped_gates.update(|gates| {
                        if let Some(gate) = gates.iter_mut().find(|g| g.get_id() == id.to_string()) {
                            gate.set_pos(x, y);
                        }
                    });
                } else if let Some(gate_type) = GateType::from_str(&data) {
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
                    key=|gate| gate.get_id()
                    children=move |gate| view!{<RotatableGate gate draggable=true />}
                />
                <CircuitConnector />
            </Show>
        </div>
    }
}

#[component]
pub fn CircuitConnector() -> impl IntoView {
    // TODO add circles to end of wires
    // TODO add way to edit wire start and end points
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
            style="top: 0; left: 0; width: 100%; height: 100%; z-index: 0;"
            on:mousemove=on_mouse_move
            on:click=on_click
        >
            {move || connections.get().into_iter().map(|((x1, y1), (x2, y2))| {
                view! {
                    <line x1=x1 y1=y1 x2=x2 y2=y2 stroke="black" stroke-width="2"/>
                    <circle cx=x1 cy=y1 r="4" fill="black"/>
                    <circle cx=x2 cy=y2 r="4" fill="black"/>
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

#[component]
fn Legend() -> impl IntoView {
    view! {
        <div style="padding: 8px; border: 1px solid #888;">
            <h3 style="margin: 0 0 8px 0;">"Legend"</h3>
            <div style="display: flex; align-items: center; gap: 8px;">
                <span>"On:"</span>
                <svg width="120" height="10">
                    <line x1=0 y1=5 x2=90 y2=5 stroke="green" stroke-width="2"/>
                    <circle cx=4 cy=5 r="4" fill="black"/>
                    <circle cx=90 cy=5 r="4" fill="black"/>
                </svg>
            </div>
            <div style="display: flex; align-items: center; gap: 8px;">
                <span>"Off:"</span>
                <svg width="120" height="10">
                    <line x1=0 y1=5 x2=90 y2=5 stroke="black" stroke-width="2"/>
                    <circle cx=4 cy=5 r="4" fill="black"/>
                    <circle cx=90 cy=5 r="4" fill="black"/>
                </svg>
            </div>
            <div style="display: flex; align-items: center; gap: 8px;">
                <span>"None:"</span>
                <svg width="120" height="10">
                    <line x1=0 y1=5 x2=100 y2=5 stroke="red" stroke-dasharray="4"/>
                </svg>
            </div>
        </div>
    }
}