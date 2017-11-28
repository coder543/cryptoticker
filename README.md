# CryptoTicker

This is a tool to show the current cryptocurrency prices in a nice ticker format for use in tmux. When used in the one-shot (short-lived command) mode that `tmux` prefers, prices will be cached automatically and updated only after 30 minutes, no matter how many times the command is executed. This is a nice balance between API usage and keeping the values current.

# Installation

`cargo install --git https://github.com/coder543/cryptoticker`

# Usage

`cryptoticker --help` provides a useful summary of the options that are supported.

In your `~/.tmux.conf` file, you can add this to your statusbar to track ethereum and bitcoin prices: `#[fg=white,bg=default,bright]#(cryptoticker ethereum bitcoin)` 

If you're using something other than `tmux`, it might be useful to run the command in a continuous, interval mode, where the command stays alive and refreshes at a set interval. To use this mode, simply do `cryptoticker -i ethereum bitcoin`, which will use a default timeout of 5 minutes, or `cryptoticker -i -t SECS ethereum bitcoin` to set the timeout interval manually.

`cryptoticker --clear-cache` will completely remove the cache directory. Useful if uninstalling or if you want to force the one-shot mode to refresh.

`cryptoticker` uses `api.coinmarketcap.com`, which only updates once every 5 minutes, so refreshing any faster than that is just poor etiquette. `ethereum` and `bitcoin` are provided as examples, but `cryptoticker` supports all currencies that coinmarketcap supports on the API. 
