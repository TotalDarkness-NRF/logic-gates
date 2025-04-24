use leptos::*;
use leptos::leptos_dom::logging::console_log;
use crate::model::{gate_type::GateType, logic_gate::LogicGate};

#[component]
pub fn RenderGateType(gate_type: GateType) -> impl IntoView {
    match gate_type {
        GateType::And => view!{<AndGate />},
        GateType::Or => view!{<OrGate />},
        GateType::Not => view!{<NotGate />},
        GateType::Nand => view!{<AndGate not=true />},
        GateType::Nor => view!{<OrGate not=true /> },
        GateType::Xor => view!{<XorGate />},
        GateType::Xnor => view!{<XorGate not=true />},
    }
}

#[component]
pub fn RenderGate(gate_type: GateType, #[prop(default = false)] draggable: bool) -> impl IntoView {
    let gate_str = gate_type.as_str();
    view! {
        <div
            draggable=draggable.to_string()
            on:dragstart=move |e| {
                let dt = e.data_transfer().unwrap();
                dt.set_data("text/plain", gate_str).unwrap();
            }
            style="height: 100px; padding: 1px; border: 1px solid #888; margin: 1px; background: #fff; cursor: grab;"
        >
            <RenderGateType gate_type />
        </div>
    }
}

#[component]
pub fn RotatableGate(#[prop(default = 0)] angle: i32, #[prop(default = false)] draggable: bool, mut gate: LogicGate) -> impl IntoView {
    // TODO really stupid way of moving a gate.
    // TODO find out if gate is the same gate in the dropped_gate list
    let (x, set_x) = create_signal(gate.get_x().unwrap_or(0) + 125);
    let (y, set_y) = create_signal(gate.get_y().unwrap_or(0) - (125/3));
    let gate_id = gate.get_id();
    let gate_type = gate.gate_type;
    view! {
        <div
            draggable=draggable.to_string()
            on:dragstart=move |e| {
                if let Some(dt) = e.data_transfer() {
                    dt.set_data("text/plain", gate_id.as_str()).ok();
                }
            }
            on:dragend=move |e| {
                let new_x = e.client_x();
                let new_y = e.client_y();
                gate.set_pos(new_x, new_y);
                set_x.update(|x| *x = new_x - 125/2);
                set_y.update(|y| *y = new_y - 125/3);
            }
            
            style=move || format!("cursor: grab; position: absolute; left: {}px; top: {}px;", x.get(), y.get())
        >
            <svg width="125" height="125" xmlns="http://www.w3.org/2000/svg">
                <g transform=format!("rotate({}, 63, 63)", angle)>
                    <RenderGateType gate_type />
                </g>
            </svg>
        </div>
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