import machine
import ssd1306
import time

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

# Threshold, in cm, for the device to consider a bin full
THRESHOLD = 8

# Initialize variables
duration = 0
distance_cm = 0
full = False

# Setup display
display.fill(0)
display.text('Distance:', 0, 0, 1)
display.show()

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

    # Check if the distance is within the threshold and set status to full if necessary
    full = distance_cm <= THRESHOLD

    # Prints the distance
    print('Distance (cm):', distance_cm)

    # Prints distance or "full" in the display
    display.fill(0)
    display.text('Distance:', 0, 0, 1)
    display.text('FULL' if full else f'{distance_cm} cm', 0, 16, 1)
    display.show()

    # Update every 3 seconds
    time.sleep(3)
