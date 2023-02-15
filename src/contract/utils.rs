use crate::contract::tx_manager::TransactionGuard;
use dao_io::{DaoAction, DaoEvent};
use ft_logic_io::Action as FTLAction;
use ft_main_io::{FTokenAction, FTokenEvent};
use gear_lib::non_fungible_token::{
    io::NFTTransfer,
    token::{TokenId, TokenMetadata},
};
use gstd::{
    errors::Result as GstdResult,
    msg::{self, CodecMessageFuture},
    prelude::*,
    ActorId,
};
use nft_io::{NFTAction, NFTEvent};
use supply_chain_io::*;

fn send<T: Decode>(actor: ActorId, payload: impl Encode) -> GstdResult<CodecMessageFuture<T>> {
    msg::send_for_reply_as(actor, payload, 0)
}

fn nft_event_to_transfer(event: GstdResult<NFTEvent>) -> Result<NFTTransfer, Error> {
    if let NFTEvent::Transfer(transfer) = event? {
        Ok(transfer)
    } else {
        Err(Error::NFTTransferFailed)
    }
}

pub async fn mint_nft<T>(
    tx_guard: &mut TransactionGuard<'_, T>,
    non_fungible_token: ActorId,
    token_metadata: TokenMetadata,
) -> Result<TokenId, Error> {
    let transfer = nft_event_to_transfer(
        send(
            non_fungible_token,
            NFTAction::Mint {
                transaction_id: tx_guard.step()?,
                token_metadata,
            },
        )?
        .await,
    )
    .map_err(|error| {
        if error == Error::NFTTransferFailed {
            Error::NFTMintingFailed
        } else {
            error
        }
    })?;

    Ok(transfer.token_id)
}

pub async fn transfer_nft<T>(
    tx_guard: &mut TransactionGuard<'_, T>,
    non_fungible_token: ActorId,
    to: ActorId,
    token_id: TokenId,
) -> Result<(), Error> {
    nft_event_to_transfer(
        send(
            non_fungible_token,
            NFTAction::Transfer {
                transaction_id: tx_guard.step()?,
                to,
                token_id,
            },
        )?
        .await,
    )?;

    Ok(())
}

pub async fn transfer_ftokens<T>(
    tx_guard: &mut TransactionGuard<'_, T>,
    fungible_token: ActorId,
    sender: ActorId,
    recipient: ActorId,
    amount: u128,
) -> Result<(), Error> {
    let payload = FTokenAction::Message {
        transaction_id: tx_guard.step()?,
        payload: FTLAction::Transfer {
            sender,
            recipient,
            amount,
        }
        .encode(),
    };

    if FTokenEvent::Ok != send(fungible_token, payload)?.await? {
        Err(Error::FTTransferFailed)
    } else {
        Ok(())
    }
}

pub async fn dao_action<T>(
    dao: ActorId,
) -> Result<(), Error> {
    // TODO: implementation dao action

    todo!();
}