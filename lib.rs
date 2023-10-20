#![cfg_attr(not(feature = "std"), no_std, no_main)]
/// The `Raiser` contract is a blockchain-based funding system implemented in Rust.
/// It allows users (contributors) to contribute funds to a pool and request payouts.
///
/// The contract maintains a record of the total supply of funds, the amount funded by each address,
/// and a list of contributors. It also keeps track of the minimum contribution amount, the owner of the contract,
/// and the maximum number of contributors allowed.
///
/// The contract emits events when a token transfer occurs and when an approval occurs that a spender is allowed to withdraw.
///
/// The contract has several key functions:
/// - `new` and `default`: Constructors for creating a new instance of the contract.
/// - `set_max_contributors`: Sets a new maximum number of contributors. Only the owner can call this function.
/// - `get_max_contributors`: Returns the maximum number of contributors.
/// - `contribute`: Allows a user to contribute funds to the pool.
/// - `get_contributors`: Returns a list of contributors and their respective balances.
/// - `request_token`: Allows a contributor to request a payout.
/// - `approve_request`: Allows the owner to approve a payout request.
/// - `get_next_requester`: Returns the AccountId of the next eligible requester.
/// - `get_completed_payouts`: Returns the number of completed payouts.
/// - `get_payout_history`: Returns the payout history.
/// - `next_contribution_cycle`: Initiates the next contribution cycle.
/// - `all_paid`: Checks if all contributors have been paid.
/// - `get_total_supply`: Returns the total token supply.
/// - `total_contributors`: Returns the total number of contributors.
/// - `balance_of`: Returns the balance of a specific account.
///
/// The contract also defines several error types for handling common error scenarios.
///
/// The contract uses the ink! smart contract programming language, which is designed for writing secure and efficient blockchain contracts with Rust.

#[ink::contract]
/// The `Raiser` struct represents a blockchain-based funding system.
///
/// It contains several fields:
/// - `total_supply`: The total amount of funds in the system.
/// - `address_to_amount_funded`: A mapping from account IDs to the amount they have funded and a boolean indicating if they have contributed.
/// - `contributed`: A mapping from account IDs to a boolean indicating if they have contributed.
/// - `balance`: A vector of tuples, each containing an account ID and the balance of that account.
/// - `min_amount`: The minimum amount that can be contributed.
/// - `owner`: The account ID of the owner of the contract.
/// - `contributors`: A vector of account IDs of the contributors.
/// - `contributors_count`: The total number of contributors.
/// - `requests`: A vector of tuples, each containing an account ID and the amount they have requested.
/// - `completed_payouts`: The total number of completed payouts.
/// - `payout_history`: A vector of tuples, each containing an account ID and the amount they have been paid.
/// - `max_contributors`: The maximum number of contributors allowed.
/// - `contribution_cycle`: The current contribution cycle.
///
/// The struct is used to manage the state of the contract, including the total supply of funds, the contributors, and the payouts.
mod raiser {
    use ink::storage::Mapping;
    #[ink(storage)]
    pub struct Raiser {
        total_supply: Balance,
        address_to_amount_funded: Mapping<AccountId, (Balance, bool)>,
        contributed: Mapping<AccountId, bool>,
        balance: Vec<(AccountId, Balance)>,
        min_amount:Balance,
        owner:AccountId,
        contributors: Vec<AccountId>, 
        contributors_count: u128, 
        requests: Vec<(AccountId, Balance)>,
        completed_payouts: u128,
        payout_history: Vec<(AccountId, Balance)>,
        max_contributors:u128,
        contribution_cycle:u128

    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }


    /// The ERC-20 error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
   
