# omo
Simple pomodoro timer with notifications

## Usage
```
$ omo get --notify # 🍅 08:11
$ omo reset # 🍅 20:00
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
