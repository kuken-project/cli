# cli

## Installation

### Linux and macOS

```bash
curl -fsSL https://kuken.io/binaries/cli/install.sh | bash
```

### Windows

```powershell
irm https://kuken.io/binaries/cli/install.ps1 | iex
```

## Getting Started

For available commands:

```bash
kuken help
```

## Troubleshooting

### Permission Denied (Linux/macOS)

If you get a permission denied error:
```bash
chmod +x kuken
```

### Command Not Found

Make sure the installation directory is in your PATH:

**Linux/macOS:**
```bash
echo $PATH
```

**Windows:**
```powershell
$env:Path
```

### macOS "Cannot be opened because the developer cannot be verified"

Run:
```bash
xattr -d com.apple.quarantine /usr/.local/bin/kuken
```

## Uninstallation

### Linux / macOS

```bash
sudo rm $HOME/.local/bin/kuken
```

### Windows

```powershell
Remove-Item "$HOME\AppData\Local\Kuken\kuken-cli.exe"
```