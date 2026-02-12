// 🏛️ PROJECT BAZAAR | SMART CONTRACT v3.0 (Master)
// Includes: Academy Trust Score, Legacy Vault, Medical Emergency, and Panic Protocol.

#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec};

// ============================================================
// 📦 DATA STRUCTURES
// ============================================================

// 1. THE MERCHANT IDENTITY (The Academy)
#[contracttype]
#[derive(Clone)]
pub struct Merchant {
    pub trust_score: u32,
    pub bond_staked: bool,
    pub bzr_balance: i128,
    pub badges: Vec<Symbol>,
    pub is_disputed: bool,
    pub nickname: Symbol,
    pub messages: Vec<Message>,
}

#[contracttype]
#[derive(Clone)]
pub struct Message {
    pub sender: Address,
    pub text: String,
    pub timestamp: u64,
}

// 2. THE LEGACY VAULT (Social Security)
#[contracttype]
#[derive(Clone)]
pub struct LegacyVault {
    pub heir: Option<Address>, // Who gets the funds?
    pub last_heartbeat: u64,   // Timestamp of last "I AM ALIVE"
    pub is_locked: bool,       // Is the protocol active?
    pub is_frozen: bool,       // Has the Panic Button been triggered?
}

// 3. MEDICAL EMERGENCY
#[contracttype]
#[derive(Clone)]
pub struct MedicalEmergency {
    pub target_user: Address,
    pub votes_collected: u32,
    pub is_unlocked: bool,
}

// ============================================================
// 🔑 STORAGE KEYS
// ============================================================
#[contracttype]
pub enum DataKey {
    Merchant(Address),
    Vault(Address),
    Witnesses(Address),    // Stores Vec<Address> of the 5 Security Circle members
    Emergency(Address),    // Stores MedicalEmergency status
    PanicVotes(Address),   // Stores u32 count of Panic votes
}

// ============================================================
// ⚙️ THE CONTRACT
// ============================================================
#[contract]
pub struct TrustContract;

#[contractimpl]
impl TrustContract {

    // ============================================================
    // 🛡️ FEATURE 1: LEGACY VAULT (Deadman Switch)
    // ============================================================

    // A. Initialize Vault (Set Heir)
    pub fn create_vault(env: Env, user: Address, heir: Address) {
        user.require_auth();
        
        let vault = LegacyVault {
            heir: Some(heir),
            last_heartbeat: env.ledger().timestamp(),
            is_locked: true,
            is_frozen: false,
        };
        env.storage().persistent().set(&DataKey::Vault(user), &vault);
    }

    // B. The "PING" Button (I Am Alive)
    pub fn ping_heartbeat(env: Env, user: Address) {
        user.require_auth();
        
        let mut vault: LegacyVault = env.storage().persistent().get(&DataKey::Vault(user.clone())).expect("Vault not found");
        
        // If Frozen by Panic Button, Owner must unfreeze first (or wait)
        if vault.is_frozen {
            // Optional: Logic to allow owner to cancel panic could go here
            vault.is_frozen = false; 
        }

        // Reset Timer
        vault.last_heartbeat = env.ledger().timestamp();
        env.storage().persistent().set(&DataKey::Vault(user), &vault);
    }

    // C. The Claim (Called by Heir)
    pub fn claim_legacy(env: Env, target_user: Address) {
        // 'target_user' is the Owner. The caller must be the Heir.
        
        let vault: LegacyVault = env.storage().persistent().get(&DataKey::Vault(target_user.clone())).expect("Vault not found");
        let heir = vault.heir.unwrap();
        
        // 1. Only the Heir can trigger this
        heir.require_auth(); 

        // 2. Check Time Logic
        // 180 Days = 15,552,000 Seconds
        let deadman_limit = 15_552_000;
        let time_elapsed = env.ledger().timestamp() - vault.last_heartbeat;

        if time_elapsed < deadman_limit {
            panic!("Owner is still alive (Timer has not expired)");
        }

        // 3. EXECUTE TRANSFER (Mock Logic for Demo)
        // In full version, this moves tokens. For now, we return success.
        // "Assets Transferred to Heir."
    }

    // ============================================================
    // 🚑 FEATURE 2: MEDICAL & PANIC PROTOCOLS (The Security Circle)
    // ============================================================

