use crate::{tangle, Tangle};

use bee_bundle::Hash;

use std::collections::{HashMap, HashSet};

use async_std::{
    prelude::*,
    sync::{Arc, Barrier},
    task::block_on,
};
use dashmap::DashMap;
use flume::Receiver;

pub struct SolidifierState {
    solidifier_recv: Receiver<Option<Hash>>,
    drop_barrier: Arc<Barrier>,
}

impl SolidifierState {
    pub fn new(solidifier_recv: Receiver<Option<Hash>>, drop_barrier: Arc<Barrier>) -> Self {
        Self {
            solidifier_recv,
            drop_barrier,
        }
    }

    fn propagate(&self, hash: Hash) {
        let mut stack = vec![hash];
        let mut already_solid = HashSet::new();

        while let Some(hash) = stack.pop() {
            if !already_solid.contains(&hash) {
                if let Some(v) = tangle().vertices.get(&hash).map(|r| r.value().get_ref_to_inner()) {
                    if tangle().is_solid_transaction(v.trunk()) && tangle().is_solid_transaction(v.branch()) {
                        // NOTE: unwrap should be safe since we just added it to the Tangle
                        tangle().vertices.get_mut(&hash).unwrap().set_solid();
                        already_solid.insert(hash);

                        if let Some(approvers) = tangle().approvers.get(&hash) {
                            let approvers = approvers.value();
                            for approver in approvers {
                                stack.push(*approver);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Attempt to perform solidification upon a vertex (and its approvers).
    pub async fn run(mut self) {
        while let Ok(hash) = self.solidifier_recv.recv_async().await {
            if let Some(hash) = hash {
                // println!("solidif: before propagate");
                self.propagate(hash);
            // println!("solidif: after propagate");
            } else {
                // println!("solidif: before drop_barrier.wait");
                // block_on(self.drop_barrier.wait());
                self.drop_barrier.wait().await;
                // println!("solidif: after drop_barrier.wait");
                break;
            }
        }
    }
}
