use crate::{
    instructions::{
        attest,
        complete_native,
        complete_wrapped,
        create_wrapped,
        register_chain,
        transfer_native,
        transfer_wrapped,
        upgrade_contract,
    },
    messages::{
        GovernancePayloadUpgrade,
        PayloadAssetMeta,
        PayloadGovernanceRegisterChain,
        PayloadTransfer,
    },
    CompleteNativeData,
    CompleteWrappedData,
    CreateWrappedData,
    RegisterChainData,
    TransferNativeData,
    TransferWrappedData,
};
use bridge::{
    accounts::MessageDerivationData,
    vaa::VAA,
    DeserializePayload,
    PostVAAData,
};
use solana_program::pubkey::Pubkey;
use solitaire::{
    processors::seeded::Seeded,
    AccountState,
};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn attest_ix(
    program_id: String,
    bridge_id: String,
    payer: String,
    mint: String,
    decimals: u8,
    mint_meta: String,
    nonce: u32,
) -> JsValue {
    let program_id = Pubkey::from_str(program_id.as_str()).unwrap();
    let bridge_id = Pubkey::from_str(bridge_id.as_str()).unwrap();
    let payer = Pubkey::from_str(payer.as_str()).unwrap();
    let mint = Pubkey::from_str(mint.as_str()).unwrap();
    let mint_meta = Pubkey::from_str(mint_meta.as_str()).unwrap();

    let ix = attest(
        program_id, bridge_id, payer, mint, decimals, mint_meta, nonce,
    )
    .unwrap();

    JsValue::from_serde(&ix).unwrap()
}

#[wasm_bindgen]
pub fn transfer_native_ix(
    program_id: String,
    bridge_id: String,
    payer: String,
    from: String,
    mint: String,
    nonce: u32,
    amount: u64,
    fee: u64,
    target_address: Vec<u8>,
    target_chain: u16,
) -> JsValue {
    let program_id = Pubkey::from_str(program_id.as_str()).unwrap();
    let bridge_id = Pubkey::from_str(bridge_id.as_str()).unwrap();
    let payer = Pubkey::from_str(payer.as_str()).unwrap();
    let from = Pubkey::from_str(from.as_str()).unwrap();
    let mint = Pubkey::from_str(mint.as_str()).unwrap();

    let mut target_addr = [0u8; 32];
    target_addr.copy_from_slice(target_address.as_slice());

    let ix = transfer_native(
        program_id,
        bridge_id,
        payer,
        from,
        mint,
        TransferNativeData {
            nonce,
            amount,
            fee,
            target_address: target_addr,
            target_chain,
        },
    )
    .unwrap();

    JsValue::from_serde(&ix).unwrap()
}

#[wasm_bindgen]
pub fn transfer_wrapped_ix(
    program_id: String,
    bridge_id: String,
    payer: String,
    from: String,
    from_owner: String,
    token_chain: u16,
    token_address: Vec<u8>,
    nonce: u32,
    amount: u64,
    fee: u64,
    target_address: Vec<u8>,
    target_chain: u16,
) -> JsValue {
    let program_id = Pubkey::from_str(program_id.as_str()).unwrap();
    let bridge_id = Pubkey::from_str(bridge_id.as_str()).unwrap();
    let payer = Pubkey::from_str(payer.as_str()).unwrap();
    let from = Pubkey::from_str(from.as_str()).unwrap();
    let from_owner = Pubkey::from_str(from_owner.as_str()).unwrap();

    let mut target_addr = [0u8; 32];
    target_addr.copy_from_slice(target_address.as_slice());
    let mut token_addr = [0u8; 32];
    token_addr.copy_from_slice(token_address.as_slice());

    let ix = transfer_wrapped(
        program_id,
        bridge_id,
        payer,
        from,
        from_owner,
        token_chain,
        token_addr,
        TransferWrappedData {
            nonce,
            amount,
            fee,
            target_address: target_addr,
            target_chain,
        },
    )
    .unwrap();

    JsValue::from_serde(&ix).unwrap()
}

