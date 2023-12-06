#import bevy_pbr::forward_io::VertexOutput

@group(1) @binding(0) var image: texture_2d<f32>;
@group(1) @binding(1) var image_sampler: sampler;
@group(1) @binding(2) var<uniform> data: ShaderData;

struct ShaderData {
	time: f32,
	color: vec4<f32>,
	threshold : f32,
	intensity : f32,
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {

	var out = textureSample(image, image_sampler, mesh.uv);
	let energy = (out.r + out.b + out.g) / 3.0;

	let threshold = step(data.threshold, energy);
	out.r += data.intensity * threshold * abs(sin(data.time));

	//let border = 0.01;
	//let border_color = data.color * vec4<f32>(vec3<f32>(5.),1.);
	//let border_mix = step(1.- border, mesh.uv.x) + step(mesh.uv.x, border) + step(1.- border, mesh.uv.y) + step(mesh.uv.y, border);
	//out = mix(out, border_color, border_mix);

    return out;
}
