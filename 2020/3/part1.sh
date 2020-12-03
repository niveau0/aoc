#!/bin/bash

DATA=(`cat puzzle.txt`)
LINELEN=${#DATA}
X=0
Y=0
COUNT=0
for i in ${!DATA[@]}; do 
  if [[ $Y < 1 ]] 
  then
    Y=$(($Y+1))
    #echo "skip first"
    continue
  fi
  Y=$(($Y+1))
  X=$(( ($X+3) % $LINELEN))
  LINE=${DATA[$i]}
  #echo $X $LINE 
  FIELD=${DATA[$i]:$X:1}
  #echo $FIELD

  if [ "$FIELD" == "#" ]
  then 
    COUNT=$(($COUNT+1))
  fi
done

echo $COUNT