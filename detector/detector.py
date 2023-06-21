import machine
import ssd1306
import time
import urequests
from ujson import dumps
import network
import math

SCREEN_WIDTH = 128  # OLED display width, in pixels
SCREEN_HEIGHT = 64  # OLED display height, in pixels

# Initialize I2C interface and SSD1306 display
i2c = machine.SoftI2C(scl=machine.Pin(22), sda=machine.Pin(21))
display = ssd1306.SSD1306_I2C(SCREEN_WIDTH, SCREEN_HEIGHT, i2c)

# Ultrasonic sensor's pins
trig_pin = machine.Pin(5, machine.Pin.OUT)
echo_pin = machine.Pin(18, machine.Pin.IN)

# Speed of sound in cm/uS
SOUND_SPEED = 0.034

# Full height of the bin in cm
FULL_HEIGHT = 50

# Wi-Fi credentials
WIFI_SSID = "Wokwi-GUEST"
WIFI_PASSWORD = ""

# Initialize variables
duration = distance_cm = status = previous_status = 0

# Connect to Wi-Fi
print("Connecting to WiFi", end="")
wifi = network.WLAN(network.STA_IF)
wifi.active(True)
wifi.connect(WIFI_SSID, WIFI_PASSWORD)
while not wifi.isconnected():
    print(".", end="")
    time.sleep(0.1)
print(" Connected!")

def update_display():
    # Update the display
    display.fill(0)
    display.text('Status:', 0, 0, 1)
    display.text(f'{status * 100}%', 0, 16, 1)
    display.show()

update_display()

while True:
    # Clears the trigPin
    trig_pin.off()
    time.sleep_us(2)

    # Sets the trigPin on HIGH state for 10 microseconds
    trig_pin.on()
    time.sleep_us(10)
    trig_pin.off()

    # Wait for the echoPin to go high and measure pulse duration
    pulse_start = pulse_end = time.ticks_us()
    while echo_pin.value() == 0:
        pulse_start = time.ticks_us()
    while echo_pin.value() == 1:
        pulse_end = time.ticks_us()

    # Calculate the pulse duration and convert to distance in cm
    pulse_duration = pulse_end - pulse_start
    distance_cm = pulse_duration * SOUND_SPEED / 2

    # Calculate the current status as a percentage
    if distance_cm <= FULL_HEIGHT:
        status = round(1 - distance_cm / FULL_HEIGHT, 2)
    else:
        status = 0.0

    print('Distance (cm):', distance_cm)
    print('Status (%):', status)

    # Check if the status has a variation of over 2%
    # This should prevent unnecessary calls to the API
    if math.fabs(status - previous_status) > 0.02:
        # Update previous_status for the next iteration
        previous_status = status

        update_display()

        # Send HTTP PATCH request
        request_url = "https://bin-monitor-api.fly.dev/bin"
        bin_data = dumps({"id": 2, "status": str(status)})
        r = urequests.patch(request_url, headers={'content-type': 'application/json'}, data=bin_data)
        print(dumps(r.json()))
        r.close()

    # Update every 3 seconds
    time.sleep(3)
