#[namespace = "electical_circuit_editor"]

#[rust(use = "crate::editor::editor_state")]

enum ElementId {
    Resistor,
    Capacitor,
    VoltageSource,
    CurrentSource,
}

enum UIEvent {
    PlaceElement {
        element_id: ElementId
    },
    RotateElement {
        element_id: ElementId
    },
}

struct UIState;

fn editor_state(event: UIEvent) -> UIState;
