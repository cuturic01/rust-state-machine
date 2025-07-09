use crate::support::Dispatch;

mod balances;
mod system;
mod support;
mod proof_of_existance;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
    pub type BlockNumber = u32;
	pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existance::Config for Runtime {
    type Content = types::Content;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[macros::runtime]
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existance: proof_of_existance::Pallet<Self>
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	runtime.balances.set_balance(&alice, 100);

    let block = types::Block {
	header: support::Header { block_number: 1 },
	extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: bob.clone(), amount: 30 }),
            },
            support::Extrinsic {
                caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: charlie.clone(), amount: 20 }),
            },
        ],
    };

    let block2 = types::Block {
	header: support::Header { block_number: 2 },
	extrinsics: vec![
            support::Extrinsic {
                caller: bob.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: charlie.clone(), amount: 30 }),
            },
            support::Extrinsic {
                caller: charlie.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: alice.clone(), amount: 20 }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existance(proof_of_existance::Call::create_claim { claim: "Hello alice." })
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existance(proof_of_existance::Call::revoke_claim { claim: "Hello." })
            }
        ],
    };

    runtime.execute_block(block).expect("Invalid block");
    runtime.execute_block(block2).expect("Invalid block");

	println!("{:#?}", runtime);
}