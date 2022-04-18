#!/bin/bash
cc server.c comm_lib.c -o server -lpthread
cc client.c comm_lib.c -o client
cc payload_receiver.c comm_lib.c -o payload_receiver
cc helloworld.c -o helloworld
