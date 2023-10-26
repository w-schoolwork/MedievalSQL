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
--
-- CREATE VIEW Winners AS
-- SELECT events.event_id as e_id, plays.user_id as playerID, Plays.score as winScore
-- FROM Events, Plays
-- WHERE *unsure about the .finished, will likely need to work with dates to check on status of event*
-- AND Events.user_id= plays.e_id AND 
--
--Is there supposed to be a player table?
--
-- CREATE TABLE Location (
--     LocationID UUID PRIMARY KEY,
--     LocationName VARCHAR(100) NOT NULL,
--     Village VARCHAR(50),
--     State VARCHAR(50),
--     Nation VARCHAR(50)
-- );

-- CREATE TABLE Competitors (
--     user_id UUID PRIMARY KEY,
--     CONSTRAINT fk_competitor_is_user
--     FOREIGN KEY (user_id)
--     REFERENCES users(user_id)
--     ON DELETE CASCADE;
--     FirstName VARCHAR(50) NOT NULL,
--     LastName VARCHAR(50) NOT NULL,
--     KnightlyRank VARCHAR(100),
--     HorseName VARCHAR(50),
--     email TEXT UNIQUE NOT NULL
-- );

-- CREATE TABLE Rewards (
--     RewardID UUID PRIMARY KEY,
--     RewardName VARCHAR(100) NOT NULL,
--     Description TEXT,
--     EventID UUID,
--     CONSTRAINT fk_event
--         FOREIGN KEY (EventID)
--         REFERENCES Events(EventID)
-- );
--
-- CREATE TABLE Deposits (
--    user_id UUID
--    points tinyint
--);
--
-- CREATE TABLE Bets (
--    gamblerID (userID)
--    gambledID (competitor)
--    Amount int
--    event_id UUID
--
--    Ask about this
--);
--
-- CREATE VIEW Winners AS
-- SELECT events.event_id as e_id, plays.user_id as playerID, Plays.score as winScore
-- FROM events, Plays
-- WHERE *unsure about the .finished, will likely need to work with dates to check on status of event*
-- AND event.user_id= plays.e_id AND plays.score <> NULL
-- GROUP    BY e_id, playerID
-- ORDER BY winScore
-- DESC LIMIT 1
-- 
--Is there supposed to be a player table? No
--
--CREATE VIEW BetsOnBy AS
--SELECT events.event_d as e_id, Bets.gamblerID as g_id, Bets.gambledID as p_id, SUM(Bets.Amount) as bet_amount 
--FROM events, Bets
--WHERE events.event_id = Bets.event_id
--GROUP BY events.event_id, 
--
--CREATE VIEW BetsON AS
--SELECT e_id, p_id, bet_amount
--FROM BetsOnBy
--GROUP BY e_id, p_id
--
--CREATE VIEW Pool AS
--SELECT e_id, bet_amount
--FROM BetsOn
--GROUP BY e_id
--
--CREATE VIEW Shares AS
--SELECT BetsOnBy.g_id, (BetsOnBy.bet_amount / BetsOn.bet_amount)
--FROM BetsOnBy, BetsOn
--WHERE BetsOnBy.e_id = BetsOn.e_id AND BetsOnBy.p_id = BetsOn.p_id
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