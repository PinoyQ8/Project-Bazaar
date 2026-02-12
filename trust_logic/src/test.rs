
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Ledger, LedgerInfo},
        Address, Env,
    };

    fn create_contract(env: &Env) -> TrustContractClient<'static> {
        let contract_id = env.register_contract(None, TrustContract);
        TrustContractClient::new(env, &contract_id)
    }

    #[test]
    #[should_panic]
    fn vouch_from_non_existent_profile_panics() {
        let env = Env::default();
        let client = create_contract(&env);

        let voucher = Address::random(&env);
        let target = Address::random(&env);

        client.vouch(&voucher, &target);
    }

    #[test]
    fn test_stake_increases_score() {
        let env = Env::default();
        env.mock_all_auths(); // Mock signatures for testing
        let client = create_contract(&env);
        let user = Address::random(&env);

        assert_eq!(client.get_trust(&user), 0);
        assert_eq!(client.is_bonded(&user), false);
        
        client.stake(&user, &None);
        
        assert_eq!(client.get_trust(&user), 10);
        assert_eq!(client.is_bonded(&user), true);
    }

    #[test]
    fn test_decay() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let user = Address::random(&env);
        let admin = Address::random(&env);

        client.init(&admin);

        client.stake(&user, &None); // Score = 10
        client.add_trust(&user);    // Score = 11
        client.decay(&user);        // Score = 8
        assert_eq!(client.get_trust(&user), 8);
    }

    #[test]
    #[should_panic(expected = "Bond is still locked")]
    fn test_withdraw_too_early_panics() {
        let env = Env::default();
        env.mock_all_auths();
        
        // Set mock time
        env.ledger().with_mut(|li| { li.timestamp = 1000; });

        let client = create_contract(&env);
        let user = Address::random(&env);

        client.stake(&user, &None);
        
        // Try to withdraw immediately (should fail)
        client.withdraw(&user);
    }

    #[test]
    fn test_transfer_admin() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let admin = Address::random(&env);
        let new_admin = Address::random(&env);

        client.init(&admin);
        assert_eq!(client.get_admin(), admin);

        client.transfer_admin(&new_admin);
        assert_eq!(client.get_admin(), new_admin);
    }

    #[test]
    fn test_force_unbond() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let admin = Address::random(&env);
        let user = Address::random(&env);

        client.init(&admin);
        client.stake(&user, &None);
        assert_eq!(client.is_bonded(&user), true);

        // Admin forces unbond
        client.force_unbond(&user);
        assert_eq!(client.is_bonded(&user), false);
        assert_eq!(client.get_trust(&user), 0); // Score drops back
    }

    #[test]
    fn test_vouch_rewards_bzr() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let voucher = Address::random(&env);
        let target = Address::random(&env);

        // Setup: Voucher and Target must exist
        client.stake(&voucher, &None);
        client.stake(&target, &None);

        client.vouch(&voucher, &target);
        assert_eq!(client.get_balance(&voucher), 5);
    }

    #[test]
    fn test_transfer_bzr() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let user1 = Address::random(&env);
        let user2 = Address::random(&env);
        let target = Address::random(&env);

        // Setup
        client.stake(&user1, &None);
        client.stake(&user2, &None); // Receiver must exist
        client.stake(&target, &None);

        client.vouch(&user1, &target); // user1 gets 5
        client.vouch(&user1, &target); // user1 gets 5 (total 10)

        client.transfer_bzr(&user1, &user2, &3);

        assert_eq!(client.get_balance(&user1), 7);
        assert_eq!(client.get_balance(&user2), 3);
    }

    #[test]
    fn test_shop_buy_badge() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let user = Address::random(&env);
        let target = Address::random(&env);

        client.stake(&user, &None);
        client.stake(&target, &None);

        for _ in 0..10 { client.vouch(&user, &target); } // Earn 50 BZR
        
        let badge = symbol_short!("verified");
        client.buy_badge(&user, &badge);
        
        assert_eq!(client.has_badge(&user, &badge), true);
        assert_eq!(client.get_balance(&user), 0);
    }

    #[test]
    fn test_crowdfund() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let user = Address::random(&env);
        let target = Address::random(&env);

        client.stake(&user, &None);
        client.stake(&target, &None);

        // Earn BZR
        for _ in 0..4 { client.vouch(&user, &target); } // 20 BZR

        client.deposit_crowdfund(&user, &15);
        
        assert_eq!(client.get_balance(&user), 5);
        assert_eq!(client.get_crowdfund_balance(), 15);
    }

    #[test]
    fn test_referral() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let user = Address::random(&env);
        let referrer = Address::random(&env);

        client.stake(&user, &Some(referrer.clone()));
        
        assert_eq!(client.get_balance(&referrer), 10);
    }

    #[test]
    fn test_voting() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let user = Address::random(&env);
        let target = Address::random(&env);

        client.stake(&user, &None);
        client.stake(&target, &None);

        // Earn 10 BZR
        client.vouch(&user, &target); 
        client.vouch(&user, &target);

        client.vote(&user, &1, &true); // Vote Yes on Prop 1
        assert_eq!(client.get_proposal_stats(&1), (10, 0));
    }

    #[test]
    fn test_create_proposal() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let user = Address::random(&env);
        let target = Address::random(&env);

        client.stake(&user, &None);
        client.stake(&target, &None);

        // Earn BZR (Need 100)
        for _ in 0..20 { client.vouch(&user, &target); } 

        let id = client.create_proposal(&user);
        assert_eq!(id, 1);
        assert_eq!(client.get_balance(&user), 0);
    }

    #[test]
    fn test_dispute() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let accuser = Address::random(&env);
        let target = Address::random(&env);

        client.stake(&target, &None); // Target must exist

        assert_eq!(client.is_disputed(&target), false);
        client.raise_dispute(&accuser, &target);
        assert_eq!(client.is_disputed(&target), true);
    }

    #[test]
    fn test_resolve_dispute() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let admin = Address::random(&env);
        let accuser = Address::random(&env);
        let target = Address::random(&env);

        client.init(&admin);
        client.stake(&target, &None); // Target must exist

        client.raise_dispute(&accuser, &target);
        assert_eq!(client.is_disputed(&target), true);

        client.resolve_dispute(&target);
        assert_eq!(client.is_disputed(&target), false);
    }

    #[test]
    fn test_nickname() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let user = Address::random(&env);

        assert_eq!(client.get_nickname(&user), symbol_short!("User"));
        
        client.set_nickname(&user, &symbol_short!("BazaarMer"));
        assert_eq!(client.get_nickname(&user), symbol_short!("BazaarMer"));
    }

    #[test]
    fn test_nickname_search() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let user = Address::random(&env);
        let nick = symbol_short!("MerchantX");

        client.set_nickname(&user, &nick);
        assert_eq!(client.get_address_by_nickname(&nick), Some(user));
    }

    #[test]
    #[should_panic(expected = "Maintenance mode active")]
    fn test_maintenance_mode() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let admin = Address::random(&env);
        let user = Address::random(&env);
        let target = Address::random(&env);

        client.init(&admin);
        client.set_maintenance(&true);

        client.vouch(&user, &target); // Should panic
    }

    #[test]
    fn test_chat() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let user1 = Address::random(&env);
        let user2 = Address::random(&env);

        client.stake(&user2, &None); // Receiver must exist

        client.send_message(&user1, &user2, &soroban_sdk::String::from_str(&env, "Hello!"));
        let msgs = client.get_messages(&user2);
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs.get(0).unwrap().text, soroban_sdk::String::from_str(&env, "Hello!"));
    }

    #[test]
    fn test_escrow_flow() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let buyer = Address::random(&env);
        let seller = Address::random(&env);
        let target = Address::random(&env);

        client.stake(&buyer, &None);
        client.stake(&seller, &None); // Seller must exist to receive funds
        client.stake(&target, &None);

        for _ in 0..20 { client.vouch(&buyer, &target); } // Fund buyer with 100 BZR

        let id = client.create_escrow(&buyer, &seller, &50);
        client.approve_escrow(&id, &buyer); // Buyer approves
        client.approve_escrow(&id, &seller); // Seller approves -> Release
        
        assert_eq!(client.get_balance(&seller), 50);
    }

    #[test]
    fn test_subscription() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let user = Address::random(&env);
        let target = Address::random(&env);

        client.stake(&user, &None);
        client.stake(&target, &None);

        for _ in 0..10 { client.vouch(&user, &target); } // 50 BZR
        
        assert_eq!(client.is_subscribed(&user), false);
        client.subscribe(&user);
        assert_eq!(client.is_subscribed(&user), true);
        assert_eq!(client.get_balance(&user), 0);
    }

    #[test]
    fn test_multisig() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let user1 = Address::random(&env);
        let user2 = Address::random(&env);
        let target = Address::random(&env);

        client.stake(&user1, &None);
        client.stake(&target, &None);

        for _ in 0..20 { client.vouch(&user1, &target); } // 100 BZR

        let owners = soroban_sdk::vec![&env, user1.clone(), user2.clone()];
        let wid = client.create_wallet(&user1, &owners, &2);
        
        client.deposit_wallet(&user1, &wid, &100);
        let tx_id = client.propose_tx(&user1, &wid, &target, &50);
        
        // User1 approved implicitly or explicitly. Need User2.
        client.approve_tx(&user2, &tx_id);
        
        assert_eq!(client.get_balance(&target), 50);
    }

    #[test]
    fn test_lottery() {
        let env = Env::default();
        env.mock_all_auths();
        let client = create_contract(&env);
        let admin = Address::random(&env);
        let user1 = Address::random(&env);
        let user2 = Address::random(&env);
        let target = Address::random(&env);

        client.init(&admin);

        client.stake(&user1, &None);
        client.stake(&user2, &None);
        client.stake(&target, &None);

        // Fund users
        for _ in 0..4 { client.vouch(&user1, &target); } // 20 BZR
        for _ in 0..4 { client.vouch(&user2, &target); } // 20 BZR

        client.buy_ticket(&user1);
        client.buy_ticket(&user2);
        
        assert_eq!(client.get_lottery_info(), 2);
        assert_eq!(client.get_balance(&user1), 10);

        client.run_lottery();
        
        // One should have 10 (loser) + 0, one should have 10 (winner) + 20 = 30
        let bal1 = client.get_balance(&user1);
        let bal2 = client.get_balance(&user2);
        
        assert!( (bal1 == 30 && bal2 == 10) || (bal1 == 10 && bal2 == 30) );
        assert_eq!(client.get_lottery_info(), 0);
    }

    #[test]
    fn test_poverty_observation_window() {
        let mut po = PovertyObservation {
            start_time: 0,
            is_active: false,
        };

        let current_time = 1000;
        po.start_observation(current_time);
        
        // Check before 7 days (604800 seconds)
        assert_eq!(po.verify_window(current_time + 604799), false);
        assert_eq!(po.is_active, true);

        // Check after 7 days
        assert_eq!(po.verify_window(current_time + 604800), true);
        assert_eq!(po.is_active, false);
    }

    #[test]
    fn test_poverty_observation_enforcement() {
        let env = Env::default();
        env.mock_all_auths();
        
        let client = create_contract(&env);
        let user = Address::random(&env);

        // Set initial time
        env.ledger().with_mut(|li| { li.timestamp = 1000; });

        // Stake bond (starts 7-day timer)
        client.stake(&user, &None);

        // Attempt verification immediately (Should Fail)
        assert_eq!(client.verify_status(&user), false);

        // Warp time forward 7 days (604800 seconds) + 1 second
        env.ledger().with_mut(|li| { li.timestamp = 1000 + 604800 + 1; });

        // Attempt verification again (Should Pass)
        assert_eq!(client.verify_status(&user), true);
    }
}
