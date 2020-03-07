# beryl

Also available on source hut:

    https://git.sr.ht/~brian-dawn/beryl

Auto reconnect to select bluetooth devices and run commands when they do.

Requires `bluez`.

## Installation

    cargo install --path .

Then add beryl to your startup applications.

## Usage

First create:

    ~/.config/beryl/config.toml

Here's an example config:

```
[[devices]]
id="35_82_2D_D2_5C_6E"
command="setxkbmap -option ctrl:swapcaps"   
```

The command field is optional, it's something that will get run once we connect to the device.
