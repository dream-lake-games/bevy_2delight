use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, ldtk::LayerInstance, IntGridCell};
use bevy_ecs_tilemap::map::TilemapType;

use crate::prelude::{Frac, Layer, Pos};

use super::{
    ldtk_roots::{LdtkRootKind, LdtkRootResGeneric},
    load::BlockLdtkLoad,
};

#[derive(Resource, Default)]
pub(super) struct LdtkIntCellLayerInfo {
    map: HashMap<String, Layer>,
}

pub trait LdtkIntCellValue<R: LdtkRootKind>: Bundle {
    const ROOT: R;
    fn from_ldtk(pos: Pos, value: i32) -> Self;
}

#[derive(Component, Default)]
struct LdtkIntCellWrapper<R: LdtkRootKind, B: LdtkIntCellValue<R>> {
    _pd: std::marker::PhantomData<(R, B)>,
    value: i32,
    _blocker: BlockLdtkLoad,
}
impl<R: LdtkRootKind, B: LdtkIntCellValue<R>> bevy_ecs_ldtk::app::LdtkIntCell
    for LdtkIntCellWrapper<R, B>
{
    fn bundle_int_cell(int_grid_cell: IntGridCell, _layer_instance: &LayerInstance) -> Self {
        Self {
            _pd: default(),
            value: int_grid_cell.value,
            _blocker: BlockLdtkLoad::ticks(10),
        }
    }
}

#[derive(Component)]
struct LayerHandled;

fn post_ldtk_int_cell_layer_blessing(
    layer_info: Res<LdtkIntCellLayerInfo>,
    layer_q: Query<(Entity, &Name), (With<TilemapType>, Without<LayerHandled>)>,
    mut commands: Commands,
) {
    for (eid, name) in &layer_q {
        let Some(layer) = layer_info.map.get(name.as_str()) else {
            continue;
        };
        commands
            .entity(eid)
            .insert((LayerHandled, layer.render_layers()));
    }
}

fn post_ldtk_int_cell_value_blessing<R: LdtkRootKind, B: LdtkIntCellValue<R>>(
    mut commands: Commands,
    mut wrappers: Query<(Entity, &GlobalTransform, &LdtkIntCellWrapper<R, B>)>,
    roots: Res<LdtkRootResGeneric<R>>,
) {
    for (ldtk_eid, gt, wrapper) in &mut wrappers {
        if gt.translation().x == 0.0 && gt.translation().y == 0.0 {
            // One of these days I'll find a better way to fix this
            continue;
        }
        let pos = Pos::new(
            Frac::whole(gt.translation().x.round() as i32),
            Frac::whole(gt.translation().y.round() as i32),
        );
        let bund = B::from_ldtk(pos, wrapper.value);
        commands.spawn(bund).set_parent(roots.get_eid(B::ROOT));
        commands
            .entity(ldtk_eid)
            .remove::<LdtkIntCellWrapper<R, B>>();
    }
}

#[doc(hidden)]
pub trait LdtkIntCellLayerer {
    fn register_ldtk_int_cell_layer(&mut self, layer_id: &str, layer: Layer);
}
impl LdtkIntCellLayerer for App {
    fn register_ldtk_int_cell_layer(&mut self, layer_id: &str, layer: Layer) {
        let layer_id = layer_id.to_string();
        self.add_systems(
            Startup,
            move |mut layer_info: ResMut<LdtkIntCellLayerInfo>| {
                if layer_info.map.contains_key(&layer_id) {
                    panic!(
                        "Registered the same ldtk int cell layer twice: {:?}",
                        layer_id,
                    );
                }
                layer_info.map.insert(layer_id.clone(), layer.clone());
            },
        );
    }
}

pub struct LdtkIntCellValuePluginGeneric<R: LdtkRootKind, B: LdtkIntCellValue<R>> {
    layer_id: &'static str,
    values: Vec<i32>,
    _pd: std::marker::PhantomData<(R, B)>,
}
impl<R: LdtkRootKind, B: LdtkIntCellValue<R>> LdtkIntCellValuePluginGeneric<R, B> {
    pub fn single(layer_id: &'static str, value: i32) -> Self {
        Self {
            layer_id,
            values: vec![value],
            _pd: default(),
        }
    }
    pub fn multiple<I: Iterator<Item = i32>>(layer_id: &'static str, values: I) -> Self {
        Self {
            layer_id,
            values: values.collect(),
            _pd: default(),
        }
    }
}
impl<R: LdtkRootKind, B: LdtkIntCellValue<R>> Plugin for LdtkIntCellValuePluginGeneric<R, B> {
    fn build(&self, app: &mut App) {
        for value in &self.values {
            app.register_ldtk_int_cell_for_layer::<LdtkIntCellWrapper<R, B>>(
                &self.layer_id,
                *value,
            );
        }
        app.add_systems(PreUpdate, post_ldtk_int_cell_value_blessing::<R, B>);
    }
}

pub(super) fn register_ldtk_int_cell(app: &mut App) {
    app.insert_resource(LdtkIntCellLayerInfo::default());
    app.add_systems(Update, post_ldtk_int_cell_layer_blessing);
}
