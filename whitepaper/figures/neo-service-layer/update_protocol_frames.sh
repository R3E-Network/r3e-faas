#!/bin/bash

# Script to update protocol files with consistent tcolorbox frames
# This script adds tcolorbox frames to protocol files in the neo-service-layer directory

# Function to process a file
process_file() {
    local file=$1
    echo "Processing $file..."
    
    # Create a temporary file
    local temp_file="${file}.tmp"
    
    # Check if the file contains a protocol environment
    if grep -q "\\begin{protocol}" "$file"; then
        # Add tcolorbox frame around the protocol environment
        awk '
        BEGIN { in_protocol = 0; protocol_title = ""; }
        /\\begin{protocol}{.*}/ {
            if (!in_protocol) {
                in_protocol = 1;
                match($0, /\\begin{protocol}{([^}]*)}/, arr);
                protocol_title = arr[1];
                print "\\begin{tcolorbox}[";
                print "    enhanced,";
                print "    colback=blue!5!white,";
                print "    colframe=blue!75!black,";
                print "    arc=5mm,";
                print "    boxrule=1.5pt,";
                print "    title=" protocol_title ",";
                print "    fonttitle=\\bfseries,";
                print "    coltitle=white,";
                print "    attach boxed title to top left={yshift=-2mm, xshift=5mm},";
                print "    boxed title style={colback=blue!75!black, rounded corners},";
                print "    shadow={2mm}{-2mm}{0mm}{black!50},";
                print "    drop fuzzy shadow";
                print "]";
            }
            print $0;
        }
        /\\end{protocol}/ {
            print $0;
            if (in_protocol) {
                in_protocol = 0;
                print "\\end{tcolorbox}";
            }
        }
        !/\\begin{protocol}{.*}/ && !/\\end{protocol}/ {
            print $0;
        }
        ' "$file" > "$temp_file"
        
        # Replace the original file with the temporary file
        mv "$temp_file" "$file"
    else
        echo "No protocol environment found in $file, skipping."
    fi
}

# Process all protocol files in the directory
for file in *_protocol.tex; do
    if [ -f "$file" ]; then
        process_file "$file"
    fi
done

echo "All protocol files have been updated with tcolorbox frames."
