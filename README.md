# 🚀 PC Optimizer - Tauri + Rust

A modern Windows PC optimization application built with **Tauri** (Rust backend) and **React** (TypeScript frontend). Features a beautiful dark-themed UI similar to Paragon and Hone.

## ✨ Features

### 🔧 Tweaks System (7+ Tweaks)
- **Privacy**: Disable telemetry, Cortana, and data collection
- **Performance**: Disable animations, search indexing
- **Gaming**: Enable Game Mode, disable fullscreen optimizations
- **Bloatware**: Disable Xbox services, OneDrive
- **System**: Enable long paths, disable fast startup

### 📊 Dashboard
- Real-time CPU, memory, and disk usage monitoring
- Quick action buttons for cleaning operations
- System performance metrics

### 💾 Backup & Restore
- Automatic backup of system state before applying tweaks
- Easy restore functionality with timestamped backups

### 🔍 Search & Filter
- Search tweaks by name, description, or category
- Filter by category (Privacy, Performance, Gaming, etc.)
- Real-time filtering

## 🛠️ Tech Stack

- **Frontend**: React 18 + TypeScript + Tailwind CSS
- **Backend**: Rust + Tauri 1.5
- **System APIs**: Windows Registry, PowerShell, WMI
- **State Management**: React Hooks
- **IPC**: Tauri Commands

## 📋 Prerequisites

- **Node.js 16+**
- **Rust 1.56+** (install from https://rustup.rs/)
- **Windows 10/11**
- **Administrator rights** (for applying tweaks)

## 🚀 Quick Start

### 1. Clone the Repository
```bash
git clone https://github.com/maikimanuel/pc-optimizer-tauri.git
cd pc-optimizer-tauri
```

### 2. Install Dependencies
```bash
npm install
```

### 3. Run in Development Mode
```bash
npm run tauri dev
```

The app will open in a new window with hot-reload enabled!

### 4. Build for Production
```bash
npm run tauri build
```

The executable will be in `src-tauri/target/release/`

## 📁 Project Structure

```
pc-optimizer-tauri/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs                 # Tauri app entry
│   │   ├── commands.rs             # IPC command handlers
│   │   ├── tweaks/
│   │   │   ├── mod.rs             # Tweaks definitions (7 tweaks)
│   │   │   ├── registry.rs        # Registry operations
│   │   │   ├── services.rs        # Service management
│   │   │   └── cleanup.rs         # File cleanup
│   │   ├── backup.rs              # Backup/restore system
│   │   └── monitor.rs             # System monitoring
│   ├── tauri.conf.json            # Tauri config
│   └── Cargo.toml                 # Rust dependencies
│
├── src/
│   ├── App.tsx                    # Main app component
│   ├── components/
│   │   ├── Dashboard.tsx          # System stats dashboard
│   │   ├── TweaksPanel.tsx        # Tweaks list with filters
│   │   └── Search.tsx             # Search component
│   ├── App.css                    # Tailwind styles
│   └── main.tsx                   # React entry point
│
├── public/
├── package.json
├── tsconfig.json
├── vite.config.ts
└── README.md
```

## 🎯 Available Tweaks

### Privacy & Telemetry
- ✅ **Disable Telemetry** - Disable Windows data collection
- ✅ **Disable Cortana** - Turn off Cortana assistant

### Performance
- ✅ **Disable Animations** - Faster UI response
- ✅ **Disable Search Indexing** - Free up disk I/O

### Gaming
- ✅ **Enable Game Mode** - Better gaming performance

### Bloatware
- ✅ **Disable Xbox Services** - Remove Xbox background services
- ✅ **Disable OneDrive** - Stop cloud sync

### System
- ✅ **Enable Long Paths** - Support paths >260 characters

## 💻 Usage

### Applying a Tweak
1. Click on the **Tweaks** tab
2. Search or filter by category
3. Click the toggle button next to a tweak
4. Confirm the action
5. Tweak is applied instantly!

### Creating a Backup
1. Go to the **Backups** tab
2. Click "Create Backup"
3. Your system state is saved
4. Easily restore later if needed

### Searching Tweaks
1. Use the search bar at the top
2. Filter by category
3. Results update in real-time

## 🔐 Security & Safety

- ✅ All tweaks are **revertible**
- ✅ **Automatic backups** before applying changes
- ✅ Registry operations are **validated**
- ✅ Service operations are **safe** (stop/start only)
- ✅ **Administrator required** for safety
- ⚠️ Always backup before making changes

## 🐛 Troubleshooting

### Error: "Administrator rights required"
- Run the application as Administrator
- Right-click the .exe → Run as administrator

### Build error: "Could not find cargo"
- Install Rust: https://rustup.rs/
- Restart your terminal/IDE

### Registry operations fail
- Ensure you're running as Administrator
- Check Windows edition compatibility
- Verify registry permissions aren't blocked by policy

### npm install fails
- Delete `node_modules` and `package-lock.json`
- Run `npm install` again
- If still failing, try: `npm cache clean --force`

## 🔨 Development Commands

```bash
# Development with hot-reload
npm run tauri dev

# Build frontend
npm run build

# Build production app
npm run tauri build

# View Tauri config
npm run tauri info
```

## 📦 Building for Release

```bash
# Create optimized production build
npm run tauri build -- --release

# Output location
sr-tauri/target/release/pc_optimizer.exe
```

## 🤝 Contributing

Contributions are welcome! To add a new tweak:

1. Edit `src-tauri/src/tweaks/mod.rs`
2. Add a new `Tweak` struct to `get_all_tweaks()`
3. Define registry keys and services
4. Test thoroughly
5. Submit a Pull Request

## 📄 License

MIT License - feel free to use and modify

## ⚠️ Disclaimer

This tool modifies:
- Windows Registry
- System Services
- System Files

**Use at your own risk!** Always:
- Create backups before applying tweaks
- Test on a non-critical system first
- Understand what each tweak does
- Keep restore points enabled

The developers are **NOT responsible** for any system damage caused by improper use.

## 🎨 Inspired By

- Paragon
- Hone
- Chris Titus Tech's WinUtil
- SophiApp

## 🌟 Star & Follow

If you found this useful, please star the repository! ⭐

---

**Made with ❤️ using Tauri and Rust**

**Questions?** Open an issue on GitHub!
