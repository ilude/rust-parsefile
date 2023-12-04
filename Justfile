
setup:
  git submodule update --init --recursive 

edot:
  code ~/.dotfiles

usm: # update git submodules
  @grep "submodule" .gitmodules | awk '{gsub(/"/, "", $2); gsub(/]/, "", $2);  print $2}' | xargs git add 
  @git status -s 
  @git commit -m "update submodules"
  git pull --rebase --no-recurse-submodules --autostash
  git submodule update --init --recursive
  git push
