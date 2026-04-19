
# xEdge Kernel

**AI-native RTOS for tiny devices**  
The scheduler *thinks*. It predicts workloads, saves 30-60% power, detects anomalies, and runs lightweight OpenClaw-style agents directly on microcontrollers.

## Why xEdge
Most embedded devices still use dumb fixed rules in 2026. xEdge puts a tiny quantized AI model *inside the kernel* so the device can predict instead of just react.

- Predictive scheduling & power management
- Anomaly detection and self-healing
- OpenClaw-ready agent layer
- <6 KB inference footprint
- Real-time on cheap chips (ESP32, RISC-V, Cortex-M)

## Quick Try
Live browser demo coming very soon via Wokwi (no hardware needed).

## Features
- Tiny int8 GrokEdge model runs every scheduling tick
- xClaw agents trigger on predictions (e.g. vibration watchdog)
- Designed for OpenClaw wave — make agents smarter and longer-lasting on real hardware
- Future: Zero-trust P2P with Diode.io for secure agent swarms

## Status
Prototype v0.3 complete. Public demo and custom model training coming next.

## License
MIT

## Contributing
See CLA.md (coming soon). All contributions welcome — this is early stage.

Built with vibe + Grok energy.  
Let's ride the OpenClaw wave.