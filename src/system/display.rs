use anyhow::Result;
use log::info;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct DisplayManager {
    config_path: PathBuf,
    hypr_config_path: PathBuf,
    waybar_config_path: PathBuf,
}

impl DisplayManager {
    pub fn new() -> Self {
        Self {
            config_path: PathBuf::from("/etc/xbitos/display"),
            hypr_config_path: PathBuf::from("/etc/hypr"),
            waybar_config_path: PathBuf::from("/etc/xdg/waybar"),
        }
    }

    pub fn setup_hyprland(&self) -> Result<()> {
        info!("Setting up Hyprland environment...");

        // تثبيت الحزم الإضافية المطلوبة
        let packages = vec![
            "hyprland",
            "waybar",
            "alacritty",
            "wofi",
            "dunst",
            "swaylock-effects",
            "swayidle",
            "wl-clipboard",
            "grim",
            "slurp",
            "pamixer",
            "brightnessctl",
            "blueman",
            "network-manager-applet",
            "polkit-kde-agent",
            "xdg-desktop-portal-hyprland",
            "qt5-wayland",
            "qt6-wayland",
            "nwg-look",
            "hyprpaper",
        ];

        let pkg_manager = crate::system::package_manager::PackageManager::new();
        pkg_manager.install_packages(&packages)?;

        // إنشاء مجلدات التكوين
        self.create_config_directories()?;

        // إعداد تكوينات Hyprland
        self.setup_hyprland_config()?;

        // إعداد تكوينات Waybar
        self.setup_waybar_config()?;

        // إعداد تكوينات Alacritty
        self.setup_alacritty_config()?;

        // إعداد تكوينات Wofi
        self.setup_wofi_config()?;

        // إعداد خلفية سطح المكتب
        self.setup_wallpaper()?;

        // إضافة سكريبت بدء التشغيل
        self.setup_startup_script()?;

        // إضافة ملف جلسة لـ SDDM
        self.setup_sddm_session()?;

        // تثبيت الخطوط المطلوبة
        self.install_required_fonts()?;

        // إعداد تكوينات GTK و Qt
        self.setup_gtk_config()?;
        self.setup_qt_config()?;

        // إعداد نظام الإشعارات
        self.setup_dunst_config()?;

        Ok(())
    }

    fn create_config_directories(&self) -> Result<()> {
        fs::create_dir_all(&self.config_path)?;
        fs::create_dir_all(&self.hypr_config_path)?;
        fs::create_dir_all(&self.waybar_config_path)?;
        fs::create_dir_all("/etc/xdg/alacritty")?;
        fs::create_dir_all("/etc/xdg/wofi")?;
        Ok(())
    }

