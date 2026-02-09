// 🏛️ BAZAAR E-NETWORK | TRUST LOGIC CORE v1.0
// This governs the 20 Pi Security Bond and Reputation math.

#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Message {
    pub sender: Address,
    pub text: String,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct Merchant {
    pub trust_score: u32,
    pub bond_staked: bool,
    pub identity_hash: String,
    pub poverty_obs: PovertyObservation,
    pub bzr_balance: i128,
    pub badges: Vec<Symbol>,
    pub is_disputed: bool,
    pub nickname: Symbol,
    pub messages: Vec<Message>,
    pub is_subscribed: bool,
}

impl Merchant {
    // Logic for the 20 Pi Bond Activation
    pub fn stake_bond(&mut self, current_time: u64) {
        self.bond_staked = true;
        self.trust_score += 10;
        self.poverty_obs.start_observation(current_time);
    }

    // Logic for Transaction Fulfillment (Trust +5)
    pub fn fulfill_order(&mut self) {
        if self.trust_score <= 95 {
            self.trust_score += 5;
        }
    }

    // Logic for Reputation Decay
    pub fn decay_reputation(&mut self) {
        if self.trust_score >= 3 {
            self.trust_score -= 3;
        }
    }
}

// 🏛️ SOCIAL LEGACY LAYER | POVERTY OBSERVATION
// Implements the 7-day monitoring window for the Uncorrupt Executive.
#[contracttype]
#[derive(Clone)]
pub struct PovertyObservation {
    pub start_time: u64,
    pub is_active: bool,
}

impl PovertyObservation {
    // Starts the 7-day clock for poverty verification
    pub fn start_observation(&mut self, current_time: u64) {
        self.start_time = current_time;
        self.is_active = true;
    }

    // Checks if 7 days (604,800 seconds) have passed
    pub fn verify_window(&mut self, current_time: u64) -> bool {
        const OBSERVATION_PERIOD: u64 = 604_800;
        
        if self.is_active && current_time >= self.start_time + OBSERVATION_PERIOD {
            self.is_active = false;
            return true; // Window passed, status verified
        }
        false
    }
}

// 🏛️ BAZAAR DAO | GOVERNANCE LAYER
#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u32,
    pub creator: Address,
    pub votes_for: u32,
    pub votes_against: u32,
}

// 🏛️ BAZAAR SECURE TRADE | ESCROW LAYER
#[contracttype]
#[derive(Clone)]
pub struct Escrow {
    pub id: u32,
    pub buyer: Address,
    pub seller: Address,
    pub amount: i128,
    pub buyer_approved: bool,
    pub seller_approved: bool,
    pub active: bool,
}

// 🏛️ BAZAAR MULTISIG VAULT
#[contracttype]
#[derive(Clone)]
pub struct Wallet {
    pub id: u32,
    pub owners: Vec<Address>,
    pub threshold: u32,
    pub balance: i128,
}

#[contracttype]
#[derive(Clone)]
pub struct WalletTx {
    pub id: u32,
    pub wallet_id: u32,
    pub proposer: Address,
    pub target: Address,
    pub amount: i128,
    pub approvals: Vec<Address>,
    pub executed: bool,
}

#[contract]
pub struct TrustContract;

