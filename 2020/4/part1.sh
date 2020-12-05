#!/bin/bash
VALID=0
INVALID=0

REQ=(byr iyr eyr hgt hcl ecl pid)

while read LINE; do
  if [[ -z $LINE ]]; then
    echo PW ${PW[@]}

    FOUNDREQ=0
    for i in ${PW[@]}; do
        for j in ${REQ[@]}; do
        if [[ $i == $j* ]]; then
            FOUNDREQ=$(($FOUNDREQ+1))
            break   
        fi
        done
    done

    if [ $FOUNDREQ == ${#REQ[@]} ]
    then
        echo VALID ${PW[@]}
        VALID=$(($VALID+1))
    else
        INVALID=$(($INVALID+1))
    fi

    PW=()

    continue
  fi

  PW=(${PW[@]} ${LINE[@]})

done <puzzle.txt

echo "Valid $VALID"
echo "Invalid $INVALID"
