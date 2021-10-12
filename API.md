# Formula 1 data endpoints

**Base url** : `https://livetiming.formula1.com/static`

## Session info

**URL** : `/<year>/<year>-<month>-<day>_<Grand_Prix_Name>/<session year>-<session month>-<session day>_<session type>/SessionInfo.json`

**Example**
```json
{
  "Meeting": {
    "Key": 1045,
    "Name": "Austrian Grand Prix",
    "Location": "Spielberg",
    "Country": {
      "Key": 17,
      "Code": "AUT",
      "Name": "Austria"
    },
    "Circuit": {
      "Key": 19,
      "ShortName": "Spielberg"
    }
  },
  "ArchiveStatus": {
    "Status": "Complete"
  },
  "Key": 5748,
  "Type": "Qualifying",
  "Name": "Qualifying",
  "StartDate": "2020-07-04T15:00:00",
  "EndDate": "2020-07-04T16:00:00",
  "GmtOffset": "02:00:00",
  "Path": "2020/2020-07-05_Austrian_Grand_Prix/2020-07-04_Qualifying/"
}
```

## Session driver listing

**URL** : `/<year>/<year>-<month>-<day>_<Grand_Prix_Name>/<session year>-<session month>-<session day>_<session type>/DriverList.json`

**Example URLs**
| Session | URL |
|---|---|
| Qualifying 2020 Austrian Grand Prix | /2020/2020-07-05_Austrian_Grand_Prix/2020-07-04_Qualifying/DriverList.json |
| Race 2020 Austrian Grand Prix | /2020/2020-07-05_Austrian_Grand_Prix/2020-07-05_Race/DriverList.json |

**Example data**
```json
{
  "44": {
    "RacingNumber": "44",
    "BroadcastName": "L HAMILTON",
    "FullName": "Lewis HAMILTON",
    "Tla": "HAM",
    "Line": 2,
    "TeamName": "Mercedes",
    "TeamColour": "00D2BE",
    "FirstName": "Lewis",
    "LastName": "Hamilton",
    "Reference": "LEWHAM01",
    "HeadshotUrl": "https://www.formula1.com/content/dam/fom-website/drivers/L/LEWHAM01_Lewis_Hamilton/lewham01.png.transform/1col/image.png"
  },
}
```

## Race session timing app data

**URL** : `https://livetiming.formula1.com/static/2020/2020-07-05_Austrian_Grand_Prix/2020-07-05_Race/TimingAppData.jsonStream`

**Example**
```json
00:33:10.171{"Lines":{"31":{"Line":15},"8":{"Line":16},"20":{"Line":14}}}
```

Example:
```
https://livetiming.formula1.com/static/2020/2020-07-05_Austrian_Grand_Prix/2020-07-05_Race/TimingAppData.jsonStream
```

## Weather data

**URL** : `/<year>/<year>-<month>-<day>_<Grand_Prix_Name>/<session year>-<session month>-<session day>_<session type>/WeatherData.jsonStream`

TODO: Add note about time prefix.

**Example**
```json
{
  "AirTemp": "24.3",
  "Humidity": "35.1",
  "Pressure": "940.8",
  "Rainfall": "0",
  "TrackTemp": "48.9",
  "WindDirection": "344",
  "WindSpeed": "0.4"
}
```
https://livetiming.formula1.com/static/2020/2020-07-05_Austrian_Grand_Prix/2020-07-04_Qualifying/Heartbeat.jsonStream
