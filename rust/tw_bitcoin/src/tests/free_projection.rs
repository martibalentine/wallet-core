use crate::entry::{
    BitcoinEntry, PlaceHolder, ProtoInputBuilder, ProtoInputRecipient, ProtoOutputBuilder,
    ProtoOutputRecipient,
};
use bitcoin::Weight;
use tw_coin_entry::coin_entry::CoinEntry;
use tw_proto::BitcoinV2::Proto;
use tw_proto::Utxo::Proto as UtxoProto;

fn hex(string: &str) -> Vec<u8> {
    tw_encoding::hex::decode(string).unwrap()
}

const ONE_BTC: u64 = 100_000_000;
const SAT_VB: u64 = 20;

#[test]
fn p2pkh_fee_projection() {
    let coin = PlaceHolder;

    let alice_private_key = hex("57a64865bce5d4855e99b1cce13327c46171434f2d72eeaf9da53ee075e7f90a");
    let alice_pubkey = hex("028d7dce6d72fb8f7af9566616c6436349c67ad379f2404dd66fe7085fe0fba28f");

    let bob_pubkey = hex("025a0af1510f0f24d40dd00d7c0e51605ca504bbc177c3e19b065f373a1efdd22f");
    let txid: Vec<u8> = hex("181c84965c9ea86a5fac32fdbd5f73a21a7a9e749fb6ab97e273af2329f6b911")
        .into_iter()
        .rev()
        .collect();

    let mut signing = Proto::SigningInput {
        version: 2,
        private_key: alice_private_key.as_slice().into(),
        lock_time: Default::default(),
        inputs: vec![],
        outputs: vec![],
        input_selector: UtxoProto::InputSelector::UseAll,
        sat_vb: SAT_VB,
        change_output: Default::default(),
        disable_change_output: true,
    };

    signing.inputs.push(Proto::Input {
        txid: txid.as_slice().into(),
        vout: 0,
        amount: 2 * ONE_BTC,
        sequence: u32::MAX,
        sighash_type: UtxoProto::SighashType::All,
        to_recipient: ProtoInputRecipient::builder(Proto::mod_Input::Builder {
            variant: ProtoInputBuilder::p2pkh(alice_pubkey.as_slice().into()),
        }),
    });

    signing.outputs.push(Proto::Output {
        amount: ONE_BTC,
        to_recipient: ProtoOutputRecipient::builder(Proto::mod_Output::Builder {
            variant: ProtoOutputBuilder::p2pkh(Proto::ToPublicKeyOrHash {
                to_address: Proto::mod_ToPublicKeyOrHash::OneOfto_address::pubkey(
                    bob_pubkey.as_slice().into(),
                ),
            }),
        }),
    });

    let prehashes = BitcoinEntry.preimage_hashes(&coin, signing.clone());
    // Check
    assert_eq!(prehashes.error, Proto::Error::OK);
    assert_eq!(prehashes.weight_projection, 768);
    assert_eq!(prehashes.fee_projection, (768 + 3) / 4 * SAT_VB);

    let output = BitcoinEntry.sign(&coin, signing);
    assert_eq!(output.weight, 768);
    assert_eq!(output.fee, (768 + 3) / 4 * SAT_VB);
}

#[test]
fn p2wpkh_fee_projection() {
    let coin = PlaceHolder;

    let alice_private_key = hex("57a64865bce5d4855e99b1cce13327c46171434f2d72eeaf9da53ee075e7f90a");
    let alice_pubkey = hex("028d7dce6d72fb8f7af9566616c6436349c67ad379f2404dd66fe7085fe0fba28f");

    let bob_pubkey = hex("025a0af1510f0f24d40dd00d7c0e51605ca504bbc177c3e19b065f373a1efdd22f");
    let txid: Vec<u8> = hex("181c84965c9ea86a5fac32fdbd5f73a21a7a9e749fb6ab97e273af2329f6b911")
        .into_iter()
        .rev()
        .collect();

    let mut signing = Proto::SigningInput {
        version: 2,
        private_key: alice_private_key.as_slice().into(),
        lock_time: Default::default(),
        inputs: vec![],
        outputs: vec![],
        input_selector: UtxoProto::InputSelector::UseAll,
        sat_vb: SAT_VB,
        change_output: Default::default(),
        disable_change_output: true,
    };

    signing.inputs.push(Proto::Input {
        txid: txid.as_slice().into(),
        vout: 0,
        amount: 2 * ONE_BTC,
        sequence: u32::MAX,
        sighash_type: UtxoProto::SighashType::All,
        to_recipient: ProtoInputRecipient::builder(Proto::mod_Input::Builder {
            variant: ProtoInputBuilder::p2wpkh(alice_pubkey.as_slice().into()),
        }),
    });

    signing.outputs.push(Proto::Output {
        amount: ONE_BTC,
        to_recipient: ProtoOutputRecipient::builder(Proto::mod_Output::Builder {
            variant: ProtoOutputBuilder::p2wpkh(Proto::ToPublicKeyOrHash {
                to_address: Proto::mod_ToPublicKeyOrHash::OneOfto_address::pubkey(
                    bob_pubkey.as_slice().into(),
                ),
            }),
        }),
    });

    let prehashes = BitcoinEntry.preimage_hashes(&coin, signing.clone());
    // Check
    assert_eq!(prehashes.error, Proto::Error::OK);
    assert_eq!(prehashes.weight_projection, 436);
    assert_eq!(prehashes.fee_projection, (436 + 3) / 4 * SAT_VB);

    let output = BitcoinEntry.sign(&coin, signing);
	// TODO: The final weight/fee is slightly off from the fee estimation. This
	// is probably good enough, but we can probably improve this.
    assert_eq!(output.weight, 438);
    assert_eq!(output.fee, (438 + 3) / 4 * SAT_VB);
}