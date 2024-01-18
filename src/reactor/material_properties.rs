enum MaterialState {
    Solid,
    Liquid,
    Gas,
    SuperCritical
}

#[derive(Copy, Clone, Debug)]
struct Material {
    name: String,
    density: f32,
    melting_point: f32,
    boiling_point: f32,
    state_at_stp: MaterialState,
    molar_mass: f32,
}

