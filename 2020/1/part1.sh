#!/bin/bash
DATA=(cat data/puzzle.txt | sed  's/$/ /' | tr -d '\n');for i in ${!DATA[@]}; do for j in ${!DATA[@]}; do s=$((${DATA[$i]}+${DATA[$j]})); echo $i $j ${DATA[$i]}+${DATA[$j]}=$s; if [ $s -eq "2020" ]; then break 2; fi;  done; done;
