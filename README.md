# g610

Simple rust-based CLI for driving the Logitech G610 keyboard LEDs.

## Setup

1. Configure udev to allow access to the hid device:

```bash
echo 'KERNEL=="hidraw*", ATTRS{idVendor}=="046d", ATTRS{idProduct}=="c338", MODE="660", GROUP="YOUR_USER_GROUP", TAG+="uaccess"' \
  | sudo tee /etc/udev/rules.d/99-g610.rules

sudo udevadm control --reload-rules
sudo udevadm trigger
```

2. Compile and run:

```bash
git clone
cargo build
cargo run -- set-mode --help
```

## Reference

* [OpenRGB docs](https://openrgb-wiki.readthedocs.io/en/latest/Logitech-Keyboards/#logitech-g410-g512-g513-g610-g810-gpro)