#[contractimpl]
impl TrustContract {
    // Stake 20 Pi Bond with Timestamp Capture
    pub fn stake(env: Env, user: Address, referrer: Option<Address>) {
        user.require_auth();
        
        let current_time = env.ledger().timestamp();
        
        let mut merchant = env.storage().persistent().get(&user).unwrap_or(Merchant {
            trust_score: 0,
            bond_staked: false,
            identity_hash: String::from_str(&env, ""),
            poverty_obs: PovertyObservation { start_time: 0, is_active: false },
            bzr_balance: 0,
            badges: Vec::new(&env),
            is_disputed: false,
            nickname: Symbol::new(&env, "User"),
            messages: Vec::new(&env),
            is_subscribed: false,
        });

        if merchant.bond_staked {
            panic!("Already bonded");
        }

        merchant.stake_bond(current_time);
        env.storage().persistent().set(&user, &merchant);

        // Referral Reward Protocol (10 BZR)
        if let Some(ref_addr) = referrer {
            if ref_addr != user {
                let mut ref_merchant = env.storage().persistent().get(&ref_addr).unwrap_or(Merchant {
                    trust_score: 0,
                    bond_staked: false,
                    identity_hash: String::from_str(&env, ""),
                    poverty_obs: PovertyObservation { start_time: 0, is_active: false },
                    bzr_balance: 0,
                    badges: Vec::new(&env),
                    is_disputed: false,
                    nickname: Symbol::new(&env, "User"),
                    messages: Vec::new(&env),
                    is_subscribed: false,
                });
                ref_merchant.bzr_balance += 10;
                env.storage().persistent().set(&ref_addr, &ref_merchant);
            }
        }
    }

    // Verify Status (Check Poverty Observation Window)
    pub fn verify_status(env: Env, user: Address) -> bool {
        if let Some(mut merchant) = env.storage().persistent().get::<Address, Merchant>(&user) {
            let current_time = env.ledger().timestamp();
            let passed = merchant.poverty_obs.verify_window(current_time);
            
            if passed {
                // Update state if window passed (to clear is_active)
                env.storage().persistent().set(&user, &merchant);
            }
            return passed;
        }
        false
    }

    // Vouch for a Peer (Trust +1, Reward +5 BZR)
    pub fn vouch(env: Env, voucher: Address, target: Address) {
        // Check Maintenance Mode
        let key = Symbol::new(&env, "MAINTENANCE");
        if env.storage().instance().has(&key) {
            let active: bool = env.storage().instance().get(&key).unwrap();
            if active {
                panic!("Maintenance mode active");
            }
        }

        voucher.require_auth();

        // 1. Verify Voucher is a Bonded Merchant
        let mut voucher_data = env.storage().persistent().get::<Address, Merchant>(&voucher).expect("Voucher not found");
        if !voucher_data.bond_staked {
            panic!("Voucher must be bonded to vouch");
        }

        // 2. Verify Target Exists
        let mut target_data = env.storage().persistent().get::<Address, Merchant>(&target).expect("Target not found");

        // 3. Apply Trust Logic
        if target_data.trust_score < 100 {
            target_data.trust_score += 1;
        }

        // 4. Reward Voucher
        voucher_data.bzr_balance += 5;

        env.storage().persistent().set(&voucher, &voucher_data);
        env.storage().persistent().set(&target, &target_data);
    }

    // Helper for tests
    pub fn get_trust(env: Env, user: Address) -> u32 {
        let merchant: Merchant = env.storage().persistent().get(&user).unwrap_or(Merchant {
            trust_score: 0,
            bond_staked: false,
            identity_hash: String::from_str(&env, ""),
            poverty_obs: PovertyObservation { start_time: 0, is_active: false },
            bzr_balance: 0,
            badges: Vec::new(&env),
            is_disputed: false,
            nickname: Symbol::new(&env, "User"),
            messages: Vec::new(&env),
            is_subscribed: false,
        });
        merchant.trust_score
    }

    pub fn is_bonded(env: Env, user: Address) -> bool {
        let merchant: Merchant = env.storage().persistent().get(&user).unwrap_or(Merchant {
            trust_score: 0,
            bond_staked: false,
            identity_hash: String::from_str(&env, ""),
            poverty_obs: PovertyObservation { start_time: 0, is_active: false },
            bzr_balance: 0,
            badges: Vec::new(&env),
            is_disputed: false,
            nickname: Symbol::new(&env, "User"),
            messages: Vec::new(&env),
            is_subscribed: false,
        });
        merchant.bond_staked
    }

