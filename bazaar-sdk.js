// 🏛️ Project Bazaar | SDK Wrapper v1.0
// Interacts with the Trust Logic Core on Stellar Futurenet

class BazaarClient {
    constructor(contractId, rpcUrl = "https://rpc-futurenet.stellar.org", networkPassphrase = "Test SDF Future Network ; October 2022") {
        this.contractId = contractId;
        this.server = new StellarSdk.Server(rpcUrl);
        this.networkPassphrase = networkPassphrase;
        this.contract = new StellarSdk.Contract(contractId);
    }

    /**
     * Stake 20 Pi Bond
     * @param {string} signerSecret - Secret key of the merchant
     * @param {string} userAddress - Public G-address of the merchant
     * @param {string|null} referrerAddress - Optional referrer address
     */
    async stake(signerSecret, userAddress, referrerAddress = null) {
        const signer = StellarSdk.Keypair.fromSecret(signerSecret);
        const account = await this.server.getAccount(signer.publicKey());

        // Prepare Arguments
        const userScVal = new StellarSdk.Address(userAddress).toScVal();
        const referrerScVal = referrerAddress 
            ? new StellarSdk.Address(referrerAddress).toScVal() 
            : StellarSdk.xdr.ScVal.scvVoid();

        // Build Transaction
        const tx = new StellarSdk.TransactionBuilder(account, { fee: "100", networkPassphrase: this.networkPassphrase })
            .addOperation(StellarSdk.Operation.invokeContractFunction({
                contract: this.contractId,
                function: "stake",
                args: [userScVal, referrerScVal]
            }))
            .setTimeout(30)
            .build();

        tx.sign(signer);

        // Submit
        return this.server.sendTransaction(tx);
    }

    /**
     * Vouch for a Peer
     * @param {string} signerSecret 
     * @param {string} voucherAddress 
     * @param {string} targetAddress 
     */
    async vouch(signerSecret, voucherAddress, targetAddress) {
        const signer = StellarSdk.Keypair.fromSecret(signerSecret);
        const account = await this.server.getAccount(signer.publicKey());

        const voucherScVal = new StellarSdk.Address(voucherAddress).toScVal();
        const targetScVal = new StellarSdk.Address(targetAddress).toScVal();

        const tx = new StellarSdk.TransactionBuilder(account, { fee: "100", networkPassphrase: this.networkPassphrase })
            .addOperation(StellarSdk.Operation.invokeContractFunction({
                contract: this.contractId,
                function: "vouch",
                args: [voucherScVal, targetScVal]
            }))
            .setTimeout(30)
            .build();

        tx.sign(signer);
        return this.server.sendTransaction(tx);
    }

    /**
     * Decay Reputation (Admin Only)
     * @param {string} signerSecret 
     * @param {string} targetAddress 
     */
    async decay(signerSecret, targetAddress) {
        const signer = StellarSdk.Keypair.fromSecret(signerSecret);
        const account = await this.server.getAccount(signer.publicKey());

        const targetScVal = new StellarSdk.Address(targetAddress).toScVal();

        const tx = new StellarSdk.TransactionBuilder(account, { fee: "100", networkPassphrase: this.networkPassphrase })
            .addOperation(StellarSdk.Operation.invokeContractFunction({
                contract: this.contractId,
                function: "decay",
                args: [targetScVal]
            }))
            .setTimeout(30)
            .build();

        tx.sign(signer);
        return this.server.sendTransaction(tx);
    }

    /**
     * Buy a Badge (Cost: 50 BZR)
     * @param {string} signerSecret 
     * @param {string} userAddress 
     * @param {string} badgeName 
     */
    async buyBadge(signerSecret, userAddress, badgeName) {
        const signer = StellarSdk.Keypair.fromSecret(signerSecret);
        const account = await this.server.getAccount(signer.publicKey());

        const userScVal = new StellarSdk.Address(userAddress).toScVal();
        const badgeScVal = StellarSdk.xdr.ScVal.scvSymbol(badgeName);

        const tx = new StellarSdk.TransactionBuilder(account, { fee: "100", networkPassphrase: this.networkPassphrase })
            .addOperation(StellarSdk.Operation.invokeContractFunction({
                contract: this.contractId,
                function: "buy_badge",
                args: [userScVal, badgeScVal]
            }))
            .setTimeout(30)
            .build();

        tx.sign(signer);
        return this.server.sendTransaction(tx);
    }

    /**
     * Read Trust Score (Read-Only)
     * @param {string} userAddress 
     */
    async getTrust(userAddress) {
        // For simulation, we use the user's address as source. 
        // Sequence number "0" is generally accepted for simulations.
        const source = new StellarSdk.Account(userAddress, "0");
        const userScVal = new StellarSdk.Address(userAddress).toScVal();
        
        const tx = new StellarSdk.TransactionBuilder(source, { 
            fee: "100", 
            networkPassphrase: this.networkPassphrase 
        })
        .addOperation(StellarSdk.Operation.invokeContractFunction({
            contract: this.contractId,
            function: "get_trust",
            args: [userScVal]
        }))
        .setTimeout(30)
        .build();

        const response = await this.server.simulateTransaction(tx);

        if (response.results && response.results[0] && response.results[0].retval) {
            const resultScVal = StellarSdk.xdr.ScVal.fromXDR(response.results[0].retval, 'base64');
            return resultScVal.u32();
        }
        return 0;
    }
}

// Export for browser
window.BazaarClient = BazaarClient;