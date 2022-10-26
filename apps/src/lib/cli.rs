//! The CLI commands that are re-used between the executables `anoma`,
//! `anoma-node` and `anoma-client`.
//!
//! The `anoma` executable groups together the most commonly used commands
//! inlined from the node and the client. The other commands for the node or the
//! client can be dispatched via `anoma node ...` or `anoma client ...`,
//! respectively.

pub mod context;
mod utils;

use clap::{AppSettings, ArgGroup, ArgMatches};
use color_eyre::eyre::Result;
pub use utils::safe_exit;
use utils::*;

pub use self::context::Context;

include!("../../version.rs");

const APP_NAME: &str = "Namada";

// Main Anoma sub-commands
const NODE_CMD: &str = "node";
const CLIENT_CMD: &str = "client";
const WALLET_CMD: &str = "wallet";

pub mod cmds {
    use clap::AppSettings;

    use super::utils::*;
    use super::{args, ArgMatches, CLIENT_CMD, NODE_CMD, WALLET_CMD};

    /// Commands for `anoma` binary.
    #[allow(clippy::large_enum_variant)]
    #[derive(Clone, Debug)]
    pub enum Anoma {
        // Sub-binary-commands
        Node(AnomaNode),
        Client(AnomaClient),
        Wallet(AnomaWallet),

        // Inlined commands from the node.
        Ledger(Ledger),

        // Inlined commands from the client.
        TxCustom(TxCustom),
        TxTransfer(TxTransfer),
        TxUpdateVp(TxUpdateVp),
        TxInitProposal(TxInitProposal),
        TxVoteProposal(TxVoteProposal),
    }

    impl Cmd for Anoma {
        fn add_sub(app: App) -> App {
            app.subcommand(AnomaNode::def())
                .subcommand(AnomaClient::def())
                .subcommand(AnomaWallet::def())
                .subcommand(Ledger::def())
                .subcommand(TxCustom::def())
                .subcommand(TxTransfer::def())
                .subcommand(TxUpdateVp::def())
                .subcommand(TxInitProposal::def())
                .subcommand(TxVoteProposal::def())
        }

        fn parse(matches: &ArgMatches) -> Option<Self> {
            let node = SubCmd::parse(matches).map(Self::Node);
            let client = SubCmd::parse(matches).map(Self::Client);
            let wallet = SubCmd::parse(matches).map(Self::Wallet);
            let ledger = SubCmd::parse(matches).map(Self::Ledger);
            let tx_custom = SubCmd::parse(matches).map(Self::TxCustom);
            let tx_transfer = SubCmd::parse(matches).map(Self::TxTransfer);
            let tx_update_vp = SubCmd::parse(matches).map(Self::TxUpdateVp);
            let tx_init_proposal =
                SubCmd::parse(matches).map(Self::TxInitProposal);
            let tx_vote_proposal =
                SubCmd::parse(matches).map(Self::TxVoteProposal);
            node.or(client)
                .or(wallet)
                .or(ledger)
                .or(tx_custom)
                .or(tx_transfer)
                .or(tx_update_vp)
                .or(tx_init_proposal)
                .or(tx_vote_proposal)
        }
    }

    /// Used as top-level commands (`Cmd` instance) in `anoman` binary.
    /// Used as sub-commands (`SubCmd` instance) in `anoma` binary.
    #[derive(Clone, Debug)]
    #[allow(clippy::large_enum_variant)]
    pub enum AnomaNode {
        Ledger(Ledger),
        Config(Config),
    }

    impl Cmd for AnomaNode {
        fn add_sub(app: App) -> App {
            app.subcommand(Ledger::def()).subcommand(Config::def())
        }

        fn parse(matches: &ArgMatches) -> Option<Self> {
            let ledger = SubCmd::parse(matches).map(Self::Ledger);
            let config = SubCmd::parse(matches).map(Self::Config);
            ledger.or(config)
        }
    }
    impl SubCmd for AnomaNode {
        const CMD: &'static str = NODE_CMD;

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .and_then(<Self as Cmd>::parse)
        }

