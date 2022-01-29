# API Server

# What it does
This repository contains a little API [webserver] completely written in the rust language.
It acts as a layer between the [WeatherGui], [Sensordrivers] and the locally sqlite database which contains all sensor data.
The frontend can send API request to the server to receive various measurement data.

# Progress

## Implemented
* Query SQLite database based on predefined API calls

## Todo
* Calculating weather forecast based on sensor data
* Monitoring during runtime
* Minimize use of external crates to make the application more lightweight

# Annotations
This application does not run stand alone and depends from [WeatherGui] and [Sensordrivers].

[webserver]: https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html (webserver template)
[WeatherGui]: https://github.com/wolfbiker1/weatherGui (WeatherGui)
[Sensordrivers]: https://github.com/wolfbiker1/sensorDrivers (Sensordrivers)
[APIServer]: https://github.com/wolfbiker1/weatherStationAPIServer (APIServer)