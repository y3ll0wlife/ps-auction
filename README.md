# PS Auction Bot

A program that grabs the data from [PS Auction](https://psauction.se/) and sends price updates to Discord.

## Running with Docker

```
docker run -t y3ll0w/psauction-bot -e DATABASE_URL='<PostgreSQL Connection String>' -e WEBHOOK_URL='<Discord Webhook Url>' \
```