    fn setup_hyprland_config(&self) -> Result<()> {
        let hyprland_conf = r#"
# xBitOS Hyprland Configuration

# المتغيرات
$mainMod = SUPER
$terminal = alacritty
$menu = wofi --show drun
$browser = firefox

# إعدادات الشاشة
monitor=,preferred,auto,1

# تشغيل تلقائي
exec-once = waybar
exec-once = hyprpaper
exec-once = dunst
exec-once = nm-applet
exec-once = blueman-applet
exec-once = /usr/lib/polkit-kde-authentication-agent-1
exec-once = swayidle -w timeout 300 'swaylock -f' timeout 600 'hyprctl dispatch dpms off' resume 'hyprctl dispatch dpms on'

# إعدادات المدخلات
input {
    kb_layout = us,ar
    kb_options = grp:alt_shift_toggle
    follow_mouse = 1
    touchpad {
        natural_scroll = true
        tap-to-click = true
    }
    sensitivity = 0
}

# إعدادات عامة
general {
    gaps_in = 5
    gaps_out = 10
    border_size = 2
    col.active_border = rgba(33ccffee)
    col.inactive_border = rgba(595959aa)
    layout = dwindle
}

# الزخارف
decoration {
    rounding = 10
    blur {
        enabled = true
        size = 5
        passes = 2
    }
    drop_shadow = true
    shadow_range = 15
    shadow_offset = 3 3
}

# الحركات
animations {
    enabled = yes
    bezier = myBezier, 0.05, 0.9, 0.1, 1.05
    animation = windows, 1, 7, myBezier
    animation = windowsOut, 1, 7, default, popin 80%
    animation = border, 1, 10, default
    animation = fade, 1, 7, default
    animation = workspaces, 1, 6, default
}

# اختصارات لوحة المفاتيح
bind = $mainMod, Return, exec, $terminal
bind = $mainMod, Q, killactive,
bind = $mainMod SHIFT, Q, exit,
bind = $mainMod, Space, togglefloating,
bind = $mainMod, D, exec, $menu
bind = $mainMod, F, fullscreen
bind = $mainMod, B, exec, $browser
bind = $mainMod, L, exec, swaylock
bind = $mainMod SHIFT, S, exec, grim -g "$(slurp)" - | wl-copy

# التنقل بين مساحات العمل
bind = $mainMod, 1, workspace, 1
bind = $mainMod, 2, workspace, 2
bind = $mainMod, 3, workspace, 3
bind = $mainMod, 4, workspace, 4
bind = $mainMod, 5, workspace, 5

# نقل النوافذ بين مساحات العمل
bind = $mainMod SHIFT, 1, movetoworkspace, 1
bind = $mainMod SHIFT, 2, movetoworkspace, 2
bind = $mainMod SHIFT, 3, movetoworkspace, 3
bind = $mainMod SHIFT, 4, movetoworkspace, 4
bind = $mainMod SHIFT, 5, movetoworkspace, 5

# قواعد النوافذ
windowrule = float, ^(pavucontrol)$
windowrule = float, ^(blueman-manager)$
windowrule = float, ^(nm-connection-editor)$
"#;

        fs::write(self.hypr_config_path.join("hyprland.conf"), hyprland_conf)?;
        Ok(())
    }

    fn setup_waybar_config(&self) -> Result<()> {
        let waybar_config = r#"{
    "layer": "top",
    "position": "top",
    "height": 30,
    "modules-left": ["hyprland/workspaces", "hyprland/window"],
    "modules-center": ["clock"],
    "modules-right": ["pulseaudio", "network", "bluetooth", "battery", "tray"],
    
    "hyprland/workspaces": {
        "format": "{icon}",
        "format-icons": {
            "1": "1",
            "2": "2",
            "3": "3",
            "4": "4",
            "5": "5"
        }
    },
    
    "clock": {
        "format": "{:%H:%M}",
        "format-alt": "{:%Y-%m-%d}"
    },
    
    "battery": {
        "format": "{capacity}% {icon}",
        "format-icons": ["", "", "", "", ""]
    },
    
    "network": {
        "format-wifi": "直 {essid}",
        "format-ethernet": " {ipaddr}",
        "format-disconnected": "睊"
    },
    
    "pulseaudio": {
        "format": "{volume}% {icon}",
        "format-bluetooth": "{volume}% {icon}",
        "format-muted": "",
        "format-icons": {
            "headphone": "",
            "hands-free": "",
            "headset": "",
            "phone": "",
            "portable": "",
            "car": "",
            "default": ["", ""]
        }
    },
    
    "bluetooth": {
        "format": " {status}",
        "format-disabled": "",
        "format-connected": " {num_connections}",
        "tooltip-format": "{controller_alias}\t{controller_address}",
        "tooltip-format-connected": "{controller_alias}\t{controller_address}\n\n{device_enumerate}",
        "tooltip-format-enumerate-connected": "{device_alias}\t{device_address}"
    },
    
    "tray": {
        "icon-size": 21,
        "spacing": 10
    }
}"#;

        let waybar_style = r#"* {
    border: none;
    border-radius: 0;
    font-family: "JetBrainsMono Nerd Font";
    font-size: 14px;
    min-height: 0;
}

window#waybar {
    background: rgba(21, 18, 27, 0.9);
    color: #cdd6f4;
}

#workspaces button {
    padding: 0 5px;
    background: transparent;
    color: #cdd6f4;
    border-bottom: 3px solid transparent;
}

#workspaces button.active {
    background: #11111b;
    border-bottom: 3px solid #cba6f7;
}

#clock,
#battery,
#pulseaudio,
#network,
#bluetooth,
#tray {
    padding: 0 10px;
    margin: 0 5px;
    color: #cdd6f4;
}

