const express = require('express');
const cors = require('cors');

const app = express();
const PORT = process.env.PORT || 3000;

app.use(cors());
app.use(express.json());

// Mock app data
const apps = [
    {
        "id": "spotify",
        "name": "Spotify",
        "description": "Music streaming service",
        "version": "1.0.0",
        "category": "Music",
        "download_url": "https://store.ranortv.com/apps/spotify.tar.gz",
        "icon_url": "https://store.ranortv.com/icons/spotify.png",
        "rating": 4.8,
        "downloads": 1250000,
        "installed": false
    },
    {
        "id": "twitch",
        "name": "Twitch",
        "description": "Live streaming platform",
        "version": "2.1.0",
        "category": "Entertainment",
        "download_url": "https://store.ranortv.com/apps/twitch.tar.gz",
        "icon_url": "https://store.ranortv.com/icons/twitch.png",
        "rating": 4.6,
        "downloads": 890000,
        "installed": false
    }
];

// API endpoints
app.get('/api/apps', (req, res) => {
    res.json({ apps });
});

app.get('/api/apps/:id', (req, res) => {
    const app = apps.find(a => a.id === req.params.id);
    if (app) {
        res.json(app);
    } else {
        res.status(404).json({ error: 'App not found' });
    }
});

app.get('/api/featured', (req, res) => {
    res.json({ apps: apps.slice(0, 3) });
});

app.listen(PORT, () => {
    console.log(`🏪 RanorTV App Store server running on port ${PORT}`);
});
