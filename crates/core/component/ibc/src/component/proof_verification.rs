use crate::component::client::StateReadExt;

use core::time::Duration;
use ibc_proto::protobuf::Protobuf;
use ibc_types2::path::ClientConsensusStatePath;
use ibc_types2::DomainType;
use ibc_types2::{
    core::{
        channel::{
            msgs::MsgAcknowledgement, msgs::MsgRecvPacket, msgs::MsgTimeout, ChannelEnd, ChannelId,
            Packet, PortId,
        },
        client::ClientId,
        client::Height,
        commitment::{MerklePrefix, MerkleProof, MerkleRoot},
        connection::{ConnectionEnd, ConnectionId},
    },
    lightclients::tendermint::{
        client_state::ClientState as TendermintClientState,
        consensus_state::ConsensusState as TendermintConsensusState,
    },
    path::{
        AckPath, ChannelEndPath, ClientStatePath, CommitmentPath, ConnectionPath, Path,
        ReceiptPath, SeqRecvPath,
    },
};

use async_trait::async_trait;
use num_traits::float::FloatCore;
use penumbra_chain::component::StateReadExt as _;
use penumbra_storage::StateRead;
use prost::Message;
use sha2::{Digest, Sha256};

// NOTE: this is underspecified.
// using the same implementation here as ibc-go:
// https://github.com/cosmos/ibc-go/blob/main/modules/core/04-channel/types/packet.go#L19
// timeout_timestamp + timeout_height.revision_number + timeout_height.revision_height
// + sha256(data)
pub fn commit_packet(packet: &Packet) -> Vec<u8> {
    let mut commit = vec![];
    commit.extend_from_slice(&packet.timeout_timestamp_on_b.nanoseconds().to_be_bytes());
    commit.extend_from_slice(
        &packet
            .timeout_height_on_b
            .commitment_revision_number()
            .to_be_bytes(),
    );
    commit.extend_from_slice(
        &packet
            .timeout_height_on_b
            .commitment_revision_height()
            .to_be_bytes(),
    );
    commit.extend_from_slice(&Sha256::digest(&packet.data)[..]);

    Sha256::digest(&commit).to_vec()
}

// NOTE: this is underspecified.
// using the same implementation here as ibc-go:
// https://github.com/cosmos/ibc-go/blob/main/modules/core/04-channel/types/packet.go#L38
pub fn commit_acknowledgement(ack_data: &[u8]) -> Vec<u8> {
    Sha256::digest(ack_data).to_vec()
}

pub fn calculate_block_delay(
    delay_period_time: &Duration,
    max_expected_time_per_block: &Duration,
) -> u64 {
    if max_expected_time_per_block.is_zero() {
        return 0;
    }

    FloatCore::ceil(delay_period_time.as_secs_f64() / max_expected_time_per_block.as_secs_f64())
        as u64
}

fn verify_merkle_absence_proof(
    proof_specs: &[ics23::ProofSpec],
    prefix: &MerklePrefix,
    proof: &MerkleProof,
    root: &MerkleRoot,
    path: impl Into<Path>,
) -> anyhow::Result<()> {
    let merkle_path = prefix.apply(vec![path.into().to_string()]);
    proof.verify_non_membership(proof_specs, root.clone().into(), merkle_path)?;

    Ok(())
}

fn verify_merkle_proof(
    proof_specs: &[ics23::ProofSpec],
    prefix: &MerklePrefix,
    proof: &MerkleProof,
    root: &MerkleRoot,
    path: impl Into<Path>,
    value: Vec<u8>,
) -> anyhow::Result<()> {
    let merkle_path = prefix.apply(vec![path.into().to_string()]);
    proof.verify_membership(proof_specs, root.clone().into(), merkle_path, value, 0)?;

    Ok(())
}

