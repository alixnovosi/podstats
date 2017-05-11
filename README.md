# podstats
==========

[![GitHub version](https://badge.fury.io/gh/andrewmichaud%2Fpodstats.svg)](https://badge.fury.io/gh/andrewmichaud%2Fpodstats)
[![Build Status](https://travis-ci.org/andrewmichaud/podstats.svg?branch=master)](https://travis-ci.org/andrewmichaud/podstats)

Provides stats on podcasts downloaded by [!puckfetcher](https://github.com/andrewmichaud/puckfetcher). Requires cache from puckfetcher to be present to do anything.

Run with `podstats` on the command line. A menu with stats you can show will be provided. `q`, `Ctrl-c` or `Ctrl-d` to quit.

Current options:
1) Get names of subscriptions in the puckfetcher cache.
2) Get entry counts of subscriptions in the puckfetcher cache.
3) Get the subscription in the cache with the highest entry count.
4) Get the name of the subscription in the cache with the highest entry count.

# PLANNED
* Means/medians of entries.
* Info on how long podcast episodes are or total runtimes are (requires fetching RSS feeds)
* Info on artists?
* Info on sites?
