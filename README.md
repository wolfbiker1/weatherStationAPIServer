# API Server

This repository contains a little API webserver completely written in the rust language.
It acts as a layer between the [weather station frontend](https://github.com/wolfbiker1/weatherGui) and the sqlite database.
The frontend can send API request to the server to receive various measurement data.
Also the [sensorsuite](https://github.com/wolfbiker1/sensorDrivers) can communicate via an udp socket to feed the database with current sensor data.