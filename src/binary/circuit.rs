// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.
use dusk_plonk::prelude::*;
use dusk_plonk::prelude::Circuit;

use crate::binary::poseidon_merkle_copy::{
    Item as PoseidonItem, Opening as PoseidonOpening, Tree as PoseidonTree,
};
use crate::binary::zk::opening_gadget;

use crate::benchmark::KEY_LOG;
use dusk_bls12_381::BlsScalar;
use dusk_plonk::prelude::Error;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct OpeningCircuit {
    opening: PoseidonOpening<(), KEY_LOG>,
    leaf: PoseidonItem<()>,
}

impl Default for OpeningCircuit {
    fn default() -> Self {
        let empty_item = PoseidonItem {
            hash: BlsScalar::zero(),
            data: (),
        };
        let mut tree = PoseidonTree::new();
        tree.insert(0, empty_item);
        let opening = tree.opening(0).expect("There is a leaf at position 0");
        Self {
            opening,
            leaf: empty_item,
        }
    }
}

impl OpeningCircuit {
    /// Create a new OpeningCircuit
    pub fn new(
        opening: PoseidonOpening<(), KEY_LOG>,
        leaf: PoseidonItem<()>,
    ) -> Self {
        Self { opening, leaf }
    }
}

impl Circuit for OpeningCircuit {
    fn circuit(&self, composer: &mut Composer) -> Result<(), Error> {
        // append the leaf and opening gadget to the circuit
        let leaf = composer.append_witness(self.leaf.hash);
        let computed_root = opening_gadget(composer, &self.opening, leaf);

        // append the public root as public input to the circuit
        // and ensure it is equal to the computed root
        let constraint = Constraint::new()
            .left(-BlsScalar::one())
            .a(computed_root)
            .public(self.opening.root().hash);
        composer.append_gate(constraint);

        Ok(())
    }
}