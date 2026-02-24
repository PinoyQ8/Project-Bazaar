let _genesisCache = [];

/**
 * Parses the Genesis_100.csv file to retrieve trusted accounts.
 * Expected CSV Format: AccountID,TrustScore,Role
 */
async function loadGenesisList() {
    try {
        const response = await fetch('Genesis_100.csv');
        if (!response.ok) throw new Error("Failed to load Genesis file");
        
        const csvText = await response.text();
        const lines = csvText.split('\n');
        
        // Skip header (index 0) and map the rest
        const genesisData = lines.slice(1).map(line => {
            const parts = line.split(',');
            if (parts.length >= 2) {
                const accountId = parts[0].trim();
                const trustScore = parseInt(parts[1].trim());
                const role = parts[2] ? parts[2].trim() : 'Merchant';
                
                // Basic validation to ensure line isn't empty
                if (accountId && !isNaN(trustScore)) {
                    return {
                        accountId: accountId,
                        trustScore: trustScore,
                        role: role
                    };
                }
            }
        }).filter(item => item !== undefined); // Remove undefined rows

        _genesisCache = genesisData;
        console.log("Genesis List Loaded:", genesisData);
        return genesisData;

    } catch (error) {
        console.error("Error parsing Genesis CSV:", error);
        return [];
    }
}

/**
 * Checks if a wallet address exists in the loaded Genesis list.
 * @param {string} accountId - The Stellar Public Key (G...)
 */
function checkGenesisStatus(accountId) {
    return _genesisCache.find(entry => entry.accountId === accountId);
}