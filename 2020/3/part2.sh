#!/bin/bash

DATA=(`cat puzzle.txt`)
LINELEN=${#DATA}

function run {
  RIGHT=$1
  DOWN=$2
  X=0
  Y=0
  COUNT=0
  for i in ${!DATA[@]}; do 
    SKIP=$(($Y % $DOWN))
    if [[ $Y < 1 ]] || [ $SKIP != 0 ] 
    then
      Y=$(($Y+1))
      #echo "skip"
      continue
    fi
    Y=$(($Y+1))
    X=$(( ($X+$RIGHT) % $LINELEN))
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
}

A=$(run 1 1)
B=$(run 3 1)
C=$(run 5 1)
D=$(run 7 1)
E=$(run 1 2)
echo $(($A*$B*$C*$D*$E))
