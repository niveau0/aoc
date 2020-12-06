#!/bin/bash

FILE=$1

function find_seat {
    SEARCH=$1
    while read LINE; do
        ROW=127
        for (( i=0; i<7; i++ )); do
            LETTER=${LINE:$i:1}
            BIT=$(( 6 - $i )) 
    #        echo $(( (1 << $BIT) )) $LETTER
            if [[ $LETTER == "F" ]]; then
                ROW=$(( $ROW ^ ((1 << $BIT) ) ))
            fi
        done
        COLUMN=7
        for (( i=0; i<3; i++ )); do
            LETTER=${LINE:$(($i + 7)):1}
            BIT=$(( 2 - $i )) 
    #        echo $(( (1 << $BIT) )) $LETTER
            if [[ $LETTER == "L" ]]; then
                COLUMN=$(( $COLUMN ^ ((1 << $BIT) ) ))
            fi
        done 
        SEAT=$(( $ROW * 8 + $COLUMN ))
        if (( $SEAT == $SEARCH )); then
            FOUND=1
            break
        fi
    done <$FILE
}

for (( j=0; j<1024; j++ )); do
    FOUND=0

    find_seat $j
    if (( $FOUND == 0 )); then
        echo Missing $j
    else 
        echo Found $j
    fi
done

