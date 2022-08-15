// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::config::{Error, RootPath};
use aptos_types::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

const GENESIS_DEFAULT: &str = "genesis.blob";

#[derive(Clone, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ExecutionConfig {
    #[serde(skip)]
    pub genesis: Option<Transaction>,
    pub genesis_file_location: PathBuf,
    pub network_timeout_ms: u64,
    pub concurrency_level: u16,
    pub num_proof_reading_threads: u16,
    pub use_native_block_prologue_without_stake_performance: bool,
}

impl std::fmt::Debug for ExecutionConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExecutionConfig {{ genesis: ")?;
        if self.genesis.is_some() {
            write!(f, "Some(...)")?;
        } else {
            write!(f, "None")?;
        }
        write!(
            f,
            ", genesis_file_location: {:?} ",
            self.genesis_file_location
        )
    }
}

impl Default for ExecutionConfig {
    fn default() -> ExecutionConfig {
        ExecutionConfig {
            genesis: None,
            genesis_file_location: PathBuf::new(),
            // Default value of 30 seconds for the network timeout.
            network_timeout_ms: 30_000,
            // Sequential execution by default.
            concurrency_level: 1,
            num_proof_reading_threads: 32,
            use_native_block_prologue_without_stake_performance: false,
        }
    }
}

impl ExecutionConfig {
    pub fn load(&mut self, root_dir: &RootPath) -> Result<(), Error> {
        if !self.genesis_file_location.as_os_str().is_empty() {
            let path = root_dir.full_path(&self.genesis_file_location);
            let mut file = File::open(&path).map_err(|e| Error::IO("genesis".into(), e))?;
            let mut buffer = vec![];
            file.read_to_end(&mut buffer)
                .map_err(|e| Error::IO("genesis".into(), e))?;
            let data = bcs::from_bytes(&buffer).map_err(|e| Error::BCS("genesis", e))?;
            self.genesis = Some(data);
        }

        Ok(())
    }

    pub fn save(&mut self, root_dir: &RootPath) -> Result<(), Error> {
        if let Some(genesis) = &self.genesis {
            if self.genesis_file_location.as_os_str().is_empty() {
                self.genesis_file_location = PathBuf::from(GENESIS_DEFAULT);
            }
            let path = root_dir.full_path(&self.genesis_file_location);
            let mut file = File::create(&path).map_err(|e| Error::IO("genesis".into(), e))?;
            let data = bcs::to_bytes(&genesis).map_err(|e| Error::BCS("genesis", e))?;
            file.write_all(&data)
                .map_err(|e| Error::IO("genesis".into(), e))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aptos_temppath::TempPath;
    use aptos_types::{
        transaction::{ChangeSet, Transaction, WriteSetPayload},
        write_set::WriteSetMut,
    };

    #[test]
    fn test_no_genesis() {
        let (mut config, path) = generate_config();
        assert_eq!(config.genesis, None);
        let root_dir = RootPath::new_path(path.path());
        let result = config.load(&root_dir);
        assert!(result.is_ok());
        assert_eq!(config.genesis_file_location, PathBuf::new());
    }

    #[test]
    fn test_some_and_load_genesis() {
        let fake_genesis = Transaction::GenesisTransaction(WriteSetPayload::Direct(
            ChangeSet::new(WriteSetMut::new(vec![]).freeze().unwrap(), vec![]),
        ));
        let (mut config, path) = generate_config();
        config.genesis = Some(fake_genesis.clone());
        let root_dir = RootPath::new_path(path.path());
        config.save(&root_dir).expect("Unable to save");
        // Verifies some without path
        assert_eq!(config.genesis_file_location, PathBuf::from(GENESIS_DEFAULT));

        config.genesis = None;
        let result = config.load(&root_dir);
        assert!(result.is_ok());
        assert_eq!(config.genesis, Some(fake_genesis));
    }

    fn generate_config() -> (ExecutionConfig, TempPath) {
        let temp_dir = TempPath::new();
        temp_dir.create_as_dir().expect("error creating tempdir");
        let execution_config = ExecutionConfig::default();
        (execution_config, temp_dir)
    }
}
