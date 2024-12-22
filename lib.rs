#![no_std]

// Import necessary modules and macros from the elrond_wasm library.
elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait CitizenNFT {
    /// Initialize the contract. This function is called once when the contract is deployed.
    #[init]
    fn init(&self) {}

    /// Endpoint to allow users to mint a "CITIZEN" NFT by burning 10 wood and 15 food tokens.
    #[payable("*")]
    #[endpoint(mintCitizen)]
    fn mint_citizen(&self) -> SCResult<()> {
        // Ensure the correct amounts of "WOOD" and "FOOD" tokens are sent.
        let wood_amount = self.call_value().get_extra("WOOD");
        let food_amount = self.call_value().get_extra("FOOD");

        require!(wood_amount == 10u64.into(), "Require 10 WOOD tokens");
        require!(food_amount == 15u64.into(), "Require 15 FOOD tokens");

        // Burn the tokens by sending them to the zero address.
        self.send().direct(&ManagedAddress::zero(), &TokenIdentifier::from("WOOD"), 0, &wood_amount, &[]);
        self.send().direct(&ManagedAddress::zero(), &TokenIdentifier::from("FOOD"), 0, &food_amount, &[]);

        // Schedule the NFT minting event for 1 hour later.
        let current_block = self.blockchain().get_block_round();
        let mint_block = current_block + self.rounds_for_one_hour();
        self.pending_mint(&self.blockchain().get_caller()).set(mint_block);

        Ok(())
    }

    /// Endpoint to claim the "CITIZEN" NFT after the waiting period.
    #[endpoint(claimCitizen)]
    fn claim_citizen(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let current_block = self.blockchain().get_block_round();
        let mint_block = self.pending_mint(&caller).get();

        require!(current_block >= mint_block, "Minting period not yet passed");

        // Mint the "CITIZEN" NFT for the caller.
        self.issue_nft(&caller)?;

        // Clear the pending mint entry.
        self.pending_mint(&caller).clear();

        Ok(())
    }

    /// Helper function to issue the NFT.
    fn issue_nft(&self, recipient: &ManagedAddress) -> SCResult<()> {
        // Details of the NFT issuance would go here.
        // For simplicity, assume a function that handles NFT issuance.
        Ok(())
    }

    /// Calculate the number of blockchain rounds equivalent to one hour.
    fn rounds_for_one_hour(&self) -> u64 {
        // Assume approximately 6 seconds per round.
        600
    }

    /// Storage mapper to keep track of pending mints for each user.
    #[view(getPendingMint)]
    #[storage_mapper("pendingMint")]
    fn pending_mint(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;
}