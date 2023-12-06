#import bevy_pbr::forward_io::VertexOutput

@group(1) @binding(0) var image: texture_2d<f32>;
@group(1) @binding(1) var image_sampler: sampler;
@group(1) @binding(2) var<uniform> data: ShaderData;

struct ShaderData {
	time: f32,
	color: vec4<f32>,
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {

	var out = vec4<f32>(0.0, 0.0, 0.0, 1.0);

	let red = textureSample(image, image_sampler, ( mesh.uv + vec2<f32>(0.04, 0.04) * data.time ) % vec2<f32>(1.));
	let blue = textureSample(image, image_sampler, ( mesh.uv + vec2<f32>(0.08, 0.08) * data.time )% vec2<f32>(1.));
	let green = textureSample(image, image_sampler, ( mesh.uv + vec2<f32>(0.1, 0.1) * data.time )% vec2<f32>(1.));

	let energy = ( red + blue + green ) / 3.0;



	var color = data.color + vec4<f32>(0.3, 0.3, 0.3, 0.0);

	out = mix(out, color * color * color, energy);


	let border = 0.01;
	let border_color = data.color * vec4<f32>(vec3<f32>(5.),1.);
	let border_mix = step(1.- border, mesh.uv.x) + step(mesh.uv.x, border) + step(1.- border, mesh.uv.y) + step(mesh.uv.y, border);
	//out = mix(out, border_color, border_mix);

    return out;
}