    // A. Assign Security Circle (5 Witnesses)
    pub fn assign_witnesses(env: Env, user: Address, witnesses: Vec<Address>) {
        user.require_auth();
        if witnesses.len() > 5 { panic!("Max 5 witnesses allowed"); }
        env.storage().persistent().set(&DataKey::Witnesses(user), &witnesses);
    }

    // B. Medical Emergency (15% Release)
    pub fn declare_emergency(env: Env, target_user: Address) {
        let key = DataKey::Emergency(target_user.clone());
        if env.storage().persistent().has(&key) { panic!("Emergency already active"); }

        let emergency = MedicalEmergency {
            target_user: target_user,
            votes_collected: 0,
            is_unlocked: false,
        };
        env.storage().persistent().set(&key, &emergency);
    }

    pub fn witness_vote_medical(env: Env, witness: Address, target_user: Address) {
        witness.require_auth();
        
        // Verify Witness
        let circle: Vec<Address> = env.storage().persistent().get(&DataKey::Witnesses(target_user.clone())).expect("No Circle found");
        if !circle.contains(witness.clone()) { panic!("Not a witness"); }

        // Count Vote
        let key = DataKey::Emergency(target_user.clone());
        let mut emergency: MedicalEmergency = env.storage().persistent().get(&key).expect("No emergency");
        
        emergency.votes_collected += 1;
        
        // Threshold: 3/5
        if emergency.votes_collected >= 3 {
            emergency.is_unlocked = true; // UNLOCK 15%
        }
        env.storage().persistent().set(&key, &emergency);
    }

    // C. PANIC BUTTON (The Anti-Hack Freeze)
    pub fn panic_button(env: Env, witness: Address, target_user: Address) {
        witness.require_auth();

        // Verify Witness
        let circle: Vec<Address> = env.storage().persistent().get(&DataKey::Witnesses(target_user.clone())).expect("No Circle found");
        if !circle.contains(witness.clone()) { panic!("Not a witness"); }

        // Count Vote
        let key = DataKey::PanicVotes(target_user.clone());
        let mut votes: u32 = env.storage().persistent().get(&key).unwrap_or(0);
        votes += 1;
        env.storage().persistent().set(&key, &votes);

        // Threshold: 3/5 to FREEZE
        if votes >= 3 {
            let mut vault: LegacyVault = env.storage().persistent().get(&DataKey::Vault(target_user.clone())).expect("Vault not found");
            
            vault.is_frozen = true;

            // ACCELERATE TIMER: Set heartbeat to 173 days ago.
            // This leaves exactly 7 days (604,800 sec) until "180 days" is reached.
            // This allows the Heir to claim in 1 week.
            let time_warp = 15_552_000 - 604_800; 
            vault.last_heartbeat = env.ledger().timestamp() - time_warp;

            env.storage().persistent().set(&DataKey::Vault(target_user), &vault);
        }
    }

    // ============================================================
    // 🏛️ FEATURE 3: MERCHANT TRUST (The Academy)
    // ============================================================

    pub fn stake(env: Env, user: Address) {
        user.require_auth();
        let mut merchant = env.storage().persistent().get(&DataKey::Merchant(user.clone())).unwrap_or(Merchant {
            trust_score: 0, bond_staked: false, bzr_balance: 0, badges: Vec::new(&env), 
            is_disputed: false, nickname: Symbol::new(&env, "User"), messages: Vec::new(&env)
        });

        if merchant.bond_staked { panic!("Already bonded"); }
        merchant.bond_staked = true;
        merchant.trust_score += 10;
        env.storage().persistent().set(&DataKey::Merchant(user), &merchant);
    }

    pub fn vouch(env: Env, voucher: Address, target: Address) {
        voucher.require_auth();
        let mut target_data = env.storage().persistent().get::<DataKey, Merchant>(&DataKey::Merchant(target.clone())).expect("Target not found");
        if target_data.trust_score < 100 { target_data.trust_score += 1; }
        env.storage().persistent().set(&DataKey::Merchant(target), &target_data);
    }

    pub fn get_trust(env: Env, user: Address) -> u32 {
        let merchant = env.storage().persistent().get::<DataKey, Merchant>(&DataKey::Merchant(user)).unwrap_or(Merchant {
            trust_score: 0, bond_staked: false, bzr_balance: 0, badges: Vec::new(&env), 
            is_disputed: false, nickname: Symbol::new(&env, "User"), messages: Vec::new(&env)
        });
        merchant.trust_score
    }
}