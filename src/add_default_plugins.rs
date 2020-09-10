use crate::app::AppBuilder;

pub trait AddDefaultPlugins {
    fn add_default_plugins(&mut self) -> &mut Self;
}

impl AddDefaultPlugins for AppBuilder {
    fn add_default_plugins(&mut self) -> &mut Self {
        self.add_plugin(bevy_type_registry::TypeRegistryPlugin::default());
        self.add_plugin(bevy_core::CorePlugin::default());
        self.add_plugin(bevy_transform::TransformPlugin::default());
        self.add_plugin(bevy_diagnostic::DiagnosticsPlugin::default());
        self.add_plugin(bevy_input::InputPlugin::default());
        self.add_plugin(bevy_window::WindowPlugin::default());
        self.add_plugin(bevy_asset::AssetPlugin::default());
        self.add_plugin(bevy_scene::ScenePlugin::default());
        self.add_plugin(bevy_render::RenderPlugin::default());
        self.add_plugin(bevy_sprite::SpritePlugin::default());
        self.add_plugin(bevy_pbr::PbrPlugin::default());
        self.add_plugin(bevy_ui::UiPlugin::default());
        self.add_plugin(bevy_text::TextPlugin::default());
        self.add_plugin(bevy_animation::AnimationPlugin::default());

        #[cfg(feature = "bevy_audio")]
        self.add_plugin(bevy_audio::AudioPlugin::default());

        #[cfg(feature = "bevy_gltf")]
        self.add_plugin(bevy_gltf::GltfPlugin::default());

        #[cfg(feature = "bevy_winit")]
        self.add_plugin(bevy_winit::WinitPlugin::default());

        #[cfg(feature = "bevy_wgpu")]
        self.add_plugin(bevy_wgpu::WgpuPlugin::default());

        self
    }
}
