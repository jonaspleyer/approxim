OLD_VERSION='approxim'
NEW_VERSION='approxim'

for file in $(grep -lr $OLD_VERSION --exclude-dir target *); do
    sed -i "s/$OLD_VERSION/$NEW_VERSION/" $file
done
