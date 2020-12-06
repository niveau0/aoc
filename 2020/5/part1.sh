#!/bin/bash
MAX=0

while read LINE; do
    echo $LINE
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

    echo Row: $ROW
    echo Column: $COLUMN
    echo Seat: $SEAT
    
    if (( $SEAT > $MAX )); then
        MAX=$SEAT
    fi 
done <$1

echo Max $MAX
