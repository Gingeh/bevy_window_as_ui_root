//! Easily spawn windows as independent UI roots. Ideal for debug widgets!
//!
//! ## Usage
//!
//! ```
//! app.add_plugins(WindowAsUiRootPlugin);
//!
//! commands.spawn((
//!   WindowAsUiRoot,
//!   BackgroundColor(Color::WHITE),
//!   children![(
//!     Text::new("Hello World!"),
//!     TextColor(Color::BLACK),
//!   )],
//! ));
//! ```
//!
//! Insert the [`WindowAsUiRoot`] component on an entity and this plugin will:
//!
//! - Insert `Node`, `Window`, and `Camera` components if they don't already exist.
//! - Connect the camera to the root node and window.
//! - Automatically resize the window to match the root node's layout size (if it is auto or fixed).
//!
//! ## Tips
//! - Use the [`CloseWith`] relation to automatically close nested windows.
//! - Set `WindowPlugin.exit_condition` to `ExitCondition::OnPrimaryClosed` to prevent popups from outliving the main window.
//! - Insert `Node { width: percent(100), height: percent(100), ..default() }` on the UI root to make it resizable by the user.

use bevy_app::{App, Plugin, Update};
use bevy_camera::{Camera, Camera2d, RenderTarget};
use bevy_ecs::{
    component::Component,
    lifecycle::HookContext,
    query::{Changed, With, Without},
    system::{Commands, Query},
    world::DeferredWorld,
};
use bevy_ecs::{entity::Entity, query::Has, schedule::IntoScheduleConfigs};
use bevy_ui::{ComputedNode, Node, UiTargetCamera, Val};
use bevy_window::{Window, WindowRef};

/// This plugin handles creating and resizing windows, add it to use this library.
pub struct WindowAsUiRootPlugin;

impl Plugin for WindowAsUiRootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_resizable, update_size).chain());
    }
}

/// Add this relation from sub-windows to their parents to automatically close them.
#[derive(Component, Clone, Copy)]
#[relationship(relationship_target = AlsoClose)]
pub struct CloseWith(pub Entity);

#[doc(hidden)]
#[derive(Component, Clone)]
#[relationship_target(relationship = CloseWith, linked_spawn)]
pub struct AlsoClose(Vec<Entity>);

/// Use this component to spawn windows as independent UI roots.
///
/// Inserting it on an entity will:
///
/// - Insert `Node`, `Window`, and `Camera` components if they don't already exist.
/// - Connect the camera to the root node and window.
/// - Automatically resize the window to match the root node's layout size (if it is auto or fixed).
#[derive(Component, Clone, Copy, Default)]
#[require(Window, Node, Camera2d, Camera)]
#[component(on_add = add_hook)]
pub struct WindowAsUiRoot;

fn add_hook(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let mut camera = world.get_mut::<Camera>(entity).unwrap();
    camera.target = RenderTarget::Window(WindowRef::Entity(entity));
    world
        .commands()
        .get_entity(entity)
        .unwrap()
        .insert(UiTargetCamera(entity));
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
