use bevy::{
    prelude::*,
    window::{
        ApplicationLifetime, RequestRedraw, WindowBackendScaleFactorChanged, WindowCloseRequested,
        WindowClosed, WindowCreated, WindowDestroyed, WindowFocused, WindowResized,
        WindowScaleFactorChanged, WindowThemeChanged,
    },
};

/// A plugin that adds all the default plugins, except for the `WindowPlugin`.
/// it adds the necessary elements for the rest of the plugins to work.
///
/// **IMPORTANT**: Bevy cannot take screenshots without a window, so this plugin prevents
/// playback test gear from taking screenshots.
///
/// **ALSO IMPORTANT**: To correctly pick up a missing window, this plugin must be inserted **before**
/// `bitt::PlaybackTestGear`.
pub struct HeadlessDefaultPlugins;

impl Plugin for HeadlessDefaultPlugins {
    fn build(&self, app: &mut App) {
        app.add_event::<WindowResized>()
            .add_event::<WindowCreated>()
            .add_event::<WindowClosed>()
            .add_event::<WindowCloseRequested>()
            .add_event::<WindowDestroyed>()
            .add_event::<RequestRedraw>()
            .add_event::<CursorMoved>()
            .add_event::<CursorEntered>()
            .add_event::<CursorLeft>()
            .add_event::<ReceivedCharacter>()
            .add_event::<Ime>()
            .add_event::<WindowFocused>()
            .add_event::<WindowScaleFactorChanged>()
            .add_event::<WindowBackendScaleFactorChanged>()
            .add_event::<FileDragAndDrop>()
            .add_event::<WindowMoved>()
            .add_event::<WindowThemeChanged>()
            .add_event::<ApplicationLifetime>()
            .add_plugins(DefaultPlugins.build().disable::<WindowPlugin>());
    }
}
