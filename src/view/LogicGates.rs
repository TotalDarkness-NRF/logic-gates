use leptos::*;
use leptos_meta::Title;
use web_sys::DragEvent;

#[component]
pub fn render() -> impl IntoView {
    let dropped_gates = create_rw_signal::<Vec<GateType>>(vec![]);
    view! {
        <Title text="Logic Gates"/>
        <div style="display: flex; height: 100vh; font-family: sans-serif;">
            <GatePalette />
            <DropZone dropped_gates=dropped_gates />
        </div>
    }
}

#[component]
fn GatePalette() -> impl IntoView {
    let all_gates = vec![
        GateType::And,
        GateType::Or,
        GateType::Not,
        GateType::Nand,
        GateType::Nor,
        GateType::Xor,
        GateType::Xnor,
    ];

    view! {
        <div style="width: 200px; padding: 10px; border-right: 2px solid #ccc;">
            <h3>"Logic Gates"</h3>
            <For
                each=move || all_gates.clone()
                key=|gate| gate.as_str().to_string()
                children=move |gate| {
                    let gate_str = gate.as_str();
                    view! {
                        <div
                            draggable="true"
                            on:dragstart=move |e| {
                                let dt = e.data_transfer().unwrap();
                                dt.set_data("text/plain", gate.as_str()).unwrap();
                            }
                            style="padding: 8px; border: 1px solid #888; margin: 6px; background: #fff; cursor: grab;"
                        >
                            {gate_str}
                        </div>
                    }
                }
            />
        </div>
    }
}

#[component]
fn DropZone(dropped_gates: RwSignal<Vec<GateType>>) -> impl IntoView {
    let handle_drop = move |e: DragEvent| {
        e.prevent_default();
        if let Some(dt) = e.data_transfer() {
            if let Ok(data) = dt.get_data("text/plain") {
                if let Some(gate) = GateType::from_str(&data) {
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
                    key=|gate| gate.as_str().to_string() + &uuid::Uuid::new_v4().to_string()
                    children=move |gate| {
                        view! {
                            <div style="margin: 6px; padding: 6px; border: 1px solid #666; background: #fff;">
                                {gate.as_str()}
                            </div>
                        }
                    }
                />
            </Show>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq)]
enum GateType {
    And,
    Or,
    Not,
    Nand,
    Nor,
    Xor,
    Xnor,
}

impl GateType {
    fn as_str(&self) -> &'static str {
        match self {
            GateType::And => "AND",
            GateType::Or => "OR",
            GateType::Not => "NOT",
            GateType::Nand => "NAND",
            GateType::Nor => "NOR",
            GateType::Xor => "XOR",
            GateType::Xnor => "XNOR",
        }
    }

    fn from_str(name: &str) -> Option<Self> {
        match name {
            "AND" => Some(GateType::And),
            "OR" => Some(GateType::Or),
            "NOT" => Some(GateType::Not),
            "NAND" => Some(GateType::Nand),
            "NOR" => Some(GateType::Nor),
            "XOR" => Some(GateType::Xor),
            "XNOR" => Some(GateType::Xnor),
            _ => None,
        }
    }
}