#[async_trait]
pub trait ChannelProofVerifier: StateReadExt {
    async fn verify_channel_proof(
        &self,
        connection: &ConnectionEnd,
        proof: &MerkleProof,
        proof_height: &Height,
        channel_id: &ChannelId,
        port_id: &PortId,
        expected_channel: &ChannelEnd,
    ) -> anyhow::Result<()> {
        // get the stored client state for the counterparty
        let trusted_client_state = self.get_client_state(&connection.client_id).await?;

        // check if the client is frozen
        // TODO: should we also check if the client is expired here?
        if trusted_client_state.is_frozen() {
            return Err(anyhow::anyhow!("client is frozen"));
        }

        // get the stored consensus state for the counterparty
        let trusted_consensus_state = self
            .get_verified_consensus_state(*proof_height, connection.client_id.clone())
            .await?;

        trusted_client_state.verify_height(*proof_height)?;
        let value = expected_channel.encode_vec();

        verify_merkle_proof(
            &trusted_client_state.proof_specs,
            &connection.counterparty.prefix.clone().into(),
            proof,
            &trusted_consensus_state.root,
            ChannelEndPath::new(port_id, channel_id),
            value,
        )?;

        Ok(())
    }
}

impl<T: StateRead> ChannelProofVerifier for T {}

pub fn verify_connection_state(
    client_state: &TendermintClientState,
    height: Height,
    prefix: &MerklePrefix,
    proof: &MerkleProof,
    root: &MerkleRoot,
    conn_path: &ConnectionPath,
    expected_connection_end: &ConnectionEnd,
) -> anyhow::Result<()> {
    client_state.verify_height(height)?;

    let value = expected_connection_end.encode_vec();

    verify_merkle_proof(
        &client_state.proof_specs,
        prefix,
        proof,
        root,
        conn_path.clone(),
        value,
    )?;

    Ok(())
}

pub fn verify_client_full_state(
    client_state: &TendermintClientState,
    height: Height,
    prefix: &MerklePrefix,
    proof: &MerkleProof,
    root: &MerkleRoot,
    client_state_path: &ClientStatePath,
    expected_client_state: TendermintClientState,
) -> anyhow::Result<()> {
    client_state.verify_height(height)?;

    let value: Vec<u8> = expected_client_state.encode_to_vec();

    verify_merkle_proof(
        &client_state.proof_specs,
        prefix,
        proof,
        root,
        client_state_path.clone(),
        value,
    )?;

    Ok(())
}

pub fn verify_client_consensus_state(
    client_state: &TendermintClientState,
    height: Height,
    prefix: &MerklePrefix,
    proof: &MerkleProof,
    root: &MerkleRoot,
    client_cons_state_path: &ClientConsensusStatePath,
    expected_consenus_state: TendermintConsensusState,
) -> anyhow::Result<()> {
    client_state.verify_height(height)?;

    let value: Vec<u8> = expected_consenus_state.encode_to_vec();

    verify_merkle_proof(
        &client_state.proof_specs,
        prefix,
        proof,
        root,
        client_cons_state_path.clone(),
        value,
    )?;

    Ok(())
}

#[async_trait]
pub trait PacketProofVerifier: StateReadExt + inner::Inner {
    async fn verify_packet_recv_proof(
        &self,
        connection: &ConnectionEnd,
        msg: &MsgRecvPacket,
    ) -> anyhow::Result<()> {
        let (trusted_client_state, trusted_consensus_state) = self
            .get_trusted_client_and_consensus_state(
                &connection.client_id,
                &msg.proof_height_on_a,
                connection,
            )
            .await?;

        let commitment_path = CommitmentPath {
            port_id: msg.packet.port_on_b.clone(),
            channel_id: msg.packet.chan_on_b.clone(),
            sequence: msg.packet.sequence,
        };

        let commitment_bytes = commit_packet(&msg.packet);
        let proof: MerkleProof = msg.proof_commitment_on_a.clone().try_into()?;

        verify_merkle_proof(
            &trusted_client_state.proof_specs,
            &connection.counterparty.prefix.clone().into(),
            &proof,
            &trusted_consensus_state.root,
            commitment_path,
            commitment_bytes,
        )?;

        Ok(())
    }

