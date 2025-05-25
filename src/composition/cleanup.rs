//! This is just cleaning up the world hierarchy.

use bevy::prelude::*;

fn collect_observers(
    mut parent: Local<Option<Entity>>,
    parentless: Query<Entity, (With<Observer>, Without<ChildOf>)>,
    mut commands: Commands,
) {
    if parent.is_none() {
        *parent = Some(commands.spawn((Name::new("ObserverRoot"),)).id());
    }
    let parent_eid = parent.unwrap();
    for eid in &parentless {
        commands.entity(eid).insert(ChildOf(parent_eid));
    }
}

pub(super) fn register_cleanup(app: &mut App) {
    app.add_systems(Update, collect_observers);
}
