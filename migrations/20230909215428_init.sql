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
  score smallint -- Does this need to be small? A player could probably score more than 32767
  --  Possibly rename/reorganize so there's a different ID to be used in later views
);

CREATE TABLE bets (
  -- Ask about this  
);
--
-- CREATE VIEW Winners AS
-- SELECT Events.eventID as e_id, plays.user_id as playerID, Plays.score as winScore
-- FROM Events, Plays
-- WHERE *unsure about the .finished, will likely need to work with dates to check on status of event*
-- AND Events.user_id= plays.e_id AND 
--
--Is there supposed to be a player table?
--
--CREATE VIEW
--
--
--
--
--
--
--
--
--
--
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