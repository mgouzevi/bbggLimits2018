#!/bin/bash

DIR=$1

OUT=${DIR}/Node_SM/SignalShapes

mkdir ${OUT}

for icat in {0..5}
do
    python scripts/MakeSigPlot.py -p ${OUT} -w ${DIR}/Node_SM/ws_hhbbgg.HH.sig.mH125_13TeV.root -c ${icat} -o "mjj,mgg" -l 35.9 -a "#font[61]{pp#rightarrowHH#rightarrowb#bar{b}#gamma#gamma}|SM-like HH|Low mass region" -b 24,160 -d 1

done