#clock {
    background-color: #1e1e2e;
    border-radius: 10px;
}

#battery {
    background-color: #1e1e2e;
    border-radius: 10px;
}

#battery.charging {
    color: #a6e3a1;
}

#battery.warning:not(.charging) {
    color: #f38ba8;
}

#network {
    background-color: #1e1e2e;
    border-radius: 10px;
}

#pulseaudio {
    background-color: #1e1e2e;
    border-radius: 10px;
}

#bluetooth {
    background-color: #1e1e2e;
    border-radius: 10px;
}

#tray {
    background-color: #1e1e2e;
    border-radius: 10px;
}"#;

        fs::write(self.waybar_config_path.join("config"), waybar_config)?;
        fs::write(self.waybar_config_path.join("style.css"), waybar_style)?;
        Ok(())
    }

    fn setup_alacritty_config(&self) -> Result<()> {
        let alacritty_config = r#"
window:
  padding:
    x: 10
    y: 10
  opacity: 0.95

font:
  normal:
    family: JetBrainsMono Nerd Font
    style: Regular
  size: 11.0

colors:
  primary:
    background: '#1E1E2E'
    foreground: '#CDD6F4'

cursor:
  style:
    shape: Block
    blinking: On
  blink_interval: 750

shell:
  program: /bin/bash
"#;

        fs::write("/etc/xdg/alacritty/alacritty.yml", alacritty_config)?;
        Ok(())
    }

    fn setup_wofi_config(&self) -> Result<()> {
        let wofi_config = r#"
width=600
height=400
location=center
show=drun
prompt=Search...
filter_rate=100
allow_markup=true
no_actions=true
halign=fill
orientation=vertical
content_halign=fill
insensitive=true
"#;

        let wofi_style = r#"
window {
    margin: 0px;
    border: 2px solid #cba6f7;
    background-color: #1e1e2e;
    border-radius: 15px;
}

#input {
    padding: 10px;
    margin: 10px;
    border: none;
    color: #cdd6f4;
    background-color: #313244;
    border-radius: 10px;
}

#inner-box {
    margin: 5px;
    border: none;
    background-color: transparent;
}

#outer-box {
    margin: 5px;
    border: none;
    background-color: transparent;
}

#text {
    margin: 5px;
    border: none;
    color: #cdd6f4;
}

