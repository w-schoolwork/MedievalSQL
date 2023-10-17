# Abstract idea for database structure

* Users
  * Users have a UUID, email, and TOTP secret.
* Sessions
  * Sessions associate a long secret with a user id and permit users to be recognized across multiple requests
* Events
  * Events have a UUID, name, and some other flavor information.
  * An event has an "organizer", who is a user with permission to modify it.
  * Once an event is finished, it contains the UUID of the winning player.
* Plays
  * Plays associate users with events as players. They contain a user ID and an event ID. Once the user finishes playing, they contain a score.
* Deposits
  * The amount of points a user starts with.
  * Will probably just contain 100 points for each user.
* Bets
  * Bets associate users with events as gamblers. They contain a user ID for the gambler, a user ID for the player they're betting on, an event ID for the event they're betting in, and a wager.
* BetsOnBy
  * A view, associating events, gamblers, and players with the sum of all bets on that player for that event.
  * Something like `SELECT event.id as e_id, bet.g_id as g_id, bet.p_id as p_id, SUM(bet.amount) AS bet_amount FROM event, bet WHERE bet.e_id = event.id GROUP BY event.id, p_id`
* BetsOn
  * A view, associating events and players with the sum of all bets on that player
  * Something like `SELECT e_id, p_id, SUM(bet_amount) as bet_amount FROM BetsOnBy GROUP BY e_id, p_id;`
* Pool
  * A view, associating events with the total betting pool
  * Something like `SELECT e_id, SUM(bet_amount) as bet_amount FROM BetsOn GROUP BY e_id;`
<!-- * Outcomes
  * A view, associating players and events with the winnings from that event associated with that player.
  * Something like `SELECT event.id, event.winner, SUM(bet_amount) FROM event, Pool WHERE Pool.e_id = event.id AND event.winner <> NULL;` -->
* Shares
  * A view, associating gamblers, events, and players with the fraction of money bet on that player that the gambler provided for that event.
  * Something like `SELECT BetsOnBy.g_id, BetsOnBy.bet_amount / BetsOn.bet_amount FROM BetsOnBy, BetsOn WHERE BetsOnBy.e_id = BetsOn.e_id AND BetsOnBy.p_id = BetsOn.p_id;`
* Winnings
  * A materialized view, associating gamblers and events with the gambler's winnings from that event.
* Balances
  * A view, associating gamblers with the sum of their deposits, bets, and winnings.
