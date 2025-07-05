git fetch --prune && git branch -vv | grep ': gone]' | awk '{print $1}' | xargs -n 1 git branch -d