    pub fn get_balance(env: Env, user: Address) -> i128 {
        if let Some(merchant) = env.storage().persistent().get::<Address, Merchant>(&user) {
            return merchant.bzr_balance;
        }
        0
    }

    // Transfer BZR Tokens (Peer-to-Peer)
    pub fn transfer_bzr(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        if amount <= 0 { panic!("Amount must be positive"); }
        
        let mut sender = env.storage().persistent().get::<Address, Merchant>(&from).expect("Sender not found");
        let mut receiver = env.storage().persistent().get::<Address, Merchant>(&to).expect("Receiver not found");
        
        if sender.bzr_balance < amount {
            panic!("Insufficient balance");
        }
        
        sender.bzr_balance -= amount;
        receiver.bzr_balance += amount;
        
        env.storage().persistent().set(&from, &sender);
        env.storage().persistent().set(&to, &receiver);
    }

    // Withdraw Bond (30-day lock)
    pub fn withdraw(env: Env, user: Address) {
        user.require_auth();
        let mut merchant = env.storage().persistent().get::<Address, Merchant>(&user).expect("Merchant not found");
        
        if !merchant.bond_staked {
            panic!("Not bonded");
        }

        let current_time = env.ledger().timestamp();
        // 30 days = 2,592,000 seconds
        if current_time < merchant.poverty_obs.start_time + 2_592_000 {
            panic!("Bond is still locked");
        }

        merchant.bond_staked = false;
        merchant.trust_score = 0;
        env.storage().persistent().set(&user, &merchant);
    }

    // Buy a Badge (Cost: 50 BZR)
    pub fn buy_badge(env: Env, user: Address, badge: Symbol) {
        user.require_auth();
        let mut merchant = env.storage().persistent().get::<Address, Merchant>(&user).expect("Merchant not found");
        
        if merchant.bzr_balance < 50 {
            panic!("Insufficient balance");
        }

        merchant.bzr_balance -= 50;
        merchant.badges.push_back(badge);
        env.storage().persistent().set(&user, &merchant);
    }

    pub fn has_badge(env: Env, user: Address, badge: Symbol) -> bool {
        if let Some(merchant) = env.storage().persistent().get::<Address, Merchant>(&user) {
            return merchant.badges.contains(badge);
        }
        false
    }

    // Deposit to Community Crowdfund
    pub fn deposit_crowdfund(env: Env, user: Address, amount: i128) {
        user.require_auth();
        if amount <= 0 { panic!("Amount must be positive"); }

        let mut merchant = env.storage().persistent().get::<Address, Merchant>(&user).expect("Merchant not found");
        
        if merchant.bzr_balance < amount {
            panic!("Insufficient balance");
        }

        merchant.bzr_balance -= amount;
        env.storage().persistent().set(&user, &merchant);

        let key = Symbol::new(&env, "CROWDFUND");
        let mut current_fund: i128 = env.storage().instance().get(&key).unwrap_or(0);
        current_fund += amount;
        env.storage().instance().set(&key, &current_fund);
    }

    pub fn get_crowdfund_balance(env: Env) -> i128 {
        let key = Symbol::new(&env, "CROWDFUND");
        env.storage().instance().get(&key).unwrap_or(0)
    }

    // DAO Governance: Create Proposal (Cost: 100 BZR)
    pub fn create_proposal(env: Env, user: Address) -> u32 {
        user.require_auth();
        let mut merchant = env.storage().persistent().get::<Address, Merchant>(&user).expect("Merchant not found");
        
        if merchant.bzr_balance < 100 {
            panic!("Insufficient balance for proposal creation");
        }

        merchant.bzr_balance -= 100;
        env.storage().persistent().set(&user, &merchant);

        let key_count = Symbol::new(&env, "PROP_COUNT");
        let mut count: u32 = env.storage().instance().get(&key_count).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&key_count, &count);