#[wasm_bindgen]
pub fn complete_transfer_native_ix(
    program_id: String,
    bridge_id: String,
    payer: String,
    vaa: Vec<u8>,
) -> JsValue {
    let program_id = Pubkey::from_str(program_id.as_str()).unwrap();
    let bridge_id = Pubkey::from_str(bridge_id.as_str()).unwrap();
    let payer = Pubkey::from_str(payer.as_str()).unwrap();
    let vaa = VAA::deserialize(vaa.as_slice()).unwrap();
    let payload = PayloadTransfer::deserialize(&mut vaa.payload.as_slice()).unwrap();
    let message_key = bridge::accounts::Message::<'_, { AccountState::Uninitialized }>::key(
        &MessageDerivationData {
            emitter_key: vaa.emitter_address,
            emitter_chain: vaa.emitter_chain,
            nonce: vaa.nonce,
            payload: vaa.payload.clone(),
            sequence: None,
        },
        &program_id,
    );
    let post_vaa_data = PostVAAData {
        version: vaa.version,
        guardian_set_index: vaa.guardian_set_index,
        timestamp: vaa.timestamp,
        nonce: vaa.nonce,
        emitter_chain: vaa.emitter_chain,
        emitter_address: vaa.emitter_address,
        sequence: vaa.sequence,
        consistency_level: vaa.consistency_level,
        payload: vaa.payload,
    };

    let ix = complete_native(
        program_id,
        bridge_id,
        payer,
        message_key,
        post_vaa_data,
        Pubkey::new(&payload.to[..]),
        Pubkey::new(&payload.token_address),
        CompleteNativeData {},
    )
    .unwrap();

    JsValue::from_serde(&ix).unwrap()
}

#[wasm_bindgen]
pub fn complete_transfer_wrapped_ix(
    program_id: String,
    bridge_id: String,
    payer: String,
    vaa: Vec<u8>,
) -> JsValue {
    let program_id = Pubkey::from_str(program_id.as_str()).unwrap();
    let bridge_id = Pubkey::from_str(bridge_id.as_str()).unwrap();
    let payer = Pubkey::from_str(payer.as_str()).unwrap();
    let vaa = VAA::deserialize(vaa.as_slice()).unwrap();
    let payload = PayloadTransfer::deserialize(&mut vaa.payload.as_slice()).unwrap();
    let message_key = bridge::accounts::Message::<'_, { AccountState::Uninitialized }>::key(
        &MessageDerivationData {
            emitter_key: vaa.emitter_address,
            emitter_chain: vaa.emitter_chain,
            nonce: vaa.nonce,
            payload: vaa.payload.clone(),
            sequence: None,
        },
        &program_id,
    );
    let post_vaa_data = PostVAAData {
        version: vaa.version,
        guardian_set_index: vaa.guardian_set_index,
        timestamp: vaa.timestamp,
        nonce: vaa.nonce,
        emitter_chain: vaa.emitter_chain,
        emitter_address: vaa.emitter_address,
        sequence: vaa.sequence,
        consistency_level: vaa.consistency_level,
        payload: vaa.payload,
    };

    let ix = complete_wrapped(
        program_id,
        bridge_id,
        payer,
        message_key,
        post_vaa_data,
        payload.clone(),
        Pubkey::new(&payload.to),
        CompleteWrappedData {},
    )
    .unwrap();

    JsValue::from_serde(&ix).unwrap()
}

