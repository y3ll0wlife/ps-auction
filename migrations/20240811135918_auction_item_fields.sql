-- Add migration script here
ALTER TABLE AuctionItems
ADD is_active BOOLEAN NOT NULL DEFAULT TRUE,
ADD is_cancelled BOOLEAN NOT NULL DEFAULT FALSE,
ADD is_visible BOOLEAN NOT NULL DEFAULT TRUE;