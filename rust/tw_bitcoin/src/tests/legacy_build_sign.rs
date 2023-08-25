#![allow(deprecated)]

use super::{ONE_BTC, hex};
use crate::modules::legacy::*;
use crate::modules::transactions::{BRC20TransferInscription, Brc20Ticker, OrdinalNftInscription};
use bitcoin::{PublicKey, ScriptBuf, PrivateKey};
use secp256k1::XOnlyPublicKey;
use secp256k1::ffi::CPtr;
use std::collections::HashMap;
use std::ffi::CString;
use tw_encoding::hex;
use tw_proto::Bitcoin::Proto as LegacyProto;

#[test]
fn print_wif_keys() {
	let pk = PrivateKey::from_wif("cTk5wSci88FPka7JwHpNEA82dUMjAysdDbCiuYB2fegfgGESAZVn").unwrap();
	let seckey = tw_encoding::hex::encode(pk.to_bytes(), false);
	dbg!(seckey);

	let pubkey = pk.public_key(&secp256k1::Secp256k1::new());
	let pubkey = tw_encoding::hex::encode(pubkey.to_bytes(), false);
	dbg!(pubkey);
}

const FULL_SATOSHIS: i64 = (ONE_BTC * 50) as i64;
const SEND_SATOSHIS: i64 = FULL_SATOSHIS - (ONE_BTC / 100) as i64;


fn ffi_tw_taproot_build_and_sign_transaction() {
	let alice_private_key = hex("56429688a1a6b00b90ccd22a0de0a376b6569d8684022ae92229a28478bfb657");
	let alice_pubkey = hex("036666dd712e05a487916384bfcd5973eb53e8038eccbbf97f7eed775b87389536");
	let bob_pubkey = hex("037ed9a436e11ec4947ac4b7823787e24ba73180f1edd2857bff19c9f4d62b65bf");

	let txid = hex("1e1cdc48aa990d7e154a161d5b5f1cad737742e97d2712ab188027bb42e6e47b").into_iter().rev().collect();

	// Input.
	let input = unsafe {
		tw_build_p2pkh_script(FULL_SATOSHIS, alice_pubkey.as_c_ptr(), alice_pubkey.len()).into_vec()
	};
	let input: LegacyProto::TransactionOutput = tw_proto::deserialize(&input).unwrap();

	// Output.
	let output = unsafe {
		tw_build_p2pkh_script(SEND_SATOSHIS, bob_pubkey.as_c_ptr(), bob_pubkey.len()).into_vec()
	};
	let output: LegacyProto::TransactionOutput = tw_proto::deserialize(&output).unwrap();

	let signing = LegacyProto::SigningInput {
		private_key: vec![
			alice_private_key.into(),
		],
		utxo: vec![
			LegacyProto::UnspentTransaction {
				out_point: Some(LegacyProto::OutPoint {
					hash: txid,
					index: 0,
					..Default::default()
				}),
				script: input.script,
				amount: input.value,
				variant: LegacyProto::TransactionVariant::P2PKH,
				spendingScript: Default::default()
			}
		],
		plan: Some(LegacyProto::TransactionPlan {
			utxos: vec![
				LegacyProto::UnspentTransaction {
					out_point: Default::default(),
					script: output.script,
					amount: output.value,
					variant: LegacyProto::TransactionVariant::P2PKH,
					spendingScript: Default::default()
				}
			],
			..Default::default()
		}),
		..Default::default()
	};

	let serialized = tw_proto::serialize(&signing).unwrap();

	let res = unsafe {
		tw_taproot_build_and_sign_transaction(serialized.as_c_ptr(), serialized.len()).into_vec()
	};

	todo!()
}

/*
	let signing = LegacyProto::SigningInput {
		hash_type: 0,
		amount: 0,
		byte_fee: 0,
		to_address: Default::default(),
		change_address: Default::default(),
		private_key: vec![],
		scripts: HashMap::new(),
		utxo: vec![],
		use_max_amount: false,
		coin_type: 0,
		plan: Some(LegacyProto::TransactionPlan {
			amount: 0,
			available_amount: 0,
			fee: 0,
			change: 0,
			utxos: vec![],
			branch_id: Default::default(),
			error: Default::default(),
			output_op_return: Default::default(),
			preblockhash: Default::default(),
			preblockheight: Default::default(),
		}),
		lock_time: 0,
		output_op_return: Default::default(),
		extra_outputs: vec![],
		use_max_utxo: false,
		disable_dust_filter: false,
		time: 0,
		is_it_brc_operation: false,
	};
*/