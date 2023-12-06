use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
};
pub struct LiquidMaterialsPlugin;
impl Plugin for LiquidMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<LiquidMaterial>::default());
        app.add_systems(Update, update_liquid);
    }
}

fn update_liquid(time: Res<Time>, mut materials: ResMut<Assets<LiquidMaterial>>) {
    materials.iter_mut().for_each(|(_, material)| {
        material.uniforms.time += time.delta_seconds();
    });
}

#[derive(ShaderType, Clone, Default)]
pub struct LiquidData {
    pub time: f32,
    pub color: Color,
}

#[derive(Asset, TypePath, Default, Clone, AsBindGroup)]
pub struct LiquidMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub noise: Handle<Image>,
    #[uniform(2)]
    pub uniforms: LiquidData,
}

impl Material for LiquidMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/liquid.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
