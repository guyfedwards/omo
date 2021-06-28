# omo
Simple pomodoro timer with notifications

## Usage
```
$ omo get # 🍅 08:11
$ omo get --notify "Go outside" # 🍅 08:11
$ omo reset # 🍅 20:00
$ omo help
```

## Waybar
![waybar](/screenshots/waybar.png)
```
# ~/.config/waybar/config
"custom/omo": {
    "exec": "omo get --notify",
    "interval": 1,
},
```
