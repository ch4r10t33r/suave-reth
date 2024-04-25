//! cmmand for launching a bootnode

use crate::{
    args::{
        utils::{chain_help, genesis_value_parser, parse_socket_address, SUPPORTED_CHAINS},
        DatabaseArgs, DebugArgs, DevArgs, NetworkArgs, PayloadBuilderArgs, PruningArgs,
        RpcServerArgs, TxPoolArgs,
    },
    dirs::{DataDirPath, MaybePlatformPath},
};
use clap::{value_parser, Args, Parser};
use reth_cli_runner::CliContext;
use reth_db::{init_db, DatabaseEnv};
use std::{ffi::OsString, fmt, net::SocketAddr, net::SocketAddrV4, net::Ipv4Addr};

/// Start the node
#[derive(Debug, Parser)]
pub struct BootNodeCommand<Ext: clap::Args + fmt::Debug = NoArgs> {
    #[arg(long = "addr", value_name = "ADDR", default_value_t = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 30301)))]
    pub addr: SocketAddr,

    #[arg(long = "genkey", value_name = "GEN_KEY", default_value_t = String::from(""))]
    pub gen_key: String,

    #[arg(long = "writeaddress", value_name = "WRITE_ADDRESS", default_value_t = false)]
    pub write_addr: bool,

    pub node_key_file: String,

    pub node_key_hex: String,

    pub nat_desc: String,

    pub net_restrict: String,

    pub runv5: bool,

    pub verbosity: u8,

    pub vm_module: String
   
}

impl BootNodeCommand {
    /// Parsers only the default CLI arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Parsers only the default [BootNodeCommand] arguments from the given iterator
    pub fn try_parse_args_from<I, T>(itr: I) -> Result<Self, clap::error::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::try_parse_from(itr)
    }
}

impl<Ext: clap::Args + fmt::Debug> BootNodeCommand<Ext> {
    /// Execute `bootnode` command
    pub async fn execute(&self) -> eyre::Result<()> {

    }
}

/// No Additional arguments
#[derive(Debug, Clone, Copy, Default, Args)]
#[non_exhaustive]
pub struct NoArgs;

#[cfg(test)]
mod tests {
    use super::*;
    
}
