use crate::{
    render_graph::{Node, ResourceSlotInfo, ResourceSlots},
    renderer::{RenderContext, RenderResourceId, RenderResourceType},
    texture::TextureDescriptor,
};
use bevy_app::prelude::{EventReader, Events};
use bevy_ecs::{Resources, World};
use bevy_window::{WindowCreated, WindowId, WindowResized, Windows};
use std::borrow::Cow;

pub struct WindowTextureNode {
    window_id: WindowId,
    descriptor: TextureDescriptor,
    window_created_event_reader: EventReader<WindowCreated>,
    window_resized_event_reader: EventReader<WindowResized>,
}

impl WindowTextureNode {
    pub const OUT_TEXTURE: &'static str = "texture";

    pub fn new(window_id: WindowId, descriptor: TextureDescriptor) -> Self {
        WindowTextureNode {
            window_id,
            descriptor,
            window_created_event_reader: Default::default(),
            window_resized_event_reader: Default::default(),
        }
    }
}

impl Node for WindowTextureNode {
    fn output(&self) -> &[ResourceSlotInfo] {
        static OUTPUT: &[ResourceSlotInfo] = &[ResourceSlotInfo {
            name: Cow::Borrowed(WindowTextureNode::OUT_TEXTURE),
            resource_type: RenderResourceType::Texture,
        }];
        OUTPUT
    }

    fn update(
        &mut self,
        _world: &World,
        resources: &Resources,
        render_context: &mut dyn RenderContext,
        _input: &ResourceSlots,
        output: &mut ResourceSlots,
    ) {
        const WINDOW_TEXTURE: usize = 0;
        let window_created_events = resources.get::<Events<WindowCreated>>().unwrap();
        let window_resized_events = resources.get::<Events<WindowResized>>().unwrap();
        let windows = resources.get::<Windows>().unwrap();

        let window = windows
            .get(self.window_id)
            .expect("Received window resized event for non-existent window");

        if self
            .window_created_event_reader
            .find_latest(&window_created_events, |e| e.id == window.id())
            .is_some()
            || self
                .window_resized_event_reader
                .find_latest(&window_resized_events, |e| e.id == window.id())
                .is_some()
        {
            let render_resource_context = render_context.resources_mut();
            if let Some(RenderResourceId::Texture(old_texture)) = output.get(WINDOW_TEXTURE) {
                render_resource_context.remove_texture(old_texture);
            }

            self.descriptor.size.width = window.width();
            self.descriptor.size.height = window.height();
            let texture_resource = render_resource_context.create_texture(self.descriptor);
            output.set(WINDOW_TEXTURE, RenderResourceId::Texture(texture_resource));
        }
    }
}
