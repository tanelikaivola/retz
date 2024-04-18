# retz - retimezone

Convert timestamps to other timezones.

## Installation

### With Cargo

```
cargo install retz
```

## Usage

```
Command-line tool to convert timestamps to other timezones

Usage: retz [OPTIONS] [DATE]

Arguments:
  [DATE]  Date time in only proper format (RFC3339)

Options:
  -t, --to <TZ>  Convert to timezone
  -q, --quiet    Quiet, return just either target (if defined) or UTC
  -a, --all      List all timezones
  -o, --order    Order by offset instead of alphabetical
  -h, --help     Print help
  -V, --version  Print version
```

### Current time to UTC and local

```bash
$ retz
   UTC: 2024-03-31T09:34:56Z
 Local: 2024-03-31T12:34:56+03:00
```

### Current time to UTC (scriptable output)

```bash
$ retz -q
2024-03-31T12:00:00Z
```

### Input time to convert to UTC and local timezones

```bash
$ retz "2024-03-31T12:00:00Z"   
 Input: 2024-03-31T12:00:00Z
   UTC: 2024-03-31T12:00:00Z
 Local: 2024-03-31T15:00:00+03:00
```

### Convert to specific timezone

```bash
$ retz -t "Australia/Eucla" 
   UTC: 2024-03-31T12:00:00Z
 Local: 2024-03-31T15:00:00+03:00
Target: 2024-03-31T20:45:00+08:45
```

### Convert to all known timezones

```bash
$ retz -a 
                             UTC: 2024-03-31T12:00:00Z
                           Local: 2024-03-31T15:00:00+03:00
                  Africa/Abidjan: 2024-03-31T12:00:00Z
                    Africa/Accra: 2024-03-31T12:00:00Z
              Africa/Addis_Ababa: 2024-03-31T15:00:00+03:00
                             ...
                             UTC: 2024-03-31T12:00:00Z
                       Universal: 2024-03-31T12:00:00Z
                            W-SU: 2024-03-31T15:00:00+03:00
                             WET: 2024-03-31T13:00:00+01:00
                            Zulu: 2024-03-31T12:00:00Z
```