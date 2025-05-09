# RC Car Controller (Rust + Xbox Controller)

Control your Traxxas Stampede XL-5 RC car using an Xbox controller over WiFi.

---

## How to Use

1. **Power on your RC car.**
2. **Connect your computer to the Wi-Fi network named `RC_CAR`.**
3. **Plug in or connect your Xbox controller.**

4. **Run the program using one of the following methods:**

---

### Option 1: Run via Nix

If you have Nix installed, run it directly from the repository:

```bash
nix run github:Feelfeel20088/Traxxas_Stampede_XL-5
```

---

### Option 2: Download the Prebuilt Binary

1. Go to the [Releases](https://github.com/Feelfeel20088/Traxxas_Stampede_XL-5/releases) section.
2. Download the binary for your system.
3. Give it execute permissions if needed:

   ```bash
   chmod +x rc-car-controller
   ./rc-car-controller
   ```

---

### Option 3: Build from Source

1. [Install Rust](https://rustup.rs)
2. Clone the repository:

   ```bash
   git clone https://github.com/Feelfeel20088/Traxxas_Stampede_XL-5
   cd Traxxas_Stampede_XL-5
   ```

3. Build the binary:

   ```bash
   cargo build --release
   ```

4. Navigate to the output directory and run the binary:

   ```bash
   ./target/release/rc-car-controller
   ```

---

## ⚠️ Supported Hardware

This software is designed **specifically** for the **Traxxas Stampede XL-5**.  
Using it with other RC cars **may or may not work** and is not officially supported.

---

## Requirements

- Xbox controller (USB or Bluetooth)
- Your RC car must be listening on UDP port `1337` at `192.168.1.1`
- Internet connection (for Nix-based installation only)
