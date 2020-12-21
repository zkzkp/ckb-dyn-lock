pub mod secp256k1_keccak256 {
    use std::vec::Vec;

    use ckb_tool::ckb_crypto::secp::Privkey;
    use ckb_tool::ckb_types::core::TransactionView;
    use ckb_tool::ckb_types::packed::WitnessArgs;
    use ckb_tool::ckb_types::{bytes::Bytes, prelude::*, H256};
    use sha3::{Digest, Keccak256};

    pub const SIGNATURE_SIZE: usize = 65;

    pub fn sign_tx(tx: TransactionView, priv_key: &Privkey) -> TransactionView {
        let witnesses_len = tx.witnesses().len();
        sign_tx_by_group(tx, priv_key, 0, witnesses_len)
    }

    pub fn sign_tx_by_group(
        tx: TransactionView,
        priv_key: &Privkey,
        begin_index: usize,
        len: usize,
    ) -> TransactionView {
        let tx_hash = tx.hash();

        let gen_witness_args = |i: usize| {
            let mut hasher = Keccak256::default();
            let mut message = [0u8; 32];

            hasher.input(&tx_hash.raw_data());

            // digest the first witness
            let witness = WitnessArgs::new_unchecked(tx.witnesses().get(i).unwrap().unpack());
            let zero_lock: Bytes = {
                let mut buf = Vec::new();
                buf.resize(SIGNATURE_SIZE, 0);
                buf.into()
            };
            let witness_for_digest = witness
                .clone()
                .as_builder()
                .lock(Some(zero_lock).pack())
                .build();
            let witness_len = witness_for_digest.as_bytes().len() as u64;
            hasher.input(&witness_len.to_le_bytes());
            hasher.input(&witness_for_digest.as_bytes());

            ((i + 1)..(i + len)).for_each(|n| {
                let witness = tx.witnesses().get(n).unwrap();
                let witness_len = witness.raw_data().len() as u64;
                hasher.input(&witness_len.to_le_bytes());
                hasher.input(&witness.raw_data());
            });
            message.copy_from_slice(&hasher.result()[0..32]);

            let prefix: [u8; 28] = [
                0x19, 0x45, 0x74, 0x68, 0x65, 0x72, 0x65, 0x75, 0x6d, 0x20, 0x53, 0x69, 0x67, 0x6e,
                0x65, 0x64, 0x20, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x3a, 0x0a, 0x33, 0x32,
            ];
            hasher = Keccak256::default();
            hasher.input(&prefix);
            hasher.input(&message);

            message.copy_from_slice(&hasher.result()[0..32]);
            let message = H256::from(message);
            let sig = priv_key.sign_recoverable(&message).expect("sign");

            witness
                .as_builder()
                .lock(Some(Bytes::from(sig.serialize())).pack())
                .build()
                .as_bytes()
                .pack()
        };

        let mut signed_witnesses: Vec<_> = tx
            .inputs()
            .into_iter()
            .enumerate()
            .map(|(i, _)| {
                if i == begin_index {
                    gen_witness_args(i)
                } else {
                    tx.witnesses().get(i).unwrap_or_default()
                }
            })
            .collect();

        for i in signed_witnesses.len()..tx.witnesses().len() {
            signed_witnesses.push(tx.witnesses().get(i).unwrap());
        }

        // calculate message
        tx.as_advanced_builder()
            .set_witnesses(signed_witnesses)
            .build()
    }
}
