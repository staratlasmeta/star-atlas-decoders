use super::MarketplaceDecoder;
pub mod add_fee_exemption;
pub mod add_royalty_tier;
pub mod delete_royalty_tier;
pub mod deregister_currency;
pub mod initialize_marketplace;
pub mod initialize_open_orders_counter;
pub mod noop;
pub mod process_cancel;
pub mod process_exchange;
pub mod process_initialize_buy;
pub mod process_initialize_buy_pda;
pub mod process_initialize_sell;
pub mod process_initialize_sell_pda;
pub mod register_currency;
pub mod remove_fee_exemption;
pub mod update_atlas_rate;
pub mod update_currency_royalty;
pub mod update_currency_vault;
pub mod update_royalty_tier;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum MarketplaceInstruction {
    AddFeeExemption(add_fee_exemption::AddFeeExemption),
    AddRoyaltyTier(add_royalty_tier::AddRoyaltyTier),
    DeleteRoyaltyTier(delete_royalty_tier::DeleteRoyaltyTier),
    DeregisterCurrency(deregister_currency::DeregisterCurrency),
    InitializeMarketplace(initialize_marketplace::InitializeMarketplace),
    InitializeOpenOrdersCounter(initialize_open_orders_counter::InitializeOpenOrdersCounter),
    Noop(noop::Noop),
    ProcessCancel(process_cancel::ProcessCancel),
    ProcessExchange(process_exchange::ProcessExchange),
    ProcessInitializeBuy(process_initialize_buy::ProcessInitializeBuy),
    ProcessInitializeBuyPda(process_initialize_buy_pda::ProcessInitializeBuyPda),
    ProcessInitializeSell(process_initialize_sell::ProcessInitializeSell),
    ProcessInitializeSellPda(process_initialize_sell_pda::ProcessInitializeSellPda),
    RegisterCurrency(register_currency::RegisterCurrency),
    RemoveFeeExemption(remove_fee_exemption::RemoveFeeExemption),
    UpdateAtlasRate(update_atlas_rate::UpdateAtlasRate),
    UpdateCurrencyRoyalty(update_currency_royalty::UpdateCurrencyRoyalty),
    UpdateCurrencyVault(update_currency_vault::UpdateCurrencyVault),
    UpdateRoyaltyTier(update_royalty_tier::UpdateRoyaltyTier),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for MarketplaceDecoder {
    type InstructionType = MarketplaceInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            MarketplaceInstruction::AddFeeExemption => add_fee_exemption::AddFeeExemption,
            MarketplaceInstruction::AddRoyaltyTier => add_royalty_tier::AddRoyaltyTier,
            MarketplaceInstruction::DeleteRoyaltyTier => delete_royalty_tier::DeleteRoyaltyTier,
            MarketplaceInstruction::DeregisterCurrency => deregister_currency::DeregisterCurrency,
            MarketplaceInstruction::InitializeMarketplace => initialize_marketplace::InitializeMarketplace,
            MarketplaceInstruction::InitializeOpenOrdersCounter => initialize_open_orders_counter::InitializeOpenOrdersCounter,
            MarketplaceInstruction::Noop => noop::Noop,
            MarketplaceInstruction::ProcessCancel => process_cancel::ProcessCancel,
            MarketplaceInstruction::ProcessExchange => process_exchange::ProcessExchange,
            MarketplaceInstruction::ProcessInitializeBuy => process_initialize_buy::ProcessInitializeBuy,
            MarketplaceInstruction::ProcessInitializeBuyPda => process_initialize_buy_pda::ProcessInitializeBuyPda,
            MarketplaceInstruction::ProcessInitializeSell => process_initialize_sell::ProcessInitializeSell,
            MarketplaceInstruction::ProcessInitializeSellPda => process_initialize_sell_pda::ProcessInitializeSellPda,
            MarketplaceInstruction::RegisterCurrency => register_currency::RegisterCurrency,
            MarketplaceInstruction::RemoveFeeExemption => remove_fee_exemption::RemoveFeeExemption,
            MarketplaceInstruction::UpdateAtlasRate => update_atlas_rate::UpdateAtlasRate,
            MarketplaceInstruction::UpdateCurrencyRoyalty => update_currency_royalty::UpdateCurrencyRoyalty,
            MarketplaceInstruction::UpdateCurrencyVault => update_currency_vault::UpdateCurrencyVault,
            MarketplaceInstruction::UpdateRoyaltyTier => update_royalty_tier::UpdateRoyaltyTier,
        )
    }
}
