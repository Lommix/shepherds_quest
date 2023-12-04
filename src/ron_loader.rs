#[macro_export]
macro_rules! ron_asset_loader {
    (
        $plugin_name : ident,
        $asset_loader_name : ident,
        $asset_type : ident,
        $extensions : expr,
        $( $field_name:ident -> $handle_name:ident )*
        $(; $list_field:ident -> ( $sub_field:ident -> $sub_handle:ident ))*
    ) => {
        use bevy::{
            asset::{AssetLoader,AsyncReadExt, LoadedAsset},
            prelude::Plugin,
        };

        use bevy::prelude::*;

        pub struct $plugin_name;
        impl Plugin for $plugin_name {
            fn build(&self, app: &mut App) {
                app.init_asset::<$asset_type>();
                app.register_asset_loader($asset_loader_name);
                // app.add_systems(Update, asset_watcher);
            }
        }

        pub struct $asset_loader_name;
        impl AssetLoader for $asset_loader_name {

            type Asset = $asset_type;
            type Settings = ();
            type Error = bevy::asset::LoadDirectError;

            fn load<'a>(
                &'a self,
                reader: &'a mut bevy::asset::io::Reader,
                settings: &'a Self::Settings,
                load_context: &'a mut bevy::asset::LoadContext,
            ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
                Box::pin(async move {
                    let mut bytes = Vec::new();
                    reader.read_to_end(&mut bytes).await.unwrap();
                    let mut asset = ron::de::from_bytes::<$asset_type>(bytes.as_slice()).unwrap();

                    $(
                        asset.$handle_name = load_context.load(&asset.$field_name);
                    )*

                    $(
                        for sub_asset in &mut asset.$list_field {
                            sub_asset.$sub_handle = load_context.load(&sub_asset.$sub_field);
                        }
                    )*

                    Ok(asset)
                })
            }

            fn extensions(&self) -> &[&str] {
                $extensions
            }
        }

        // fn asset_watcher(
        //     mut game_assets: ResMut<GameAssets>,
        //     mut asset_event: EventReader<AssetEvent<$asset_type>>,
        // ) {
        //     asset_event.read().for_each(|ev| match ev {
        //         AssetEvent::Added { id } => {
        //             game_assets.handles.push(id.untyped());
        //         }
        //         _ => (),
        //     });
        // }
    };
}
