use bevy::{
    feathers::{
        self, FeathersPlugins,
        controls::{ButtonProps, button},
        dark_theme::create_dark_theme,
        theme::{ThemeBackgroundColor, ThemedText, UiTheme},
    },
    prelude::*,
    ui_widgets::{Activate, Callback},
    window::{ExitCondition, PrimaryWindow},
};
use bevy_window_as_ui_root::{CloseWith, WindowAsUiRoot, WindowAsUiRootPlugin};

#[derive(Component)]
#[require(TextSpan)]
struct CounterSpan;

#[derive(Resource, Deref, DerefMut)]
struct Count(usize);

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                exit_condition: ExitCondition::OnPrimaryClosed,
                ..default()
            }),
            WindowAsUiRootPlugin,
            FeathersPlugins,
        ))
        .insert_resource(UiTheme(create_dark_theme()))
        .insert_resource(Count(0))
        .add_systems(Startup, setup)
        .add_systems(Update, update_counter_span)
        .run()
}

fn update_counter_span(mut span: Single<&mut TextSpan, With<CounterSpan>>, count: Res<Count>) {
    if count.is_changed() {
        span.0 = count.to_string();
    }
}

fn setup(mut commands: Commands, primary_window: Single<Entity, With<PrimaryWindow>>) {
    let on_click_system = commands.register_system(|_: In<Activate>, mut count: ResMut<Count>| {
        **count += 1;
    });

    let button_window = commands
        .spawn((
            WindowAsUiRoot,
            ThemeBackgroundColor(feathers::tokens::WINDOW_BG),
            Node {
                padding: px(8).all(),
                ..default()
            },
            children![button(
                ButtonProps {
                    on_click: Callback::System(on_click_system),
                    ..default()
                },
                (),
                Spawn((Text::new("Increment"), ThemedText))
            )],
        ))
        .id();

    commands.get_entity(*primary_window).unwrap().insert((
        WindowAsUiRoot,
        CloseWith(button_window),
        ThemeBackgroundColor(feathers::tokens::WINDOW_BG),
        Node {
            padding: px(8).all(),
            ..default()
        },
        children![(
            Text::new("Counter: "),
            ThemedText,
            TextLayout {
                linebreak: LineBreak::NoWrap,
                ..default()
            },
            children![(CounterSpan, ThemedText)],
        )],
    ));
}
