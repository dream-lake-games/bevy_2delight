use bevy::prelude::*;

use crate::{
    glue::{bullet_time::BulletTime, fvec::FVec2, Fx},
    physics::{
        colls::{StaticCollRec, StaticColls, TriggerCollRecGeneric, TriggerCollsGeneric},
        dyno::Dyno,
        hbox::HBox,
        pos::Pos,
        prelude::{
            StaticRx, StaticRxKind, StaticTx, StaticTxKind, TriggerKindTrait, TriggerRxGeneric,
            TriggerTxGeneric,
        },
        PhysicsSet,
    },
};

use super::spat_hash::{SpatHash, SpatHashStaticTx, SpatHashTriggerTx, SpatKeys};

/// A helpful function to make sure physics things exist as we expect them to
#[cfg(debug_assertions)]
fn invariants(
    dyno_without_pos: Query<Entity, (With<Dyno>, Without<Pos>)>,
    static_rx_n_tx: Query<Entity, (With<StaticRx>, With<StaticTx>)>,
    moving_static_tx_vert_only: Query<&Dyno, With<StaticTx>>,
) {
    debug_assert!(dyno_without_pos.is_empty());
    debug_assert!(static_rx_n_tx.is_empty());
    for dyno in &moving_static_tx_vert_only {
        debug_assert!(dyno.vel.x.abs() == Fx::ZERO);
    }
}

/// Moves dynos that have no statics and no trigger receivers
fn move_uninteresting_dynos<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait>(
    bullet_time: Res<BulletTime>,
    mut ents: Query<
        (
            Entity,
            &Dyno,
            &mut Pos,
            Option<&TriggerTxGeneric<TriggerTxKind>>,
            Option<&mut SpatKeys<SpatHashTriggerTx>>,
        ),
        (
            Without<StaticRx>,
            Without<StaticTx>,
            Without<TriggerRxGeneric<TriggerRxKind>>,
        ),
    >,
    mut spat_hash_trigger_tx: ResMut<SpatHash<SpatHashTriggerTx>>,
) {
    for (eid, dyno, mut pos, ttx, mut spat_keys) in &mut ents {
        *pos += dyno.vel * bullet_time.delta_secs();
        match (ttx, spat_keys.as_mut()) {
            (Some(ttx), Some(spat_keys)) => {
                let new_keys = spat_hash_trigger_tx.update(
                    eid,
                    &spat_keys,
                    pos.clone(),
                    ttx.comps.iter().map(|c| c.hbox.clone()).collect(),
                );
                **spat_keys = new_keys;
            }
            _ => (),
        }
    }
}

/// Moves static txs
fn move_static_txs<TriggerTxKind: TriggerKindTrait>(
    bullet_time: Res<BulletTime>,
    mut ents: Query<
        (
            Entity,
            &Dyno,
            &mut Pos,
            &StaticTx,
            &mut SpatKeys<SpatHashStaticTx>,
            Option<&TriggerTxGeneric<TriggerTxKind>>,
            Option<&mut SpatKeys<SpatHashTriggerTx>>,
        ),
        Without<StaticRx>,
    >,
    mut spat_hash_static_tx: ResMut<SpatHash<SpatHashStaticTx>>,
    mut spat_hash_trigger_tx: ResMut<SpatHash<SpatHashTriggerTx>>,
) {
    for (eid, dyno, mut pos, stx, mut stx_spat_keys, ttx, mut ttx_spat_keys) in &mut ents {
        *pos += dyno.vel * bullet_time.delta_secs();
        let new_stx_spat_keys = spat_hash_static_tx.update(
            eid,
            &stx_spat_keys,
            pos.clone(),
            stx.comps.iter().map(|c| c.hbox.clone()).collect(),
        );
        *stx_spat_keys = new_stx_spat_keys;
        match (ttx, ttx_spat_keys.as_mut()) {
            (Some(ttx), Some(ttx_spat_keys)) => {
                let new_ttx_spat_keys = spat_hash_trigger_tx.update(
                    eid,
                    &ttx_spat_keys,
                    pos.clone(),
                    ttx.comps.iter().map(|c| c.hbox.clone()).collect(),
                );
                **ttx_spat_keys = new_ttx_spat_keys;
            }
            _ => (),
        }
    }
}

