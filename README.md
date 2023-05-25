# sorew
SOundcloud RElease Watcher

This bot watches a configurable selection of soundcloud profiles and posts a message when one of the profiles posts a new track. It's implemented using a scraper mechanism because "Unfortunately we're currently not accepting any new developer requests for API keys and don't have an ETA of when it will be open again."

Currently the bot responds to the following text commands:

- !list - list all SC usernames and latest found tracks
- !follow [username] - start following a user
- !unfollow [username] - stop following a user

Once following a user, sorew periodically looks at the tracks page of that user and if a new track has been deteced, the new track is posted to the #releases discord channel.
