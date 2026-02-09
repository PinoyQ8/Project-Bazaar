const express = require('express');
const path = require('path');
const fs = require('fs'); // Added: To read your J: drive files
const app = express();
const PORT = 3000;

// 🛠️ MIDDLEWARE: Parse JSON bodies for engagement logs
app.use(express.json());

// 🛡️ SECURITY HEADER: Bypasses the ngrok warning page for the S23
app.use((req, res, next) => {
    res.setHeader('ngrok-skip-browser-warning', 'true');
    res.setHeader('Access-Control-Allow-Origin', '*');
    next();
});

// 📂 ASSET DELIVERY: Serves your Bazaar UI files
app.use(express.static(path.join(__dirname)));

// 📦 BAZAAR DATA PROTOCOL: This reads your CSV and sends it to the S23
app.get('/api/merchants', (req, res) => {
    try {
        const filePath = path.join(__dirname, 'Genesis_100.csv');

        if (!fs.existsSync(filePath)) {
            return res.status(404).json({ error: "Genesis_100.csv not found on J: drive" });
        }

        const data = fs.readFileSync(filePath, 'utf8');
        const lines = data.split('\n').filter(line => line.trim() !== '');
        const headers = lines[0].split(',').map(h => h.trim());

        const merchants = lines.slice(1).map(line => {
            const values = line.split(',');
            return headers.reduce((obj, header, i) => {
                obj[header] = values[i] ? values[i].trim() : "";
                return obj;
            }, {});
        });

        res.json(merchants);
    } catch (err) {
        res.status(500).json({ error: "Data Sync Failed" });
    }
});

// 🔍 ENGAGEMENT LOGS: Records discovery events from the frontend
app.post('/api/discovery', (req, res) => {
    const { merchant, points, user } = req.body;
    const timestamp = new Date().toISOString();
    const logEntry = `[${timestamp}] DISCOVERY_EVENT: User '${user}' opened '${merchant}'. Score: ${points}`;
    console.log(logEntry);

    // Persistent Storage: Append to log file on J: Drive
    fs.appendFileSync(path.join(__dirname, 'bazaar_transactions.log'), logEntry + '\n');
    res.json({ status: "Logged" });
});

// 🚀 START SERVER
app.listen(PORT, '0.0.0.0', () => {
    console.clear();
    console.log("🏛️  PROJECT BAZAAR: MASTER NODE UPDATED");
    console.log(`🔗 Local Bridge: http://localhost:${PORT}`);
    console.log(`📂 Registry: Genesis_100.csv Linked`);
    console.log("📡 Status: Awaiting S23 Connection...");
    console.log("📍 ACTUAL ROOT FOLDER:", __dirname);
    fs.writeFileSync(path.join(__dirname, 'FORCE_TEST.txt'), 'J-Drive-Search');
});