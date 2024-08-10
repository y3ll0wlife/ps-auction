-- Add migration script here
CREATE TABLE Auctions (
    auction_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    url VARCHAR(255) NOT NULL
);

CREATE TABLE AuctionItems (
    item_id INT PRIMARY KEY,
    auction_id INT NOT NULL,
    item_name VARCHAR(255) NOT NULL,
    item_price INT NOT NULL,
    item_url VARCHAR(255) NOT NULL,
    FOREIGN KEY (auction_id) REFERENCES Auctions(auction_id) ON DELETE CASCADE
)