    /// `Error` is an enumeration of all possible errors that can occur in our blockchain application.
    ///
    /// Variants:
    /// - `InsufficientBalance`: This error occurs when a user tries to make a transaction but their balance is too low.
    /// - `LowAmount`: This error occurs when the amount specified for a transaction is considered too low.
    /// - `NotContractOwner`: This error occurs when a user who is not the owner of the contract tries to perform an action that requires ownership.
    /// - `AlreadyContributed`: This error occurs when a user tries to contribute to a contract but they have already made a contribution.
    /// - `NotNextContributor`: This error occurs when a user tries to request for withdrawal but it's not their turn to withdraw.
    /// - `NotPaymentPhase`: This error occurs when a user tries to request a payment but is not in the payment phase.
    /// - `TransferError`: This error occurs when there's a problem transferring funds between accounts.
    pub enum Error {
        InsufficientBalance,
        LowAmount,
        NotContractOwner,
        AlreadyContributed,
        NotNextContributor,
        NotPaymentPhase,
        TransferError,
    }

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;
    /// Constructs a new instance of the contract.
    ///
    /// The `new` function is called when the contract is deployed. It initializes the contract with the following default values:
    /// - `owner`: The account ID of the caller who deploys the contract.
    /// - `address_to_amount_funded`: An empty mapping of account IDs to the amount they have funded.
    /// - `contributed`: An empty mapping of contributors.
    /// - `total_supply`: The total supply of tokens, initially set to 0.
    /// - `contributors`: An empty vector of contributors.
    /// - `contributors_count`: The count of contributors, initially set to 0.
    /// - `requests`: An empty vector of requests.
    /// - `completed_payouts`: The count of completed payouts, initially set to 0.
    /// - `payout_history`: An empty vector of payout history.
    /// - `max_contributors`: The maximum number of contributors, initially set to 0.
    /// - `contribution_cycle`: The contribution cycle, initially set to 1.
    /// - `min_amount`: The minimum contribution amount, initially set to 50.
    /// - `balance`: An empty vector of balances.
    ///
    /// Returns the newly created contract instance.
    impl Raiser {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller: ink::primitives::AccountId = Self::env().caller();
            Self{
                owner:caller, 
                address_to_amount_funded:Mapping::default(), 
                contributed:Mapping::default(), 
                total_supply:0,
                contributors:Vec::default(), 
                contributors_count:0, 
                requests:Vec::default(),
                completed_payouts: 0,
                payout_history:Vec::default(),
                max_contributors:0,
                contribution_cycle:1,
                min_amount:50,
                balance:Vec::default(),

            }

        }

        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        /// Sets a new maximum number of contributors.
        ///
        /// This function updates the `max_contributors` field of the contract. 
        /// It can only be called by the owner of the contract. If a non-owner 
        /// attempts to call this function, it will return an `NotContractOwner` error.
        ///
        /// # Arguments
        ///
        /// * `new_max` - The new maximum number of contributors.
        ///
        /// # Returns
        ///
        /// * `Ok(())` if the `max_contributors` was successfully updated.
        /// * `Err(Error::InsufficientAllowance)` if the caller is not the owner of the contract.
        
