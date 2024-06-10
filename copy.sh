#!/bin/bash

# Create the /tmp/programs directory if it doesn't exist
mkdir -p /tmp/programs

# Copy programs to /tmp and log the action
cp -a /home/appuser/programs/* /tmp/programs

# Set permissions for the /tmp/programs directory and log the action
echo "Setting permissions for /tmp/programs"
chmod -R 755 /tmp/programs

# Start the server using the prebuilt binary and log the action
/home/appuser/bootstrap
