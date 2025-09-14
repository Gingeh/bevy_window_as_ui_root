use bevy_app::{App, Plugin, Update};
use bevy_camera::{Camera, Camera2d, RenderTarget};
use bevy_ecs::{
    component::Component,
    lifecycle,
    observer::On,
    query::{Changed, With, Without},
    system::{Commands, Query},
};
use bevy_ecs::{entity::Entity, query::Has, schedule::IntoScheduleConfigs};
use bevy_ui::{ComputedNode, Node, UiTargetCamera, Val};
use bevy_window::{Window, WindowRef};

pub struct WindowAsUiRootPlugin;

impl Plugin for WindowAsUiRootPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(creation_observer)
            .add_systems(Update, (update_resizable, update_size).chain());
    }
}

#[derive(Component, Clone, Copy)]
#[relationship(relationship_target = AlsoClose)]
pub struct CloseWith(pub Entity);

#[derive(Component, Clone)]
#[relationship_target(relationship = CloseWith, linked_spawn)]
pub struct AlsoClose(Vec<Entity>);

#[derive(Component, Clone, Copy)]
#[require(Window, Node)]
pub struct WindowAsUiRoot;

fn creation_observer(event: On<lifecycle::Add, WindowAsUiRoot>, mut commands: Commands) {
    let entity = event.entity;

    let camera = commands
        .spawn((
            Camera2d,
            Camera {
                target: RenderTarget::Window(WindowRef::Entity(entity)),
                ..Default::default()
            },
        ))
        .id();

    commands
        .get_entity(entity)
        .unwrap()
        .insert(UiTargetCamera(camera))
        .add_child(camera);
}

#[derive(Component)]
struct Resizable;

fn update_size(
    mut windows: Query<
        (&ComputedNode, &mut Window),
        (
            Changed<ComputedNode>,
            With<WindowAsUiRoot>,
            Without<Resizable>,
        ),
    >,
) {
    for (computed_node, mut window) in &mut windows {
        let size = computed_node.size();
        if size.x <= 0.0 || size.y <= 0.0 {
            continue;
        }
        window.resolution.set(size.x, size.y);
    }
}

fn update_resizable(
    mut commands: Commands,
    mut windows: Query<
        (Entity, &Node, &mut Window, Has<Resizable>),
        (Changed<Node>, With<WindowAsUiRoot>),
    >,
) {
    for (entity, node, mut window, has_resizable_component) in &mut windows {
        let resizable = !matches!(node.width, Val::Px(_) | Val::Auto)
            && !matches!(node.height, Val::Px(_) | Val::Auto);

        if window.resizable != resizable {
            window.resizable = resizable;
        }

        if has_resizable_component != resizable {
            let mut entity_commands = commands.get_entity(entity).unwrap();
            if resizable {
                entity_commands.insert(Resizable);
            } else {
                entity_commands.remove::<Resizable>();
            }
        }
    }
}
