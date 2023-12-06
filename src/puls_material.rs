use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
};
pub struct LiquidMaterialsPlugin;
impl Plugin for LiquidMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<PulsMaterial>::default());
        app.add_systems(Update, update_liquid);
    }
}

fn update_liquid(time: Res<Time>, mut materials: ResMut<Assets<PulsMaterial>>) {
    materials.iter_mut().for_each(|(_, material)| {
        material.uniforms.time += time.delta_seconds();
    });
}

#[derive(ShaderType, Clone, Default)]
pub struct LiquidData {
    pub time: f32,
    pub color: Color,
    pub energy_threshold: f32,
    pub intensity: f32,
}

#[derive(Asset, TypePath, Default, Clone, AsBindGroup)]
pub struct PulsMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub noise: Handle<Image>,
    #[uniform(2)]
    pub uniforms: LiquidData,
}

impl Material for PulsMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/puls.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
