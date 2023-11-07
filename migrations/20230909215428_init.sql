-- Add migration script here

CREATE TABLE users (
	user_id UUID PRIMARY KEY,
	email TEXT UNIQUE NOT NULL,
	totp_secret TEXT UNIQUE NOT NULL
);

CREATE TABLE session (
  user_id UUID NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
  secret TEXT NOT NULL UNIQUE
);

CREATE TABLE deposit (
  user_id UUID NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
  amt NUMERIC(100,50)
);

CREATE TABLE events (
  event_id UUID PRIMARY KEY,
  event_name VARCHAR(100) NOT NULL,
  begins TIMESTAMPTZ NOT NULL,
  flavor TEXT,
  organizer UUID,
  FOREIGN KEY (organizer)
  REFERENCES users(user_id)
  ON DELETE SET NULL,
  finished BOOLEAN NOT NULL
);

CREATE TABLE plays (
  user_id UUID NOT NULL,
  event_id UUID NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
  FOREIGN KEY (event_id) REFERENCES events(event_id) ON DELETE CASCADE,
  PRIMARY KEY (user_id, event_id),
  score bigint
  --  Possibly rename/reorganize so there's a different ID to be used in later views
);

CREATE TABLE bets (
  gambler UUID NOT NULL,
  player UUID NOT NULL,
  event_id UUID NOT NULL,
  amount NUMERIC(100,50) NOT NULL,
  FOREIGN KEY(gambler) REFERENCES users(user_id) ON DELETE CASCADE,
  FOREIGN KEY(player) REFERENCES users(user_id) ON DELETE CASCADE,
  FOREIGN KEY(event_id) REFERENCES events(event_id) ON DELETE CASCADE
);

CREATE MATERIALIZED VIEW winners AS
SELECT events.event_id, plays.user_id, plays.score
FROM events, plays
WHERE events.finished = true
  AND events.event_id = plays.event_id
  AND plays.score IS NOT NULL
  AND plays.score = (SELECT MAX(score) FROM plays WHERE plays.event_id = events.event_id);

CREATE VIEW BetsOnBy AS
SELECT bets.event_id,
  bets.gambler,
  bets.player,
  SUM(bets.amount) as bet_amount 
FROM bets
GROUP BY bets.event_id, bets.gambler, bets.player;

CREATE VIEW BetsOn AS
SELECT bets.event_id,
  bets.player,
  SUM(bets.amount) as bet_amount
FROM bets
GROUP BY bets.event_id, bets.player;

CREATE VIEW BettingPool AS
SELECT bets.event_id, SUM(amount) as bet_amount
FROM bets
GROUP BY bets.event_id;

CREATE VIEW Shares AS
SELECT BetsOnBy.gambler,
  BetsOnBy.event_id,
  BetsOnBy.player,
  (BetsOnBy.bet_amount / BetsOn.bet_amount) as share
FROM BetsOnBy, BetsOn
WHERE BetsOnBy.event_id = BetsOn.event_id
  AND BetsOnBy.player = BetsOn.player
  AND BetsOn.bet_amount > 0;

-- Winnings should be calculated by multiplying a gambler's share in the pool for each of the events they gambled on successfully with the size of the pool for that event
CREATE VIEW Winnings AS 
SELECT DISTINCT Shares.gambler,
  Shares.event_id,
  (Shares.share * BettingPool.bet_amount) as winnings
FROM Shares, BettingPool, Winners
WHERE Shares.event_id = winners.event_id
  AND Shares.event_id = BettingPool.event_id
  AND Shares.player = winners.user_id;

-- Balances should be calculated by summing up a user's deposits and winnings, and subtracting out their bets.
CREATE VIEW Balances AS
SELECT u.user_id AS gambler_id,
  COALESCE(SUM(d.amt), 0) AS total_deposits,
  COALESCE(SUM(b.amount), 0) AS total_bets,
  COALESCE(SUM(w.winnings), 0) AS total_winnings,
  (
    COALESCE(SUM(d.amt), 0)
    + COALESCE(SUM(w.winnings), 0)
    - COALESCE(SUM(b.amount), 0)
  ) AS balance
FROM users u
LEFT JOIN deposit d ON u.user_id = d.user_id
LEFT JOIN bets b ON u.user_id = b.gambler
LEFT JOIN Winnings w ON u.user_id = w.gambler
GROUP BY u.user_id;
