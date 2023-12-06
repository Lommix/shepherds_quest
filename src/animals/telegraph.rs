use bevy::{prelude::*, render::render_resource::AsBindGroup, pbr::NotShadowCaster};
pub struct TelegraphPlugin;
impl Plugin for TelegraphPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<TelegraphMaterial>::default());
    }
}

#[derive(Component)]
pub struct TelegraphTag;

#[derive(Bundle)]
pub struct TelegraphBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<TelegraphMaterial>,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub tag: TelegraphTag,
    pub no_shadow: NotShadowCaster,
}

impl Default for TelegraphBundle {
    fn default() -> Self {
        Self {
            mesh: Handle::default(),
            material: Handle::default(),
            visibility: Visibility::Inherited,
            inherited_visibility: InheritedVisibility::HIDDEN,
            view_visibility: ViewVisibility::HIDDEN,
            transform: Transform::IDENTITY,
            global_transform: GlobalTransform::IDENTITY,
            no_shadow: NotShadowCaster,
            tag: TelegraphTag,
        }
    }
}

#[derive(Asset, TypePath, Default, Clone, AsBindGroup)]
pub struct TelegraphMaterial {
    // only need one f32, but wasm wants 128-bit alignment
    #[uniform(0)]
    pub progress: Vec4,
    #[uniform(1)]
    pub color: Color,
}

impl Material for TelegraphMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/telegraph.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
