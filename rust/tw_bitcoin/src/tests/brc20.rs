use super::hex;
use crate::entry::{
    BitcoinEntry, PlaceHolder, ProtoInputBuilder, ProtoInputRecipient, ProtoOutputBuilder,
    ProtoOutputRecipient,
};
use tw_coin_entry::coin_entry::CoinEntry;
use tw_proto::BitcoinV2::Proto;
use tw_proto::Utxo::Proto as UtxoProto;

#[test]
fn coin_entry_sign_brc20_commit_reveal_transfer() {
    let coin = PlaceHolder;

    let alice_private_key = hex("e253373989199da27c48680e3a3fc0f648d50f9a727ef17a7fe6a4dc3b159129");
    let alice_pubkey = hex("030f209b6ada5edb42c77fd2bc64ad650ae38314c8f451f3e36d80bc8e26f132cb");

    let txid: Vec<u8> = hex("8ec895b4d30adb01e38471ca1019bfc8c3e5fbd1f28d9e7b5653260d89989008")
        .into_iter()
        .rev()
        .collect();

    let tx1 = Proto::Input {
        txid: txid.as_slice().into(),
        vout: 1,
        amount: 26_400,
        sequence: u32::MAX,
        sighash_type: UtxoProto::SighashType::All,
        to_recipient: ProtoInputRecipient::builder(Proto::mod_Input::Builder {
            variant: ProtoInputBuilder::p2wpkh(alice_pubkey.as_slice().into()),
        }),
    };

    let out1 = Proto::Output {
        amount: 7_000,
        to_recipient: ProtoOutputRecipient::builder(Proto::mod_Output::Builder {
            variant: ProtoOutputBuilder::brc20_inscribe(Proto::mod_Output::Brc20Inscription {
                inscribe_to: alice_pubkey.as_slice().into(),
                ticker: "oadf".into(),
                transfer_amount: 20,
            }),
        }),
    };

    // Change/return transaction.
    let out2 = Proto::Output {
        amount: 16_400,
        to_recipient: ProtoOutputRecipient::builder(Proto::mod_Output::Builder {
            variant: ProtoOutputBuilder::p2wpkh(Proto::ToPublicKeyOrHash {
                to_address: Proto::mod_ToPublicKeyOrHash::OneOfto_address::pubkey(
                    alice_pubkey.as_slice().into(),
                ),
            }),
        }),
    };

    let signing = Proto::SigningInput {
        version: 2,
        private_key: alice_private_key.as_slice().into(),
        lock_time: Default::default(),
        inputs: vec![tx1],
        outputs: vec![out1, out2],
        input_selector: UtxoProto::InputSelector::UseAll,
        sat_vb: 0,
        change_output: Default::default(),
        disable_change_output: true,
    };

    let output = BitcoinEntry.sign(&coin, signing);
    let encoded = tw_encoding::hex::encode(output.encoded, false);
    let transaction = output.transaction.unwrap();

    assert_eq!(transaction.inputs.len(), 1);
    assert_eq!(transaction.outputs.len(), 2);
    assert_eq!(&encoded, "02000000000101089098890d2653567b9e8df2d1fbe5c3c8bf1910ca7184e301db0ad3b495c88e0100000000ffffffff02581b000000000000225120e8b706a97732e705e22ae7710703e7f589ed13c636324461afa443016134cc051040000000000000160014e311b8d6ddff856ce8e9a4e03bc6d4fe5050a83d02483045022100a44aa28446a9a886b378a4a65e32ad9a3108870bd725dc6105160bed4f317097022069e9de36422e4ce2e42b39884aa5f626f8f94194d1013007d5a1ea9220a06dce0121030f209b6ada5edb42c77fd2bc64ad650ae38314c8f451f3e36d80bc8e26f132cb00000000");

    let txid: Vec<u8> = hex("797d17d47ae66e598341f9dfdea020b04d4017dcf9cc33f0e51f7a6082171fb1")
        .into_iter()
        .rev()
        .collect();

    let tx1 = Proto::Input {
        txid: txid.as_slice().into(),
        vout: 0,
        amount: 7_000,
        sequence: u32::MAX,
        sighash_type: UtxoProto::SighashType::UseDefault,
        to_recipient: ProtoInputRecipient::builder(Proto::mod_Input::Builder {
            variant: ProtoInputBuilder::brc20_inscribe(Proto::mod_Input::Brc20Inscription {
                one_prevout: false,
                inscribe_to: alice_pubkey.as_slice().into(),
                ticker: "oadf".into(),
                transfer_amount: 20,
            }),
        }),
    };

    let out1 = Proto::Output {
        amount: 546,
        to_recipient: ProtoOutputRecipient::builder(Proto::mod_Output::Builder {
            variant: ProtoOutputBuilder::p2wpkh(Proto::ToPublicKeyOrHash {
                to_address: Proto::mod_ToPublicKeyOrHash::OneOfto_address::pubkey(
                    alice_pubkey.as_slice().into(),
                ),
            }),
        }),
    };

    let signing = Proto::SigningInput {
        version: 2,
        private_key: alice_private_key.as_slice().into(),
        lock_time: Default::default(),
        inputs: vec![tx1],
        outputs: vec![out1],
        input_selector: UtxoProto::InputSelector::UseAll,
        sat_vb: 0,
        change_output: Default::default(),
        disable_change_output: true,
    };

    let output = BitcoinEntry.sign(&coin, signing);
    let encoded = tw_encoding::hex::encode(output.encoded, false);
    let transaction = output.transaction.unwrap();

    const REVEAL_RAW: &str = "02000000000101b11f1782607a1fe5f033ccf9dc17404db020a0dedff94183596ee67ad4177d790000000000ffffffff012202000000000000160014e311b8d6ddff856ce8e9a4e03bc6d4fe5050a83d0340de6fd13e43700f59876d305e5a4a5c41ad7ada10bc5a4e4bdd779eb0060c0a78ebae9c33daf77bb3725172edb5bd12e26f00c08f9263e480d53b93818138ad0b5b0063036f7264010118746578742f706c61696e3b636861727365743d7574662d3800377b2270223a226272632d3230222c226f70223a227472616e73666572222c227469636b223a226f616466222c22616d74223a223230227d6821c00f209b6ada5edb42c77fd2bc64ad650ae38314c8f451f3e36d80bc8e26f132cb00000000";

    assert_eq!(transaction.inputs.len(), 1);
    assert_eq!(transaction.outputs.len(), 1);

    assert_eq!(encoded[..164], REVEAL_RAW[..164]);
    assert_ne!(encoded[164..292], REVEAL_RAW[164..292]);
    assert_eq!(encoded[292..], REVEAL_RAW[292..]);
}