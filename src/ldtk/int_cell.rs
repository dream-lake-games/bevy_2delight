use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, ldtk::LayerInstance, IntGridCell};
use bevy_ecs_tilemap::map::TilemapType;

use crate::{
    glue::aabbify::{aabbify_make_hollow, aabify_consolidate, Pixel},
    prelude::{Fx, HBox, Layer, OccludeLight, Pos, StaticTx},
};

use super::{
    ldtk_roots::{LdtkRootKind, LdtkRootResGeneric},
    load::BlockLdtkLoad,
    LdtkSet,
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

#[derive(Resource)]
pub struct LdtkIntCellConsolidate<R: LdtkRootKind, B: LdtkIntCellValue<R>> {
    grid_size: u32,
    _pd: std::marker::PhantomData<(R, B)>,
}
impl<R: LdtkRootKind, B: LdtkIntCellValue<R>> LdtkIntCellConsolidate<R, B> {
    pub fn grid_size(grid_size: u32) -> Self {
        Self {
            grid_size,
            _pd: default(),
        }
    }
}

#[derive(Component)]
struct LdtkNeedsConsolidation<R: LdtkRootKind, B: LdtkIntCellValue<R>> {
    _pd: std::marker::PhantomData<(R, B)>,
}
impl<R: LdtkRootKind, B: LdtkIntCellValue<R>> Default for LdtkNeedsConsolidation<R, B> {
    fn default() -> Self {
        Self { _pd: default() }
    }
}

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
    maybe_consolidate: Option<Res<LdtkIntCellConsolidate<R, B>>>,
) {
    for (ldtk_eid, gt, wrapper) in &mut wrappers {
        if gt.translation().x == 0.0 && gt.translation().y == 0.0 {
            // One of these days I'll find a better way to fix this
            continue;
        }
        let pos = Pos::new(
            Fx::from_num(gt.translation().x.round() as i32),
            Fx::from_num(gt.translation().y.round() as i32),
        );
        let bund = B::from_ldtk(pos, wrapper.value);
        let spawned_eid = commands.spawn(bund).set_parent(roots.get_eid(B::ROOT)).id();
        commands
            .entity(ldtk_eid)
            .remove::<LdtkIntCellWrapper<R, B>>();

        if maybe_consolidate.is_some() {
            commands
                .entity(spawned_eid)
                .insert(LdtkNeedsConsolidation::<R, B>::default());
        }
    }
}

fn ldtk_int_cell_consolidate<R: LdtkRootKind, B: LdtkIntCellValue<R>>(
    consolidate_res: Res<LdtkIntCellConsolidate<R, B>>,
    needs_consolidation: Query<(Entity, &Pos), With<LdtkNeedsConsolidation<R, B>>>,
    stability_q: Query<(&StaticTx, &OccludeLight), With<LdtkNeedsConsolidation<R, B>>>,
    mut commands: Commands,
) {
    let shell_set = aabbify_make_hollow(
        needs_consolidation
            .iter()
            .map(|(eid, pos)| Pixel::from_pos(*pos, consolidate_res.grid_size, eid)),
    );
    for (eid, _) in &needs_consolidation {
        if !shell_set.contains(&eid) {
            commands.entity(eid).remove::<StaticTx>();
        }
        commands
            .entity(eid)
            .remove::<LdtkNeedsConsolidation<R, B>>();
    }

    let consolidated_groups: Vec<Vec<Entity>> = aabify_consolidate(
        needs_consolidation
            .iter()
            .filter(|(eid, _)| shell_set.contains(eid))
            .map(|(eid, pos)| Pixel::from_pos(*pos, consolidate_res.grid_size, eid)),
    );
    for group in consolidated_groups {
        let first = group.first().unwrap().clone();
        let last = group.last().unwrap().clone();
        if first == last {
            continue;
        }
        let first_pos = needs_consolidation.get(first).unwrap().1.clone();
        let last_pos = needs_consolidation.get(last).unwrap().1.clone();
        let w = last_pos.x - first_pos.x + Fx::from_num(consolidate_res.grid_size);
        let h = last_pos.y - first_pos.y + Fx::from_num(consolidate_res.grid_size);
        let new_hbox = HBox::new(w.round().to_num(), h.round().to_num()).with_offset(
            w / 2 - Fx::from_num(consolidate_res.grid_size) / 2,
            h / 2 - Fx::from_num(consolidate_res.grid_size) / 2,
        );
        let existing_comps = &stability_q.get(first).unwrap().0.comps;
        debug_assert!(existing_comps.len() == 1);
        let existing_kind = existing_comps[0].kind;
        let existing_occlude = stability_q.get(first).map(|pair| pair.1.clone()).ok();
        // If we're providing a custom occlude here, we're gonna be f'd (unless I were smarter)
        debug_assert!(
            existing_occlude.is_none() || matches!(existing_occlude, Some(OccludeLight::StaticTx))
        );
        for inner_eid in &group {
            commands.entity(*inner_eid).remove::<StaticTx>();
        }
        commands
            .entity(first)
            .insert(StaticTx::single(existing_kind, new_hbox));
        if let Some(existing_occlude) = existing_occlude {
            commands.entity(first).insert(existing_occlude);
        }
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
    /// When set to Some(x), will consolidate hboxes assuming a grid size of x.
    /// This involves both hollowing and aabbifying.
    consolidate: Option<u32>,
    _pd: std::marker::PhantomData<(R, B)>,
}
impl<R: LdtkRootKind, B: LdtkIntCellValue<R>> LdtkIntCellValuePluginGeneric<R, B> {
    pub fn single(layer_id: &'static str, value: i32) -> Self {
        Self {
            layer_id,
            values: vec![value],
            consolidate: None,
            _pd: default(),
        }
    }
    pub fn multiple<I: Iterator<Item = i32>>(layer_id: &'static str, values: I) -> Self {
        Self {
            layer_id,
            values: values.collect(),
            consolidate: None,
            _pd: default(),
        }
    }
    pub fn with_consolidate(mut self, grid_size: u32) -> Self {
        self.consolidate = Some(grid_size);
        self
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
        app.add_systems(
            Update,
            post_ldtk_int_cell_value_blessing::<R, B>.in_set(LdtkSet),
        );

        if let Some(grid_size) = self.consolidate {
            app.insert_resource(LdtkIntCellConsolidate::<R, B>::grid_size(grid_size));
            app.add_systems(
                Update,
                ldtk_int_cell_consolidate::<R, B>
                    .in_set(LdtkSet)
                    .after(post_ldtk_int_cell_value_blessing::<R, B>),
            );
        }
    }
}

pub(super) fn register_ldtk_int_cell(app: &mut App) {
    app.insert_resource(LdtkIntCellLayerInfo::default());
    app.add_systems(Update, post_ldtk_int_cell_layer_blessing.in_set(LdtkSet));
}
