# Almanac

Simple .ics parser to pretty print the events on the terminal.

## Usage

```
$ almanac week personal.ics other.ics

Mon Dec 10 2018
    17:00-18:00 event 1

Wed Dec 12 2018
    ----------- all day event
                description
    19:00-20:00 nother event
```

## Config file

There is a config file, in toml format in your config folder:

* Lin: /home/alice/.config/almanac.toml
* Win: C:\Users\Alice\AppData\Roaming\almanac.toml
* Mac: /Users/Alice/Library/Preferences/almanac.toml

The format is:
```
# a list of icals to be used if none is provided to the program
cals = ["/home/foo/mycal.ics", "/tmp/anothercal.ics"]
```
