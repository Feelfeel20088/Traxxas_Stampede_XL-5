#include <ESP8266WiFi.h>
#include <WiFiUdp.h>
#include <Servo.h>

struct RC_DRIVE {
  uint8_t magic;
  uint16_t steer;
  uint16_t throttle;
 
} __attribute__((packed));



IPAddress local_ip(192, 168, 1, 1);  // Static IP
IPAddress subnet(255, 255, 255, 0);  // Subnet mask
unsigned int localPort = 1337;       // Port to listen for incoming UDP packets
#define PAYLOAD_PACKET_SIZE 5
WiFiUDP Udp;

uint8_t incomingPacket[PAYLOAD_PACKET_SIZE];

Servo steerServo; // Servo object for steering
Servo throttleESC; // Servo object for throttle


void setup() {
  Serial.begin(9600);
  Serial.println();
  throttleESC.writeMicroseconds(1500);
  delay(1000);

  WiFi.softAP("RC_CAR", "");
  
  WiFi.softAPConfig(local_ip, local_ip, subnet);

  Udp.begin(localPort);
  Serial.print("Listening for UDP packets on port ");
  Serial.println(localPort);

  steerServo.attach(D1); 
  throttleESC.attach(D2); 

  
}

void loop() {
  if (!(Udp.parsePacket() == PAYLOAD_PACKET_SIZE)) {
    // Serial.println("either no packet or packet is not conformed packet size");
    return;
  }

  Udp.read(incomingPacket, PAYLOAD_PACKET_SIZE);
  RC_DRIVE* payload = (RC_DRIVE*)incomingPacket;

    //   Serial.println("Magic: " + String(payload->magic) +
    //                  ", Steer: " + String(payload->steer) +
    //                  ", Throttle: " + String(payload->throttle));

  if (payload->magic != 255) {
    return;
    // Serial.println("magic number is not what it is exspected to be (255) magic number: " + String(payload->magic));
  }

  steerServo.write(payload->steer);
  throttleESC.writeMicroseconds(payload->throttle);
}