#entry:selected {
    background-color: #313244;
    border-radius: 10px;
}
"#;

        fs::write("/etc/xdg/wofi/config", wofi_config)?;
        fs::write("/etc/xdg/wofi/style.css", wofi_style)?;
        Ok(())
    }

    fn setup_wallpaper(&self) -> Result<()> {
        let hyprpaper_config = r#"
preload = /usr/share/backgrounds/xbitos/default.jpg
wallpaper = ,/usr/share/backgrounds/xbitos/default.jpg
"#;

        fs::create_dir_all("/usr/share/backgrounds/xbitos")?;
        fs::write(self.hypr_config_path.join("hyprpaper.conf"), hyprpaper_config)?;

        // هنا يمكنك إضافة صورة خلفية افتراضية
        // يجب نسخ الصورة إلى /usr/share/backgrounds/xbitos/default.jpg

        Ok(())
    }

    fn setup_startup_script(&self) -> Result<()> {
        let startup_script = r#"#!/bin/bash
# تصدير المتغيرات البيئية الضرورية
export XDG_SESSION_TYPE=wayland
export XDG_CURRENT_DESKTOP=Hyprland
export XDG_SESSION_DESKTOP=Hyprland
export QT_QPA_PLATFORM=wayland
export QT_WAYLAND_DISABLE_WINDOWDECORATION=1
export GDK_BACKEND=wayland
export MOZ_ENABLE_WAYLAND=1
export _JAVA_AWT_WM_NONREPARENTING=1

# تشغيل Hyprland
exec Hyprland
"#;

        let script_path = PathBuf::from("/usr/local/bin/start-hyprland");
        fs::write(&script_path, startup_script)?;
        
        // جعل السكريبت قابل للتنفيذ
        Command::new("chmod")
            .args(["+x", script_path.to_str().unwrap()])
            .status()?;

        Ok(())
    }

    fn setup_sddm_session(&self) -> Result<()> {
        let session_file = r#"[Desktop Entry]
Name=xBitOS Hyprland
Comment=Hyprland Wayland Compositor
Exec=/usr/local/bin/start-hyprland
Type=Application
"#;

        fs::create_dir_all("/usr/share/wayland-sessions")?;
        fs::write(
            "/usr/share/wayland-sessions/hyprland.desktop",
            session_file
        )?;

        Ok(())
    }

    fn install_required_fonts(&self) -> Result<()> {
        let font_packages = vec![
            "ttf-jetbrains-mono-nerd",
            "ttf-nerd-fonts-symbols",
            "noto-fonts",
            "noto-fonts-cjk",
            "noto-fonts-emoji",
        ];

        let pkg_manager = crate::system::package_manager::PackageManager::new();
        pkg_manager.install_packages(&font_packages)?;

        Ok(())
    }

    fn setup_gtk_config(&self) -> Result<()> {
        let gtk_settings = r#"[Settings]
gtk-theme-name=Breeze
gtk-icon-theme-name=Papirus
gtk-font-name=Noto Sans 10
gtk-cursor-theme-name=Breeze_Snow
gtk-cursor-theme-size=24
gtk-toolbar-style=GTK_TOOLBAR_BOTH_HORIZ
gtk-toolbar-icon-size=GTK_ICON_SIZE_LARGE_TOOLBAR
gtk-button-images=1
gtk-menu-images=1
gtk-enable-event-sounds=1
gtk-enable-input-feedback-sounds=0
gtk-xft-antialias=1
gtk-xft-hinting=1
gtk-xft-hintstyle=hintslight
gtk-xft-rgba=rgb
gtk-application-prefer-dark-theme=1
"#;

        fs::create_dir_all("/etc/gtk-3.0")?;
        fs::write("/etc/gtk-3.0/settings.ini", gtk_settings)?;

        Ok(())
    }

    fn setup_qt_config(&self) -> Result<()> {
        let qt_settings = r#"[Appearance]
color_scheme_path=/usr/share/color-schemes/BreezeDark.colors
style=Breeze
icon_theme=Papirus
font="Noto Sans,10,-1,5,50,0,0,0,0,0"
"#;

        fs::create_dir_all("/etc/xdg/qt5ct")?;
        fs::write("/etc/xdg/qt5ct/qt5ct.conf", qt_settings)?;

        Ok(())
    }

    fn setup_dunst_config(&self) -> Result<()> {
        let dunst_config = r###"
[global]
    monitor = 0
    follow = mouse
    width = 300
    height = 300
    origin = top-right
    offset = 10x50
    scale = 0
    notification_limit = 0
    progress_bar = true
    progress_bar_height = 10
    progress_bar_frame_width = 1
    progress_bar_min_width = 150
    progress_bar_max_width = 300
    indicate_hidden = yes
    transparency = 0
    separator_height = 2
    padding = 8
    horizontal_padding = 8
    text_icon_padding = 0
    frame_width = 2
    frame_color = "#89B4FA"
    separator_color = frame
    sort = yes
    font = "JetBrains Mono Nerd Font 10"
    line_height = 0
    markup = full
    format = "<b>%s</b>\n%b"
    alignment = left
    vertical_alignment = center
    show_age_threshold = 60
    ellipsize = middle
    ignore_newline = no
    stack_duplicates = true
    hide_duplicate_count = false
    show_indicators = yes
    icon_position = left
    min_icon_size = 0
    max_icon_size = 32
    sticky_history = yes
    history_length = 20
    browser = "/usr/bin/xdg-open"
    always_run_script = true
    title = "Dunst"
    class = "Dunst"
    corner_radius = 10

[urgency_low]
    background = "#1E1E2E"
    foreground = "#CDD6F4"
    timeout = 10

[urgency_normal]
    background = "#1E1E2E"
    foreground = "#CDD6F4"
    timeout = 10

[urgency_critical]
    background = "#1E1E2E"
    foreground = "#F38BA8"
    frame_color = "#F38BA8"
    timeout = 0
"###;

        fs::create_dir_all("/etc/dunst")?;
        fs::write("/etc/dunst/dunstrc", dunst_config)?;
        Ok(())
    }
} 