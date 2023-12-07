#import bevy_pbr::forward_io::VertexOutput

@group(1) @binding(0) var<uniform> progress: vec4<f32>;
@group(1) @binding(1) var<uniform> color: vec4<f32>;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {

	var out = color;

	let dist = distance(mesh.uv, vec2<f32>(0.5, 0.5));

	let p = progress.x /2.0;

	let mask = 1.0 - smoothstep(p, p, dist);

	out.a *= mask * 0.5;

	let border_thickness: f32 = 0.005;
	let distance_from_edge: f32 = abs(0.495 - dist);
	let border_mask: f32 = step(distance_from_edge, border_thickness);

	out = mix(out, mix(color, vec4<f32>(8.), 0.3), border_mask);

    return out;
}
