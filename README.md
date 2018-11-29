This version of the code is designed for the projections

# How to

* Get latest combine tools ([link](https://cms-hcomb.gitbooks.io/combine/content/part1/#for-end-users-that-dont-need-to-commit-or-do-any-development)):

```
export SCRAM_ARCH=slc6_amd64_gcc530
cmsrel CMSSW_8_1_0
cd CMSSW_8_1_0/src 
cmsenv
git clone https://github.com/cms-analysis/HiggsAnalysis-CombinedLimit.git HiggsAnalysis/CombinedLimit
cd HiggsAnalysis/CombinedLimit
git fetch origin
git checkout v7.0.9
scramv1 b clean; scramv1 b
```
Get this repository code:
```
cd ../
git clone git@github.com:mgouzevi/bbggLimits2018.git
cd bbggLimits2018
scramv1 b
```

## Fits and limits
* Run the fits and limits on the provided LT:

```
./runLimit.py -f conf_default.json --node=SM -o LIMS_OutDir
sh SmartScripts/Analyzer.sh LIMS_OutDir
sh SmartScripts/ForApproval_MakeSMHHSignalFits_UPGRADE.sh LIMS_OutDir
sh SmartScripts/ForApproval_MakeSMHHFullBkgPlots_UPGRADE.sh LIMS_OutDir
```  
The process may take a while to complete, especially when running with many categories.  
The config file `conf_default.json` can be edited to provide needed parameters. Some of them are:  
```
 LTDIR: location of the input Limit Trees (expected to be in the local diractory, after running previous step)
 ncat: number of categories. This should much the number of categories produced in limit tries (currently, should be 4 or 12)
 fitStrategy: 2 - for 2D fit of (mgg, mjj); 1 - for 1D fit of mgg, in which case a cut is set to 100<mjj<150 somewhere in runLimit.py script.
```

The results of the limit will be in `LIMS_OutDir/Node_SM/result_1.log`. In case of problems,
the logfile _mainLog_data-time.log[.bbgg2D]_ can be useful
  