/// Resolves collisions for a single entity.
/// If it has statics, it resolves static collisions and may update pos and vel
/// If it has triggers, it will trigger as needed (duh)
fn resolve_collisions<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait>(
    my_eid: Entity,
    my_pos: &mut Pos,
    my_vel: &mut FVec2,
    my_srx: Option<(Entity, &StaticRx)>,
    my_trx: Option<(Entity, &TriggerRxGeneric<TriggerRxKind>)>,
    pos_q: &Query<&mut Pos>,
    dyno_q: &Query<&mut Dyno>,
    stx_q: &Query<(Entity, &mut StaticTx)>,
    ttx_q: &Query<(Entity, &mut TriggerTxGeneric<TriggerTxKind>)>,
    static_colls: &mut ResMut<StaticColls>,
    trigger_colls: &mut ResMut<TriggerCollsGeneric<TriggerRxKind, TriggerTxKind>>,
    spat_hash_stx: &SpatHash<SpatHashStaticTx>,
    spat_hash_ttx: &SpatHash<SpatHashTriggerTx>,
) {
    // Handle static collisions
    struct StaticCollCandidate {
        eid: Entity,
        pos: Pos,
        kind: StaticTxKind,
        thbox: HBox,
    }

    // Update all pos/dyno for static collisions, create records
    if let Some((_, my_srx)) = my_srx {
        for my_srx_comp in &my_srx.comps {
            // First use our spatial hash to narrow down our search
            let stx_keys = spat_hash_stx.get_keys(my_pos.clone(), vec![my_srx_comp.hbox.clone()]);
            let candidate_eids = spat_hash_stx.get_eids(stx_keys);
            // Then further whittle down to things that are not us that we overlap with
            let mut my_thbox = my_srx_comp.hbox.translated(my_pos.as_fvec2());
            let mut candidates = candidate_eids
                .iter()
                .filter_map(|eid| stx_q.get(*eid).ok())
                .flat_map(|(eid, stx)| {
                    let pos = pos_q.get(eid).expect("Missing pos on stx");
                    stx.comps.iter().map(move |comp| StaticCollCandidate {
                        eid,
                        pos: pos.clone(),
                        kind: comp.kind,
                        thbox: comp.hbox.translated(pos.as_fvec2()),
                    })
                })
                .filter(|candidate| candidate.eid != my_eid)
                .filter(|candidate| my_thbox.overlaps_with(&candidate.thbox))
                .collect::<Vec<_>>();
            // Sorting by the amount of overlap allows sliding in the "right" way (I think)
            candidates.sort_by(|a, b| {
                let dist_a = a.thbox.area_overlapping_assuming_overlap(&my_thbox);
                let dist_b = b.thbox.area_overlapping_assuming_overlap(&my_thbox);
                dist_b.cmp(&dist_a)
            });
            for candidate in candidates {
                let Some(push) = my_thbox.get_push_out(&candidate.thbox) else {
                    // Likely means that resolving an earlier collision pushed us out of this box, do nothing
                    continue;
                };

                // COLLISION ACTUALLY HAPPENING
                let tx_dyno = dyno_q.get(candidate.eid).cloned().unwrap_or_default();
                let mut old_perp = if push.x.abs() != Fx::ZERO {
                    FVec2::new(my_vel.x, Fx::ZERO)
                } else {
                    FVec2::new(Fx::ZERO, my_vel.y)
                };
                let old_par = *my_vel - old_perp;
                if push.y.abs() > Fx::ZERO {
                    old_perp.y -= tx_dyno.vel.y;
                }

                let coll_rec = StaticCollRec {
                    push,
                    rx_pos: my_pos.clone(),
                    rx_perp: old_perp,
                    rx_par: old_par,
                    rx_ctrl: my_eid,
                    rx_kind: my_srx_comp.kind,
                    rx_hbox: my_srx_comp.hbox.get_marker(),
                    tx_pos: candidate.pos,
                    tx_ctrl: candidate.eid,
                    tx_kind: candidate.kind,
                    tx_hbox: candidate.thbox.get_marker(),
                };

                let mut do_push = |grr: &mut HBox| {
                    *my_pos += push;
                    *grr = grr.translated(push);
                };

                match (my_srx_comp.kind, candidate.kind) {
                    (StaticRxKind::Default, StaticTxKind::Solid) => {
                        static_colls.insert(coll_rec);
                        do_push(&mut my_thbox);
                        *my_vel = old_par + FVec2::new(Fx::ZERO, tx_dyno.vel.y);
                        if old_perp.dot(push) > Fx::ZERO {
                            *my_vel += old_perp;
                        }
                    }
                    (StaticRxKind::Default, StaticTxKind::PassUp) => {
                        if push.y > 0.0
                            && old_perp.y < 0.0
                            && candidate.thbox.max_y() - Fx::from_num(1) < my_thbox.min_y()
                        {
                            static_colls.insert(coll_rec);
                            do_push(&mut my_thbox);
                            *my_vel = old_par + FVec2::new(Fx::ZERO, tx_dyno.vel.y);
                            if old_perp.dot(push) > Fx::ZERO {
                                *my_vel += old_perp;
                            }
                        }
                    }
                    (StaticRxKind::Observe, _) => {
                        static_colls.insert(coll_rec);
                    }
                }
            }
        }
    }

    // Handle trigger collisions
    struct TriggerCollCandidate<InnerTriggerTxKind> {
        eid: Entity,
        pos: Pos,
        kind: InnerTriggerTxKind,
        thbox: HBox,
    }

    // Create trigger coll records
    if let Some((_, my_trx)) = my_trx {
        for my_trx_comp in &my_trx.comps {
            // First use our spatial hash to narrow down our search
            let ttx_keys = spat_hash_ttx.get_keys(my_pos.clone(), vec![my_trx_comp.hbox.clone()]);
            let candidate_eids = spat_hash_ttx.get_eids(ttx_keys);
            // Then further whittle down to things that are not us that we overlap with
            let my_thbox = my_trx_comp.hbox.translated(my_pos.as_fvec2());
            let candidates = candidate_eids
                .iter()
                .filter_map(|eid| ttx_q.get(*eid).ok())
                .flat_map(|(eid, ttx)| {
                    let pos = pos_q.get(eid).expect("Missing pos on ttx");
                    ttx.comps.iter().map(move |comp| TriggerCollCandidate {
                        eid,
                        pos: pos.clone(),
                        kind: comp.kind.clone(),
                        thbox: comp.hbox.translated(pos.as_fvec2()),
                    })
                })
                .filter(|candidate| candidate.eid != my_eid)
                .filter(|candidate| my_thbox.overlaps_with(&candidate.thbox));
            for candidate in candidates {
                let coll_rec = TriggerCollRecGeneric {
                    rx_pos: my_pos.clone(),
                    rx_ctrl: my_eid,
                    rx_kind: my_trx_comp.kind.clone(),
                    rx_hbox: my_trx_comp.hbox.get_marker(),
                    tx_pos: candidate.pos,
                    tx_ctrl: candidate.eid,
                    tx_kind: candidate.kind,
                    tx_hbox: candidate.thbox.get_marker(),
                };
                trigger_colls.insert(coll_rec);
            }
        }
    }
}

