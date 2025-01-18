# Konductor 
![Build Status](https://github.com/Manukhurana97/konductor/workflows/Konductor%20Builder/badge.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/Manukhurana97/konductor.svg?style=social&label=Star)](https://github.com/Manukhurana97/konductor/)
[![GitHub top language](https://img.shields.io/github/languages/top/Manukhurana97/konductor)](https://github.com/Manukhurana97/konductor/)






Konductor is a lightweight desktop application built with Angular 16 and Rust on the Tauri framework. It provides real-time monitoring of CPU, memory, and disk usage on Windows.

![image](https://github.com/Manukhurana97/konductor/assets/38296197/de5d99ef-c79e-47ef-8f5c-0cabfaf5f571)




## Features

- **CPU Usage:** Displays the average CPU usage across all cores.
- **Memory Usage:** Shows the current RAM consumption.
- **Disk Usage:** Presents an average of read and write operations.
- **Auto-Start:** The app automatically starts whenever the user logs in to Windows.

## System Requirements

- Windows OS
- Space Required: 10Mb
- Memory Required: 40-50Mb

## Installation

Download the latest release for Windows [here](https://github.com/Manukhurana97/konductor/releases/download/v0.1.4/konductor_0.1.4_x64_en-US.msi).

Install the application, and it will automatically start with each Windows login, providing real-time system resource information.

## Usage

Once installed, Konductor runs silently in the background, providing real-time system resource information. No additional configuration is required.

## Future Features

In future releases, we plan to add the following features:

- **Power Options:** Restart, sleep, and shutdown options.
- **Favicon Customization:** Choose from a variety of favicon options.
- **MacOS Support:** Extend the application to support MacOS.

## Contributing

We welcome contributions! To contribute to Konductor, follow these steps:

1. Fork the repository.
2. Create a new branch: `git checkout -b feature/new-feature`.
3. Commit your changes: `git commit -am 'Add new feature'`.
4. Push to the branch: `git push origin feature/new-feature`.
5. Submit a pull request.

## Version History

- **Beta 0.1.4:** [Download](https://github.com/Manukhurana97/konductor/releases/download/v0.1.4/konductor_0.1.4_x64_en-US.msi)

## License

This project is licensed under the [MIT License](LICENSE) - see the LICENSE file for details.





[alias]
    gitship = "!f() { git add . && git commit -m \"$1\" && git push origin $(git symbolic-ref --short HEAD); }; f"


