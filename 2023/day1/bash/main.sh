#!/usr/bin/env bash 

set -e 

counter=0

while read line
do
    echo $line
    if [[ $line =~ ^[^0-9]*([0-9]) ]]; then
        first_digit="${BASH_REMATCH[1]}"
    fi
    if [[ $line =~ ([0-9])[^0-9]*$ ]]; then
        second_digit="${BASH_REMATCH[1]}"
    fi
    combined="$first_digit$second_digit"
    echo $combined
    counter=$(($counter + $combined))
done < <(cat $1; echo)

echo $counter
