this could be improved with

HashMap<usize, Symmetry> over Vec<(usize,Symmetry)>

early exits when we reach Symmetry::Asymmetric in iteration; try_fold rather than map().sum()?