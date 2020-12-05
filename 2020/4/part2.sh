#!/bin/bash
VALID=0
INVALID=0

REQ=(byr iyr eyr hgt hcl ecl pid)

REGEX="^((byr:((19[2-9][0-9])|(200[0-2])))|(iyr:((201[0-9])|2020))|(eyr:((202[0-9])|2030))|(hgt:((1(([5-8][0-9])|9[0-3])cm)|((59|(6[0-9])|(7[0-6]))in)))|(hcl:#[0-9a-fA-F]{6})|(ecl:(amb|blu|brn|gry|grn|hzl|oth))|(pid:[0-9]{9}))$"
#
while read LINE; do
  if [[ -z $LINE ]]; then
    #echo PW ${PW[@]}

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
        for i in ${PW[@]}; do
            if ! [[ $i == "cid"* ]] && ! [[ $i =~ $REGEX ]]; then
                echo Bad $i
                FOUNDREQ=$(($FOUNDREQ-1))
                break
            fi
        done
        if [ $FOUNDREQ == ${#REQ[@]} ]; then
            #echo VALID ${PW[@]}
            VALID=$(($VALID+1))
        else
            INVALID=$(($INVALID+1))
        fi
    else
        INVALID=$(($INVALID+1))
    fi

    PW=()

    continue
  fi

  PW=(${PW[@]} ${LINE[@]})

done <$1

echo "Valid $VALID"
echo "Invalid $INVALID"