    async fn verify_packet_ack_proof(
        &self,
        connection: &ConnectionEnd,
        msg: &MsgAcknowledgement,
    ) -> anyhow::Result<()> {
        let (trusted_client_state, trusted_consensus_state) = self
            .get_trusted_client_and_consensus_state(
                &connection.client_id,
                &msg.proof_height_on_b,
                connection,
            )
            .await?;

        let ack_path = AckPath {
            port_id: msg.packet.port_on_b.clone(),
            channel_id: msg.packet.chan_on_b.clone(),
            sequence: msg.packet.sequence,
        };

        let proof: MerkleProof = msg.proof_acked_on_b.clone().try_into()?;

        verify_merkle_proof(
            &trusted_client_state.proof_specs,
            &connection.counterparty.prefix.clone().into(),
            &proof,
            &trusted_consensus_state.root,
            ack_path,
            msg.acknowledgement.clone().into(),
        )?;

        Ok(())
    }

    async fn verify_packet_timeout_proof(
        &self,
        connection: &ConnectionEnd,
        msg: &MsgTimeout,
    ) -> anyhow::Result<()> {
        let (trusted_client_state, trusted_consensus_state) = self
            .get_trusted_client_and_consensus_state(
                &connection.client_id,
                &msg.proof_height_on_b,
                connection,
            )
            .await?;

        let mut seq_bytes = Vec::new();
        u64::from(msg.next_seq_recv_on_b)
            .encode(&mut seq_bytes)
            .expect("buffer size too small");

        let seq_path = SeqRecvPath(msg.packet.port_on_b.clone(), msg.packet.chan_on_b.clone());

        let proof: MerkleProof = msg.proof_unreceived_on_b.clone().try_into()?;

        verify_merkle_proof(
            &trusted_client_state.proof_specs,
            &connection.counterparty.prefix.clone().into(),
            &proof,
            &trusted_consensus_state.root,
            seq_path,
            seq_bytes,
        )?;

        Ok(())
    }

    async fn verify_packet_timeout_absence_proof(
        &self,
        connection: &ConnectionEnd,
        msg: &MsgTimeout,
    ) -> anyhow::Result<()> {
        let (trusted_client_state, trusted_consensus_state) = self
            .get_trusted_client_and_consensus_state(
                &connection.client_id,
                &msg.proof_height_on_b,
                connection,
            )
            .await?;

        let receipt_path = ReceiptPath {
            port_id: msg.packet.port_on_b.clone(),
            channel_id: msg.packet.chan_on_b.clone(),
            sequence: msg.packet.sequence,
        };

        let proof = MerkleProof::try_from(msg.proof_unreceived_on_b.clone())?;

        verify_merkle_absence_proof(
            &trusted_client_state.proof_specs,
            &connection.counterparty.prefix.clone().into(),
            &proof,
            &trusted_consensus_state.root,
            receipt_path,
        )?;

        Ok(())
    }
}

impl<T: StateRead> PacketProofVerifier for T {}

mod inner {
    use super::*;

    #[async_trait]
    pub trait Inner: StateReadExt {
        async fn get_trusted_client_and_consensus_state(
            &self,
            client_id: &ClientId,
            height: &Height,
            connection: &ConnectionEnd,
        ) -> anyhow::Result<(TendermintClientState, TendermintConsensusState)> {
            let trusted_client_state = self.get_client_state(client_id).await?;

            // TODO: should we also check if the client is expired here?
            if trusted_client_state.is_frozen() {
                return Err(anyhow::anyhow!("client is frozen"));
            }

            let trusted_consensus_state = self
                .get_verified_consensus_state(*height, client_id.clone())
                .await?;

            let tm_client_state = trusted_client_state;

            tm_client_state.verify_height(*height)?;

            // verify that the delay time has passed (see ICS07 tendermint IBC client spec for
            // more details)
            let current_timestamp = self.get_block_timestamp().await?;
            let current_height = self.get_block_height().await?;
            let processed_height = self.get_client_update_height(client_id, height).await?;
            let processed_time = self.get_client_update_time(client_id, height).await?;

            // NOTE: hardcoded for now, should probably be a chain parameter.
            let max_time_per_block = std::time::Duration::from_secs(20);

            let delay_period_time = connection.delay_period;
            let delay_period_blocks =
                calculate_block_delay(&delay_period_time, &max_time_per_block);

            TendermintClientState::verify_delay_passed(
                current_timestamp.into(),
                Height::new(0, current_height)?,
                processed_time,
                processed_height,
                delay_period_time,
                delay_period_blocks,
            )?;

            Ok((tm_client_state, trusted_consensus_state))
        }
    }

    impl<T: StateReadExt> Inner for T {}
}