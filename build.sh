files=()
for file in "$PWD/build"/*; do
  if [ -f "$file" ]; then
    files+="${file##*/}"
  fi
done
cp -R $PWD/build/. $PWD/

python ./setup.py develop

for file in "$PWD"/*; do
  if [[ "${files[*]}" =~ ${file##*/} ]]; then
    rm $file
  fi
done

read -n 1 -s