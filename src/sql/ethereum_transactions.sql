SELECT
    t.transaction_hash,
    t.base_fee_per_gas,
    t.block_number,
    t.contract_address,
    t.fees_burned,
    t.fees_rewarded,
    t.fees_saved,
    t.from_address,
    t.gas_limit,
    t.gas_price,
    t.gas_used,
    t.input,
    t.internal_failed_transaction_count,
    t.internal_transaction_count,
    t.log_count,
    t.max_fee_per_gas,
    t.max_priority_fee_per_gas,
    t.nonce,
    t.output,
    t.position,
    t.timestamp,
    t.to_address,
    t.transaction_fee,
    t.type,
    t.value
FROM ethereum.transactions t
WHERE t.from_address = '{{wallet_address}}'
   OR t.to_address = '{{wallet_address}}'
ORDER BY t.timestamp DESC
LIMIT {{limit}}
OFFSET {{offset}}