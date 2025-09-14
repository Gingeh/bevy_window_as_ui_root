use bevy::{
    feathers::{
        self, FeathersPlugins,
        controls::{ButtonProps, button},
        dark_theme::create_dark_theme,
        theme::{ThemeBackgroundColor, ThemedText, UiTheme},
    },
    prelude::*,
    ui_widgets::{Activate, Callback},
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

fn new_popup(commands: &mut Commands) -> impl Bundle {
    (
        WindowAsUiRoot,
        ThemeBackgroundColor(feathers::tokens::WINDOW_BG),
        Node {
            padding: px(8).all(),
            ..default()
        },
        children![button(
            ButtonProps {
                on_click: Callback::System(commands.register_system(
                    |In(Activate(this)), mut commands: Commands| {
                        let popup = new_popup(&mut commands);
                        commands
                            .get_entity(this)
                            .unwrap()
                            .with_related::<CloseWith>(popup);
                    },
                )),
                ..default()
            },
            (),
            Spawn((Text::new("Open a Popup"), ThemedText)),
        )],
    )
}

fn setup(mut commands: Commands, primary_window: Single<Entity, With<PrimaryWindow>>) {
    let popup = new_popup(&mut commands);
    commands.get_entity(*primary_window).unwrap().insert(popup);
}
