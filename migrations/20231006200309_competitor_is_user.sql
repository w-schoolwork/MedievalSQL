-- Add migration script here
ALTER TABLE Competitors
RENAME COLUMN CompetitorId TO user_id;

ALTER TABLE Competitors
ADD CONSTRAINT fk_competitor_is_user
FOREIGN KEY (user_id)
REFERENCES users(user_id);

ALTER TABLE Competitors
DROP COLUMN email;