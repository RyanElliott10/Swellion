# Swellion

## Quickly grab current ocean swell predictions from the command line

#### Usage

```
$ swellion fetch 3.233452 78.232345
[] 1.2ft 16s SW (216°)
[] 1.8ft 8s WNW (287°)
[] 1.3ft 5s WSW (249°)
[] 5kts
[] 6.4ft
[] 64°f
Last updated: 4m ago

$ swellion fetch alias "11th St"
(output) ...

$ swellion --alias 23.45253 155.2354565 "11th St" # creates an alias to allow the above usage
11th St -> 23.233452 78.232345

$ swellion --units
Surf Height
    3. Hawaiian
    2. Imperial
    1. Metric
Swell Height
    2. Imperial
    1. Metric
Temperature
    1. Celsius
    2. Fahrenheit
Wind Speed
    1. Knots
    2. KPH
    3. MPH
```