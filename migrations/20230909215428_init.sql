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
  amt BIGINT
);

CREATE TABLE events (
  event_id UUID PRIMARY KEY,
  event_name VARCHAR(100) NOT NULL,
  begins TIMESTAMPTZ NOT NULL,
  flavor TEXT,
  organizer UUID,
  FOREIGN KEY (organizer)
  REFERENCES users(user_id),
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
  amount BIGINT NOT NULL,
  FOREIGN KEY(gambler) REFERENCES users(user_id),
  FOREIGN KEY(player) REFERENCES users(user_id),
  FOREIGN KEY(event_id) REFERENCES events(event_id)
);

CREATE MATERIALIZED VIEW winners AS
SELECT events.event_id, plays.user_id, plays.score
FROM events, plays
WHERE events.finished = true
AND events.event_id = plays.event_id
AND plays.score <> NULL
AND plays.score = (SELECT MAX(score) FROM plays WHERE plays.event_id = events.event_id);

CREATE VIEW BetsOnBy AS
SELECT events.event_id as e_id, bets.gambler as g_id, bets.player as p_id, SUM(bets.amount) as bet_amount 
FROM events, bets
WHERE events.event_id = bets.event_id
GROUP BY e_id, g_id, p_id;

CREATE VIEW BetsOn AS
SELECT e_id, p_id, SUM(bet_amount) as bet_amount
FROM BetsOnBy
GROUP BY e_id, p_id;

CREATE VIEW Pool AS
SELECT e_id, SUM(bet_amount) as bet_amount
FROM BetsOn
GROUP BY e_id;

CREATE VIEW Shares AS
SELECT BetsOnBy.g_id as g_id, BetsOnBy.e_id as e_id, BetsOnBy.p_id as p_id, (BetsOnBy.bet_amount / BetsOn.bet_amount) as share
FROM BetsOnBy, BetsOn
WHERE BetsOnBy.e_id = BetsOn.e_id
AND BetsOnBy.p_id = BetsOn.p_id;
--
--CREATE VIEW Winnings AS 
--SELECT
--FROM
--
--
--
--
--
--
--