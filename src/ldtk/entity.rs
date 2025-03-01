use bevy::{prelude::*, utils::HashMap};
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

pub trait LdtkEntity<R: LdtkRootKind>: Bundle {
    const ROOT: R;
    fn from_ldtk(pos: Pos, fields: &HashMap<String, FieldValue>, iid: String) -> Self;
}

#[derive(Component, Default)]
struct LdtkEntityWrapper<R: LdtkRootKind, B: LdtkEntity<R>> {
    _pd: std::marker::PhantomData<(R, B)>,
    _blocker: BlockLdtkLoad,
    fields: HashMap<String, FieldValue>,
    iid: String,
}
impl<R: LdtkRootKind, B: LdtkEntity<R>> bevy_ecs_ldtk::app::LdtkEntity for LdtkEntityWrapper<R, B> {
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

fn post_ldtk_entity_blessing<R: LdtkRootKind, B: LdtkEntity<R>>(
    mut commands: Commands,
    wrappers: Query<(Entity, &GlobalTransform, &LdtkEntityWrapper<R, B>)>,
    roots: Res<LdtkRootResGeneric<R>>,
) {
    for (ldtk_eid, gt, wrapper) in &wrappers {
        if gt.translation().x == 0.0 && gt.translation().y == 0.0 {
            // One of these days I'll find a better way to fix this
            continue;
        }
        let pos = Pos::new(
            Fx::from_num(gt.translation().x.round() as i32),
            Fx::from_num(gt.translation().y.round() as i32),
        );
        let bund = B::from_ldtk(pos, &wrapper.fields, wrapper.iid.clone());
        commands.spawn(bund).set_parent(roots.get_eid(B::ROOT));
        commands
            .entity(ldtk_eid)
            .remove::<LdtkEntityWrapper<R, B>>();
    }
}

pub struct LdtkEntityPluginGeneric<R: LdtkRootKind, B: LdtkEntity<R>> {
    _pd: std::marker::PhantomData<(R, B)>,
    layer_id: &'static str,
    entity_id: &'static str,
}
impl<R: LdtkRootKind, B: LdtkEntity<R>> LdtkEntityPluginGeneric<R, B> {
    pub fn new(layer_id: &'static str, entity_id: &'static str) -> Self {
        Self {
            layer_id,
            entity_id,
            _pd: default(),
        }
    }
}
impl<R: LdtkRootKind, B: LdtkEntity<R>> Plugin for LdtkEntityPluginGeneric<R, B> {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity_for_layer::<LdtkEntityWrapper<R, B>>(
            &self.layer_id,
            &self.entity_id,
        );
        app.add_systems(PreUpdate, post_ldtk_entity_blessing::<R, B>);
    }
}
