-- Add migration script here

CREATE TABLE users (
	user_id UUID PRIMARY KEY,
	email TEXT UNIQUE NOT NULL,
	totp_secret TEXT UNIQUE NOT NULL
);

CREATE TABLE Events (
    EventID UUID PRIMARY KEY,
    EventName VARCHAR(100) NOT NULL,
    EventDate DATE NOT NULL,
    Description TEXT,
    LocationID UUID,
    CONSTRAINT fk_location
        FOREIGN KEY (LocationID)
        REFERENCES Location(LocationID)
);

CREATE TABLE Competitors (
    CompetitorID UUID PRIMARY KEY,
    FirstName VARCHAR(50) NOT NULL,
    LastName VARCHAR(50) NOT NULL,
    KnightlyRank VARCHAR(100),
    HorseName VARCHAR(50),
    email TEXT UNIQUE NOT NULL
);


CREATE TABLE Location (
    LocationID UUID PRIMARY KEY,
    LocationName VARCHAR(100) NOT NULL,
    Village VARCHAR(50),
    State VARCHAR(50),
    Nation VARCHAR(50)
);


CREATE TABLE Rewards (
    RewardID UUID PRIMARY KEY,
    RewardName VARCHAR(100) NOT NULL,
    Description TEXT,
    EventID UUID,
    CONSTRAINT fk_event
        FOREIGN KEY (EventID)
        REFERENCES Events(EventID)
);
