# Discord IP Notifier
Post global IP Address on Discord with Webhook when global ip changed.

## Introduction
1. Clone this repository
```
git clone https://github.com/cykps/discord-ip-notifier.git

cd discord-ip-notifier
```

2. Create `.env` file in  the current directory or parents.

```Dotenv
DISCORD_WEBHOOK_URL="{Your Discord Webhook URL}" #include 'http(s)://'
CHECKIP_URL="https://checkip.amazonaws.com/" #include 'http(s)://'
LOG_FILE_NAME="discord-ip-notifier.log"
INTERVAL_MIN=20 #interval minutes
```
> List of URLs that can be used as CHECKIP_URL: [グローバルIPを確認できるサービスたち - Qiita](https://qiita.com/fruscianteee/items/7fe2b2663d670e34d143)


3. Build & Run
```
cargo run
```
