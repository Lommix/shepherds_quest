#import bevy_pbr::forward_io::VertexOutput

@group(1) @binding(0) var image: texture_2d<f32>;
@group(1) @binding(1) var image_sampler: sampler;
@group(1) @binding(2) var<uniform> data: ShaderData;

struct ShaderData {
	time: f32,
	color: vec4<f32>,
	threshold : vec4<f32>,
	intensity : f32,
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {

	var out = textureSample(image, image_sampler, mesh.uv);

	let threshold = step(1. - data.threshold.rbg, out.rgb);

	out.r += data.intensity * threshold.r * abs(sin(data.time));
	out.g += data.intensity * threshold.g * abs(sin(data.time));
	out.b += data.intensity * threshold.b * abs(sin(data.time));

    return out;
}
