use bevy::{
    feathers::{
        self, FeathersPlugins,
        controls::{ButtonProps, button},
        dark_theme::create_dark_theme,
        theme::{ThemeBackgroundColor, ThemedText, UiTheme},
    },
    prelude::*,
    ui_widgets::{Activate, observe},
    window::PrimaryWindow,
};
use bevy_window_as_ui_root::{CloseWith, WindowAsUiRoot, WindowAsUiRootPlugin};

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, WindowAsUiRootPlugin, FeathersPlugins))
        .insert_resource(UiTheme(create_dark_theme()))
        .add_systems(Startup, setup)
        .run()
}

fn new_popup() -> impl Bundle {
    (
        WindowAsUiRoot,
        ThemeBackgroundColor(feathers::tokens::WINDOW_BG),
        Node {
            padding: px(8).all(),
            ..default()
        },
        children![(
            button(
                ButtonProps::default(),
                (),
                Spawn((Text::new("Open a Popup"), ThemedText)),
            ),
            observe(|activate: On<Activate>, mut commands: Commands| {
                let this = activate.entity;
                let popup = new_popup();
                commands
                    .get_entity(this)
                    .unwrap()
                    .with_related::<CloseWith>(popup);
            })
        )],
    )
}

fn setup(mut commands: Commands, primary_window: Single<Entity, With<PrimaryWindow>>) {
    let popup = new_popup();
    commands.get_entity(*primary_window).unwrap().insert(popup);
}
