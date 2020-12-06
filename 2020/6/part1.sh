#!/bin/bash

function ascii {
  LC_CTYPE=C printf '%d' "'$1"
}

SUM=0
BITS=0
while read LINE; do
  if [[ -z $LINE ]]; then
    for (( i=0; i<26; i++ )); do
      BIT=$(( $BITS & (1 << $i) ))
      if (( $BIT != 0 )); then
        SUM=$(( $SUM + 1 ))
      fi
    done
    BITS=0
    continue
  fi

  for (( i=0; i<${#LINE}; i++ )); do
    LETTER=${LINE:$i:1}
    BIT=$(( $(ascii $LETTER) - 97 ))
    #echo $LETTER $BIT
    BITS=$(( $BITS | (1 << $BIT) ))
  done
  #echo "obase=2;$BITS" | bc

done <$1

echo Sum: $SUM 
