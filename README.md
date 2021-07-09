# Bus Protocol Learning
Rust program to analyze bus dumps and automatically identify messages such as vehicle speed, battery charge, acceleration, ...

The program is written in a generic way to allow analysis of any bus architecture (CAN, UART, I2C, Modbus, ...) as long as the bus consists of a header and corresponding values.

It is currently tested using bus dumps of
- A Renault Twizy
- An ESA 5000 (e Scooter sold by Lidl)
- A Tesla Model 3 Performance

## Filters
A filter json file shall contain a JSON array of zero or more of the following implemented filters
These filters usually return a confidence bewteen `0.0` and `1.0`. The confidence of all filters are multiplied together resulting the overall confidence.
Your goal is to cleverly combine the below filters to find the bus messages you are looking for.

### Common scale
```json
{
	"CommonScale": {
		"min": 80.0,
		"max": 120.0
	}
}
```
Often bus messages have to be scaled or offset. Bus definition formats such as DBC or KCD allow to define a slope and offset.
Given a known value range (i.e. the speed of a vehicle during a test drive) this filter tries to fit a given extraction into that value range using common slopes.

### Require spread
```json
{
	"RequireSpread": {
		"min": 80.0,
		"max": 120.0,
		"ratio": 0.9
	}
}
```
Given a constant value x there always exists a slope y to fit x into any given value range. For example a constant value of 10 can be fit into `80-120` with a slope of `8` or `10`.
This filter makes sure the extraction fills the the given `min-max` range by at least `range`%. For example a value which constantly rises from `100` to `120` fills the range `80-120` by `50%`.

### Require in range
```json
{
	"RequireInRange": {
		"min": 80.0,
		"max": 120.0,
		"ratio": 0.9
	}
},
```
This filter simply rejects all extractions where less than `ratio`% of the values do not fall into the given value range.

### Oscillating sensor
```json
{
	"OscillatingSensor": {
		"min_oscillating_ratio": 0.5
	}
},
```
This filter is specifically for sensor values with a rounding error. E.g. a temperature value often oscillates between two temperature values.

### Require local minima/maxima
```json
{
	"RequireLocalMinMax": {
		"window_size": 10,
		"required_count": 5
	}
},
```
Values like the speed of a vehicle, its temperature of acceleration have multiple local minima / maxima (i.e. turning points in the graph)
This filter allows to require a minimum amount of these points in an extraction allowing to easily filter out false positivies with linear characteristics

### Monotonic change
```json
{
	"MonotonicChange": {
		"max_change": 3.5,
		"required_monotonic_ratio": 0.97
	}
},
```
This filter makes sure the value of an extraction changes at most `max_change`.
The `required_monotonic_ratio` parameter defines on how many percent of all value changes this criteria has to be met.

### Strict rising
```json
"StrictRising",
```
Simple filter dropping all extractions with falling values

```json
"StrictFalling",
```
Simple filter dropping all extractions with rising values

```json
"RequireNonConstant",
```
Simple filter dropping all constant extractions

```json
"RequireConstant",
```
Simple filter dropping all non-constant extractions

```json
"StripInitialization"
```
This filter removes all values which are zero or all 1 which often occur during initialization of the bus.
This does filter does not filter extractions but instead removes values inspected by other filters.


## License
Copyright (C) 2021 Jakob Löw <jakob@löw.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
