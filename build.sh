# create list of build filenames for matching later
files=()
for file in "$PWD/build"/*; do
  if [ -f "$file" ]; then
    files+="${file##*/}"
  fi
done

# copy build files to project root
cp -R "$PWD"/build/. "$PWD"/

# run build
python ./setup.py develop

# remove all build files from project root
for file in "$PWD"/*; do
  if [[ "${files[*]}" =~ ${file##*/} ]]; then
    rm "$file"
  fi
done

# wait for user input to close
read -n 1 -s