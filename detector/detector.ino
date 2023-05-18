#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>

#define SCREEN_WIDTH 128 // OLED display width, in pixels
#define SCREEN_HEIGHT 64 // OLED display height, in pixels

// Declare SSD1306 display
Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, -1);

// Ultrassonic sensor's pins
const int trigPin = 5;
const int echoPin = 18;

// Speed of sound in cm/uS
#define SOUND_SPEED 0.034

long duration = 0;
int distanceCm = 0;

#define THRESHOLD 8 // Threshold, in cm, for the device to consider a bin full

bool full = false;

void setup() {
  Serial.begin(115200);

  pinMode(trigPin, OUTPUT); // Sets the trigPin as an Output
  pinMode(echoPin, INPUT); // Sets the echoPin as an Input

  // Try to allocate display at 0x3C
  if(!display.begin(SSD1306_SWITCHCAPVCC, 0x3C)) {
    Serial.println(F("SSD1306 allocation failed"));
    for(;;);
  }

  display.clearDisplay();
  display.setTextSize(2);
  display.setTextColor(WHITE);
}

void loop() {
  // https://randomnerdtutorials.com/esp32-hc-sr04-ultrasonic-arduino/

  // Clears the trigPin
  digitalWrite(trigPin, LOW);
  delayMicroseconds(2);

  // Sets the trigPin on HIGH state for 10 micro seconds
  digitalWrite(trigPin, HIGH);
  delayMicroseconds(10);
  digitalWrite(trigPin, LOW);
  
  // Reads the echoPin, returns the sound wave travel time in microseconds
  duration = pulseIn(echoPin, HIGH);
  
  // Calculate the distance
  distanceCm = duration * SOUND_SPEED/2;

  // Check if the distance is within the threshold and set status to full if necessary
  full = distanceCm <= THRESHOLD;
  
  // Prints the distance in the Serial Monitor
  Serial.print("Distance (cm): ");
  Serial.println(distanceCm);

  // Prints distance or "full" in the display
  display.clearDisplay();
  display.setCursor(0, 25);

  if (full) {
    display.print("FULL");

    // Only make API call if full

  } else {
    display.print(distanceCm);
    display.print(" cm");
  }

  display.display(); 

  // Update every 3 seconds 
  delay(3 * 1000);  
}