        fn def() -> App {
            <Self as Cmd>::add_sub(
                App::new(Self::CMD)
                    .about("Node sub-commands.")
                    .setting(AppSettings::SubcommandRequiredElseHelp),
            )
        }
    }

    /// Used as top-level commands (`Cmd` instance) in `anomac` binary.
    /// Used as sub-commands (`SubCmd` instance) in `anoma` binary.
    #[derive(Clone, Debug)]
    #[allow(clippy::large_enum_variant)]
    pub enum AnomaClient {
        /// The [`super::Context`] provides access to the wallet and the
        /// config. It will generate a new wallet and config, if they
        /// don't exist.
        WithContext(AnomaClientWithContext),
        /// Utils don't have [`super::Context`], only the global arguments.
        WithoutContext(Utils),
    }

    impl Cmd for AnomaClient {
        fn add_sub(app: App) -> App {
            app
                // Simple transactions
                .subcommand(TxCustom::def().display_order(1))
                .subcommand(TxTransfer::def().display_order(1))
                .subcommand(TxUpdateVp::def().display_order(1))
                .subcommand(TxInitAccount::def().display_order(1))
                .subcommand(TxInitValidator::def().display_order(1))
                // Proposal transactions
                .subcommand(TxInitProposal::def().display_order(1))
                .subcommand(TxVoteProposal::def().display_order(1))
                // PoS transactions
                .subcommand(Bond::def().display_order(2))
                .subcommand(Unbond::def().display_order(2))
                .subcommand(Withdraw::def().display_order(2))
                // Queries
                .subcommand(QueryEpoch::def().display_order(3))
                .subcommand(QueryBlock::def().display_order(3))
                .subcommand(QueryBalance::def().display_order(3))
                .subcommand(QueryBonds::def().display_order(3))
                .subcommand(QueryVotingPower::def().display_order(3))
                .subcommand(QuerySlashes::def().display_order(3))
                .subcommand(QueryResult::def().display_order(3))
                .subcommand(QueryRawBytes::def().display_order(3))
                .subcommand(QueryProposal::def().display_order(3))
                .subcommand(QueryProposalResult::def().display_order(3))
                .subcommand(QueryProtocolParameters::def().display_order(3))
                // Utils
                .subcommand(Utils::def().display_order(5))
        }

        fn parse(matches: &ArgMatches) -> Option<Self> {
            use AnomaClientWithContext::*;
            let tx_custom = Self::parse_with_ctx(matches, TxCustom);
            let tx_transfer = Self::parse_with_ctx(matches, TxTransfer);
            let tx_update_vp = Self::parse_with_ctx(matches, TxUpdateVp);
            let tx_init_account = Self::parse_with_ctx(matches, TxInitAccount);
            let tx_init_validator =
                Self::parse_with_ctx(matches, TxInitValidator);
            let tx_init_proposal =
                Self::parse_with_ctx(matches, TxInitProposal);
            let tx_vote_proposal =
                Self::parse_with_ctx(matches, TxVoteProposal);
            let bond = Self::parse_with_ctx(matches, Bond);
            let unbond = Self::parse_with_ctx(matches, Unbond);
            let withdraw = Self::parse_with_ctx(matches, Withdraw);
            let query_epoch = Self::parse_with_ctx(matches, QueryEpoch);
            let query_block = Self::parse_with_ctx(matches, QueryBlock);
            let query_balance = Self::parse_with_ctx(matches, QueryBalance);
            let query_bonds = Self::parse_with_ctx(matches, QueryBonds);
            let query_voting_power =
                Self::parse_with_ctx(matches, QueryVotingPower);
            let query_slashes = Self::parse_with_ctx(matches, QuerySlashes);
            let query_result = Self::parse_with_ctx(matches, QueryResult);
            let query_raw_bytes = Self::parse_with_ctx(matches, QueryRawBytes);
            let query_proposal = Self::parse_with_ctx(matches, QueryProposal);
            let query_proposal_result =
                Self::parse_with_ctx(matches, QueryProposalResult);
            let query_protocol_parameters =
                Self::parse_with_ctx(matches, QueryProtocolParameters);
            let utils = SubCmd::parse(matches).map(Self::WithoutContext);
            tx_custom
                .or(tx_transfer)
                .or(tx_update_vp)
                .or(tx_init_account)
                .or(tx_init_validator)
                .or(tx_init_proposal)
                .or(tx_vote_proposal)
                .or(bond)
                .or(unbond)
                .or(withdraw)
                .or(query_epoch)
                .or(query_block)
                .or(query_balance)
                .or(query_bonds)
                .or(query_voting_power)
                .or(query_slashes)
                .or(query_result)
                .or(query_raw_bytes)
                .or(query_proposal)
                .or(query_proposal_result)
                .or(query_protocol_parameters)
                .or(utils)
        }
    }

    impl AnomaClient {
        /// A helper method to parse sub cmds with context
        fn parse_with_ctx<T: SubCmd>(
            matches: &ArgMatches,
            sub_to_self: impl Fn(T) -> AnomaClientWithContext,
        ) -> Option<Self> {
            SubCmd::parse(matches)
                .map(|sub| Self::WithContext(sub_to_self(sub)))
        }
    }

    impl SubCmd for AnomaClient {
        const CMD: &'static str = CLIENT_CMD;

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .and_then(<Self as Cmd>::parse)
        }

        fn def() -> App {
            <Self as Cmd>::add_sub(
                App::new(Self::CMD)
                    .about("Client sub-commands.")
                    .setting(AppSettings::SubcommandRequiredElseHelp),
            )
        }
    }

    #[derive(Clone, Debug)]
    pub enum AnomaClientWithContext {
        // Ledger cmds
        TxCustom(TxCustom),
        TxTransfer(TxTransfer),
        QueryResult(QueryResult),
        TxUpdateVp(TxUpdateVp),
        TxInitAccount(TxInitAccount),
        TxInitValidator(TxInitValidator),
        TxInitProposal(TxInitProposal),
        TxVoteProposal(TxVoteProposal),
        Bond(Bond),
        Unbond(Unbond),
        Withdraw(Withdraw),
        QueryEpoch(QueryEpoch),
        QueryBlock(QueryBlock),
        QueryBalance(QueryBalance),
        QueryBonds(QueryBonds),
        QueryVotingPower(QueryVotingPower),
        QueryCommissionRate(QueryCommissionRate),
        QuerySlashes(QuerySlashes),
        QueryRawBytes(QueryRawBytes),
        QueryProposal(QueryProposal),
        QueryProposalResult(QueryProposalResult),
        QueryProtocolParameters(QueryProtocolParameters),
    }

    #[derive(Clone, Debug)]
    pub enum AnomaWallet {
        /// Key management commands
        Key(WalletKey),
        /// Address management commands
        Address(WalletAddress),
    }

    impl Cmd for AnomaWallet {
        fn add_sub(app: App) -> App {
            app.subcommand(WalletKey::def())
                .subcommand(WalletAddress::def())
        }

        fn parse(matches: &ArgMatches) -> Option<Self> {
            let key = SubCmd::parse(matches).map(Self::Key);
            let address = SubCmd::parse(matches).map(Self::Address);
            key.or(address)
        }
    }

    impl SubCmd for AnomaWallet {
        const CMD: &'static str = WALLET_CMD;

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .and_then(<Self as Cmd>::parse)
        }

        fn def() -> App {
            <Self as Cmd>::add_sub(
                App::new(Self::CMD)
                    .about("Wallet sub-commands.")
                    .setting(AppSettings::SubcommandRequiredElseHelp),
            )
        }
    }

    #[derive(Clone, Debug)]
    #[allow(clippy::large_enum_variant)]
    pub enum WalletKey {
        Gen(KeyGen),
        Find(KeyFind),
        List(KeyList),
        Export(Export),
    }

    impl SubCmd for WalletKey {
        const CMD: &'static str = "key";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches.subcommand_matches(Self::CMD).and_then(|matches| {
                let generate = SubCmd::parse(matches).map(Self::Gen);
                let lookup = SubCmd::parse(matches).map(Self::Find);
                let list = SubCmd::parse(matches).map(Self::List);
                let export = SubCmd::parse(matches).map(Self::Export);
                generate.or(lookup).or(list).or(export)
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about(
                    "Keypair management, including methods to generate and \
                     look-up keys.",
                )
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(KeyGen::def())
                .subcommand(KeyFind::def())
                .subcommand(KeyList::def())
                .subcommand(Export::def())
        }
    }

    /// Generate a new keypair and an implicit address derived from it
    #[derive(Clone, Debug)]
    pub struct KeyGen(pub args::KeyAndAddressGen);

    impl SubCmd for KeyGen {
        const CMD: &'static str = "gen";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| Self(args::KeyAndAddressGen::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about(
                    "Generates a keypair with a given alias and derive the \
                     implicit address from its public key. The address will \
                     be stored with the same alias.",
                )
                .add_args::<args::KeyAndAddressGen>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct KeyFind(pub args::KeyFind);

    impl SubCmd for KeyFind {
        const CMD: &'static str = "find";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| (Self(args::KeyFind::parse(matches))))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Searches for a keypair from a public key or an alias.")
                .add_args::<args::KeyFind>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct KeyList(pub args::KeyList);

    impl SubCmd for KeyList {
        const CMD: &'static str = "list";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| (Self(args::KeyList::parse(matches))))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("List all known keys.")
                .add_args::<args::KeyList>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct Export(pub args::KeyExport);

    impl SubCmd for Export {
        const CMD: &'static str = "export";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| (Self(args::KeyExport::parse(matches))))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Exports a keypair to a file.")
                .add_args::<args::KeyExport>()
        }
    }

    #[derive(Clone, Debug)]
    pub enum WalletAddress {
        Gen(AddressGen),
        Find(AddressOrAliasFind),
        List(AddressList),
        Add(AddressAdd),
    }

    impl SubCmd for WalletAddress {
        const CMD: &'static str = "address";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches.subcommand_matches(Self::CMD).and_then(|matches| {
                let gen = SubCmd::parse(matches).map(Self::Gen);
                let find = SubCmd::parse(matches).map(Self::Find);
                let list = SubCmd::parse(matches).map(Self::List);
                let add = SubCmd::parse(matches).map(Self::Add);
                gen.or(find).or(list).or(add)
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about(
                    "Address management, including methods to generate and \
                     look-up addresses.",
                )
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(AddressGen::def())
                .subcommand(AddressOrAliasFind::def())
                .subcommand(AddressList::def())
                .subcommand(AddressAdd::def())
        }
    }

    /// Generate a new keypair and an implicit address derived from it
    #[derive(Clone, Debug)]
    pub struct AddressGen(pub args::KeyAndAddressGen);

    impl SubCmd for AddressGen {
        const CMD: &'static str = "gen";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches.subcommand_matches(Self::CMD).map(|matches| {
                AddressGen(args::KeyAndAddressGen::parse(matches))
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about(
                    "Generates a keypair with a given alias and derive the \
                     implicit address from its public key. The address will \
                     be stored with the same alias.",
                )
                .add_args::<args::KeyAndAddressGen>()
        }
    }

    /// Find an address by its alias
    #[derive(Clone, Debug)]
    pub struct AddressOrAliasFind(pub args::AddressOrAliasFind);

    impl SubCmd for AddressOrAliasFind {
        const CMD: &'static str = "find";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches.subcommand_matches(Self::CMD).map(|matches| {
                AddressOrAliasFind(args::AddressOrAliasFind::parse(matches))
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about(
                    "Find an address by its alias or an alias by its address.",
                )
                .add_args::<args::AddressOrAliasFind>()
        }
    }

    /// List known addresses
    #[derive(Clone, Debug)]
    pub struct AddressList;

    impl SubCmd for AddressList {
        const CMD: &'static str = "list";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|_matches| AddressList)
        }

        fn def() -> App {
            App::new(Self::CMD).about("List all known addresses.")
        }
    }

    /// Generate a new keypair and an implicit address derived from it
    #[derive(Clone, Debug)]
    pub struct AddressAdd(pub args::AddressAdd);

    impl SubCmd for AddressAdd {
        const CMD: &'static str = "add";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| AddressAdd(args::AddressAdd::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Store an alias for an address in the wallet.")
                .add_args::<args::AddressAdd>()
        }
    }

    #[derive(Clone, Debug)]
    pub enum Ledger {
        Run(LedgerRun),
        Reset(LedgerReset),
    }

    impl SubCmd for Ledger {
        const CMD: &'static str = "ledger";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches.subcommand_matches(Self::CMD).and_then(|matches| {
                let run = SubCmd::parse(matches).map(Self::Run);
                let reset = SubCmd::parse(matches).map(Self::Reset);
                run.or(reset)
                    // The `run` command is the default if no sub-command given
                    .or(Some(Self::Run(LedgerRun)))
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about(
                    "Ledger node sub-commands. If no sub-command specified, \
                     defaults to run the node.",
                )
                .subcommand(LedgerRun::def())
                .subcommand(LedgerReset::def())
        }
    }

    #[derive(Clone, Debug)]
    pub struct LedgerRun;

    impl SubCmd for LedgerRun {
        const CMD: &'static str = "run";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches.subcommand_matches(Self::CMD).map(|_matches| Self)
        }

        fn def() -> App {
            App::new(Self::CMD).about("Run Anoma ledger node.")
        }
    }

    #[derive(Clone, Debug)]
    pub struct LedgerReset;

    impl SubCmd for LedgerReset {
        const CMD: &'static str = "reset";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches.subcommand_matches(Self::CMD).map(|_matches| Self)
        }

        fn def() -> App {
            App::new(Self::CMD).about(
                "Delete Anoma ledger node's and Tendermint node's storage \
                 data.",
            )
        }
    }

    #[derive(Clone, Debug)]
    pub enum Config {
        Gen(ConfigGen),
    }

    impl SubCmd for Config {
        const CMD: &'static str = "config";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .and_then(|matches| SubCmd::parse(matches).map(Self::Gen))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .about("Configuration sub-commands.")
                .subcommand(ConfigGen::def())
        }
    }

    #[derive(Clone, Debug)]
    pub struct ConfigGen;

    impl SubCmd for ConfigGen {
        const CMD: &'static str = "gen";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches.subcommand_matches(Self::CMD).map(|_matches| Self)
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Generate the default configuration file.")
        }
    }

    #[derive(Clone, Debug)]
    pub struct QueryResult(pub args::QueryResult);

    impl SubCmd for QueryResult {
        const CMD: &'static str = "tx-result";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| QueryResult(args::QueryResult::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Query the result of a transaction.")
                .add_args::<args::QueryResult>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct QueryProposal(pub args::QueryProposal);

    impl SubCmd for QueryProposal {
        const CMD: &'static str = "query-proposal";

        fn parse(matches: &ArgMatches) -> Option<Self>
        where
            Self: Sized,
        {
            matches.subcommand_matches(Self::CMD).map(|matches| {
                QueryProposal(args::QueryProposal::parse(matches))
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Query proposals.")
                .add_args::<args::QueryProposal>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct QueryProposalResult(pub args::QueryProposalResult);

    impl SubCmd for QueryProposalResult {
        const CMD: &'static str = "query-proposal-result";

        fn parse(matches: &ArgMatches) -> Option<Self>
        where
            Self: Sized,
        {
            matches.subcommand_matches(Self::CMD).map(|matches| {
                QueryProposalResult(args::QueryProposalResult::parse(matches))
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Query proposals result.")
                .add_args::<args::QueryProposalResult>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct QueryProtocolParameters(pub args::QueryProtocolParameters);

    impl SubCmd for QueryProtocolParameters {
        const CMD: &'static str = "query-protocol-parameters";

        fn parse(matches: &ArgMatches) -> Option<Self>
        where
            Self: Sized,
        {
            matches.subcommand_matches(Self::CMD).map(|matches| {
                QueryProtocolParameters(args::QueryProtocolParameters::parse(
                    matches,
                ))
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Query protocol parameters.")
                .add_args::<args::QueryProtocolParameters>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct TxCustom(pub args::TxCustom);

    impl SubCmd for TxCustom {
        const CMD: &'static str = "tx";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| TxCustom(args::TxCustom::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Send a transaction with custom WASM code.")
                .add_args::<args::TxCustom>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct TxTransfer(pub args::TxTransfer);

    impl SubCmd for TxTransfer {
        const CMD: &'static str = "transfer";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| TxTransfer(args::TxTransfer::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Send a signed transfer transaction.")
                .add_args::<args::TxTransfer>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct TxUpdateVp(pub args::TxUpdateVp);

    impl SubCmd for TxUpdateVp {
        const CMD: &'static str = "update";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| TxUpdateVp(args::TxUpdateVp::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about(
                    "Send a signed transaction to update account's validity \
                     predicate.",
                )
                .add_args::<args::TxUpdateVp>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct TxInitAccount(pub args::TxInitAccount);

    impl SubCmd for TxInitAccount {
        const CMD: &'static str = "init-account";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches.subcommand_matches(Self::CMD).map(|matches| {
                TxInitAccount(args::TxInitAccount::parse(matches))
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about(
                    "Send a signed transaction to create a new established \
                     account.",
                )
                .add_args::<args::TxInitAccount>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct TxInitValidator(pub args::TxInitValidator);

    impl SubCmd for TxInitValidator {
        const CMD: &'static str = "init-validator";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches.subcommand_matches(Self::CMD).map(|matches| {
                TxInitValidator(args::TxInitValidator::parse(matches))
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about(
                    "Send a signed transaction to create a new validator \
                     account.",
                )
                .add_args::<args::TxInitValidator>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct Bond(pub args::Bond);

    impl SubCmd for Bond {
        const CMD: &'static str = "bond";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| Bond(args::Bond::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Bond tokens in PoS system.")
                .add_args::<args::Bond>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct Unbond(pub args::Unbond);

    impl SubCmd for Unbond {
        const CMD: &'static str = "unbond";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| Unbond(args::Unbond::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Unbond tokens from a PoS bond.")
                .add_args::<args::Unbond>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct Withdraw(pub args::Withdraw);

    impl SubCmd for Withdraw {
        const CMD: &'static str = "withdraw";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| Withdraw(args::Withdraw::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Withdraw tokens from previously unbonded PoS bond.")
                .add_args::<args::Withdraw>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct QueryEpoch(pub args::Query);

    impl SubCmd for QueryEpoch {
        const CMD: &'static str = "epoch";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| QueryEpoch(args::Query::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Query the epoch of the last committed block.")
                .add_args::<args::Query>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct QueryBlock(pub args::Query);

    impl SubCmd for QueryBlock {
        const CMD: &'static str = "block";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| QueryBlock(args::Query::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Query the last committed block.")
                .add_args::<args::Query>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct QueryBalance(pub args::QueryBalance);

    impl SubCmd for QueryBalance {
        const CMD: &'static str = "balance";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| QueryBalance(args::QueryBalance::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Query balance(s) of tokens.")
                .add_args::<args::QueryBalance>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct QueryBonds(pub args::QueryBonds);

    impl SubCmd for QueryBonds {
        const CMD: &'static str = "bonds";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| QueryBonds(args::QueryBonds::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Query PoS bond(s).")
                .add_args::<args::QueryBonds>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct QueryVotingPower(pub args::QueryVotingPower);

    impl SubCmd for QueryVotingPower {
        const CMD: &'static str = "voting-power";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches.subcommand_matches(Self::CMD).map(|matches| {
                QueryVotingPower(args::QueryVotingPower::parse(matches))
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Query PoS voting power.")
                .add_args::<args::QueryVotingPower>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct QueryCommissionRate(pub args::QueryCommissionRate);

    impl SubCmd for QueryCommissionRate {
        const CMD: &'static str = "commission-rate";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches.subcommand_matches(Self::CMD).map(|matches| {
                QueryCommissionRate(args::QueryCommissionRate::parse(matches))
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Query commission rate.")
                .add_args::<args::QueryCommissionRate>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct QuerySlashes(pub args::QuerySlashes);

    impl SubCmd for QuerySlashes {
        const CMD: &'static str = "slashes";

        fn parse(matches: &ArgMatches) -> Option<Self>
        where
            Self: Sized,
        {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| QuerySlashes(args::QuerySlashes::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Query PoS applied slashes.")
                .add_args::<args::QuerySlashes>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct QueryRawBytes(pub args::QueryRawBytes);

    impl SubCmd for QueryRawBytes {
        const CMD: &'static str = "query-bytes";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches.subcommand_matches(Self::CMD).map(|matches| {
                QueryRawBytes(args::QueryRawBytes::parse(matches))
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Query the raw bytes of a given storage key")
                .add_args::<args::QueryRawBytes>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct TxInitProposal(pub args::InitProposal);

    impl SubCmd for TxInitProposal {
        const CMD: &'static str = "init-proposal";

        fn parse(matches: &ArgMatches) -> Option<Self>
        where
            Self: Sized,
        {
            matches.subcommand_matches(Self::CMD).map(|matches| {
                TxInitProposal(args::InitProposal::parse(matches))
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Create a new proposal.")
                .add_args::<args::InitProposal>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct TxVoteProposal(pub args::VoteProposal);

    impl SubCmd for TxVoteProposal {
        const CMD: &'static str = "vote-proposal";

        fn parse(matches: &ArgMatches) -> Option<Self>
        where
            Self: Sized,
        {
            matches.subcommand_matches(Self::CMD).map(|matches| {
                TxVoteProposal(args::VoteProposal::parse(matches))
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Vote a proposal.")
                .add_args::<args::VoteProposal>()
        }
    }

    #[derive(Clone, Debug)]
    pub enum Utils {
        JoinNetwork(JoinNetwork),
        FetchWasms(FetchWasms),
        InitNetwork(InitNetwork),
        InitGenesisValidator(InitGenesisValidator),
    }

    impl SubCmd for Utils {
        const CMD: &'static str = "utils";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches.subcommand_matches(Self::CMD).and_then(|matches| {
                let join_network =
                    SubCmd::parse(matches).map(Self::JoinNetwork);
                let fetch_wasms = SubCmd::parse(matches).map(Self::FetchWasms);
                let init_network =
                    SubCmd::parse(matches).map(Self::InitNetwork);
                let init_genesis =
                    SubCmd::parse(matches).map(Self::InitGenesisValidator);
                join_network
                    .or(fetch_wasms)
                    .or(init_network)
                    .or(init_genesis)
            })
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Utilities.")
                .subcommand(JoinNetwork::def())
                .subcommand(FetchWasms::def())
                .subcommand(InitNetwork::def())
                .subcommand(InitGenesisValidator::def())
                .setting(AppSettings::SubcommandRequiredElseHelp)
        }
    }

    #[derive(Clone, Debug)]
    pub struct JoinNetwork(pub args::JoinNetwork);

    impl SubCmd for JoinNetwork {
        const CMD: &'static str = "join-network";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| Self(args::JoinNetwork::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Configure Anoma to join an existing network.")
                .add_args::<args::JoinNetwork>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct FetchWasms(pub args::FetchWasms);

    impl SubCmd for FetchWasms {
        const CMD: &'static str = "fetch-wasms";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| Self(args::FetchWasms::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Ensure pre-built wasms are present")
                .add_args::<args::FetchWasms>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct InitNetwork(pub args::InitNetwork);

    impl SubCmd for InitNetwork {
        const CMD: &'static str = "init-network";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| Self(args::InitNetwork::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about("Initialize a new test network.")
                .add_args::<args::InitNetwork>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct InitGenesisValidator(pub args::InitGenesisValidator);

    impl SubCmd for InitGenesisValidator {
        const CMD: &'static str = "init-genesis-validator";

        fn parse(matches: &ArgMatches) -> Option<Self> {
            matches
                .subcommand_matches(Self::CMD)
                .map(|matches| Self(args::InitGenesisValidator::parse(matches)))
        }

        fn def() -> App {
            App::new(Self::CMD)
                .about(
                    "Initialize genesis validator's address, consensus key \
                     and validator account key and use it in the ledger's \
                     node.",
                )
                .add_args::<args::InitGenesisValidator>()
        }
    }
}

pub mod args {

    use std::env;
    use std::net::SocketAddr;
    use std::path::PathBuf;
    use std::str::FromStr;

    use namada::types::address::Address;
    use namada::types::chain::{ChainId, ChainIdPrefix};
    use namada::types::governance::ProposalVote;
    use namada::types::key::*;
    use namada::types::storage::{self, Epoch};
    use namada::types::token;
    use namada::types::transaction::GasLimit;
    use rust_decimal::Decimal;

    use super::context::{WalletAddress, WalletKeypair, WalletPublicKey};
    use super::utils::*;
    use super::{ArgGroup, ArgMatches};
    use crate::config;
    use crate::config::TendermintMode;
    use crate::facade::tendermint::Timeout;
    use crate::facade::tendermint_config::net::Address as TendermintAddress;

    const ADDRESS: Arg<WalletAddress> = arg("address");
    const ALIAS_OPT: ArgOpt<String> = ALIAS.opt();
    const ALIAS: Arg<String> = arg("alias");
    const ALLOW_DUPLICATE_IP: ArgFlag = flag("allow-duplicate-ip");
    const AMOUNT: Arg<token::Amount> = arg("amount");
    const ARCHIVE_DIR: ArgOpt<PathBuf> = arg_opt("archive-dir");
    const BASE_DIR: ArgDefault<PathBuf> = arg_default(
        "base-dir",
        DefaultFn(|| match env::var("ANOMA_BASE_DIR") {
            Ok(dir) => dir.into(),
            Err(_) => config::DEFAULT_BASE_DIR.into(),
        }),
    );
    const BROADCAST_ONLY: ArgFlag = flag("broadcast-only");
    const CHAIN_ID: Arg<ChainId> = arg("chain-id");
    const CHAIN_ID_OPT: ArgOpt<ChainId> = CHAIN_ID.opt();
    const CHAIN_ID_PREFIX: Arg<ChainIdPrefix> = arg("chain-prefix");
    const CODE_PATH: Arg<PathBuf> = arg("code-path");
    const CODE_PATH_OPT: ArgOpt<PathBuf> = CODE_PATH.opt();
    const COMMISSION_RATE: Arg<Decimal> = arg("commission-rate");
    const CONSENSUS_TIMEOUT_COMMIT: ArgDefault<Timeout> = arg_default(
        "consensus-timeout-commit",
        DefaultFn(|| Timeout::from_str("1s").unwrap()),
    );
    const DATA_PATH_OPT: ArgOpt<PathBuf> = arg_opt("data-path");
    const DATA_PATH: Arg<PathBuf> = arg("data-path");
    const DECRYPT: ArgFlag = flag("decrypt");
    const DONT_ARCHIVE: ArgFlag = flag("dont-archive");
    const DRY_RUN_TX: ArgFlag = flag("dry-run");
    const EPOCH: ArgOpt<Epoch> = arg_opt("epoch");
    const FEE_AMOUNT: ArgDefault<token::Amount> =
        arg_default("fee-amount", DefaultFn(|| token::Amount::from(0)));
    const FEE_TOKEN: ArgDefaultFromCtx<WalletAddress> =
        arg_default_from_ctx("fee-token", DefaultFn(|| "NAM".into()));
    const FORCE: ArgFlag = flag("force");
    const DONT_PREFETCH_WASM: ArgFlag = flag("dont-prefetch-wasm");
    const GAS_LIMIT: ArgDefault<token::Amount> =
        arg_default("gas-limit", DefaultFn(|| token::Amount::from(0)));
    const GENESIS_PATH: Arg<PathBuf> = arg("genesis-path");
    const GENESIS_VALIDATOR: ArgOpt<String> = arg("genesis-validator").opt();
    const LEDGER_ADDRESS_ABOUT: &str =
        "Address of a ledger node as \"{scheme}://{host}:{port}\". If the \
         scheme is not supplied, it is assumed to be TCP.";
    const LEDGER_ADDRESS_DEFAULT: ArgDefault<TendermintAddress> =
        LEDGER_ADDRESS.default(DefaultFn(|| {
            let raw = "127.0.0.1:26657";
            TendermintAddress::from_str(raw).unwrap()
        }));

    const LEDGER_ADDRESS: Arg<TendermintAddress> = arg("ledger-address");
    const LOCALHOST: ArgFlag = flag("localhost");
    const MAX_COMMISSION_RATE_CHANGE: Arg<Decimal> =
        arg("max-commission-rate-change");
    const MODE: ArgOpt<String> = arg_opt("mode");
    const NET_ADDRESS: Arg<SocketAddr> = arg("net-address");
    const OWNER: ArgOpt<WalletAddress> = arg_opt("owner");
    const PROPOSAL_OFFLINE: ArgFlag = flag("offline");
    const PROTOCOL_KEY: ArgOpt<WalletPublicKey> = arg_opt("protocol-key");
    const PRE_GENESIS_PATH: ArgOpt<PathBuf> = arg_opt("pre-genesis-path");
    const PUBLIC_KEY: Arg<WalletPublicKey> = arg("public-key");
    const PROPOSAL_ID: Arg<u64> = arg("proposal-id");
    const PROPOSAL_ID_OPT: ArgOpt<u64> = arg_opt("proposal-id");
    const PROPOSAL_VOTE: Arg<ProposalVote> = arg("vote");
    const RAW_ADDRESS: Arg<Address> = arg("address");
    const RAW_ADDRESS_OPT: ArgOpt<Address> = RAW_ADDRESS.opt();
    const RAW_PUBLIC_KEY_OPT: ArgOpt<common::PublicKey> = arg_opt("public-key");
    const SCHEME: ArgDefault<SchemeType> =
        arg_default("scheme", DefaultFn(|| SchemeType::Ed25519));
    const SIGNER: ArgOpt<WalletAddress> = arg_opt("signer");
    const SIGNING_KEY_OPT: ArgOpt<WalletKeypair> = SIGNING_KEY.opt();
    const SIGNING_KEY: Arg<WalletKeypair> = arg("signing-key");
    const SOURCE: Arg<WalletAddress> = arg("source");
    const SOURCE_OPT: ArgOpt<WalletAddress> = SOURCE.opt();
    const STORAGE_KEY: Arg<storage::Key> = arg("storage-key");
    const SUB_PREFIX: ArgOpt<String> = arg_opt("sub-prefix");
    const TARGET: Arg<WalletAddress> = arg("target");
    const TOKEN_OPT: ArgOpt<WalletAddress> = TOKEN.opt();
    const TOKEN: Arg<WalletAddress> = arg("token");
    const TX_HASH: Arg<String> = arg("tx-hash");
    const UNSAFE_DONT_ENCRYPT: ArgFlag = flag("unsafe-dont-encrypt");
    const UNSAFE_SHOW_SECRET: ArgFlag = flag("unsafe-show-secret");
    const VALIDATOR: Arg<WalletAddress> = arg("validator");
    const VALIDATOR_OPT: ArgOpt<WalletAddress> = VALIDATOR.opt();
    const VALIDATOR_ACCOUNT_KEY: ArgOpt<WalletPublicKey> =
        arg_opt("account-key");
    const VALIDATOR_CONSENSUS_KEY: ArgOpt<WalletKeypair> =
        arg_opt("consensus-key");
    const VALIDATOR_CODE_PATH: ArgOpt<PathBuf> = arg_opt("validator-code-path");
    const VALUE: ArgOpt<String> = arg_opt("value");
    const WASM_CHECKSUMS_PATH: Arg<PathBuf> = arg("wasm-checksums-path");
    const WASM_DIR: ArgOpt<PathBuf> = arg_opt("wasm-dir");

    /// Global command arguments
    #[derive(Clone, Debug)]
    pub struct Global {
        pub chain_id: Option<ChainId>,
        pub base_dir: PathBuf,
        pub wasm_dir: Option<PathBuf>,
        pub mode: Option<TendermintMode>,
    }

    impl Global {
        /// Parse global arguments
        pub fn parse(matches: &ArgMatches) -> Self {
            let chain_id = CHAIN_ID_OPT.parse(matches);
            let base_dir = BASE_DIR.parse(matches);
            let wasm_dir = WASM_DIR.parse(matches);
            let mode = MODE.parse(matches).map(TendermintMode::from);
            Global {
                chain_id,
                base_dir,
                wasm_dir,
                mode,
            }
        }

        /// Add global args definition. Should be added to every top-level
        /// command.
        pub fn def(app: App) -> App {
            app.arg(CHAIN_ID_OPT.def().about("The chain ID."))
                .arg(BASE_DIR.def().about(
                    "The base directory is where the nodes, client and wallet \
                     configuration and state is stored. This value can also \
                     be set via `ANOMA_BASE_DIR` environment variable, but \
                     the argument takes precedence, if specified. Defaults to \
                     `.anoma`.",
                ))
                .arg(WASM_DIR.def().about(
                    "Directory with built WASM validity predicates, \
                     transactions. This value can also be set via \
                     `ANOMA_WASM_DIR` environment variable, but the argument \
                     takes precedence, if specified.",
                ))
                .arg(MODE.def().about(
                    "The mode in which to run Anoma. Options are \n\t * \
                     Validator (default)\n\t * Full\n\t * Seed",
                ))
        }
    }

    /// Transaction associated results arguments
    #[derive(Clone, Debug)]
    pub struct QueryResult {
        /// Common query args
        pub query: Query,
        /// Hash of transaction to lookup
        pub tx_hash: String,
    }

    impl Args for QueryResult {
        fn parse(matches: &ArgMatches) -> Self {
            let query = Query::parse(matches);
            let tx_hash = TX_HASH.parse(matches);
            Self { query, tx_hash }
        }

        fn def(app: App) -> App {
            app.add_args::<Query>().arg(
                TX_HASH
                    .def()
                    .about("The hash of the transaction being looked up."),
            )
        }
    }

    /// Custom transaction arguments
    #[derive(Clone, Debug)]
    pub struct TxCustom {
        /// Common tx arguments
        pub tx: Tx,
        /// Path to the tx WASM code file
        pub code_path: PathBuf,
        /// Path to the data file
        pub data_path: Option<PathBuf>,
    }

    impl Args for TxCustom {
        fn parse(matches: &ArgMatches) -> Self {
            let tx = Tx::parse(matches);
            let code_path = CODE_PATH.parse(matches);
            let data_path = DATA_PATH_OPT.parse(matches);
            Self {
                tx,
                code_path,
                data_path,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Tx>()
                .arg(
                    CODE_PATH
                        .def()
                        .about("The path to the transaction's WASM code."),
                )
                .arg(DATA_PATH_OPT.def().about(
                    "The data file at this path containing arbitrary bytes \
                     will be passed to the transaction code when it's \
                     executed.",
                ))
        }
    }

    /// Transfer transaction arguments
    #[derive(Clone, Debug)]
    pub struct TxTransfer {
        /// Common tx arguments
        pub tx: Tx,
        /// Transfer source address
        pub source: WalletAddress,
        /// Transfer target address
        pub target: WalletAddress,
        /// Transferred token address
        pub token: WalletAddress,
        /// Transferred token address
        pub sub_prefix: Option<String>,
        /// Transferred token amount
        pub amount: token::Amount,
    }

    impl Args for TxTransfer {
        fn parse(matches: &ArgMatches) -> Self {
            let tx = Tx::parse(matches);
            let source = SOURCE.parse(matches);
            let target = TARGET.parse(matches);
            let token = TOKEN.parse(matches);
            let sub_prefix = SUB_PREFIX.parse(matches);
            let amount = AMOUNT.parse(matches);
            Self {
                tx,
                source,
                target,
                token,
                sub_prefix,
                amount,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Tx>()
                .arg(SOURCE.def().about(
                    "The source account address. The source's key is used to \
                     produce the signature.",
                ))
                .arg(TARGET.def().about("The target account address."))
                .arg(TOKEN.def().about("The transfer token."))
                .arg(SUB_PREFIX.def().about("The token's sub prefix."))
                .arg(AMOUNT.def().about("The amount to transfer in decimal."))
        }
    }

    /// Transaction to initialize a new account
    #[derive(Clone, Debug)]
    pub struct TxInitAccount {
        /// Common tx arguments
        pub tx: Tx,
        /// Address of the source account
        pub source: WalletAddress,
        /// Path to the VP WASM code file for the new account
        pub vp_code_path: Option<PathBuf>,
        /// Public key for the new account
        pub public_key: WalletPublicKey,
    }

    impl Args for TxInitAccount {
        fn parse(matches: &ArgMatches) -> Self {
            let tx = Tx::parse(matches);
            let source = SOURCE.parse(matches);
            let vp_code_path = CODE_PATH_OPT.parse(matches);
            let public_key = PUBLIC_KEY.parse(matches);
            Self {
                tx,
                source,
                vp_code_path,
                public_key,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Tx>()
                .arg(SOURCE.def().about(
                    "The source account's address that signs the transaction.",
                ))
                .arg(CODE_PATH_OPT.def().about(
                    "The path to the validity predicate WASM code to be used \
                     for the new account. Uses the default user VP if none \
                     specified.",
                ))
                .arg(PUBLIC_KEY.def().about(
                    "A public key to be used for the new account in \
                     hexadecimal encoding.",
                ))
        }
    }

    /// Transaction to initialize a new account
    #[derive(Clone, Debug)]
    pub struct TxInitValidator {
        pub tx: Tx,
        pub source: WalletAddress,
        pub scheme: SchemeType,
        pub account_key: Option<WalletPublicKey>,
        pub consensus_key: Option<WalletKeypair>,
        pub protocol_key: Option<WalletPublicKey>,
        pub commission_rate: Decimal,
        pub max_commission_rate_change: Decimal,
        pub validator_vp_code_path: Option<PathBuf>,
        pub unsafe_dont_encrypt: bool,
    }

    impl Args for TxInitValidator {
        fn parse(matches: &ArgMatches) -> Self {
            let tx = Tx::parse(matches);
            let source = SOURCE.parse(matches);
            let scheme = SCHEME.parse(matches);
            let account_key = VALIDATOR_ACCOUNT_KEY.parse(matches);
            let consensus_key = VALIDATOR_CONSENSUS_KEY.parse(matches);
            let protocol_key = PROTOCOL_KEY.parse(matches);
            let commission_rate = COMMISSION_RATE.parse(matches);
            let max_commission_rate_change =
                MAX_COMMISSION_RATE_CHANGE.parse(matches);
            let validator_vp_code_path = VALIDATOR_CODE_PATH.parse(matches);
            let unsafe_dont_encrypt = UNSAFE_DONT_ENCRYPT.parse(matches);
            Self {
                tx,
                source,
                scheme,
                account_key,
                consensus_key,
                protocol_key,
                commission_rate,
                max_commission_rate_change,
                validator_vp_code_path,
                unsafe_dont_encrypt,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Tx>()
                .arg(SOURCE.def().about(
                    "The source account's address that signs the transaction.",
                ))
                .arg(SCHEME.def().about(
                    "The key scheme/type used for the validator keys. \
                     Currently supports ed25519 and secp256k1.",
                ))
                .arg(VALIDATOR_ACCOUNT_KEY.def().about(
                    "A public key for the validator account. A new one will \
                     be generated if none given.",
                ))
                .arg(VALIDATOR_CONSENSUS_KEY.def().about(
                    "A consensus key for the validator account. A new one \
                     will be generated if none given.",
                ))
                .arg(PROTOCOL_KEY.def().about(
                    "A public key for signing protocol transactions. A new \
                     one will be generated if none given.",
                ))
                .arg(COMMISSION_RATE.def().about(
                    "The commission rate charged by the validator for \
                     delegation rewards. This is a required parameter.",
                ))
                .arg(MAX_COMMISSION_RATE_CHANGE.def().about(
                    "The maximum change per epoch in the commission rate \
                     charged by the validator for delegation rewards. This is \
                     a required parameter.",
                ))
                .arg(VALIDATOR_CODE_PATH.def().about(
                    "The path to the validity predicate WASM code to be used \
                     for the validator account. Uses the default validator VP \
                     if none specified.",
                ))
                .arg(UNSAFE_DONT_ENCRYPT.def().about(
                    "UNSAFE: Do not encrypt the generated keypairs. Do not \
                     use this for keys used in a live network.",
                ))
        }
    }

    /// Transaction to update a VP arguments
    #[derive(Clone, Debug)]
    pub struct TxUpdateVp {
        /// Common tx arguments
        pub tx: Tx,
        /// Path to the VP WASM code file
        pub vp_code_path: PathBuf,
        /// Address of the account whose VP is to be updated
        pub addr: WalletAddress,
    }

    impl Args for TxUpdateVp {
        fn parse(matches: &ArgMatches) -> Self {
            let tx = Tx::parse(matches);
            let vp_code_path = CODE_PATH.parse(matches);
            let addr = ADDRESS.parse(matches);
            Self {
                tx,
                vp_code_path,
                addr,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Tx>()
                .arg(
                    CODE_PATH.def().about(
                        "The path to the new validity predicate WASM code.",
                    ),
                )
                .arg(ADDRESS.def().about(
                    "The account's address. It's key is used to produce the \
                     signature.",
                ))
        }
    }

    /// Bond arguments
    #[derive(Clone, Debug)]
    pub struct Bond {
        /// Common tx arguments
        pub tx: Tx,
        /// Validator address
        pub validator: WalletAddress,
        /// Amount of tokens to stake in a bond
        pub amount: token::Amount,
        /// Source address for delegations. For self-bonds, the validator is
        /// also the source.
        pub source: Option<WalletAddress>,
    }

    impl Args for Bond {
        fn parse(matches: &ArgMatches) -> Self {
            let tx = Tx::parse(matches);
            let validator = VALIDATOR.parse(matches);
            let amount = AMOUNT.parse(matches);
            let source = SOURCE_OPT.parse(matches);
            Self {
                tx,
                validator,
                amount,
                source,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Tx>()
                .arg(VALIDATOR.def().about("Validator address."))
                .arg(AMOUNT.def().about("Amount of tokens to stake in a bond."))
                .arg(SOURCE_OPT.def().about(
                    "Source address for delegations. For self-bonds, the \
                     validator is also the source.",
                ))
        }
    }

    /// Unbond arguments
    #[derive(Clone, Debug)]
    pub struct Unbond {
        /// Common tx arguments
        pub tx: Tx,
        /// Validator address
        pub validator: WalletAddress,
        /// Amount of tokens to unbond from a bond
        pub amount: token::Amount,
        /// Source address for unbonding from delegations. For unbonding from
        /// self-bonds, the validator is also the source
        pub source: Option<WalletAddress>,
    }

    impl Args for Unbond {
        fn parse(matches: &ArgMatches) -> Self {
            let tx = Tx::parse(matches);
            let validator = VALIDATOR.parse(matches);
            let amount = AMOUNT.parse(matches);
            let source = SOURCE_OPT.parse(matches);
            Self {
                tx,
                validator,
                amount,
                source,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Tx>()
                .arg(VALIDATOR.def().about("Validator address."))
                .arg(
                    AMOUNT
                        .def()
                        .about("Amount of tokens to unbond from a bond."),
                )
                .arg(SOURCE_OPT.def().about(
                    "Source address for unbonding from delegations. For \
                     unbonding from self-bonds, the validator is also the \
                     source.",
                ))
        }
    }

    #[derive(Clone, Debug)]
    pub struct InitProposal {
        /// Common tx arguments
        pub tx: Tx,
        /// The proposal file path
        pub proposal_data: PathBuf,
        /// Flag if proposal should be run offline
        pub offline: bool,
    }

    impl Args for InitProposal {
        fn parse(matches: &ArgMatches) -> Self {
            let tx = Tx::parse(matches);
            let proposal_data = DATA_PATH.parse(matches);
            let offline = PROPOSAL_OFFLINE.parse(matches);

            Self {
                tx,
                proposal_data,
                offline,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Tx>()
                .arg(DATA_PATH.def().about(
                    "The data path file (json) that describes the proposal.",
                ))
                .arg(
                    PROPOSAL_OFFLINE
                        .def()
                        .about("Flag if the proposal vote should run offline."),
                )
        }
    }

    #[derive(Clone, Debug)]
    pub struct VoteProposal {
        /// Common tx arguments
        pub tx: Tx,
        /// Proposal id
        pub proposal_id: Option<u64>,
        /// The vote
        pub vote: ProposalVote,
        /// Flag if proposal vote should be run offline
        pub offline: bool,
        /// The proposal file path
        pub proposal_data: Option<PathBuf>,
    }

    impl Args for VoteProposal {
        fn parse(matches: &ArgMatches) -> Self {
            let tx = Tx::parse(matches);
            let proposal_id = PROPOSAL_ID_OPT.parse(matches);
            let vote = PROPOSAL_VOTE.parse(matches);
            let offline = PROPOSAL_OFFLINE.parse(matches);
            let proposal_data = DATA_PATH_OPT.parse(matches);

            Self {
                tx,
                proposal_id,
                vote,
                offline,
                proposal_data,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Tx>()
                .arg(
                    PROPOSAL_ID_OPT
                        .def()
                        .about("The proposal identifier.")
                        .conflicts_with_all(&[
                            PROPOSAL_OFFLINE.name,
                            DATA_PATH_OPT.name,
                        ]),
                )
                .arg(
                    PROPOSAL_VOTE
                        .def()
                        .about("The vote for the proposal. Either yay or nay."),
                )
                .arg(
                    PROPOSAL_OFFLINE
                        .def()
                        .about("Flag if the proposal vote should run offline.")
                        .conflicts_with(PROPOSAL_ID.name),
                )
                .arg(
                    DATA_PATH_OPT
                        .def()
                        .about(
                            "The data path file (json) that describes the \
                             proposal.",
                        )
                        .conflicts_with(PROPOSAL_ID.name),
                )
        }
    }

    #[derive(Clone, Debug)]
    pub struct QueryProposal {
        /// Common query args
        pub query: Query,
        /// Proposal id
        pub proposal_id: Option<u64>,
    }

    impl Args for QueryProposal {
        fn parse(matches: &ArgMatches) -> Self {
            let query = Query::parse(matches);
            let proposal_id = PROPOSAL_ID_OPT.parse(matches);

            Self { query, proposal_id }
        }

        fn def(app: App) -> App {
            app.add_args::<Tx>()
                .arg(PROPOSAL_ID_OPT.def().about("The proposal identifier."))
        }
    }

    #[derive(Clone, Debug)]
    pub struct QueryProposalResult {
        /// Common query args
        pub query: Query,
        /// Proposal id
        pub proposal_id: Option<u64>,
        /// Flag if proposal result should be run on offline data
        pub offline: bool,
        /// The folder containing the proposal and votes
        pub proposal_folder: Option<PathBuf>,
    }

    impl Args for QueryProposalResult {
        fn parse(matches: &ArgMatches) -> Self {
            let query = Query::parse(matches);
            let proposal_id = PROPOSAL_ID_OPT.parse(matches);
            let offline = PROPOSAL_OFFLINE.parse(matches);
            let proposal_folder = DATA_PATH_OPT.parse(matches);

            Self {
                query,
                proposal_id,
                offline,
                proposal_folder,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Query>()
                .arg(PROPOSAL_ID_OPT.def().about("The proposal identifier."))
                .arg(
                    PROPOSAL_OFFLINE
                        .def()
                        .about(
                            "Flag if the proposal result should run on \
                             offline data.",
                        )
                        .conflicts_with(PROPOSAL_ID.name),
                )
                .arg(
                    DATA_PATH_OPT
                        .def()
                        .about(
                            "The path to the folder containing the proposal \
                             json and votes",
                        )
                        .conflicts_with(PROPOSAL_ID.name),
                )
        }
    }

    #[derive(Clone, Debug)]
    pub struct QueryProtocolParameters {
        /// Common query args
        pub query: Query,
    }

    impl Args for QueryProtocolParameters {
        fn parse(matches: &ArgMatches) -> Self {
            let query = Query::parse(matches);

            Self { query }
        }

        fn def(app: App) -> App {
            app.add_args::<Query>()
        }
    }

    /// Withdraw arguments
    #[derive(Clone, Debug)]
    pub struct Withdraw {
        /// Common tx arguments
        pub tx: Tx,
        /// Validator address
        pub validator: WalletAddress,
        /// Source address for withdrawing from delegations. For withdrawing
        /// from self-bonds, the validator is also the source
        pub source: Option<WalletAddress>,
    }

    impl Args for Withdraw {
        fn parse(matches: &ArgMatches) -> Self {
            let tx = Tx::parse(matches);
            let validator = VALIDATOR.parse(matches);
            let source = SOURCE_OPT.parse(matches);
            Self {
                tx,
                validator,
                source,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Tx>()
                .arg(VALIDATOR.def().about("Validator address."))
                .arg(SOURCE_OPT.def().about(
                    "Source address for withdrawing from delegations. For \
                     withdrawing from self-bonds, the validator is also the \
                     source.",
                ))
        }
    }

    /// Query token balance(s)
    #[derive(Clone, Debug)]
    pub struct QueryBalance {
        /// Common query args
        pub query: Query,
        /// Address of an owner
        pub owner: Option<WalletAddress>,
        /// Address of a token
        pub token: Option<WalletAddress>,
        /// Sub prefix of an account
        pub sub_prefix: Option<String>,
    }

    impl Args for QueryBalance {
        fn parse(matches: &ArgMatches) -> Self {
            let query = Query::parse(matches);
            let owner = OWNER.parse(matches);
            let token = TOKEN_OPT.parse(matches);
            let sub_prefix = SUB_PREFIX.parse(matches);
            Self {
                query,
                owner,
                token,
                sub_prefix,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Query>()
                .arg(
                    OWNER
                        .def()
                        .about("The account address whose balance to query."),
                )
                .arg(
                    TOKEN_OPT
                        .def()
                        .about("The token's address whose balance to query."),
                )
                .arg(
                    SUB_PREFIX.def().about(
                        "The token's sub prefix whose balance to query.",
                    ),
                )
        }
    }

    /// Query PoS bond(s)
    #[derive(Clone, Debug)]
    pub struct QueryBonds {
        /// Common query args
        pub query: Query,
        /// Address of an owner
        pub owner: Option<WalletAddress>,
        /// Address of a validator
        pub validator: Option<WalletAddress>,
    }

    impl Args for QueryBonds {
        fn parse(matches: &ArgMatches) -> Self {
            let query = Query::parse(matches);
            let owner = OWNER.parse(matches);
            let validator = VALIDATOR_OPT.parse(matches);
            Self {
                query,
                owner,
                validator,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Query>()
                .arg(
                    OWNER.def().about(
                        "The owner account address whose bonds to query.",
                    ),
                )
                .arg(
                    VALIDATOR_OPT
                        .def()
                        .about("The validator's address whose bonds to query."),
                )
        }
    }

    /// Query PoS voting power
    #[derive(Clone, Debug)]
    pub struct QueryVotingPower {
        /// Common query args
        pub query: Query,
        /// Address of a validator
        pub validator: Option<WalletAddress>,
        /// Epoch in which to find voting power
        pub epoch: Option<Epoch>,
    }

    impl Args for QueryVotingPower {
        fn parse(matches: &ArgMatches) -> Self {
            let query = Query::parse(matches);
            let validator = VALIDATOR_OPT.parse(matches);
            let epoch = EPOCH.parse(matches);
            Self {
                query,
                validator,
                epoch,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Query>()
                .arg(VALIDATOR_OPT.def().about(
                    "The validator's address whose voting power to query.",
                ))
                .arg(EPOCH.def().about(
                    "The epoch at which to query (last committed, if not \
                     specified).",
                ))
        }
    }

    /// Query PoS commission rate
    #[derive(Clone, Debug)]
    pub struct QueryCommissionRate {
        /// Common query args
        pub query: Query,
        /// Address of a validator
        pub validator: Option<WalletAddress>,
        /// Epoch in which to find commission rate
        pub epoch: Option<Epoch>,
    }

    impl Args for QueryCommissionRate {
        fn parse(matches: &ArgMatches) -> Self {
            let query = Query::parse(matches);
            let validator = VALIDATOR_OPT.parse(matches);
            let epoch = EPOCH.parse(matches);
            Self {
                query,
                validator,
                epoch,
            }
        }

        fn def(app: App) -> App {
            app.add_args::<Query>()
                .arg(VALIDATOR_OPT.def().about(
                    "The validator's address whose commission rate to query.",
                ))
                .arg(EPOCH.def().about(
                    "The epoch at which to query (last committed, if not \
                     specified).",
                ))
        }
    }

    /// Query PoS slashes
    #[derive(Clone, Debug)]
    pub struct QuerySlashes {
        /// Common query args
        pub query: Query,
        /// Address of a validator
        pub validator: Option<WalletAddress>,
    }

    impl Args for QuerySlashes {
        fn parse(matches: &ArgMatches) -> Self {
            let query = Query::parse(matches);
            let validator = VALIDATOR_OPT.parse(matches);
            Self { query, validator }
        }

        fn def(app: App) -> App {
            app.add_args::<Query>().arg(
                VALIDATOR_OPT
                    .def()
                    .about("The validator's address whose slashes to query."),
            )
        }
    }
    /// Query the raw bytes of given storage key
    #[derive(Clone, Debug)]
    pub struct QueryRawBytes {
        /// The storage key to query
        pub storage_key: storage::Key,
        /// Common query args
        pub query: Query,
    }

    impl Args for QueryRawBytes {
        fn parse(matches: &ArgMatches) -> Self {
            let storage_key = STORAGE_KEY.parse(matches);
            let query = Query::parse(matches);
            Self { storage_key, query }
        }

        fn def(app: App) -> App {
            app.add_args::<Query>()
                .arg(STORAGE_KEY.def().about("Storage key"))
        }
    }
    /// Common transaction arguments
    #[derive(Clone, Debug)]
    pub struct Tx {
        /// Simulate applying the transaction
        pub dry_run: bool,
        /// Submit the transaction even if it doesn't pass client checks
        pub force: bool,
        /// Do not wait for the transaction to be added to the blockchain
        pub broadcast_only: bool,
        /// The address of the ledger node as host:port
        pub ledger_address: TendermintAddress,
        /// If any new account is initialized by the tx, use the given alias to
        /// save it in the wallet.
        pub initialized_account_alias: Option<String>,
        /// The amount being payed to include the transaction
        pub fee_amount: token::Amount,
        /// The token in which the fee is being paid
        pub fee_token: WalletAddress,
        /// The max amount of gas used to process tx
        pub gas_limit: GasLimit,
        /// Sign the tx with the key for the given alias from your wallet
        pub signing_key: Option<WalletKeypair>,
        /// Sign the tx with the keypair of the public key of the given address
        pub signer: Option<WalletAddress>,
    }

    impl Args for Tx {
        fn def(app: App) -> App {
            app.arg(
                DRY_RUN_TX
                    .def()
                    .about("Simulate the transaction application."),
            )
            .arg(FORCE.def().about(
                "Submit the transaction even if it doesn't pass client checks.",
            ))
            .arg(BROADCAST_ONLY.def().about(
                "Do not wait for the transaction to be applied. This will \
                 return once the transaction is added to the mempool.",
            ))
            .arg(LEDGER_ADDRESS_DEFAULT.def().about(LEDGER_ADDRESS_ABOUT))
            .arg(ALIAS_OPT.def().about(
                "If any new account is initialized by the tx, use the given \
                 alias to save it in the wallet. If multiple accounts are \
                 initialized, the alias will be the prefix of each new \
                 address joined with a number.",
            ))
            .arg(FEE_AMOUNT.def().about(
                "The amount being paid for the inclusion of this transaction",
            ))
            .arg(FEE_TOKEN.def().about("The token for paying the fee"))
            .arg(
                GAS_LIMIT.def().about(
                    "The maximum amount of gas needed to run transaction",
                ),
            )
            .arg(
                SIGNING_KEY_OPT
                    .def()
                    .about(
                        "Sign the transaction with the key for the given \
                         public key, public key hash or alias from your \
                         wallet.",
                    )
                    .conflicts_with(SIGNER.name),
            )
            .arg(
                SIGNER
                    .def()
                    .about(
                        "Sign the transaction with the keypair of the public \
                         key of the given address.",
                    )
                    .conflicts_with(SIGNING_KEY_OPT.name),
            )
        }

        fn parse(matches: &ArgMatches) -> Self {
            let dry_run = DRY_RUN_TX.parse(matches);
            let force = FORCE.parse(matches);
            let broadcast_only = BROADCAST_ONLY.parse(matches);
            let ledger_address = LEDGER_ADDRESS_DEFAULT.parse(matches);
            let initialized_account_alias = ALIAS_OPT.parse(matches);
            let fee_amount = FEE_AMOUNT.parse(matches);
            let fee_token = FEE_TOKEN.parse(matches);
            let gas_limit = GAS_LIMIT.parse(matches).into();

            let signing_key = SIGNING_KEY_OPT.parse(matches);
            let signer = SIGNER.parse(matches);
            Self {
                dry_run,
                force,
                broadcast_only,
                ledger_address,
                initialized_account_alias,
                fee_amount,
                fee_token,
                gas_limit,
                signing_key,
                signer,
            }
        }
    }

    /// Common query arguments
    #[derive(Clone, Debug)]
    pub struct Query {
        /// The address of the ledger node as host:port
        pub ledger_address: TendermintAddress,
    }

    impl Args for Query {
        fn def(app: App) -> App {
            app.arg(LEDGER_ADDRESS_DEFAULT.def().about(LEDGER_ADDRESS_ABOUT))
        }

        fn parse(matches: &ArgMatches) -> Self {
            let ledger_address = LEDGER_ADDRESS_DEFAULT.parse(matches);
            Self { ledger_address }
        }
    }

    /// Wallet generate key and implicit address arguments
    #[derive(Clone, Debug)]
    pub struct KeyAndAddressGen {
        /// Scheme type
        pub scheme: SchemeType,
        /// Key alias
        pub alias: Option<String>,
        /// Don't encrypt the keypair
        pub unsafe_dont_encrypt: bool,
    }

    impl Args for KeyAndAddressGen {
        fn parse(matches: &ArgMatches) -> Self {
            let scheme = SCHEME.parse(matches);
            let alias = ALIAS_OPT.parse(matches);
            let unsafe_dont_encrypt = UNSAFE_DONT_ENCRYPT.parse(matches);
            Self {
                scheme,
                alias,
                unsafe_dont_encrypt,
            }
        }

        fn def(app: App) -> App {
            app.arg(SCHEME.def().about(
                "The type of key that should be generated. Argument must be \
                 either ed25519 or secp256k1. If none provided, the default \
                 key scheme is ed25519.",
            ))
            .arg(ALIAS_OPT.def().about(
                "The key and address alias. If none provided, the alias will \
                 be the public key hash.",
            ))
            .arg(UNSAFE_DONT_ENCRYPT.def().about(
                "UNSAFE: Do not encrypt the keypair. Do not use this for keys \
                 used in a live network.",
            ))
        }
    }

    /// Wallet key lookup arguments
    #[derive(Clone, Debug)]
    pub struct KeyFind {
        pub public_key: Option<common::PublicKey>,
        pub alias: Option<String>,
        pub value: Option<String>,
        pub unsafe_show_secret: bool,
    }

    impl Args for KeyFind {
        fn parse(matches: &ArgMatches) -> Self {
            let public_key = RAW_PUBLIC_KEY_OPT.parse(matches);
            let alias = ALIAS_OPT.parse(matches);
            let value = VALUE.parse(matches);
            let unsafe_show_secret = UNSAFE_SHOW_SECRET.parse(matches);

            Self {
                public_key,
                alias,
                value,
                unsafe_show_secret,
            }
        }

        fn def(app: App) -> App {
            app.arg(
                RAW_PUBLIC_KEY_OPT
                    .def()
                    .about("A public key associated with the keypair.")
                    .conflicts_with_all(&[ALIAS_OPT.name, VALUE.name]),
            )
            .arg(
                ALIAS_OPT
                    .def()
                    .about("An alias associated with the keypair.")
                    .conflicts_with(VALUE.name),
            )
            .arg(
                VALUE.def().about(
                    "A public key or alias associated with the keypair.",
                ),
            )
            .arg(
                UNSAFE_SHOW_SECRET
                    .def()
                    .about("UNSAFE: Print the secret key."),
            )
        }
    }

    /// Wallet list keys arguments
    #[derive(Clone, Debug)]
    pub struct KeyList {
        pub decrypt: bool,
        pub unsafe_show_secret: bool,
    }

    impl Args for KeyList {
        fn parse(matches: &ArgMatches) -> Self {
            let decrypt = DECRYPT.parse(matches);
            let unsafe_show_secret = UNSAFE_SHOW_SECRET.parse(matches);
            Self {
                decrypt,
                unsafe_show_secret,
            }
        }

        fn def(app: App) -> App {
            app.arg(DECRYPT.def().about("Decrypt keys that are encrypted."))
                .arg(
                    UNSAFE_SHOW_SECRET
                        .def()
                        .about("UNSAFE: Print the secret keys."),
                )
        }
    }

    /// Wallet key export arguments
    #[derive(Clone, Debug)]
    pub struct KeyExport {
        pub alias: String,
    }

    impl Args for KeyExport {
        fn parse(matches: &ArgMatches) -> Self {
            let alias = ALIAS.parse(matches);

            Self { alias }
        }

        fn def(app: App) -> App {
            app.arg(
                ALIAS
                    .def()
                    .about("The alias of the key you wish to export."),
            )
        }
    }

    /// Wallet address lookup arguments
    #[derive(Clone, Debug)]
    pub struct AddressOrAliasFind {
        pub alias: Option<String>,
        pub address: Option<Address>,
    }

    impl Args for AddressOrAliasFind {
        fn parse(matches: &ArgMatches) -> Self {
            let alias = ALIAS_OPT.parse(matches);
            let address = RAW_ADDRESS_OPT.parse(matches);
            Self { alias, address }
        }

        fn def(app: App) -> App {
            app.arg(
                ALIAS_OPT
                    .def()
                    .about("An alias associated with the address."),
            )
            .arg(
                RAW_ADDRESS_OPT
                    .def()
                    .about("The bech32m encoded address string."),
            )
            .group(
                ArgGroup::new("find_flags")
                    .args(&[ALIAS_OPT.name, RAW_ADDRESS_OPT.name])
                    .required(true),
            )
        }
    }

    /// Wallet address add arguments
    #[derive(Clone, Debug)]
    pub struct AddressAdd {
        pub alias: String,
        pub address: Address,
    }

    impl Args for AddressAdd {
        fn parse(matches: &ArgMatches) -> Self {
            let alias = ALIAS.parse(matches);
            let address = RAW_ADDRESS.parse(matches);
            Self { alias, address }
        }

        fn def(app: App) -> App {
            app.arg(
                ALIAS
                    .def()
                    .about("An alias to be associated with the address."),
            )
            .arg(
                RAW_ADDRESS
                    .def()
                    .about("The bech32m encoded address string."),
            )
        }
    }

    #[derive(Clone, Debug)]
    pub struct JoinNetwork {
        pub chain_id: ChainId,
        pub genesis_validator: Option<String>,
        pub pre_genesis_path: Option<PathBuf>,
        pub dont_prefetch_wasm: bool,
    }

    impl Args for JoinNetwork {
        fn parse(matches: &ArgMatches) -> Self {
            let chain_id = CHAIN_ID.parse(matches);
            let genesis_validator = GENESIS_VALIDATOR.parse(matches);
            let pre_genesis_path = PRE_GENESIS_PATH.parse(matches);
            let dont_prefetch_wasm = DONT_PREFETCH_WASM.parse(matches);
            Self {
                chain_id,
                genesis_validator,
                pre_genesis_path,
                dont_prefetch_wasm,
            }
        }

        fn def(app: App) -> App {
            app.arg(CHAIN_ID.def().about("The chain ID. The chain must be known in the https://github.com/heliaxdev/anoma-network-config repository."))
                .arg(GENESIS_VALIDATOR.def().about("The alias of the genesis validator that you want to set up as, if any."))
                .arg(PRE_GENESIS_PATH.def().about("The path to the pre-genesis directory for genesis validator, if any. Defaults to \"{base-dir}/pre-genesis/{genesis-validator}\"."))
            .arg(DONT_PREFETCH_WASM.def().about(
                "Do not pre-fetch WASM.",
            ))
        }
    }

    #[derive(Clone, Debug)]
    pub struct FetchWasms {
        pub chain_id: ChainId,
    }

    impl Args for FetchWasms {
        fn parse(matches: &ArgMatches) -> Self {
            let chain_id = CHAIN_ID.parse(matches);
            Self { chain_id }
        }

        fn def(app: App) -> App {
            app.arg(CHAIN_ID.def().about("The chain ID. The chain must be known in the https://github.com/heliaxdev/anoma-network-config repository, in which case it should have pre-built wasms available for download."))
        }
    }

    #[derive(Clone, Debug)]
    pub struct InitNetwork {
        pub genesis_path: PathBuf,
        pub wasm_checksums_path: PathBuf,
        pub chain_id_prefix: ChainIdPrefix,
        pub unsafe_dont_encrypt: bool,
        pub consensus_timeout_commit: Timeout,
        pub localhost: bool,
        pub allow_duplicate_ip: bool,
        pub dont_archive: bool,
        pub archive_dir: Option<PathBuf>,
    }

    impl Args for InitNetwork {
        fn parse(matches: &ArgMatches) -> Self {
            let genesis_path = GENESIS_PATH.parse(matches);
            let wasm_checksums_path = WASM_CHECKSUMS_PATH.parse(matches);
            let chain_id_prefix = CHAIN_ID_PREFIX.parse(matches);
            let unsafe_dont_encrypt = UNSAFE_DONT_ENCRYPT.parse(matches);
            let consensus_timeout_commit =
                CONSENSUS_TIMEOUT_COMMIT.parse(matches);
            let localhost = LOCALHOST.parse(matches);
            let allow_duplicate_ip = ALLOW_DUPLICATE_IP.parse(matches);
            let dont_archive = DONT_ARCHIVE.parse(matches);
            let archive_dir = ARCHIVE_DIR.parse(matches);
            Self {
                genesis_path,
                wasm_checksums_path,
                chain_id_prefix,
                unsafe_dont_encrypt,
                consensus_timeout_commit,
                localhost,
                allow_duplicate_ip,
                dont_archive,
                archive_dir,
            }
        }

        fn def(app: App) -> App {
            app.arg(
                GENESIS_PATH.def().about(
                    "Path to the preliminary genesis configuration file.",
                ),
            )
            .arg(
                WASM_CHECKSUMS_PATH
                    .def()
                    .about("Path to the WASM checksums file."),
            )
            .arg(CHAIN_ID_PREFIX.def().about(
                "The chain ID prefix. Up to 19 alphanumeric, '.', '-' or '_' \
                 characters.",
            ))
            .arg(UNSAFE_DONT_ENCRYPT.def().about(
                "UNSAFE: Do not encrypt the generated keypairs. Do not use \
                 this for keys used in a live network.",
            ))
            .arg(CONSENSUS_TIMEOUT_COMMIT.def().about(
                "The Tendermint consensus timeout_commit configuration as \
                 e.g. `1s` or `1000ms`. Defaults to 10 seconds.",
            ))
            .arg(LOCALHOST.def().about(
                "Use localhost address for P2P and RPC connections for the \
                 validators ledger",
            ))
            .arg(ALLOW_DUPLICATE_IP.def().about(
                "Toggle to disable guard against peers connecting from the \
                 same IP. This option shouldn't be used in mainnet.",
            ))
            .arg(
                DONT_ARCHIVE
                    .def()
                    .about("Do NOT create the release archive."),
            )
            .arg(ARCHIVE_DIR.def().about(
                "Specify a directory into which to store the archive. Default \
                 is the current working directory.",
            ))
        }
    }

    #[derive(Clone, Debug)]
    pub struct InitGenesisValidator {
        pub alias: String,
        pub commission_rate: Decimal,
        pub max_commission_rate_change: Decimal,
        pub net_address: SocketAddr,
        pub unsafe_dont_encrypt: bool,
        pub key_scheme: SchemeType,
    }

    impl Args for InitGenesisValidator {
        fn parse(matches: &ArgMatches) -> Self {
            let alias = ALIAS.parse(matches);
            let commission_rate = COMMISSION_RATE.parse(matches);
            let max_commission_rate_change =
                MAX_COMMISSION_RATE_CHANGE.parse(matches);
            let net_address = NET_ADDRESS.parse(matches);
            let unsafe_dont_encrypt = UNSAFE_DONT_ENCRYPT.parse(matches);
            let key_scheme = SCHEME.parse(matches);
            Self {
                alias,
                net_address,
                unsafe_dont_encrypt,
                key_scheme,
                commission_rate,
                max_commission_rate_change,
            }
        }

        fn def(app: App) -> App {
            app.arg(ALIAS.def().about("The validator address alias."))
                .arg(NET_ADDRESS.def().about(
                    "Static {host:port} of your validator node's P2P address. \
                     Anoma uses port `26656` for P2P connections by default, \
                     but you can configure a different value.",
                ))
                .arg(COMMISSION_RATE.def().about(
                    "The commission rate charged by the validator for \
                     delegation rewards. This is a required parameter.",
                ))
                .arg(MAX_COMMISSION_RATE_CHANGE.def().about(
                    "The maximum change per epoch in the commission rate \
                     charged by the validator for delegation rewards. This is \
                     a required parameter.",
                ))
                .arg(UNSAFE_DONT_ENCRYPT.def().about(
                    "UNSAFE: Do not encrypt the generated keypairs. Do not \
                     use this for keys used in a live network.",
                ))
                .arg(SCHEME.def().about(
                    "The key scheme/type used for the validator keys. \
                     Currently supports ed25519 and secp256k1.",
                ))
        }
    }
}

pub fn anoma_cli() -> (cmds::Anoma, String) {
    let app = anoma_app();
    let matches = app.get_matches();
    let raw_sub_cmd =
        matches.subcommand().map(|(raw, _matches)| raw.to_string());
    let result = cmds::Anoma::parse(&matches);
    match (result, raw_sub_cmd) {
        (Some(cmd), Some(raw_sub)) => return (cmd, raw_sub),
        _ => {
            anoma_app().print_help().unwrap();
        }
    }
    safe_exit(2);
}

pub fn anoma_node_cli() -> Result<(cmds::AnomaNode, Context)> {
    let app = anoma_node_app();
    cmds::AnomaNode::parse_or_print_help(app)
}

pub enum AnomaClient {
    WithoutContext(cmds::Utils, args::Global),
    WithContext(Box<(cmds::AnomaClientWithContext, Context)>),
}

pub fn anoma_client_cli() -> Result<AnomaClient> {
    let app = anoma_client_app();
    let mut app = cmds::AnomaClient::add_sub(app);
    let matches = app.clone().get_matches();
    match Cmd::parse(&matches) {
        Some(cmd) => {
            let global_args = args::Global::parse(&matches);
            match cmd {
                cmds::AnomaClient::WithContext(sub_cmd) => {
                    let context = Context::new(global_args)?;
                    Ok(AnomaClient::WithContext(Box::new((sub_cmd, context))))
                }
                cmds::AnomaClient::WithoutContext(sub_cmd) => {
                    Ok(AnomaClient::WithoutContext(sub_cmd, global_args))
                }
            }
        }
        None => {
            app.print_help().unwrap();
            safe_exit(2);
        }
    }
}

pub fn anoma_wallet_cli() -> Result<(cmds::AnomaWallet, Context)> {
    let app = anoma_wallet_app();
    cmds::AnomaWallet::parse_or_print_help(app)
}

fn anoma_app() -> App {
    let app = App::new(APP_NAME)
        .version(anoma_version())
        .about("Anoma command line interface.")
        .setting(AppSettings::SubcommandRequiredElseHelp);
    cmds::Anoma::add_sub(args::Global::def(app))
}

fn anoma_node_app() -> App {
    let app = App::new(APP_NAME)
        .version(anoma_version())
        .about("Anoma node command line interface.")
        .setting(AppSettings::SubcommandRequiredElseHelp);
    cmds::AnomaNode::add_sub(args::Global::def(app))
}

fn anoma_client_app() -> App {
    let app = App::new(APP_NAME)
        .version(anoma_version())
        .about("Anoma client command line interface.")
        .setting(AppSettings::SubcommandRequiredElseHelp);
    cmds::AnomaClient::add_sub(args::Global::def(app))
}

fn anoma_wallet_app() -> App {
    let app = App::new(APP_NAME)
        .version(anoma_version())
        .about("Anoma wallet command line interface.")
        .setting(AppSettings::SubcommandRequiredElseHelp);
    cmds::AnomaWallet::add_sub(args::Global::def(app))
}
