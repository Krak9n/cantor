Simple status bar for Wayland. 
Tested on **Hyprland**.

**WIP. Not ready for publishing.**

---
### Installation
* **Arch Linux**  
Users of Arch Linux can install the package from [aur](https://aur.archlinux.org/packages/cantor).
```bash
$ yay -S cantor
# systemctl enable --now cantor.service
```

* **Compiling from source**  
You can compile everything from source using the `install.sh` script, which provides all that needed to build the package.

---
### Configuration
**1. Putting pieces**  
Cantor reads the **config.toml** file located under **~/.config/cantor/**.
You can copy the **extras/example.toml** that is in the root of this package, to the config directory.
```bash
$ mkdir -p ~/.config/cantor
$ cp -f extras/example.toml ~/.config/cantor
```

**2. Basic configuration**   
Each module, if not workspaces, should have exactly three arguments:
* position
* color
* prompt

**2.1. Allowing modules**  
In modules you just assign booleans to the values you want or don't want to use.  
Example:  
```rust
[enabled]
battery = true
time = false
workspaces = true
```

**2.2. General bar settings.**  
Here you should specify the background color, height, border, etc.
```rust
[general]
color = "#FFFFFF"
height = 20
border = true
border_color = "#000000"
```

**2.3. Icons**  
You can change the icons for workspaces. Just make sure you got the right font on your OS.  
Here's the [quick script](https://github.com/polybar/polybar/wiki/Fonts#find-fonts-for-glyphs) by polybar. Requires you to have **Perl** installed on the system.  
```rust
[modules.workspaces]
icons = { 1 = "1", 2 = "2", 3 = "none" }
```

**2.4. The prompt.**  
Here "{}" is where the configured output will be printed. Put it wherever you want.  
```rust
[modules.battery]
prompt = "𓈆 {}"
```
  
This works for each module written in the initial **[modules]** section.  

**2.5. The colors**   
Just add the hex value. This would change the whole prompt.  
```rust
[modules.time]
color = "#A11313"
```
  
**2.6. Positions**    
Same as for the color. It is either **right**, **left**, or **center**.  
```rust
[modules.battery]
position = "right"
```

---
Roadmap:
* basic coloring of the bar
* displaying modules
* editing through a custom toml file