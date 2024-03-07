// Copyright (c) 2024 Gibson Pilconis. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors
// may be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/pci_classes_phf.rs"));

/// An ID entry representing a PCI device class.
#[derive(Copy, Clone)]
pub struct PciClassEntry {
    id: u8,
    name: &'static str,
    subclasses: &'static [PciSubclassEntry],
}

/// An ID entry representing a PCI device subclass.
#[derive(Copy, Clone)]
pub struct PciSubclassEntry {
    id: u8,
    name: &'static str,
    progs: &'static [PciProgEntry],
}

/// An ID entry representing a PCI device programming interface.
#[derive(Copy, Clone)]
pub struct PciProgEntry {
    id: u8,
    name: &'static str,
}

/// Parses an integer ID to a `PciClassEntry`, if one with the ID exists.
pub fn get_class(id: u8) -> Option<PciClassEntry> {
    let result = CLASSES.get(&id);
    result?;
    Some(*result.unwrap())
}

impl PciClassEntry {
    /// Gets the ID of the class.
    pub fn get_id(&self) -> u8 {
        self.id
    }

    /// Gets the name of the class.
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    /// Gets all the subclasses associated with a class.
    pub fn get_subclasses(&self) -> Option<Vec<&PciSubclassEntry>> {
        let ret: Vec<&PciSubclassEntry> = self.subclasses.iter().collect();
        match ret.is_empty() {
            true => None,
            false => Some(ret),
        }
    }

    /// Gets a subclass associated with a class by its ID.
    pub fn get_subclass(&self, _id: u8) -> Option<&PciSubclassEntry> {
        self.subclasses.iter().find(|x| {x.id == _id})
    }
}

impl PciSubclassEntry {
    /// Gets the ID of the subclass.
    pub fn get_id(&self) -> u8 {
        self.id
    }

    /// Gets the name of the subclass.
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    /// Gets all the progs associated with a subclass.
    pub fn get_progs(&self) -> Option<Vec<&PciProgEntry>> {
        let ret: Vec<&PciProgEntry> = self.progs.iter().collect();
        match ret.is_empty() {
            true => None,
            false => Some(ret),
        }
    }

    /// Gets a prog associated with a subclass by its ID.
    pub fn get_prog(&self, _id: u8) -> Option<&PciProgEntry> {
        self.progs.iter().find(|x| {x.id == _id})
    }
}

impl PciProgEntry {
    /// Gets the ID of a programming interface.
    pub fn get_id(&self) -> u8 {
        self.id
    }

    /// Gets the name of a programming interface.
    pub fn get_name(&self) -> &'static str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::class::get_class;

    #[test]
    fn test_get_class() {
        let class = get_class(9).unwrap();
        assert_eq!(class.get_name(), "Input device controller");
    }
}
