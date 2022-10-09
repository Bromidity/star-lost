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

    let val = (uv.x - a)*(uv.x - a) + (uv.y - b)*(uv.y - b);
    let val1 = 1.0 - clamp((0.20 - val) * 10.0, 0.0, 1.0);
    let val2 = 1.0 - clamp((val - 0.20) * 10.0, 0.0, 1.0);
    let val3 = (val1 + val2 - 1.9) * 5.0;
    return vec4(1.0, 1.0, val3 / 5.0, 1.0);
}