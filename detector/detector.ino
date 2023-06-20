#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>
#include <WiFi.h>
#include <HTTPClient.h>
#include <ArduinoJson.h>
#include <string>

#define SCREEN_WIDTH 128 // OLED display width, in pixels
#define SCREEN_HEIGHT 64 // OLED display height, in pixels

// Declare SSD1306 display
Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, -1);

// Ultrassonic sensor's pins
const int trigPin = 18;
const int echoPin = 19;

// Speed of sound in cm/uS
#define SOUND_SPEED 0.034

long duration = 0;
float distanceCm = 0;

#define FULL_HEIGHT 40.0 // Full height of the bin in cm

// Wi-Fi credentials
const char* WIFI_SSID = "WOWKW";
const char* WIFI_PASSWORD = "dsadadasd";

const char* requestUrl = "https://bin-monitor-api.fly.dev/bin";

HTTPClient http;

float status = 0.0; // Represents the percentage of how full the bin is
float previousStatus = 0.0; // Tracks the previous status

void setup() {
  Serial.begin(115200);

  pinMode(trigPin, OUTPUT); // Sets the trigPin as an Output
  pinMode(echoPin, INPUT); // Sets the echoPin as an Input

  // Try to allocate display at 0x3C
  if(!display.begin(SSD1306_SWITCHCAPVCC, 0x3C)) { 
    Serial.println(F("SSD1306 allocation failed"));
    for(;;); // Don't proceed, loop forever
  }

  display.display();
  display.setTextSize(2);
  display.setTextColor(WHITE);

  display.clearDisplay();
  display.setTextSize(2);
  display.setTextColor(WHITE);


  // Connect to Wi-Fi
  Serial.print("Connecting to WiFi");
  display.clearDisplay();
  display.setCursor(0, 0);
  display.print("Connecting to WiFi");

  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

  while (WiFi.status() != WL_CONNECTED) {
    Serial.print(".");
    display.print(".");
    
    delay(100);
  }

  Serial.println(" Connected!");
  display.print(" Connected!");
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
  distanceCm = duration * SOUND_SPEED / 2.0;
  
  Serial.print("Distance (cm): ");
  Serial.println(distanceCm);
  Serial.print("Status (%): ");
  Serial.println(status);

  // Calculate the current status as a percentage
  if (distanceCm <= FULL_HEIGHT) {
    status = 1 - distanceCm / FULL_HEIGHT;
  } else {
    status = 0.0;
  }

  // Check if the status has a variation of over 3%
  // This sort of threshold exists to prevent making API calls for no reason
  if (abs(status - previousStatus) > 0.03) {
    // Update the display
    display.clearDisplay();
    display.setCursor(0, 0);
    display.print("Status:");
    display.setCursor(0, 25);
    display.print(status * 100);
    display.print("%");
    display.display();

    previousStatus = status;

    StaticJsonDocument<200> binData;
    binData["id"] = 1;
    binData["status"] = std::to_string(status);

    String payload;
    serializeJson(binData, payload);

    Serial.println("payload: " + payload);

    http.begin(requestUrl);
    http.addHeader("Content-Type", "application/json");
    int httpCode = http.PATCH(payload);

    Serial.println(httpCode);
  }

  // Update every 10 seconds 
  delay(800);  
}
