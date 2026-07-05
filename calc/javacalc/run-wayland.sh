#!/bin/bash

# Script to run the Java calculator on Wayland
# Set environment variables for Wayland compatibility

export JAVA_TOOL_OPTIONS="-Dawt.useSystemAAFontSettings=lcd -Dsun.java2d.xembedHack=true"

if [ -f "target/java-calculator-1.0.0.jar" ]; then
    java -jar target/java-calculator-1.0.0.jar
else
    java -cp target/classes:src/main/java com.javacalc.Main
fi