use bevy::prelude::*;
use bevy_ecs_ldtk::{
    app::LdtkEntityAppExt,
    ldtk::{FieldValue, LayerInstance, TilesetDefinition},
    EntityInstance,
};

use crate::prelude::*;

use super::{
    ldtk_roots::{LdtkRootKind, LdtkRootResGeneric},
    load::BlockLdtkLoad,
};

pub trait LdtkBundleEntity<R: LdtkRootKind>: Bundle {
    const ROOT: R;
    fn from_ldtk(pos: Pos, fields: &HashMap<String, FieldValue>, iid: String) -> Self;
}

#[derive(Component, Default)]
struct LdtkBundleEntityWrapper<R: LdtkRootKind, B: LdtkBundleEntity<R>> {
    _pd: std::marker::PhantomData<(R, B)>,
    _blocker: BlockLdtkLoad,
    fields: HashMap<String, FieldValue>,
    iid: String,
}
impl<R: LdtkRootKind, B: LdtkBundleEntity<R>> bevy_ecs_ldtk::app::LdtkEntity
    for LdtkBundleEntityWrapper<R, B>
{
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        Self {
            _pd: default(),
            _blocker: BlockLdtkLoad::ticks(10),
            fields: entity_instance
                .field_instances
                .clone()
                .into_iter()
                .map(|fi| (fi.identifier, fi.value))
                .collect(),
            iid: entity_instance.iid.clone(),
        }
    }
}

fn post_ldtk_bundle_entity_blessing<R: LdtkRootKind, B: LdtkBundleEntity<R>>(
    mut commands: Commands,
    wrappers: Query<(Entity, &GlobalTransform, &LdtkBundleEntityWrapper<R, B>)>,
    roots: Res<LdtkRootResGeneric<R>>,
) {
    for (ldtk_eid, gt, wrapper) in &wrappers {
        if gt.translation().x == 0.0 && gt.translation().y == 0.0 {
            // One of these days I'll find a better way to fix this
            continue;
        }
        let pos = Pos::new(
            fx!(gt.translation().x.round() as i32),
            fx!(gt.translation().y.round() as i32),
        );
        let bund = B::from_ldtk(pos, &wrapper.fields, wrapper.iid.clone());
        commands.spawn(bund).insert(ChildOf(roots.get_eid(B::ROOT)));
        commands
            .entity(ldtk_eid)
            .remove::<LdtkBundleEntityWrapper<R, B>>();
    }
}

pub struct LdtkBundleEntityPluginGeneric<R: LdtkRootKind, B: LdtkBundleEntity<R>> {
    _pd: std::marker::PhantomData<(R, B)>,
    layer_id: &'static str,
    entity_id: &'static str,
}
impl<R: LdtkRootKind, B: LdtkBundleEntity<R>> LdtkBundleEntityPluginGeneric<R, B> {
    pub fn new(layer_id: &'static str, entity_id: &'static str) -> Self {
        Self {
            layer_id,
            entity_id,
            _pd: default(),
        }
    }
}
impl<R: LdtkRootKind, B: LdtkBundleEntity<R>> Plugin for LdtkBundleEntityPluginGeneric<R, B> {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity_for_layer::<LdtkBundleEntityWrapper<R, B>>(
            &self.layer_id,
            &self.entity_id,
        );
        app.add_systems(
            Update,
            post_ldtk_bundle_entity_blessing::<R, B>.in_set(LdtkSet),
        );
    }
}

pub trait LdtkEntity<R: LdtkRootKind>: std::marker::Sync + std::marker::Send + 'static {
    const ROOT: R;
    fn spawn_from_ldtk(
        commands: &mut Commands,
        pos: Pos,
        fields: &HashMap<String, FieldValue>,
        iid: String,
    ) -> Entity;
}

#[derive(Component, Default)]
struct LdtkEntityWrapper<R: LdtkRootKind, E: LdtkEntity<R>> {
    _pd: std::marker::PhantomData<(R, E)>,
    _blocker: BlockLdtkLoad,
    fields: HashMap<String, FieldValue>,
    iid: String,
}
impl<R: LdtkRootKind, E: LdtkEntity<R>> bevy_ecs_ldtk::app::LdtkEntity for LdtkEntityWrapper<R, E> {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        Self {
            _pd: default(),
            _blocker: BlockLdtkLoad::ticks(10),
            fields: entity_instance
                .field_instances
                .clone()
                .into_iter()
                .map(|fi| (fi.identifier, fi.value))
                .collect(),
            iid: entity_instance.iid.clone(),
        }
    }
}

fn post_ldtk_entity_blessing<R: LdtkRootKind, E: LdtkEntity<R>>(
    mut commands: Commands,
    wrappers: Query<(Entity, &GlobalTransform, &LdtkEntityWrapper<R, E>)>,
    roots: Res<LdtkRootResGeneric<R>>,
) {
    for (ldtk_eid, gt, wrapper) in &wrappers {
        if gt.translation().x == 0.0 && gt.translation().y == 0.0 {
            // One of these days I'll find a better way to fix this
            continue;
        }
        let pos = Pos::new(
            fx!(gt.translation().x.round() as i32),
            fx!(gt.translation().y.round() as i32),
        );
        let eid = E::spawn_from_ldtk(&mut commands, pos, &wrapper.fields, wrapper.iid.clone());
        commands.entity(eid).insert(ChildOf(roots.get_eid(E::ROOT)));
        commands
            .entity(ldtk_eid)
            .remove::<LdtkEntityWrapper<R, E>>();
    }
}

pub struct LdtkEntityPluginGeneric<R: LdtkRootKind, E: LdtkEntity<R>> {
    _pd: std::marker::PhantomData<(R, E)>,
    layer_id: &'static str,
    entity_id: &'static str,
}
impl<R: LdtkRootKind, E: LdtkEntity<R>> LdtkEntityPluginGeneric<R, E> {
    pub fn new(layer_id: &'static str, entity_id: &'static str) -> Self {
        Self {
            layer_id,
            entity_id,
            _pd: default(),
        }
    }
}
impl<R: LdtkRootKind, E: LdtkEntity<R>> Plugin for LdtkEntityPluginGeneric<R, E> {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity_for_layer::<LdtkEntityWrapper<R, E>>(
            &self.layer_id,
            &self.entity_id,
        );
        app.add_systems(Update, post_ldtk_entity_blessing::<R, E>.in_set(LdtkSet));
    }
}
