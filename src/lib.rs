use cosmwasm_std::{
    to_binary,
    DepsMut,
    CosmosMsg,
    Env,
    MessageInfo,
    Response,
    StdError,
    StdResult,
    WasmMsg,
    Uint128,
    Addr,
};
use osmosis_std::types::cosmos::base::v1beta1::Coin;
use osmosis_std::types::osmosis::cosmwasmpool::v1beta1::SwapExactAmountIn;
// use gravity_proto::gravity::BridgeTransactionRequest;

pub fn perform_token_swap_and_bridge(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    osmosis_dex_contract: String,
    gravity_bridge_contract: String,
    token_to_swap: String,
    desired_token: String,
    amount_to_swap: Coin,
    destination_chain: String,
    recipient_address: String
) -> StdResult<Response> {
    // Step 1: Perform token swap on Osmosis DEX
    // access liquidity pools, calculate swap ratios, and execute the swap transaction

    let swap_msg = SwapExactAmountIn {
        sender: info.sender.to_string(),
        token_in: Some(Coin {
            denom: token_to_swap.clone(),
            amount: amount_to_swap.amount,
        }),
        token_out_denom: desired_token.clone(),
        token_out_min_amount: String::new(),
        swap_fee: String::new(),
    };

    let swap_cosmos_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: osmosis_dex_contract,
        msg: to_binary(&swap_msg)?,
        funds: vec![],
    });

    // Step 2: Bridge the resulting tokens to another chain using Gravity
    // access the Gravity contract, calculate the bridge fee, and execute the bridge(cross-chain) transaction

    // get token amount from swap response
    // Get token amount from swap response
    // let swap_response: SwapResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
    //     contract_addr: osmosis_dex_contract.clone(),
    //     msg: to_binary(&QueryMsg::Swap { ... })?,
    // }))?;

    let swap_amount = swap_response.amount;

    let bridge_tx_request = BridgeTransactionRequest {
        desired_token,
        destination_chain,
        recipient: recipient_address,
        sender: info.sender.to_string(),
        amount: swap_amount,
        fee: Uint128::zero().u128(),
        fee_address: Addr::unchecked(""),
        timestamp: env.block.time.nanos(),
    };
    let bridge_msg = bridge_tx_request.to_bytes().map_err(|_| StdError::generic_err("Failed to serialize bridge transaction request"))?;

    let bridge_cosmos_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: gravity_bridge_contract,
        msg: to_binary(&bridge_msg)?,
        funds: vec![],
    });

    // Create and return the response with both messages
    let msgs = vec![swap_cosmos_msg, bridge_cosmos_msg];
    let res = Response::new().add_messages(msgs);

    Ok(res)
}
