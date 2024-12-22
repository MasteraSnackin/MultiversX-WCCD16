#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait CitizenNFT {
    /// Initialize the contract. This function is called once when the contract is deployed.
    #[init]
    fn init(&self) {}

    /// Endpoint to allow users to mint a "CITIZEN" NFT by burning 10 WOOD and 15 FOOD tokens.
    #[payable("*")]
    #[endpoint(mintCitizen)]
    fn mint_citizen(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let wood_token_id: TokenIdentifier = TokenIdentifier::from("WOOD");
        let food_token_id: TokenIdentifier = TokenIdentifier::from("FOOD");

        // Check if the correct amount of WOOD tokens are sent
        let wood_amount = self.call_value().egld_value();
        require!(wood_amount == 10u64.into(), "Require 10 WOOD tokens");

        // Check if the correct amount of FOOD tokens are sent
        let food_amount = self.call_value().egld_value();
        require!(food_amount == 15u64.into(), "Require 15 FOOD tokens");

        // Burn the tokens: Use the token burn mechanism provided by the token contract
        self.send().direct(&ManagedAddress::zero(), &wood_token_id, 0, &wood_amount, &[]);
        self.send().direct(&ManagedAddress::zero(), &food_token_id, 0, &food_amount, &[]);

        // Schedule the NFT minting event for 1 hour later
        let current_block_time = self.blockchain().get_block_timestamp();
        let mint_time = current_block_time + self.one_hour_in_seconds();
        self.pending_mint(&caller).set(mint_time);

        Ok(())
    }

    /// Endpoint to claim the "CITIZEN" NFT after the waiting period.
    #[endpoint(claimCitizen)]
    fn claim_citizen(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();
        let mint_time = self.pending_mint(&caller).get();

        require!(current_time >= mint_time, "Minting period not yet passed");

        // Mint the "CITIZEN" NFT for the caller
        self.issue_nft(&caller)?;

        // Clear the pending mint entry
        self.pending_mint(&caller).clear();

        Ok(())
    }

    /// Helper function to issue the NFT.
    fn issue_nft(&self, recipient: &ManagedAddress) -> SCResult<()> {
        // Details of the NFT issuance would go here.
        // This would involve creating an NFT and assigning it to the recipient.
        Ok(())
    }

    /// Calculate the number of seconds equivalent to one hour.
    fn one_hour_in_seconds(&self) -> u64 {
        3600 // 1 hour = 3600 seconds
    }

    /// Storage mapper to keep track of pending mints for each user.
    #[view(getPendingMint)]
    #[storage_mapper("pendingMint")]
    fn pending_mint(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;
}
