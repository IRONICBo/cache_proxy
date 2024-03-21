//! `DatenLord`

#![deny(
    // The following are allowed by default lints according to
    // https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html
    anonymous_parameters,
    bare_trait_objects,
    // box_pointers,
    // elided_lifetimes_in_paths, // allow anonymous lifetime
    // missing_copy_implementations, // Copy may cause unnecessary memory copy
    missing_debug_implementations,
    missing_docs, // TODO: add documents
    single_use_lifetimes, // TODO: fix lifetime names only used once
    trivial_casts, // TODO: remove trivial casts in code
    trivial_numeric_casts,
    // unreachable_pub, allow clippy::redundant_pub_crate lint instead
    // unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    // unused_results, // TODO: fix unused results
    variant_size_differences,

    warnings, // treat all wanings as errors

    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::cargo
)]
#![allow(
    // Some explicitly allowed Clippy lints, must have clear reason to allow
    clippy::blanket_clippy_restriction_lints, // allow clippy::restriction
    clippy::implicit_return, // actually omitting the return keyword is idiomatic Rust code
    clippy::module_name_repetitions, // repeation of module name in a struct name is not big deal
    clippy::multiple_crate_versions, // multi-version dependency crates is not able to fix
    clippy::panic, // allow debug_assert, panic in production code
    clippy::unreachable,  // Use `unreachable!` instead of `panic!` when impossible cases occurs
    // clippy::panic_in_result_fn,
    clippy::missing_errors_doc, // TODO: add error docs
    clippy::exhaustive_structs,
    clippy::exhaustive_enums,
    clippy::missing_panics_doc, // TODO: add panic docs
    clippy::panic_in_result_fn,
    clippy::single_char_lifetime_names,
    clippy::separated_literal_suffix, // conflict with unseparated_literal_suffix
    clippy::undocumented_unsafe_blocks, // TODO: add safety comment
    clippy::missing_safety_doc, // TODO: add safety comment
    clippy::shadow_unrelated, //it’s a common pattern in Rust code
    clippy::shadow_reuse, //it’s a common pattern in Rust code
    clippy::shadow_same, //it’s a common pattern in Rust code
    clippy::same_name_method, // Skip for protobuf generated code
    clippy::mod_module_files, // TODO: fix code structure to pass this lint
    clippy::std_instead_of_core, // Cause false positive in src/common/error.rs
    clippy::std_instead_of_alloc,
    clippy::pub_use, // TODO: fix this
    clippy::missing_trait_methods, // TODO: fix this
    clippy::arithmetic_side_effects, // TODO: fix this
    clippy::use_debug, // Allow debug print
    clippy::print_stdout, // Allow println!
    clippy::question_mark_used, // Allow ? operator
    clippy::absolute_paths,   // Allow use through absolute paths,like `std::env::current_dir`
    clippy::ref_patterns,    // Allow Some(ref x)
    clippy::single_call_fn,  // Allow function is called only once
    clippy::pub_with_shorthand,  // Allow pub(super)
    clippy::min_ident_chars,  // Allow Err(e)
    clippy::multiple_unsafe_ops_per_block, // Mainly caused by `etcd_delegate`, will remove later
    clippy::impl_trait_in_params,  // Allow impl AsRef<Path>, it's common in Rust
    clippy::missing_assert_message, // Allow assert! without message, mainly in test code
    clippy::semicolon_outside_block, // We need to choose between this and `semicolon_inside_block`, we choose outside
    clippy::similar_names, // Allow similar names, due to the existence of uid and gid
)]

use anyhow::Ok;
use client::ETCDClient;
use config::Config;
use manager::CacheProxyManager;

/// The proxy cache config
pub mod config;

/// The proxy cache manager
pub mod manager;

/// The Proxy cache slot
pub mod slot;

/// The Proxy cache ring
pub mod ring;

/// Meta data client
pub mod client;

/// Slot hashring node
pub mod node;

/// The file cache
pub mod file_cache;

/// RPC
pub mod rpc;

/// Hash Ring
pub mod hash_ring;

/// Proxy cache server
pub async fn start_cache_proxy(slot_size: usize, meta_type_string: &str, meta_endpoints: Vec<String>, time_period: usize, rpc_ip: String, rpc_port: u16) -> anyhow::Result<()> {
    // load config
    let config = Config::new(
        slot_size,
        meta_type_string,
        meta_endpoints,
        time_period,
        rpc_ip,
        rpc_port,
    );
    
    match config.meta_type {
        config::MetaType::ETCD => {
            // start topology manager
            let manager = CacheProxyManager::<ETCDClient>::new(config);

            // Start timer worker to fetch metadata
            let manager_worker = tokio::task::spawn(
                async move {
                    manager.start().await.unwrap_or_else(|e| {
                        panic!("Manager error: {:?}", e);
                    })
                }
            );

            manager_worker
                .await
                .unwrap_or_else(|e| {
                    panic!("Manager worker error: {:?}", e);
                });
        }
        config::MetaType::Redis => {
            // start redis client
            unimplemented!()
        }
    }

    Ok(())
}