/// As we resolve collisions, we create the collisions records but don't put the corresponding
/// keys in the needed vecs in the ctrls. This helper does that, assuming all colls have been resolved.
fn populate_ctrl_coll_keys<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait>(
    srx_q: &mut Query<(Entity, &mut StaticRx)>,
    stx_q: &mut Query<(Entity, &mut StaticTx)>,
    trx_q: &mut Query<(Entity, &mut TriggerRxGeneric<TriggerRxKind>)>,
    ttx_q: &mut Query<(Entity, &mut TriggerTxGeneric<TriggerTxKind>)>,
    static_colls: &ResMut<StaticColls>,
    trigger_colls: &ResMut<TriggerCollsGeneric<TriggerRxKind, TriggerTxKind>>,
) {
    for (key, coll) in &static_colls.map {
        if let Ok((_, mut srx_ctrl)) = srx_q.get_mut(coll.rx_ctrl) {
            srx_ctrl.coll_keys.push(*key);
        }
        if let Ok((_, mut stx_ctrl)) = stx_q.get_mut(coll.tx_ctrl) {
            stx_ctrl.coll_keys.push(*key);
        }
    }
    for (key, coll) in &trigger_colls.map {
        if let Ok((_, mut trx_ctrl)) = trx_q.get_mut(coll.rx_ctrl) {
            trx_ctrl.coll_keys.push(*key);
        }
        if let Ok((_, mut ttx_ctrl)) = ttx_q.get_mut(coll.tx_ctrl) {
            ttx_ctrl.coll_keys.push(*key);
        }
    }
}

