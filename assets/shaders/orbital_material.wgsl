struct OrbitalMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: OrbitalMaterial;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> { 

    let b = 0.5;
    let a = 0.5;

    let x = (uv.x - a);
    let y = (uv.y - b);

    let circle = sqrt((x)*(x) + (y)*(y));
    let ring_component = sin(circle * 100.0 - 200.0) - 0.95;
    let distance_component = (x*x+y*y) / 5.0;
    return vec4(1.0, 1.0, 1.0, clamp(ring_component - distance_component, 0.0, 0.005));
}