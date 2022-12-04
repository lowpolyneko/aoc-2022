#!/bin/sh

PT1_TOTAL=0
PT2_TOTAL=0

pt1_valid() {
    echo "PT1"
    PT1_TOTAL=$((PT1_TOTAL+1))
}

pt2_valid() {
    echo "PT2"
    PT2_TOTAL=$((PT2_TOTAL+1))
}

while IFS=',' read -r L1 L2
do
    L1_START="${L1%-*}"
    L1_END="${L1#*-}"
    L2_START="${L2%-*}"
    L2_END="${L2#*-}"

    echo "$L1_START-$L1_END,$L2_START-$L2_END"
    if [ "${L1_START}" -ge "${L2_START}" ] && [ "${L1_END}" -le "${L2_END}" ]
    then
        pt1_valid
    elif [ "${L1_START}" -le "${L2_START}" ] && [ "${L1_END}" -ge "${L2_END}" ]
    then
        pt1_valid
    fi

    if [ "${L2_START}" -le "${L1_START}" ] && [ "${L1_START}" -le "${L2_END}" ]
    then
        pt2_valid
    elif [ "${L2_START}" -le "${L1_END}" ] && [ "${L1_END}" -le "${L2_END}" ]
    then
        pt2_valid
    elif [ "${L1_START}" -le "${L2_START}" ] && [ "${L2_START}" -le "${L1_END}" ]
    then
        pt2_valid
    elif [ "${L1_START}" -le "${L2_END}" ] && [ "${L2_END}" -le "${L1_END}" ]
    then
        pt2_valid
    fi
done

echo "Pt1: ${PT1_TOTAL} Pt2: ${PT2_TOTAL}"