/// Moves the interesting stuff and handles collisions
fn move_interesting_dynos<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait>(
    bullet_time: Res<BulletTime>,
    mut pos_q: Query<&mut Pos>,
    mut dyno_q: Query<&mut Dyno>,
    mut srx_q: Query<(Entity, &mut StaticRx)>,
    mut stx_q: Query<(Entity, &mut StaticTx)>,
    mut trx_q: Query<(Entity, &mut TriggerRxGeneric<TriggerRxKind>)>,
    mut ttx_q: Query<(Entity, &mut TriggerTxGeneric<TriggerTxKind>)>,
    mut static_colls: ResMut<StaticColls>,
    mut trigger_colls: ResMut<TriggerCollsGeneric<TriggerRxKind, TriggerTxKind>>,
    // Objects that have a static rx. They may also have a trigger rx.
    // Basically all the stuff we should move in this system
    ents_q: Query<
        Entity,
        (
            With<Pos>,
            Without<StaticTx>,
            Or<(With<StaticRx>, With<TriggerRxGeneric<TriggerRxKind>>)>,
        ),
    >,
    // To use and maintain our spatial hashing
    spat_hash_stx: Res<SpatHash<SpatHashStaticTx>>,
    mut spat_hash_ttx_q: Query<&mut SpatKeys<SpatHashTriggerTx>>,
    mut spat_hash_ttx: ResMut<SpatHash<SpatHashTriggerTx>>,
) {
    // First do the moving
    for eid in &ents_q {
        // Get the data
        let mut scratch_pos = pos_q.get(eid).expect("No pos on interesting ent").clone();
        let mut scratch_vel = dyno_q.get(eid).unwrap_or(&Dyno::default()).vel.clone();
        let srx = srx_q.get(eid).ok();
        let trx = trx_q.get(eid).ok();
        debug_assert!(srx.is_some() || trx.is_some());
        // Inch
        macro_rules! call_resolve_collisions {
            () => {{
                resolve_collisions(
                    eid,
                    &mut scratch_pos,
                    &mut scratch_vel,
                    srx,
                    trx,
                    &pos_q,
                    &dyno_q,
                    &stx_q,
                    &ttx_q,
                    &mut static_colls,
                    &mut trigger_colls,
                    &spat_hash_stx,
                    &spat_hash_ttx,
                )
            }};
        }
        const DELTA_PER_INCH: Fx = Fx::const_from_int(1);
        // Resolve collisions once always so stationary objects are still pushed out of each other
        call_resolve_collisions!();
        // Inch horizontally
        let mut amt_moved_hor: Fx = Fx::ZERO;
        let max_inch_hor = scratch_vel.x.abs() * bullet_time.delta_secs();
        while amt_moved_hor < max_inch_hor.min(scratch_vel.x.abs()) {
            let dont_overshoot =
                (max_inch_hor.min(scratch_vel.x.abs()) - amt_moved_hor).max(Fx::ZERO);
            let moving_this_step = DELTA_PER_INCH.min(dont_overshoot);
            amt_moved_hor += moving_this_step;
            scratch_pos.x += scratch_vel.x.signum() * moving_this_step;
            call_resolve_collisions!();
        }
        // Then inch vertically
        let mut amt_moved_ver: Fx = Fx::ZERO;
        let max_inch_ver = scratch_vel.y.abs() * bullet_time.delta_secs();
        while amt_moved_ver < max_inch_ver.min(scratch_vel.y.abs()) {
            let dont_overshoot =
                (max_inch_ver.min(scratch_vel.y.abs()) - amt_moved_ver).max(Fx::ZERO);
            let moving_this_step = DELTA_PER_INCH.min(dont_overshoot);
            amt_moved_ver += moving_this_step;
            scratch_pos.y += scratch_vel.y.signum() * moving_this_step;
            call_resolve_collisions!();
        }
        // NOTE: Why do this (inch horizontally then vertically)? Stops bugs going up and down against wall.
        // ^read: celeste does this
        // Set the data
        let mut set_pos = pos_q.get_mut(eid).expect("No pos on interesting ent");
        *set_pos = scratch_pos;
        if let Ok(mut set_dyno) = dyno_q.get_mut(eid) {
            set_dyno.vel = scratch_vel;
        }

        // Now that we're done moving, we need to update our spatial hashes
        // NOTE: See `invariants` fn, but we don't allow one entity to have both StaticRx and StaticTx
        //       So we only need to update a potential TriggerTx hash here

        if let (Ok((_, ttx)), Ok(mut spat_keys_ttx)) =
            (ttx_q.get(eid), spat_hash_ttx_q.get_mut(eid))
        {
            let hboxes = ttx.comps.iter().map(|c| c.hbox.clone()).collect();
            let new_keys = spat_hash_ttx.update(eid, &spat_keys_ttx, scratch_pos.clone(), hboxes);
            *spat_keys_ttx = new_keys;
        }
    }
    // Then update the records in the controls once
    populate_ctrl_coll_keys(
        &mut srx_q,
        &mut stx_q,
        &mut trx_q,
        &mut ttx_q,
        &static_colls,
        &trigger_colls,
    );
}

fn update_transforms(mut ents: Query<(&Pos, &mut Transform)>) {
    for (pos, mut tran) in &mut ents {
        tran.translation.x = pos.x.round().to_num();
        tran.translation.y = pos.y.round().to_num();
        tran.translation.z = pos.z.to_num();
    }
}

pub(super) fn register_logic<TriggerRxKind: TriggerKindTrait, TriggerTxKind: TriggerKindTrait>(
    app: &mut App,
) {
    app.add_systems(
        Update,
        (
            move_uninteresting_dynos::<TriggerRxKind, TriggerTxKind>,
            move_static_txs::<TriggerTxKind>,
            move_interesting_dynos::<TriggerRxKind, TriggerTxKind>,
            update_transforms,
        )
            .chain()
            .in_set(PhysicsSet),
    );
    #[cfg(debug_assertions)]
    {
        app.add_systems(Update, invariants);
    }
}