        let proposal = Proposal {
            id: count,
            creator: user,
            votes_for: 0,
            votes_against: 0,
        };
        
        let prop_key = (Symbol::new(&env, "PROP"), count);
        env.storage().persistent().set(&prop_key, &proposal);

        count
    }

    // DAO Governance: Vote
    pub fn vote(env: Env, user: Address, proposal_id: u32, vote: bool) {
        user.require_auth();
        
        let prop_key = (Symbol::new(&env, "PROP"), proposal_id);
        let mut proposal = env.storage().persistent().get::<(Symbol, u32), Proposal>(&prop_key).expect("Proposal not found");
        
        // Check if voted
        let vote_key = (Symbol::new(&env, "VOTE"), proposal_id, user.clone());
        if env.storage().persistent().has(&vote_key) {
            panic!("Already voted");
        }

        let merchant = env.storage().persistent().get::<Address, Merchant>(&user).expect("Merchant not found");
        let weight = merchant.bzr_balance as u32; 

        if vote {
            proposal.votes_for += weight;
        } else {
            proposal.votes_against += weight;
        }

        env.storage().persistent().set(&prop_key, &proposal);
        env.storage().persistent().set(&vote_key, &true);
    }

    pub fn get_proposal_stats(env: Env, proposal_id: u32) -> (u32, u32) {
        let prop_key = (Symbol::new(&env, "PROP"), proposal_id);
        if let Some(proposal) = env.storage().persistent().get::<(Symbol, u32), Proposal>(&prop_key) {
            (proposal.votes_for, proposal.votes_against)
        } else {
            (0, 0)
        }
    }

    // Dispute Resolution
    pub fn init(env: Env, admin: Address) {
        let key = Symbol::new(&env, "ADMIN");
        env.storage().instance().set(&key, &admin);
    }

    pub fn get_admin(env: Env) -> Address {
        let key = Symbol::new(&env, "ADMIN");
        env.storage().instance().get(&key).expect("Admin not initialized")
    }

    pub fn raise_dispute(env: Env, accuser: Address, target: Address) {
        accuser.require_auth();
        let mut merchant = env.storage().persistent().get::<Address, Merchant>(&target).expect("Target not found");
        merchant.is_disputed = true;
        env.storage().persistent().set(&target, &merchant);
    }

    pub fn resolve_dispute(env: Env, target: Address) {
        let admin = Self::get_admin(env.clone());
        admin.require_auth();
        let mut merchant = env.storage().persistent().get::<Address, Merchant>(&target).expect("Target not found");
        merchant.is_disputed = false;
        env.storage().persistent().set(&target, &merchant);
    }

    pub fn is_disputed(env: Env, target: Address) -> bool {
        if let Some(merchant) = env.storage().persistent().get::<Address, Merchant>(&target) {
            return merchant.is_disputed;
        }
        false
    }

    // Merchant Branding
    pub fn set_nickname(env: Env, user: Address, nick: Symbol) {
        user.require_auth();
        let mut merchant = env.storage().persistent().get(&user).unwrap_or(Merchant {
            trust_score: 0,
            bond_staked: false,
            identity_hash: String::from_str(&env, ""),
            poverty_obs: PovertyObservation { start_time: 0, is_active: false },
            bzr_balance: 0,
            badges: Vec::new(&env),
            is_disputed: false,
            nickname: Symbol::new(&env, "User"),
            messages: Vec::new(&env),
            is_subscribed: false,
        });

        let key = (Symbol::new(&env, "NICK"), nick.clone());
        if env.storage().persistent().has(&key) {
             let owner: Address = env.storage().persistent().get(&key).unwrap();
             if owner != user {
                 panic!("Nickname already taken");
             }
        }
        env.storage().persistent().set(&key, &user);

        merchant.nickname = nick;
        env.storage().persistent().set(&user, &merchant);
    }

    pub fn get_nickname(env: Env, user: Address) -> Symbol {
        if let Some(merchant) = env.storage().persistent().get::<Address, Merchant>(&user) {
            return merchant.nickname;
        }
        Symbol::new(&env, "User")
    }

    pub fn get_address_by_nickname(env: Env, nick: Symbol) -> Option<Address> {
        let key = (Symbol::new(&env, "NICK"), nick);
        env.storage().persistent().get(&key)
    }

    // Merchant Chat
    pub fn send_message(env: Env, from: Address, to: Address, text: String) {
        from.require_auth();
        let mut receiver = env.storage().persistent().get::<Address, Merchant>(&to).expect("Receiver not found");
        
        let msg = Message {
            sender: from,
            text,
            timestamp: env.ledger().timestamp(),
        };
        
        receiver.messages.push_back(msg);
        env.storage().persistent().set(&to, &receiver);
    }

    pub fn get_messages(env: Env, user: Address) -> Vec<Message> {
        if let Some(merchant) = env.storage().persistent().get::<Address, Merchant>(&user) {
            return merchant.messages;
        }
        Vec::new(&env)
    }

    // Secure Trade: Create Escrow
    pub fn create_escrow(env: Env, buyer: Address, seller: Address, amount: i128) -> u32 {
        buyer.require_auth();
        if amount <= 0 { panic!("Amount must be positive"); }

        let mut buyer_merchant = env.storage().persistent().get::<Address, Merchant>(&buyer).expect("Buyer not found");
        
        if buyer_merchant.bzr_balance < amount {
            panic!("Insufficient balance");
        }

        buyer_merchant.bzr_balance -= amount;
        env.storage().persistent().set(&buyer, &buyer_merchant);

        let key_count = Symbol::new(&env, "ESCROW_COUNT");
        let mut count: u32 = env.storage().instance().get(&key_count).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&key_count, &count);

        let escrow = Escrow {
            id: count,
            buyer: buyer,
            seller: seller,
            amount: amount,
            buyer_approved: false,
            seller_approved: false,
            active: true,
        };
        
        let escrow_key = (Symbol::new(&env, "ESCROW"), count);
        env.storage().persistent().set(&escrow_key, &escrow);

        count
    }

    // Secure Trade: Approve Escrow
    pub fn approve_escrow(env: Env, id: u32, approver: Address) {
        approver.require_auth();
        
        let escrow_key = (Symbol::new(&env, "ESCROW"), id);
        let mut escrow = env.storage().persistent().get::<(Symbol, u32), Escrow>(&escrow_key).expect("Escrow not found");

        if !escrow.active {
            panic!("Escrow not active");
        }

        if approver == escrow.buyer {
            escrow.buyer_approved = true;
        } else if approver == escrow.seller {
            escrow.seller_approved = true;
        } else {
            panic!("Not authorized to approve this escrow");
        }

        if escrow.buyer_approved && escrow.seller_approved {
            let mut seller_merchant = env.storage().persistent().get::<Address, Merchant>(&escrow.seller).expect("Seller not found");
            seller_merchant.bzr_balance += escrow.amount;
            env.storage().persistent().set(&escrow.seller, &seller_merchant);
            escrow.active = false;
        }

        env.storage().persistent().set(&escrow_key, &escrow);
    }

    // Premium Subscription (Cost: 50 BZR)
    pub fn subscribe(env: Env, user: Address) {
        user.require_auth();
        let mut merchant = env.storage().persistent().get::<Address, Merchant>(&user).expect("Merchant not found");
        
        if merchant.bzr_balance < 50 {
            panic!("Insufficient balance");
        }

        merchant.bzr_balance -= 50;
        merchant.is_subscribed = true;
        env.storage().persistent().set(&user, &merchant);
    }

    pub fn is_subscribed(env: Env, user: Address) -> bool {
        if let Some(merchant) = env.storage().persistent().get::<Address, Merchant>(&user) {
            return merchant.is_subscribed;
        }
        false
    }

    // Multisig: Create Wallet
    pub fn create_wallet(env: Env, creator: Address, owners: Vec<Address>, threshold: u32) -> u32 {
        creator.require_auth();
        if threshold == 0 || threshold > owners.len() {
            panic!("Invalid threshold");
        }
        
        let key_count = Symbol::new(&env, "WALLET_COUNT");
        let mut count: u32 = env.storage().instance().get(&key_count).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&key_count, &count);

        let wallet = Wallet {
            id: count,
            owners,
            threshold,
            balance: 0,
        };
        
        let wallet_key = (Symbol::new(&env, "WALLET"), count);
        env.storage().persistent().set(&wallet_key, &wallet);
        
        count
    }

    // Multisig: Deposit
    pub fn deposit_wallet(env: Env, user: Address, wid: u32, amount: i128) {
        user.require_auth();
        if amount <= 0 { panic!("Amount must be positive"); }

        let mut merchant = env.storage().persistent().get::<Address, Merchant>(&user).expect("Merchant not found");
        if merchant.bzr_balance < amount {
            panic!("Insufficient balance");
        }
        merchant.bzr_balance -= amount;
        env.storage().persistent().set(&user, &merchant);

        let wallet_key = (Symbol::new(&env, "WALLET"), wid);
        let mut wallet = env.storage().persistent().get::<(Symbol, u32), Wallet>(&wallet_key).expect("Wallet not found");
        wallet.balance += amount;
        env.storage().persistent().set(&wallet_key, &wallet);
    }

    // Multisig: Propose Transaction
    pub fn propose_tx(env: Env, user: Address, wid: u32, target: Address, amount: i128) -> u32 {
        user.require_auth();
        let wallet_key = (Symbol::new(&env, "WALLET"), wid);
        let wallet = env.storage().persistent().get::<(Symbol, u32), Wallet>(&wallet_key).expect("Wallet not found");
        
        if !wallet.owners.contains(user.clone()) {
            panic!("Not an owner");
        }

        let key_count = Symbol::new(&env, "WTX_COUNT");
        let mut count: u32 = env.storage().instance().get(&key_count).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&key_count, &count);

        let mut approvals = Vec::new(&env);
        approvals.push_back(user.clone());

        let tx = WalletTx {
            id: count,
            wallet_id: wid,
            proposer: user,
            target,
            amount,
            approvals,
            executed: false,
        };

        let tx_key = (Symbol::new(&env, "WTX"), count);
        env.storage().persistent().set(&tx_key, &tx);

        count
    }

    // Multisig: Approve Transaction
    pub fn approve_tx(env: Env, user: Address, tx_id: u32) {
        user.require_auth();
        
        let tx_key = (Symbol::new(&env, "WTX"), tx_id);
        let mut tx = env.storage().persistent().get::<(Symbol, u32), WalletTx>(&tx_key).expect("Tx not found");
        
        if tx.executed {
            panic!("Already executed");
        }

        let wallet_key = (Symbol::new(&env, "WALLET"), tx.wallet_id);
        let mut wallet = env.storage().persistent().get::<(Symbol, u32), Wallet>(&wallet_key).expect("Wallet not found");

        if !wallet.owners.contains(user.clone()) {
            panic!("Not an owner");
        }

        if !tx.approvals.contains(user.clone()) {
            tx.approvals.push_back(user);
        }

        if tx.approvals.len() >= wallet.threshold {
            if wallet.balance < tx.amount {
                panic!("Insufficient wallet balance");
            }
            wallet.balance -= tx.amount;
            
            let mut target_merchant = env.storage().persistent().get::<Address, Merchant>(&tx.target).expect("Target merchant not found");
            target_merchant.bzr_balance += tx.amount;
            
            env.storage().persistent().set(&tx.target, &target_merchant);
            env.storage().persistent().set(&wallet_key, &wallet);
            
            tx.executed = true;
        }

        env.storage().persistent().set(&tx_key, &tx);
    }

    // Lottery: Buy Ticket (Cost: 10 BZR)
    pub fn buy_ticket(env: Env, user: Address) {
        user.require_auth();
        let mut merchant = env.storage().persistent().get::<Address, Merchant>(&user).expect("Merchant not found");
        
        if merchant.bzr_balance < 10 {
            panic!("Insufficient balance");
        }

        merchant.bzr_balance -= 10;
        env.storage().persistent().set(&user, &merchant);

        let key = Symbol::new(&env, "LOTTERY");
        let mut participants: Vec<Address> = env.storage().instance().get(&key).unwrap_or(Vec::new(&env));
        participants.push_back(user);
        env.storage().instance().set(&key, &participants);
    }

    pub fn get_lottery_info(env: Env) -> u32 {
        let key = Symbol::new(&env, "LOTTERY");
        let participants: Vec<Address> = env.storage().instance().get(&key).unwrap_or(Vec::new(&env));
        participants.len()
    }

    pub fn run_lottery(env: Env) {
        let key = Symbol::new(&env, "LOTTERY");
        let participants: Vec<Address> = env.storage().instance().get(&key).unwrap_or(Vec::new(&env));
        
        if participants.len() == 0 {
            return;
        }

        // Pseudo-random winner selection using timestamp
        let timestamp = env.ledger().timestamp();
        let winner_idx = timestamp % (participants.len() as u64);
        let winner = participants.get(winner_idx as u32).unwrap();

        let pot = (participants.len() as i128) * 10;

        let mut merchant = env.storage().persistent().get::<Address, Merchant>(&winner).expect("Winner not found");
        merchant.bzr_balance += pot;
        env.storage().persistent().set(&winner, &merchant);

        // Reset lottery
        let new_participants: Vec<Address> = Vec::new(&env);
        env.storage().instance().set(&key, &new_participants);
    }
    
    // Admin: Decay Reputation (The Exile Protocol)
    pub fn decay(env: Env, target: Address) {
        let admin = Self::get_admin(env.clone());
        admin.require_auth();
        
        let mut merchant = env.storage().persistent().get::<Address, Merchant>(&target).expect("Target not found");
        
        if merchant.trust_score >= 3 {
            merchant.trust_score -= 3;
        } else {
            merchant.trust_score = 0;
        }
        
        env.storage().persistent().set(&target, &merchant);
    }

    // Admin: Transfer Authority
    pub fn transfer_admin(env: Env, new_admin: Address) -> Address {
        let admin = Self::get_admin(env.clone());
        admin.require_auth();
        
        let key = Symbol::new(&env, "ADMIN");
        env.storage().instance().set(&key, &new_admin);
        new_admin
    }

    // Admin: Force Unbond (Ban Hammer)
    pub fn force_unbond(env: Env, user: Address) {
        let admin = Self::get_admin(env.clone());
        admin.require_auth();
        
        let mut merchant = env.storage().persistent().get::<Address, Merchant>(&user).expect("Merchant not found");
        merchant.bond_staked = false;
        merchant.trust_score = 0;
        env.storage().persistent().set(&user, &merchant);
    }

    // Admin: Maintenance Mode
    pub fn set_maintenance(env: Env, active: bool) {
        let admin = Self::get_admin(env.clone());
        admin.require_auth();
        let key = Symbol::new(&env, "MAINTENANCE");
        env.storage().instance().set(&key, &active);
    }

    // Admin: Manual Trust Adjustment (Testing/Recovery)
    pub fn add_trust(env: Env, user: Address) {
        let admin = Self::get_admin(env.clone());
        admin.require_auth();
        
        let mut merchant = env.storage().persistent().get::<Address, Merchant>(&user).expect("Merchant not found");
        merchant.trust_score += 1;
        env.storage().persistent().set(&user, &merchant);
    }
}

mod test;