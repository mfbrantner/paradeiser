# Paradeiser
Minimal, terminal-based, cross-platform pomodoro timer.

## Usage
Ships with sensible defaults:
- Working time: 25 minutes
- Short break time: 5 minutes
- Long break time: 10 minutes
- Interval: [short] [short] [short] [long]

### Invocation
```
Usage: paradeiser [OPTIONS]

Options:
  -w, --work <work>          Working time (minutes) [default: 25]
  -s, --short <short_break>  Short break time (minutes) [default: 5]
  -l, --long <long_break>    Long break time (minutes) [default: 10]
  -h, --help                 Print help
  -V, --version              Print version
```
### Quitting
The application may be terminated using `Esc`, `Q`, or `CTRL-C`.

## Not implemented (yet?)
- Specifying the interval.
- More applealing GUI.

## Known Issues
- Colors are hardcoded, meaning readabilty may be affected by terminal color scheme
- Lack of tests
- Some structs are not named well
- Scuffed application architecture