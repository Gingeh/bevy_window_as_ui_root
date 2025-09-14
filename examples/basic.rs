use bevy::{
    prelude::*,
    window::{ExitCondition, WindowResolution},
};
use bevy_window_as_ui_root::{WindowAsUiRoot, WindowAsUiRootPlugin};

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                exit_condition: ExitCondition::OnPrimaryClosed,
                ..default()
            }),
            WindowAsUiRootPlugin,
        ))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn((
        WindowAsUiRoot,
        BackgroundColor(Color::WHITE),
        children![(Text::new("Hello World!"), TextColor(Color::BLACK),)],
    ));

    commands
        .spawn((
            WindowAsUiRoot,
            BackgroundColor(Color::WHITE),
            Node {
                width: percent(100.0),
                height: percent(100.0),
                ..default()
            },
            Window {
                resolution: WindowResolution::new(350, 350),
                ..default()
            },
            children![(
                Text::new("Lorem ipsum is a dummy or placeholder text commonly used in graphic design, publishing, and web development. Its purpose is to permit a page layout to be designed, independently of the copy that will subsequently populate it, or to demonstrate various fonts of a typeface without meaningful text that could be distracting."),
                TextColor(Color::BLACK),
            )],
        ));
}
