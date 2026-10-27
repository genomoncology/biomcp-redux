# The page shows show.sh. Any absolute path to the program, a symlink or its target, becomes an example path.
bash show.sh | sed -E 's#/[^" ]*/biomcp([" ]|$)#/home/you/.local/bin/biomcp\1#g'
