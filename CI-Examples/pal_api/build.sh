#!/bin/bash
cc server.c comm_lib.c -o server
cc client.c comm_lib.c -o client
