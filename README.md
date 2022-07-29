This pallet mirrors [FRAME's node-authorization](https://github.com/paritytech/substrate/tree/master/frame/node-authorization) but adds a storage migration so that it can be added via runtime upgrade, after Genesis.

The code under `src/migration.rs` adds `Alice` and `Bob` as Well-Known nodes, and the code under `util/src/main.rs` has some prints to help get the PeerID and AccoundIds on raw byte formatting.

1. Add pallet to `runtime/Cargo.toml`:
```
[dependencies]
+pallet-node-authorization = { version = "4.0.0-dev", default-features = false, git = "https://github.com/bernardoaraujor/node-authorization" }
...

[features]
default = ["std"]
std = [
    ...
+    "pallet-node-authorization/std",
    ...
```
2. Add pallet to `runtime/src/lib.rs`:
```rust
+use frame_system::EnsureRoot;

+parameter_types! {
+    pub const MaxWellKnownNodes: u32 = 8;
+    pub const MaxPeerIdLength: u32 = 128;
+}

+impl pallet_node_authorization::Config for Runtime { 
+    type Event = Event;
+    type MaxWellKnownNodes = MaxWellKnownNodes;
+    type MaxPeerIdLength = MaxPeerIdLength;
+    type AddOrigin = EnsureRoot<AccountId>;
+    type RemoveOrigin = EnsureRoot<AccountId>;
+    type SwapOrigin = EnsureRoot<AccountId>;
+    type ResetOrigin = EnsureRoot<AccountId>;
+    type WeightInfo = ();
+}

...
construct_runtime!(
    ...
+    NodeAuthorization: pallet_node_authorization::{Pallet, Call, Storage, Event<T>, Config<T>},
)
```
3. Enable the storage migration on `Executive` of `runtime/src/lib.rs`:
```rust
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
+	pallet_node_authorization::migration::v1::MigrateToV1<Runtime>
>;
```
4. Increase the runtime version on `runtime/src/lib.rs`:
```rust
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("node-template"),
	impl_name: create_runtime_str!("node-template"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
-	spec_version: 100,
+	spec_version: 101,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};
```
5. Build runtime and perform runtime upgrade.