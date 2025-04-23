use leptos::*;
use leptos_meta::Title;
use web_sys::DragEvent;
use crate::model::logic_gate::LogicGate;

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
fn RotatableGate(#[prop(default = 0)] angle: i32, gate: LogicGate) -> impl IntoView {
    let gate_view = match gate {
        LogicGate::And => view!{<AndGate />},
        LogicGate::Or => view!{<OrGate />},
        LogicGate::Not => view!{<NotGate />},
        LogicGate::Nand => view!{<AndGate not=true/>},
        LogicGate::Nor => view!{<OrGate not=true />},
        LogicGate::Xor => view!{<XorGate />},
        LogicGate::Xnor => view!{<XorGate not=true />},
    };
    view! {
        <svg width="125" height="125" xmlns="http://www.w3.org/2000/svg">
            <g transform=format!("rotate({}, 63, 63)", angle)> { gate_view } </g>
        </svg>
    }
}

#[component]
fn NotGate() -> impl IntoView {
    view! {
        <svg width="125" height="125" xmlns="http://www.w3.org/2000/svg">
            <polygon points="20,20 20,80 80,50" fill="lightgray" stroke="black" stroke-width="2"/>

            <line x1="5" y1="50" x2="20" y2="50" stroke="black" stroke-width="2"/>
            <line x1="90" y1="50" x2="110" y2="50" stroke="black" stroke-width="2"/>

            <circle cx="5" cy="50" r="4" fill="black"/>
            <circle cx="110" cy="50" r="4" fill="black"/>
            <circle cx="85" cy="50" r="6" fill="white" stroke="black" stroke-width="2"/>

            <text x="45" y="55" font-size="16" text-anchor="middle" fill="black">{"NOT"}</text>
        </svg>
    }
}

#[component]
fn OrGate(#[prop(default = false)] not: bool) -> impl IntoView {
    view! {
        <svg width="125" height="125" xmlns="http://www.w3.org/2000/svg">
            <path d="M20,20 Q50,50 20,80 Q60,80 100,50 Q60,20 20,20 Z" fill="lightgray" stroke="black" stroke-width="2"/>

            <circle cx="5" cy="30" r="4" fill="black"/>
            <circle cx="5" cy="70" r="4" fill="black"/>
            <circle cx="120" cy="50" r="4" fill="black"/>

            <line x1="5" y1="30" x2="29" y2="30" stroke="black" stroke-width="2"/>
            <line x1="5" y1="70" x2="29" y2="70" stroke="black" stroke-width="2"/>
            <line x1="100" y1="50" x2="120" y2="50" stroke="black" stroke-width="2"/>
            
            {if not {
                Some(view! {
                    <circle cx="105" cy="50" r="6" fill="white" stroke="black" stroke-width="2"/>
                })
            } else { None }}

            {if not { 
                view!{<text x="60" y="55" font-size="16" text-anchor="middle" fill="black">"NOR"</text>}
            } else { 
                view!{<text x="55" y="55" font-size="16" text-anchor="middle" fill="black">"OR"</text>}
            }}
        </svg>
    }
}

#[component]
fn AndGate(#[prop(default = false)] not: bool) -> impl IntoView {
    view! {
        <svg width="125" height="125" xmlns="http://www.w3.org/2000/svg">
            <path d="M30,20 H70 A30,30 0 0 1 70,80 H30 Z" fill="lightgray" stroke="black" stroke-width="2"/>

            <circle cx="5" cy="30" r="4" fill="black" />
            <circle cx="5" cy="70" r="4" fill="black" />
            <circle cx="120" cy="50" r="4" fill="black" />

            <line x1="5" y1="30" x2="30" y2="30" stroke="black" stroke-width="2"/>
            <line x1="5" y1="70" x2="30" y2="70" stroke="black" stroke-width="2"/>
            <line x1="100" y1="50" x2="120" y2="50" stroke="black" stroke-width="2"/>

            {if not {
                Some(view! {
                    <circle cx="106" cy="50" r="6" fill="white" stroke="black" stroke-width="2"/>
                })
            } else { None }}

            {if not {
                view!{<text x="63" y="55" font-size="16" text-anchor="middle" fill="black">"NAND"</text>} 
            } else {
                view!{<text x="60" y="55" font-size="16" text-anchor="middle" fill="black">"AND"</text>}
            }}
        </svg>
    }
}

#[component]
fn XorGate(#[prop(default = false)] not: bool) -> impl IntoView {
    view! {
        <svg width="125" height="125" xmlns="http://www.w3.org/2000/svg">
            <path d="M20,20 Q50,50 20,80 Q60,80 100,50 Q60,20 20,20 Z" fill="lightgray" stroke="black" stroke-width="2"/>
            <path d="M10,20 Q40,50 10,80" fill="none" stroke="black" stroke-width="2"/>

            <line x1="5" y1="30" x2="19" y2="30" stroke="black" stroke-width="2"/>
            <line x1="5" y1="70" x2="19" y2="70" stroke="black" stroke-width="2"/>
            <line x1="100" y1="50" x2="120" y2="50" stroke="black" stroke-width="2"/>
            
            <circle cx="5" cy="30" r="4" fill="black"/>
            <circle cx="5" cy="70" r="4" fill="black"/>
            <circle cx="120" cy="50" r="4" fill="black"/>

            {if not {
                Some(view! {
                    <circle cx="106" cy="50" r="6" fill="white" stroke="black" stroke-width="2"/>
                })
            } else { None }}

            {if not {
                view!{<text x="63" y="55" font-size="16" text-anchor="middle" fill="black">{"XNOR"}</text>} 
            } else {
                view!{<text x="60" y="55" font-size="16" text-anchor="middle" fill="black">{"XOR"}</text>}
            }}
        </svg>
    }
}


#[component]
fn RenderGate(gate: LogicGate, #[prop(default = false)] draggable: bool) -> impl IntoView {
    let gate_str = gate.as_str();
    view! {
        <div
            draggable=draggable.to_string()
            on:dragstart=move |e| {
                let dt = e.data_transfer().unwrap();
                dt.set_data("text/plain", gate_str).unwrap();
            }
            style="height: 100px; padding: 1px; border: 1px solid #888; margin: 1px; background: #fff; cursor: grab;"
        >
        <RotatableGate gate=gate />
        </div>
    }
}

#[component]
fn DragZone() -> impl IntoView {
    let all_gates = LogicGate::get_all_gates();
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
        if let Some(dt) = e.data_transfer() {
            if let Ok(data) = dt.get_data("text/plain") {
                if let Some(gate) = LogicGate::from_str(&data) {
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
                    children=move |gate| view! {<RotatableGate gate=gate />}
                />
            </Show>
        </div>
    }
}