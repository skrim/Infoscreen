import Mustache from 'https://cdnjs.cloudflare.com/ajax/libs/mustache.js/4.2.0/mustache.min.js';

export async function fetchJson(url) {
    const response = await fetch(url);
    return await response.json();
}

export function fetchIcons(url) {
    const xhr = new XMLHttpRequest();
    xhr.open('GET', url, false); // Set async to false

    try {
        xhr.send();
        if (xhr.status === 200) {
            const xmlDoc = new DOMParser().parseFromString(xhr.responseText, 'text/xml');
            const svgDict = {};

            const svgElements = xmlDoc.querySelectorAll('svg');
            svgElements.forEach((svgElement) => {
                const id = svgElement.getAttribute('id');

                // Serialize the SVG element back to a string
                const serializer = new XMLSerializer();
                const svgString = serializer.serializeToString(svgElement);

                svgDict[id] = svgString;
            });

            return svgDict;
        } else {
            throw new Error(`Failed to fetch icons. Status code: ${xhr.status}`);
        }
    } catch (error) {
        throw new Error(`Error fetching icons: ${error.message}`);
    }
}

export async function bind() {

    const dateFormatOptions = { timeZone: 'Europe/Helsinki', hour12: false, year: 'numeric', weekday: 'short',
        month: 'short', day: '2-digit', hour: '2-digit', minute: '2-digit', second: '2-digit' };
    const dateTimeFormat = new Intl.DateTimeFormat('en-US', dateFormatOptions);

    const formatTime = time => {
        const parts = dateTimeFormat.formatToParts(time);
        return `${parts.find(p => p.type === 'hour').value}.${parts.find(p => p.type === 'minute').value}`;
    };

    const formatDate = date => {
        const parts = dateTimeFormat.formatToParts(date);
        const find = n => parts.find(p => p.type === n).value;
        return `${find('weekday')} ${find('day')} ${find('month')} ${find('year')}`;
    };

    const getWeekday = date => date.toLocaleString('en-US', { weekday: 'short' });

    const allData = globalThis.data;

    /*

    const bikeData = await fetchJson('/api/bikes');
    const weatherData = await fetchJson('/api/weather');
    const tramData = await fetchJson('/api/trams');
    const gymData = await fetchJson('/api/gym');
    */
    const icons = await fetchIcons('icons.xml');

    const bikeData = allData.bikes;
    const weatherData = allData.weather;
    const tramData = allData.trams;
    const gymData = allData.wodconnect;

    const now = new Date();

    const trams = tramData.data.stops.reduce((result, stop) => {
        const stopName = stop.name;
        const entries = stop.stoptimesWithoutPatterns.map(entry => ({ stopName, ...entry }) );
        return result.concat(entries);
    }, []);

    trams.sort((a, b) => a.realtimeDeparture - b.realtimeDeparture);


    const icon = () => (text, render) => {
        const parts = render(text).split(' ');
        const id = parts[0];
        console.log(id);
        switch (id) {
            case "winddirection":
                return icons[id].replace('<svg', `<svg style="transform: rotate(${parts[1]}deg)"`);
            default:
                return icons[id];
        }
    };

    const mapWeatherIcon = icon => {
        console.log(icon);
        switch (icon) {
            case "01d": return "wi-day-sunny";
            case "01n": return "wi-night-clear";
            case "02d": return "wi-day-sunny-overcast";
            case "02n": return "wi-night-cloudy";
            case "03d":
            case "03n": return "wi-cloud";
            case "04d":
            case "04n": return "wi-cloudy";
            case "09d": return "wi-day-showers";
            case "09n": return "wi-night-showers";
            case "10d":
            case "10n": return "wi-rain";
            case "11d":
            case "11n": return "wi-thunderstorm";
            case "13d":
            case "13n": return "wi-snow";
            case "50d":
            case "50n": return "wi-fog";
            default: throw new Error(`Unknown OpenWeatherMap icon ${icon}`);
        }
    }

    const temperatureRange = data => {
        const min = (data.min - 273.15).toFixed(0);
        const max = (data.max - 273.15).toFixed(0);

        if (min === max)
            return min;

        return `${min} ... ${max}`;
    }

    const fixHeadSign = headsign => {
        const viaIndex = headsign.indexOf(' via');
        return viaIndex < 0 ? headSign : headsign.substring(0, viaIndex);
    }

    let sunrise = new Date(weatherData.current.sunrise * 1000);
    let sunset = new Date(weatherData.current.sunset * 1000);
    if (now > sunrise) {
        sunrise = new Date(weatherData.daily[1].sunrise * 1000);
        sunset = new Date(weatherData.daily[1].sunset * 1000);
    }

    const data = {
        clock: {
            time: formatTime(now),
            date: formatDate(now),
            sunrise: formatTime(sunrise),
            sunset: formatTime(sunset),
        },
        stations: bikeData.data.bikeRentalStations.map(station => ({
            name: station.name,
            bikes: station.bikesAvailable,
            capacity: station.bikesAvailable + station.spacesAvailable
        })),
        weather: {
            today: {
                temperature: (weatherData.current.temp - 273.15).toFixed(0),
                wind: weatherData.current.wind_speed.toFixed(0),
                wind_deg: weatherData.current.wind_deg.toFixed(0),
                description: weatherData.current.weather[0].description,
                weathericon: mapWeatherIcon(weatherData.current.weather[0].icon),
                summary: weatherData.daily[0].summary
            },
            hourly: weatherData.hourly.filter((_, index) => index % 2 === 0).slice(0, 7).map(hour => ({
                time: formatTime(new Date(hour.dt * 1000)),
                temperature: (hour.temp - 273.15).toFixed(0),
                description: hour.weather[0].description,
                weathericon: mapWeatherIcon(hour.weather[0].icon),
                wind: hour.wind_speed.toFixed(0),
                wind_deg: hour.wind_deg.toFixed(0)
            })),
            daily: weatherData.daily.slice(1, 3).map(day => ({
                day: new Date(day.dt * 1000).toLocaleString('default', { weekday: 'long' }),
                temperature: temperatureRange(day.temp),
                summary: day.summary,
                description: day.weather[0].description,
                weathericon: mapWeatherIcon(day.weather[0].icon),
                wind: day.wind_speed.toFixed(0),
                wind_deg: day.wind_deg.toFixed(0)
            }))
        },
        trams: trams.slice(0, 7).map(tram => {
            var scheduled = formatTime(new Date((tram.serviceDay + tram.scheduledDeparture) * 1000));
            var realtime = formatTime(new Date((tram.serviceDay + tram.realtimeDeparture) * 1000));
            return {
                departure: scheduled,
                delay: scheduled !== realtime ? `> ${realtime}` : '',
                headsign: fixHeadSign(tram.headsign),
                route: tram.trip.routeShortName
            };
        }),
        gym: {
            day: getWeekday(new Date(gymData.date)),
            workouts: gymData.workouts.map(w => ({name: w.title, description: w.description.replace(/\n/g, "<br>")}))
        },
        icon
    };

    console.log(data);

    const template = document.querySelector('#template').innerHTML;
    const html = Mustache.render(template, data);

    document.body.innerHTML += html;
    document.body.innerHTML += '<div class="done"></div>';
}

bind();
