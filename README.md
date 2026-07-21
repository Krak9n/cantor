Simple STATUS BAR for WAYLAND.

### **WIP**

### Installation
#### Arch Linux
Users of Arch Linux can install the package from aur.
```bash
$ yay -S cantor
# systemctl enable --now cantor.service
```

#### Compiling from source
You can compile everything from source using the `install.sh` script, which provides all that needed to build the package.

### Configuration
1. Putting pieces
Cantor reads the **config.toml** file located under **~/.config/cantor/**.
You can copy the **extras/example.toml** that is in the root of this package, to the config directory.
```bash
$ mkdir -p ~/.config/cantor
$ cp -f extras/example.toml ~/.config/cantor
```

2. The file
2.1. In modules you just assign booleans to the values you want or don't want to use.
For example
```rust
[modules]
battery = true
time = false
workspaces = true
```

2.2. You can change the characters on workspaces, or change the values icons. 
Just make sure you got the right font. Here's the [quick script](https://github.com/polybar/polybar/wiki/Fonts#find-fonts-for-glyphs) by polybar. Requires you to have Perl installed on the system.
```rust
[modules.bar]
1 = "1"
2 = "two"
3 = "三"
```

Changing the icons. Here "{}" is where the configured output will be printed. Put it wherever you want.
[modules.battery]
prompt = "𓈆 {}"

This works for each module written in the initial **[modules]** section.

2.3. Changing the colors
Just add the hex value. This would change the whole prompt.
[modules.time]
color = "#A11313"

---
Roadmap:
* basic coloring of the bar
* displaying modules
* editing through a custom toml file