#[wasm_bindgen]
pub fn create_wrapped_ix(
    program_id: String,
    bridge_id: String,
    payer: String,
    vaa: Vec<u8>,
) -> JsValue {
    let program_id = Pubkey::from_str(program_id.as_str()).unwrap();
    let bridge_id = Pubkey::from_str(bridge_id.as_str()).unwrap();
    let payer = Pubkey::from_str(payer.as_str()).unwrap();
    let vaa = VAA::deserialize(vaa.as_slice()).unwrap();
    let payload = PayloadAssetMeta::deserialize(&mut vaa.payload.as_slice()).unwrap();
    let message_key = bridge::accounts::Message::<'_, { AccountState::Uninitialized }>::key(
        &MessageDerivationData {
            emitter_key: vaa.emitter_address,
            emitter_chain: vaa.emitter_chain,
            nonce: vaa.nonce,
            payload: vaa.payload.clone(),
            sequence: None,
        },
        &program_id,
    );
    let post_vaa_data = PostVAAData {
        version: vaa.version,
        guardian_set_index: vaa.guardian_set_index,
        timestamp: vaa.timestamp,
        nonce: vaa.nonce,
        emitter_chain: vaa.emitter_chain,
        emitter_address: vaa.emitter_address,
        sequence: vaa.sequence,
        consistency_level: vaa.consistency_level,
        payload: vaa.payload,
    };

    let ix = create_wrapped(
        program_id,
        bridge_id,
        payer,
        message_key,
        post_vaa_data,
        payload,
        CreateWrappedData {},
    )
    .unwrap();

    JsValue::from_serde(&ix).unwrap()
}

#[wasm_bindgen]
pub fn upgrade_contract_ix(
    program_id: String,
    payer: String,
    spill: String,
    vaa: Vec<u8>,
) -> JsValue {
    let program_id = Pubkey::from_str(program_id.as_str()).unwrap();
    let spill = Pubkey::from_str(spill.as_str()).unwrap();
    let vaa = VAA::deserialize(vaa.as_slice()).unwrap();
    let payload = GovernancePayloadUpgrade::deserialize(&mut vaa.payload.as_slice()).unwrap();
    let message_key = bridge::accounts::Message::<'_, { AccountState::Uninitialized }>::key(
        &MessageDerivationData {
            emitter_key: vaa.emitter_address,
            emitter_chain: vaa.emitter_chain,
            nonce: vaa.nonce,
            payload: vaa.payload,
            sequence: None,
        },
        &program_id,
    );
    let ix = upgrade_contract(
        program_id,
        Pubkey::from_str(payer.as_str()).unwrap(),
        message_key,
        Pubkey::new(&vaa.emitter_address),
        payload.new_contract,
        spill,
        vaa.sequence,
    );
    return JsValue::from_serde(&ix).unwrap();
}

#[wasm_bindgen]
pub fn register_chain_ix(
    program_id: String,
    bridge_id: String,
    payer: String,
    vaa: Vec<u8>,
) -> JsValue {
    let program_id = Pubkey::from_str(program_id.as_str()).unwrap();
    let bridge_id = Pubkey::from_str(bridge_id.as_str()).unwrap();
    let payer = Pubkey::from_str(payer.as_str()).unwrap();
    let vaa = VAA::deserialize(vaa.as_slice()).unwrap();
    let payload = PayloadGovernanceRegisterChain::deserialize(&mut vaa.payload.as_slice()).unwrap();
    let message_key = bridge::accounts::Message::<'_, { AccountState::Uninitialized }>::key(
        &MessageDerivationData {
            emitter_key: vaa.emitter_address,
            emitter_chain: vaa.emitter_chain,
            nonce: vaa.nonce,
            payload: vaa.payload.clone(),
            sequence: None,
        },
        &program_id,
    );
    let post_vaa_data = PostVAAData {
        version: vaa.version,
        guardian_set_index: vaa.guardian_set_index,
        timestamp: vaa.timestamp,
        nonce: vaa.nonce,
        emitter_chain: vaa.emitter_chain,
        emitter_address: vaa.emitter_address,
        sequence: vaa.sequence,
        consistency_level: vaa.consistency_level,
        payload: vaa.payload,
    };
    let ix = register_chain(
        program_id,
        bridge_id,
        payer,
        message_key,
        post_vaa_data,
        payload,
        RegisterChainData {},
    )
    .unwrap();
    return JsValue::from_serde(&ix).unwrap();
}