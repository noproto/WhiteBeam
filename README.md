![WhiteBeam](https://gist.githubusercontent.com/noproto/ea7d62cd578afdd1bac2e96078c0e6b2/raw/cf895a5fef1f2295671653ece9155f4e1f0478e4/WhiteBeam.svg?sanitize=true)

[![Release](https://img.shields.io/badge/release-0.1.0-lightgrey?style=for-the-badge&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAABmJLR0QA/wD/AP+gvaeTAAAACXBIWXMAAA3XAAAN1wFCKJt4AAAE/UlEQVR42uWbXYhVVRTH/8u5MxbVpJRUZhgZYxoSpQ8lfUCURvnQBz2YBEMmGRURBD4UaBAU0cdYYdmXToSB0DwZhYgDRoVYic4kiqkTUmGTk4NjM87cub8e7r4wne65Z997zzn3Xuf/NnPP3mf9/2etvdZeZx+pCPg/9iomAGNF5h9XAgDeAw4B4xPuNd9nYDGMAeurNKgUhmMmfxA4DeQC91lYraFHEiBfQB/wfJXEdwODRYgXcFtcxm6NmTzOVf8EhiogvtGNzUXcY0m5Bpea8ETEPF8DZ6kMI2WQPwIMe857fxxPbKIox4uMXwYc8HgaURgEPixh5zEgW86Ecbnsz87dcAYcBTonjO8nXgwG7PsCGK1koiBXCxHglKSLywzDUUl/SbpMUpPqD1kzaw7+c0rIxTdJOlzmDVokzaxT8n2SFhX7oagAZnZU0kJJu9T46JR0nZnt8xbAiXBa0p2SbpbkW6kdqCPi/ZLuNbN2MzsbytNzUZwpaY9z8UbAdknLzWygCBeZWbQHBLzhd0mzJW2rc+Ijkp6WtDSMvO9DD/MEAU8GNhf1gv1AWwm7BbQCz8Sx0bgeOFknxMeBDiATQf5G4DDwqQ/BSzyuOQ/YWWPyfcDiCI8V8CjwjxuzyUeA48DtHtcZ8GwMJW8l2ApcFEH+AmBLYNxHvqVwFlgHmMf181IMiVPAI2Fl/YSnPg/oLTJ+Y7l7gW5gumdIfJkw+R3A5Z4uPxQyx4ZKNkP9wB2eIbEqAeLDwJpS3uiItwAfRMz1TqW7wRzwOjDFY/w1wImYyPcCCzzS8xyXCqPQUe12eE8pN5wwR4tbqCpFzqW3Zg/yy13/zwdvxNEPGATu86wZHqqgcPoDWOoRbhknUjl4Lc4e3idhBUhgviuAX8pIb5dGzNcEXAnsrcCzXolTgEJ3aLbHnBng/QivWh212Lp5lgB/VxhaL8ctAK7KeqyKdtt3wEqPsc3Aq1Uuqi8lIUABW4CpHvNPc9ePAt94rPLmxnwbQ1ZZm6QAuE5tm2fN0A7M8Mgmi4CBmNLqi14doSpwtaT9QHtEjwEz22xm/SXIXyhppaTdkqbHZN940gJI0lRJm4Au4PwKt9wzJH0uaUPMdqYiQAEPSDrk9Ub2v+TnS/pJ0rIEbEpVAEm6StI+4LmonaVLcSsk9UialZA9qQsgSRlJb0raHraHB1oldUj6LGG7aiJAAXdJ+iGYJdzf3ZKeSsGGmgogSW2SDrpmSxPwoKQf3dsopS1ARrWBSVoraYWkOVW1qstHrh4EKODaGtwzW+sQqDVyk12ASe8B45NdgAWTWQAkjdVTFkg79leb2cc+ApBybk4aZyQ9bGZfBX8IC4Ge4GrZwBiQdHcx8qECmNkNkjYrf/KrkdEnabGZfV+qJI3am5+S1NqAIdEr6R4z+63URZFZwMymKX9kLtdA5Lsl3RpF3jsNmtlc5c8HNcK60KX86bDBeBNovju7Bvi1Ts8IAbzt8/I2DjF2lHtIOWHkgHXpllTwVsSHCWkhCzyefk3pGpzu3WCtQmLI9y110mJ01UCEk6VOiKUtQAZ4wr2xTSMkjgFz63O7BbsS9oYeYFZdJ2LghZDvA6tFd+NsvOEW56pxhURXY3YgoDOGkHi3sdsw+a/IBhuiwElYiG1leEMWWHXuNeXyp0JGIsifObc7k/mTpD1hBc7kadHC+kCW6Evbhn8B9AX59hwJv14AAAAASUVORK5CYII=)
[![Bounty](https://img.shields.io/badge/bounty-$150-green?style=for-the-badge)](https://github.com/noproto/WhiteBeam/security/policy)
[![Discord](https://img.shields.io/discord/641744447289294859?style=for-the-badge&label=Discord)](https://discord.gg/GYSVqYx)

WhiteBeam is an OSS EDR application with cross platform application whitelisting. While it is in pre-release development, it should not be used in production environments.

# Getting Started

## Installation

### From Binaries

**Important**: Always ensure the downloaded file hash matches official hashes ([How-to](https://github.com/noproto/WhiteBeam/wiki/Verifying-file-hashes)).

| Platform       | URL                                                                | Hash(es) |
| -------------- | ------------------------------------------------------------------ | -------- |
| Linux (64-bit) | https://dist.whitebeamsec.com/linux/x86_64/WhiteBeam_latest.tar.gz | [SHA-256](https://dist.whitebeamsec.com/linux/x86_64/WhiteBeam_latest.SHA256) |

Install WhiteBeam: `./install.sh`

### From Source (Linux)

1. (Optional) Run tests:
`make test`
2. Compile:
`make`
3. Install WhiteBeam:
`make install`

## Configuring

1. Add permitted applications:
`whitebeam --add /absolute/path/to/command`
2. Enable WhiteBeam:
`whitebeam --enable`

# In Action

[![asciicast](https://asciinema.org/a/291097.svg)](https://asciinema.org/a/291097)