        #[ink(message)]
        pub fn set_max_contributors(&mut self, new_max: u128) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotContractOwner);
            }
            self.max_contributors = new_max;
            Ok(())
        }

        /// Returns the maximum number of contributors.
        ///
        /// This function returns the `max_contributors` field of the contract. 
        /// It can be called by any user to know the maximum number of contributors allowed.
        ///
        /// # Returns
        ///
        /// * `u128` - The maximum number of contributors.
        
        #[ink(message)]
        pub fn get_max_contributors(&self) -> u128 {
            self.max_contributors
        }

        /// Allows a user to contribute to the contract.
        ///
        /// The `contribute` function is called when a user wants to contribute to the contract. It performs the following operations:
        /// - Checks if the caller has already contributed. If so, it returns an `AlreadyContributed` error.
        /// - Checks if the transferred value is less than the minimum amount. If so, it returns a `LowAmount` error.
        /// - Retrieves the amount the caller has already funded.
        /// - Increments the contributors count, adds the caller to the contributors list, and marks the caller as having contributed.
        /// - Updates the amount the caller has funded and their balance.
        /// - Increases the total supply by the transferred value.
        /// - Emits a `Transfer` event with the new total supply.
        ///
        /// Returns `Ok(())` if the contribution is successful, or an `Error` if not.

        #[ink(message, payable)]
        pub fn contribute(&mut self) -> Result<()> {
            let caller: ink::primitives::AccountId = self.env().caller();

             if self.contributed.get(&caller).is_some() {
                return Err(Error::AlreadyContributed);
            }

            let value: u128 = self.env().transferred_value();
            
            if value < self.min_amount {
                return Err(Error::LowAmount);
            }

            let funded_amount: u128 = self.balance_of(caller);

            self.contributors_count += 1;
            self.contributors.push(caller);
            self.contributed.insert(caller, &true);
        
            self.address_to_amount_funded.insert(caller, &(funded_amount + value, true));
            self.balance.push((caller, funded_amount + value));

            self.total_supply += value;

            Self::env().emit_event(
                Transfer {
                from: None,
                to: Some(caller),
                value: self.total_supply,
            });
            Ok(())
        }

        /// Retrieves the list of contributors and their balances.
        ///
        /// The `get_contributors` function iterates over the list of contributors, retrieves the balance for each contributor using the `balance_of` function, and adds a tuple of the account ID and balance to the `contributors` vector.
        ///
        /// Returns a vector of tuples, where each tuple contains an account ID and the corresponding balance.

        #[ink(message)]
        pub fn get_contributors(&self) -> Vec<(AccountId, Balance)> {
            let mut contributors = Vec::new();
            for account_id in &self.contributors {
                let balance = self.balance_of(account_id.clone());
                contributors.push((account_id.clone(), balance));
            }
            contributors
        }

        /// Allows a contributor to request tokens.
        ///
        /// The `request_token` function is called when a contributor wants to request tokens. It performs the following operations:
        /// - Checks if the number of contributors has reached the maximum limit. If not, it returns a `NotPaymentPhase` error.
        /// - Checks if the caller is the first contributor in the list. If not, it returns a `NotNextContributor` error.
        /// - If the caller is the first contributor, it adds a request for the total supply of tokens to the `requests` vector.
        ///
        /// Returns `Ok(())` if the token request is successful, or an `Error` if not.

        #[ink(message)]
        pub fn request_token(&mut self) -> Result<()> {

            if self.contributors_count == self.max_contributors as u128 {
                let caller = self.env().caller();

                if Some(&caller) == self.contributors.first() {
                    let amount = self.total_supply; 
                     self.requests.push((caller, amount));
                } else {
                    return Err(Error::NotNextContributor)
                }
               
            }else{
                return Err(Error::NotPaymentPhase)
            }
            Ok(())
        }

        /// Allows the contract owner to approve a token request.
        ///
        /// The `approve_request` function is called when the contract owner wants to approve a token request. It performs the following operations:
        /// - Checks if the caller is the contract owner. If not, it returns a `NotContractOwner` error.
        /// - Attempts to transfer the requested amount of tokens to the requester. If the transfer fails, it returns a `TransferError`.
        /// - If the transfer is successful, it resets the `requests` vector, removes the first contributor, increments the `completed_payouts` count, and logs the number of completed payouts.
        /// - Adds the payout to the `payout_history`, resets the `contributed` mapping, and starts the next contribution cycle.
        /// - Emits a `Transfer` event with the amount of tokens transferred.
        ///
        /// Returns `Ok(())` if the approval and transfer are successful, or an `Error` if not.

        #[ink(message)]
        pub fn approve_request(&mut self, caller:AccountId) -> Result<()> {
           //  let caller: ink::primitives::AccountId = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotContractOwner);
            }

            // Transfer token
            let (requester, amount) = self.requests[0];
            match Self::env().transfer(requester, amount) {
                Ok(_value) => {
                    self.requests = Vec::default();
                    self.contributors.remove(0); 
                    self.completed_payouts += 1;
                    self.payout_history.push((requester, amount));
                    self.contributed = Mapping::default();
                   
                    self.next_contribution_cycle();
        
                    self.env().emit_event(Transfer {
                        from: Some(self.owner),
                        to: Some(requester.clone()),
                        value:amount,
                    });        
                },
                Err(_e) => {
                    return Err(Error::TransferError);
                }
            }

           
            Ok(())
        }
        
        /// This function returns the AccountId of the next eligible requester.
        /// It does this by checking the first contributor in the queue (the next eligible requester).
        /// If there are no contributors in the queue, it returns `None`.
        
        #[ink(message)]
        pub fn get_next_requester(&self) -> Option<AccountId> {
            // Use the `first` method to get a reference to the first contributor in the queue.
            // This returns an `Option<&AccountId>`, so we use the `map` method to transform the data.
            self.contributors.first().map(|account_id| {
                // Return the AccountId.
                // We dereference `account_id` to get the AccountId from the reference.
                *account_id
            })
        }
        
        /// Retrieves the number of completed payouts.
        ///
        /// The `get_completed_payouts` function is called to get the count of completed payouts from the contract.
        ///
        /// Returns the number of completed payouts as a `u128`.

       #[ink(message)]
        pub fn get_completed_payouts(&self) -> u128 {
            self.completed_payouts
        }
        
        /// Retrieves the payout history.
        ///
        /// The `get_payout_history` function is called to get the history of payouts from the contract. Each payout is represented as a tuple containing the account ID of the recipient and the amount of the payout.
        ///
        /// Returns a vector of tuples, where each tuple contains an account ID and the corresponding payout amount.

        #[ink(message)]
        pub fn get_payout_history(&self) -> Vec<(AccountId, Balance)> {
            self.payout_history.clone()
        }
    
        
        /// Starts the next contribution cycle.
        ///
        /// The `next_contribution_cycle` function is called to start a new contribution cycle. It performs the following operations:
        /// - Checks if all contributors have been paid. If not, it does nothing.
        /// - If all contributors have been paid and the length of the payout history is equal to the number of contributors, it resets the `address_to_amount_funded` mapping, the `payout_history` vector, and the `contributors_count`, increments the `contribution_cycle`, and resets the `completed_payouts` count.

        #[ink(message)]
        pub fn next_contribution_cycle(&mut self){
            let all_paid =  self.all_paid();
            if all_paid {
                if self.payout_history.len() as u128 == self.contributors_count {
                    self.address_to_amount_funded = Mapping::default();
                    self.payout_history = Vec::default();
                    self.contributors_count = 0;
                    self.contribution_cycle+= 1;
                    self.completed_payouts = 0;
                 }
            }
           
        }
        
        /// Checks if all contributors have been paid.
        ///
        /// The `all_paid` function is called to check if all contributors have been paid. It iterates over the list of contributors and checks the `paid` status for each contributor in the `address_to_amount_funded` mapping.
        ///
        /// Returns `true` if all contributors have been paid, or `false` if at least one contributor has not been paid.

        #[ink(message)]
        pub fn all_paid(&self) -> bool {
            for key in &self.contributors {
                let (_, paid) = self.address_to_amount_funded.get(&key).unwrap_or((0, false));
                if !paid {
                    return false;
                }
            }
            true
        }

        /// Returns the total token supply.
        #[ink(message)]
        pub fn get_total_supply(&self) -> Balance {
          //  let balance: u128 =  self.env().balance();
            let balance: u128 =  self.total_supply;
            balance
        }

        /// Retrieves the total number of contributors.
        ///
        /// The `total_contributors` function is called to get the total count of contributors from the contract.
        ///
        /// Returns the total number of contributors as a `u128`.
         #[ink(message)]
         pub fn total_contributors(&self) -> u128 {
             self.contributors_count
         }

        /// Retrieves the balance of a specific account.
        ///
        /// The `balance_of` function is called to get the balance of a specific account from the contract. It iterates over the `balance` vector and returns the balance for the given account ID.
        ///
        /// Returns the balance of the given account as a `Balance`. If the account does not exist in the `balance` vector, it returns 0.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            for (account_id, balance) in &self.balance {
                if account_id == &owner {
                    return *balance;
                }
            }
            0
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;
        /// Test case for the initialization of the `Raiser` contract.
        ///
        /// This test creates a new instance of the `Raiser` contract and checks if it is initialized with the correct default values.
        /// It asserts that the total supply of tokens, the total number of contributors, the number of completed payouts, 
        /// the maximum number of contributors, and the length of the payout history are all zero.
        #[ink::test]
        fn it_works() {
            let  contract = Raiser::default();
            assert_eq!(contract.get_total_supply(), 0);
            assert_eq!(contract.total_contributors(), 0);
            assert_eq!(contract.get_completed_payouts(), 0);
            assert_eq!(contract.get_max_contributors(), 0);
            assert_eq!(contract.get_payout_history().len(), 0);
        }

        /// Test case for the `set_max_contributors` function of the `Raiser` contract.
        ///
        /// This test simulates the owner's call to `set_max_contributors` function.
        /// It first sets up the testing environment by getting the default accounts and setting the callee and caller.
        /// The callee is set to the contract's account ID and the caller is set to Alice's account (the owner).
        ///
        /// Then it creates a new instance of the `Raiser` contract and asserts that the initial maximum number of contributors is zero.
        ///
        /// It then calls `set_max_contributors` to set the maximum number of contributors to 10 and asserts that the function returns `Ok(())`.
        ///
        /// Finally, it asserts that the maximum number of contributors is now 10.
        #[ink::test]
        fn set_max_contributors_works() {
            let accounts =
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let contract = ink::env::account_id::<ink::env::DefaultEnvironment>();
            ink::env::test::set_callee::<ink::env::DefaultEnvironment>(contract);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let mut contract: Raiser = Raiser::new();
            assert_eq!(contract.get_max_contributors(), 0);
            assert_eq!(contract.set_max_contributors(10), Ok(()));
            assert_eq!(contract.get_max_contributors(), 10);
        }

        /// This test function verifies the functionality of the `contribute` function in the `Raiser` contract.
        /// It first creates a new instance of the `Raiser` contract and asserts that the total supply of tokens is initially zero.
        /// Then, it simulates a user (Alice) contributing to the contract by setting the transferred value to 100.
        /// Finally, it asserts that the total supply of tokens has increased by the contributed amount (100 in this case).
        #[ink::test]
        fn contribute_works() {
            let mut contract = Raiser::new();
            assert_eq!(contract.get_total_supply(), 0);
            let accounts =
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert_eq!(contract.contribute(), Ok(()));
            assert_eq!(contract.get_total_supply(), 100);

             // Try to contribute again
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(200);
            // Check if the contribution fails as expected
            assert_eq!(contract.contribute(), Err(Error::AlreadyContributed));
            // Check if the total supply is still the same
            assert_eq!(contract.get_total_supply(), 100);

            // Check if Alice is in the list of contributors
            let contributors = contract.get_contributors();
            assert_eq!(contributors.len(), 1);
            assert_eq!(contributors[0].0, accounts.alice);
            assert_eq!(contributors[0].1, 100);
         }

        /// This test function verifies the functionality of the `request_token` function in the `Raiser` contract.
        /// It first creates a new instance of the `Raiser` contract and sets the maximum number of contributors to 1.
        /// Then, it simulates a user (Alice) contributing to the contract.
        /// After the contribution, Alice requests a token.
        /// Finally, it asserts that Alice's request has been added to the requests list.
        #[ink::test]
        fn request_token_works() {
            let mut contract = Raiser::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert_eq!(contract.set_max_contributors(1), Ok(()));
            assert_eq!(contract.contribute(), Ok(()));
            assert_eq!(contract.request_token(), Ok(()));
            let requests = contract.requests;
            assert_eq!(requests.len(), 1);
            assert_eq!(requests[0].0, accounts.alice);
        }


        #[ink::test]
        fn approve_request_works() {
            let mut contract = Raiser::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            // Simulate a contribution from Alice
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert_eq!(contract.set_max_contributors(1), Ok(()));
            assert_eq!(contract.contribute(), Ok(()));

            contract.request_token().unwrap();
             
            // Try to approve the request as the owner
            assert_eq!(contract.approve_request(contract.owner), Ok(()));
          
        }

        // This test checks the functionality of the `get_next_requester` function.
        // It simulates contributions and token requests from two accounts, Alice and Bob.
        // The test verifies that `get_next_requester` correctly returns the account that should be the next to receive tokens.
        // Even after Bob makes a request, Alice is returned as the next requester because her request came first.
        #[ink::test]
        fn get_next_requester_works() {
            let mut contract = Raiser::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Initially, there should be no next requester
            assert_eq!(contract.get_next_requester(), None);

            // Simulate a contribution and token request from Alice
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert_eq!(contract.set_max_contributors(2), Ok(()));
            assert_eq!(contract.contribute(), Ok(()));

            // Now, Alice should be the next requester
            assert_eq!(contract.get_next_requester(), Some(accounts.alice));

            // Simulate a contribution and token request from Bob
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert_eq!(contract.contribute(), Ok(()));

            // Alice should still be the next requester, because Bob's request comes after Alice's
            assert_eq!(contract.get_next_requester(), Some(accounts.alice));
        }

        // This test checks the functionality of the `get_completed_payouts` function.
        // It simulates a contribution and token request from Alice, and then approves the request.
        // The test verifies that `get_completed_payouts` correctly returns the number of completed payouts.
        // After the approval of Alice's request, the number of completed payouts should increase by one.

        #[ink::test]
        fn get_completed_payouts_works() {
            let mut contract = Raiser::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Initially, there should be no completed payouts
            assert_eq!(contract.get_completed_payouts(), 0);

            // Simulate a contribution and token request from Alice
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert_eq!(contract.set_max_contributors(2), Ok(()));
            assert_eq!(contract.contribute(), Ok(()));

            // Simulate a contribution from bob
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert_eq!(contract.contribute(), Ok(()));
           
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert_eq!(contract.request_token(), Ok(()));

            // Approve the request
            assert_eq!(contract.approve_request(contract.owner), Ok(()));

            // Now, there should be one completed payout
             assert_eq!(contract.get_completed_payouts(), 1);
            
            
             assert_eq!(contract.get_payout_history().len(),1);

        }

        // This test checks the functionality of the `next_contribution_cycle` function.
        // It calls `next_contribution_cycle` twice and checks that the cycle number increases each time.
        // The test verifies that `next_contribution_cycle` correctly increments the cycle counter.
        #[ink::test]
        fn next_contribution_cycle_works() {
            let mut contract = Raiser::new();

            // Initially, we should be in the first contribution cycle
            assert_eq!(contract.contribution_cycle, 1);

            // Move to the next contribution cycle
            contract.next_contribution_cycle();

            // Now, we should be in the second contribution cycle
            assert_eq!(contract.contribution_cycle, 2);
            
        }
        #[ink::test]
        fn total_contributors_works() {
            let mut contract = Raiser::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Initially, there should be no contributors
            assert_eq!(contract.total_contributors(), 0);

            // Simulate a contribution from Alice
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert_eq!(contract.set_max_contributors(1), Ok(()));
            assert_eq!(contract.contribute(), Ok(()));

            // Now, there should be one contributor
            assert_eq!(contract.total_contributors(), 1);

            // Simulate a contribution from Bob
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert_eq!(contract.contribute(), Ok(()));

            // Now, there should be two contributors
            assert_eq!(contract.total_contributors(), 2);
        }

    }
   
}


