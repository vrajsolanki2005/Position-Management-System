use crate::constants::RATE_SCALE;
use crate::errors::PerpError;
use crate::state::accounts::Side;
use anchor_lang::require;

pub fn calc_unrealized_pnl(side: Side, size: u64, entry: u64, price: u64) -> Result<i128, anchor_lang::prelude::Error> {
    let s = size as i128;
    let diff = (price as i128) - (entry as i128);
    Ok(match side {
        Side::Long => s.checked_mul(diff).ok_or(PerpError::Overflow)?,
        Side::Short => s.checked_mul(-diff).ok_or(PerpError::Overflow)?,
    })
}

pub fn calc_realized_pnl_partial(side: Side, reduce_size: u64, entry: u64, price: u64) -> Result<i128, anchor_lang::prelude::Error> {
    let s = reduce_size as i128;
    let diff = (price as i128) - (entry as i128);
    Ok(match side {
        Side::Long => s.checked_mul(diff).ok_or(PerpError::Overflow)?,
        Side::Short => s.checked_mul(-diff).ok_or(PerpError::Overflow)?,
    })
}

pub fn calc_realized_pnl_full(side: Side, size: u64, entry: u64, price: u64) -> Result<i128, anchor_lang::prelude::Error> {
    calc_realized_pnl_partial(side, size, entry, price)
}

// Long: P = (size*entry - margin) * RATE_SCALE / (size*(RATE_SCALE - mmr))
// Short: P = (margin + size*entry) * RATE_SCALE / (size*(RATE_SCALE + mmr))
pub fn calc_liquidation_price(side: Side, size: u64, entry: u64, margin: u64, mmr_scaled: u64) -> Result<u64, anchor_lang::prelude::Error> {
    use crate::math::*;
    use crate::errors::PerpError;

    require!(size > 0, PerpError::InvalidSize);
    let size_u = size as u128;
    let entry_u = entry as u128;
    let margin_u = margin as u128;
    let mmr_u = mmr_scaled as u128;
    let rs = RATE_SCALE as u128;

    let price_u = match side {
        Side::Long => {
            let numer = sub_u128(mul_u128(size_u, entry_u)?, margin_u)?;
            let denom = mul_u128(size_u, sub_u128(rs, mmr_u)?)?;
            require!(denom > 0, PerpError::InvalidState);
            div_u128(mul_u128(numer, rs)?, denom)?
        }
        Side::Short => {
            let numer = add_u128(margin_u, mul_u128(size_u, entry_u)?)?;
            let denom = mul_u128(size_u, add_u128(rs, mmr_u)?)?;
            require!(denom > 0, PerpError::InvalidState);
            div_u128(mul_u128(numer, rs)?, denom)?
        }
    };
    u128_to_u64(price_u)
}

// Safe math helpers
pub fn add_u128(a: u128, b: u128) -> Result<u128, anchor_lang::prelude::Error> { a.checked_add(b).ok_or(PerpError::Overflow.into()) }
pub fn sub_u128(a: u128, b: u128) -> Result<u128, anchor_lang::prelude::Error> { a.checked_sub(b).ok_or(PerpError::Overflow.into()) }
pub fn mul_u128(a: u128, b: u128) -> Result<u128, anchor_lang::prelude::Error> { a.checked_mul(b).ok_or(PerpError::Overflow.into()) }
pub fn mul_u128_u64(a: u128, b: u64) -> Result<u128, anchor_lang::prelude::Error> { a.checked_mul(b as u128).ok_or(PerpError::Overflow.into()) }
pub fn mul_i128_i128(a: i128, b: i128) -> Result<i128, anchor_lang::prelude::Error> { a.checked_mul(b).ok_or(PerpError::Overflow.into()) }
pub fn div_u128(a: u128, b: u128) -> Result<u128, anchor_lang::prelude::Error> {
    require!(b != 0, PerpError::DivisionByZero);
    Ok(a / b)
}
pub fn u128_to_u64(v: u128) -> Result<u64, anchor_lang::prelude::Error> { u64::try_from(v).map_err(|_| PerpError::Overflow.into()) }
pub fn i128_to_i64(v: i128) -> Result<i64, anchor_lang::prelude::Error> { i64::try_from(v).map_err(|_| PerpError::Overflow.into()) }
pub fn i128_to_u64(v: i128) -> Result<u64, anchor_lang::prelude::Error> {
    use crate::errors::PerpError;
    if v < 0 { return Err(PerpError::Underflow.into()); }
    u64::try_from(v).map_err(|_| PerpError::Overflow.into())
}