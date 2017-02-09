# LITE

## What is this?
This is a session-layer for a realtime client-server application.


This library has similar constraints that GafferUDP has -- the ratio of packets transfered between one party and another must never be greater than 32:1, or packets may appear to be dropped by the other party.
